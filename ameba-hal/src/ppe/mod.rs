//! RTL8721F PPE (Pixel Processing Engine) driver — idiomatic Rust, no_std, PAC-based.
//!
//! # Hardware summary
//! - Base address : 0x4012_6000
//! - Input layers : 3 (layer1 supports scale/rotation; layer2/3 translation-only)
//! - Result layer : 1 (blending output or XOR destination)
//! - Coordinate-transform matrix: 2×3 fixed-point Q4 (value × 16), stored as i16
//!
//! # Skipped functions
//! `PPE_XOR_init` calls `InterruptRegister` which is absent from `ameba-rom-sys`
//! and is therefore omitted.  The caller must register the ISR externally and
//! invoke `Ppe::set_xor_callback` / `Ppe::mask_int_config` to wire the callback.

use bitflags::bitflags;
use core::cell::UnsafeCell;
use rtl8721f_pac::{PpeReg, ppe_reg};

// ────────────────────────────────────────────────────────────────────────────
// mod sys — ROM / external C bindings
// None required: all operations are direct PAC register accesses.
// Kept as an explicit placeholder so the crate boundary is obvious.
// ────────────────────────────────────────────────────────────────────────────
mod sys {}

// ────────────────────────────────────────────────────────────────────────────
// Hardware constants
// ────────────────────────────────────────────────────────────────────────────

/// PSRAM address window: sources in [0x6000_0000, 0x8000_0000) require the
/// AXI read-buffer path (AXCACHE=0x6, AXSIZE=0x2, MAX_AXLEN_LOG=0).
const PSRAM_BASE: u32 = 0x6000_0000; // SDK: hal_platform.h : PSRAM_BASE
const PSRAM_END: u32 = 0x8000_0000; // SDK: hal_platform.h : PSRAM_END

// ────────────────────────────────────────────────────────────────────────────
// Public enumerations
// ────────────────────────────────────────────────────────────────────────────

/// PPE run-state values written to GLB_STATUS.RUN_STATUS.
///
/// | Value | Meaning            |
/// |-------|--------------------|
/// | 0x0   | Disabled           |
/// | 0x1   | Running            |
/// | 0x2   | Inactive (no outst.) |
/// | 0x3   | Suspend (w/ outst.) |
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum RunState {
    Disable = 0x0,
    Running = 0x1,
    Inactive = 0x2,
    Suspend = 0x3,
}

/// PPE pixel format codes.  Values match the hardware `PPE_PIXEL_FORMAT` enum
/// defined in `ameba_ppe.h`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PixelFormat {
    Abgr8888 = 0x00,
    Argb8888 = 0x01,
    Xbgr8888 = 0x02,
    Xrgb8888 = 0x03,
    Bgra8888 = 0x04,
    Rgba8888 = 0x05,
    Bgrx8888 = 0x06,
    Rgbx8888 = 0x07,
    Abgr4444 = 0x08,
    Argb4444 = 0x09,
    Xbgr4444 = 0x0a,
    Xrgb4444 = 0x0b,
    Bgra4444 = 0x0c,
    Rgba4444 = 0x0d,
    Bgrx4444 = 0x0e,
    Rgbx4444 = 0x0f,
    Abgr2222 = 0x10,
    Argb2222 = 0x11,
    Xbgr2222 = 0x12,
    Xrgb2222 = 0x13,
    Bgra2222 = 0x14,
    Rgba2222 = 0x15,
    Bgrx2222 = 0x16,
    Rgbx2222 = 0x17,
    Abgr8565 = 0x18,
    Argb8565 = 0x19,
    Xbgr8565 = 0x1a,
    Xrgb8565 = 0x1b,
    Bgra5658 = 0x1c,
    Rgba5658 = 0x1d,
    Bgrx5658 = 0x1e,
    Rgbx5658 = 0x1f,
    Abgr1555 = 0x20,
    Argb1555 = 0x21,
    Xbgr1555 = 0x22,
    Xrgb1555 = 0x23,
    Bgra5551 = 0x24,
    Rgba5551 = 0x25,
    Bgrx5551 = 0x26,
    Rgbx5551 = 0x27,
    Bgr888 = 0x28,
    Rgb888 = 0x29,
    Bgr565 = 0x2a,
    Rgb565 = 0x2b,
    A8 = 0x2c,
    X8 = 0x2d,
    Abgr8666 = 0x34,
    Argb8666 = 0x35,
    Xbgr8666 = 0x36,
    Xrgb8666 = 0x37,
    Bgra6668 = 0x38,
    Rgba6668 = 0x39,
    Bgrx6668 = 0x3a,
    Rgbx6668 = 0x3b,
    Bgr565S = 0x3c,
    Rgb565S = 0x3d,
}

/// Input-layer pixel source.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PixelSource {
    /// Pixels come from the constant-pixel register (solid colour).
    Const,
    /// Pixels are read from memory via DMA.
    Dma,
}

/// Interpolation mode used when the hardware performs scaling.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InterpType {
    NearestNeighbor,
    Bilinear,
}

/// Color-key matching mode.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum KeyMode {
    Disable = 0x0,
    Inside = 0x2,
    Outside = 0x3,
}

/// Background source for the result layer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BgSource {
    /// Use the constant-background register value.
    Const,
    /// Use layer-1 image data as the background.
    Layer1,
}

/// Input-layer index (1-based, matching hardware layer numbering).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum LayerId {
    Layer1 = 1,
    Layer2 = 2,
    Layer3 = 3,
}

// ────────────────────────────────────────────────────────────────────────────
// bitflags — interrupt flags and layer-enable masks
// ────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// PPE interrupt bits — shared by INTR_ST, INTR_CLR, and INTR_MASK.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct IrqFlags: u32 {
        const ALL_OVER   = 1 << 0;  // All frames blend/scale complete
        const FR_OVER    = 1 << 1;  // One frame complete
        const LOAD_OVER  = 1 << 2;  // Layer registers loaded to internal regs
        const LINE_OVER  = 1 << 3;  // Specific line blend/scale complete
        const SUSP_INAC  = 1 << 4;  // All channels inactive after suspend
        const SECURE_ERR = 1 << 5;  // Secure error
        const SET_ERR    = 1 << 6;  // Set error
        const BUS_ERR    = 1 << 7;  // AXI bus error
        const DIV0_ERR   = 1 << 8;  // Division-by-zero error
    }
}

bitflags! {
    /// Layer selection mask used by layer-enable, auto-reload, and linked-list
    /// APIs.  Bit encoding follows the hardware LAY_EN register:
    ///   bit 0 = result layer, bit 1 = input layer1, bit 2 = layer2, bit 3 = layer3.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct LayerMask: u32 {
        const RESULT = 1 << 0;
        const LAYER1 = 1 << 1;
        const LAYER2 = 1 << 2;
        const LAYER3 = 1 << 3;
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Configuration structs
// ────────────────────────────────────────────────────────────────────────────

/// Configuration for one PPE input layer.
///
/// Mirrors `PPE_InputLayer_InitTypeDef` from the C SDK.
#[derive(Clone, Debug)]
pub struct InputLayerConfig {
    /// Source buffer base address.
    pub src_addr: u32,
    /// Picture width in pixels.
    pub pic_width: u16,
    /// Picture height in pixels.
    pub pic_height: u16,
    /// Pixel format.
    pub format: PixelFormat,
    /// Pixel source (DMA or constant).
    pub pic_src: PixelSource,
    /// Interpolation type when scaling.
    pub interp: InterpType,
    /// Color-key matching mode.
    pub key_mode: KeyMode,
    /// Bytes per row in memory (stride). 11-bit field on input layers.
    pub line_len: u16,
    /// Constant ABGR8888 pixel value when `pic_src == Const`.
    pub const_pixel: u32,
    /// Window top-left X coordinate.
    pub win_min_x: u16,
    /// Window top-left Y coordinate.
    pub win_min_y: u16,
    /// Window bottom-right X coordinate.
    pub win_max_x: u16,
    /// Window bottom-right Y coordinate.
    pub win_max_y: u16,
    /// Color-key minimum BGR value (24-bit BGR888).
    pub key_min_bgr: u32,
    /// Color-key maximum BGR value (24-bit BGR888).
    pub key_max_bgr: u32,
    /// Rotation angle in degrees; must be 0, 90, 180, or 270.
    /// Only layer1 supports non-zero angles.
    pub angle: u32,
    /// Horizontal scale factor (1.0 = no scaling).
    pub scale_x: f32,
    /// Vertical scale factor (1.0 = no scaling).
    pub scale_y: f32,
}

impl Default for InputLayerConfig {
    fn default() -> Self {
        Self {
            src_addr: 0,
            pic_width: 0,
            pic_height: 0,
            format: PixelFormat::Rgb888,
            pic_src: PixelSource::Dma,
            interp: InterpType::NearestNeighbor,
            key_mode: KeyMode::Disable,
            line_len: 0,
            const_pixel: 0xFFFF_FFFF,
            win_min_x: 0,
            win_min_y: 0,
            win_max_x: 0,
            win_max_y: 0,
            key_min_bgr: 0,
            key_max_bgr: 0,
            angle: 0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}

/// Configuration for the PPE result layer.
///
/// Mirrors `PPE_ResultLayer_InitTypeDef` from the C SDK.
#[derive(Clone, Debug)]
pub struct ResultLayerConfig {
    /// Destination buffer base address.
    pub src_addr: u32,
    /// Canvas width in pixels.
    pub pic_width: u16,
    /// Canvas height in pixels.
    pub pic_height: u16,
    /// Output pixel format.
    pub format: PixelFormat,
    /// Background data source.
    pub bg_src: BgSource,
    /// Bytes per row in memory (stride). 16-bit field on result layer.
    pub line_len: u16,
    /// Constant background ABGR8888 value when `bg_src == Const`.
    pub const_bg: u32,
    /// Processing block width in pixels (must be > 0, <= canvas width).
    pub blk_width: u16,
    /// Processing block height in pixels (must be > 0, <= canvas height).
    pub blk_height: u16,
    /// Enable XOR mode.
    pub xor_en: bool,
}

impl Default for ResultLayerConfig {
    fn default() -> Self {
        Self {
            src_addr: 0,
            pic_width: 0,
            pic_height: 0,
            format: PixelFormat::Rgb888,
            bg_src: BgSource::Layer1,
            line_len: 0,
            const_bg: 0,
            blk_width: 8,
            blk_height: 8,
            xor_en: false,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Internal matrix math  (pure Rust, f32 — no external crate needed)
// ────────────────────────────────────────────────────────────────────────────

type Mat3 = [[f32; 3]; 3];

/// In-place left-multiply: m = m × t
fn mat_mul(m: &mut Mat3, t: &Mat3) {
    let a = *m;
    for i in 0..3 {
        for j in 0..3 {
            m[i][j] = a[i][0] * t[0][j] + a[i][1] * t[1][j] + a[i][2] * t[2][j];
        }
    }
}

/// Gauss-Jordan in-place inversion of `src` into `dst`.
/// Returns `false` if the matrix is singular.
fn mat_inv(src: &Mat3, dst: &mut Mat3) -> bool {
    let mut aug = [[0f32; 6]; 3];
    for i in 0..3 {
        for j in 0..3 {
            aug[i][j] = src[i][j];
            aug[i][j + 3] = if i == j { 1.0 } else { 0.0 };
        }
    }
    for i in 0..3 {
        if aug[i][i] == 0.0 {
            let mut swap_row = usize::MAX;
            for (offset, row) in aug[(i + 1)..].iter().enumerate() {
                if row[i] != 0.0 {
                    swap_row = i + 1 + offset;
                    break;
                }
            }
            if swap_row == usize::MAX {
                return false;
            }
            aug.swap(i, swap_row);
        }
        let diag = aug[i][i];
        for elem in &mut aug[i] {
            *elem /= diag;
        }
        for j in 0..3 {
            if j != i {
                let factor = aug[j][i];
                let row_i = aug[i];
                for (dst_elem, &src_elem) in aug[j].iter_mut().zip(row_i.iter()) {
                    *dst_elem -= factor * src_elem;
                }
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            dst[i][j] = aug[i][j + 3];
        }
    }
    true
}

/// Compute the inverse coordinate-transform matrix for the given scale/rotation.
///
/// For pure rotations (scale == 1.0 on both axes) the result is filled directly
/// without floating-point division.  Mirrors `PPE_CalulateME` from the C driver.
fn calculate_matrix(
    scale_x: f32,
    scale_y: f32,
    angle: u32,
    width: u16,
    height: u16,
    move_x: u16,
    move_y: u16,
) -> Mat3 {
    let w = width as f32;
    let h = height as f32;

    // Fast path: integer-only rotations when no scaling
    if scale_x == 1.0 && scale_y == 1.0 {
        return match angle {
            0 => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            90 => [[0.0, 1.0, 0.0], [-1.0, 0.0, h - 1.0], [0.0, 0.0, 1.0]],
            180 => [[-1.0, 0.0, w - 1.0], [0.0, -1.0, h - 1.0], [0.0, 0.0, 1.0]],
            270 => [[0.0, -1.0, w - 1.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
            // Unsupported angle — fall back to identity
            _ => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        };
    }

    // General path: scale + rotation + translation, then invert
    let mut moving: Mat3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let scale: Mat3 = [[scale_x, 0.0, 0.0], [0.0, scale_y, 0.0], [0.0, 0.0, 1.0]];
    let mut rotation: Mat3 = [[0.0; 3]; 3];
    rotation[2][2] = 1.0;

    match angle {
        0 => {
            rotation[0][0] = 1.0;
            rotation[0][1] = 0.0;
            rotation[1][0] = 0.0;
            rotation[1][1] = 1.0;
            moving[0][2] = move_x as f32;
            moving[1][2] = move_y as f32;
        }
        90 => {
            rotation[0][0] = 0.0;
            rotation[0][1] = -1.0;
            rotation[1][0] = 1.0;
            rotation[1][1] = 0.0;
            moving[0][2] = h * scale_y - 1.0;
            moving[1][2] = 0.0;
        }
        180 => {
            rotation[0][0] = -1.0;
            rotation[0][1] = 0.0;
            rotation[1][0] = 0.0;
            rotation[1][1] = -1.0;
            moving[0][2] = w * scale_x - 1.0;
            moving[1][2] = h * scale_y - 1.0;
        }
        270 => {
            rotation[0][0] = 0.0;
            rotation[0][1] = 1.0;
            rotation[1][0] = -1.0;
            rotation[1][1] = 0.0;
            moving[0][2] = 0.0;
            moving[1][2] = w * scale_x - 1.0;
        }
        _ => {
            rotation[0][0] = 1.0;
            rotation[1][1] = 1.0;
        }
    }

    mat_mul(&mut moving, &scale);
    mat_mul(&mut moving, &rotation);
    // If inversion fails (singular matrix), return a zero matrix — the hardware
    // will produce a black frame rather than a crash.
    let mut out = [[0f32; 3]; 3];
    mat_inv(&moving, &mut out);
    out
}

// ────────────────────────────────────────────────────────────────────────────
// Callback storage — interrupt-safe via UnsafeCell
// ────────────────────────────────────────────────────────────────────────────

/// Newtype wrapper so we can impl Sync (orphan rule: UnsafeCell<T> is !Sync in core).
struct SyncCell(UnsafeCell<Option<fn()>>);

// SAFETY: single-core Cortex-M; there is no preemption between a masked-IRQ
// write and the subsequent unmask.
unsafe impl Sync for SyncCell {}

/// Global XOR-done callback.  Updated via `Ppe::set_xor_callback`.
///
/// `UnsafeCell` is required because the ISR reads this pointer while normal
/// code may write it.  The caller must mask the interrupt around any write
/// (see `set_xor_callback`).
static XOR_CALLBACK: SyncCell = SyncCell(UnsafeCell::new(None));

// ────────────────────────────────────────────────────────────────────────────
// PPE driver struct
// ────────────────────────────────────────────────────────────────────────────

/// PPE peripheral handle.
///
/// Obtain via `unsafe { Ppe::steal() }`.  All other methods are safe.
pub struct Ppe {
    _private: (),
}

impl Ppe {
    /// Construct a `Ppe` handle by stealing the peripheral singleton.
    ///
    /// # Safety
    /// The caller must ensure no other code concurrently accesses the PPE
    /// peripheral registers for the lifetime of this handle.
    pub unsafe fn steal() -> Self {
        Self { _private: () }
    }

    #[inline(always)]
    fn rb(&self) -> &ppe_reg::RegisterBlock {
        unsafe { &*PpeReg::PTR }
    }

    // ── Global control ────────────────────────────────────────────────────

    /// Set the PPE run state (`RunState::Running` to start, `RunState::Disable`
    /// to stop).
    pub fn cmd(&self, state: RunState) {
        self.rb()
            .glb_status()
            .modify(|_, w| unsafe { w.run_status().bits(state as u8) });
    }

    /// Read the current run-status value from GLB_STATUS.
    pub fn global_state(&self) -> u8 {
        self.rb().glb_status().read().run_status().bits()
    }

    /// Suspend the PPE (keeps outstanding AXI transactions).
    pub fn suspend(&self) {
        self.rb()
            .glb_status()
            .modify(|_, w| unsafe { w.run_status().bits(RunState::Suspend as u8) });
    }

    /// Resume the PPE from suspend state.
    pub fn resume(&self) {
        self.rb()
            .glb_status()
            .modify(|_, w| unsafe { w.run_status().bits(RunState::Running as u8) });
    }

    /// Returns `true` if channel `chn_id` (0-based ARID/AWID index) is
    /// currently inactive (no outstanding transactions).
    pub fn channel_inactive(&self, chn_id: u8) -> bool {
        let inact = self.rb().glb_status().read().chn_inact().bits();
        (inact & (1u32 << chn_id)) != 0
    }

    // ── Layer enable ──────────────────────────────────────────────────────

    /// Enable the specified input layers.  Pass `LayerMask` bits.
    ///
    /// The hardware LAY_EN.INPUT_LAY_EN field occupies bits 1–3 (one bit per
    /// input layer).  `LayerMask::RESULT` (bit 0) is silently ignored because
    /// the result layer has no enable bit.
    pub fn layer_enable(&self, layers: LayerMask) {
        // INPUT_LAY_EN sits at bits 1:3 of LAY_EN; shift the mask right by 1.
        let input_en = ((layers.bits() >> 1) & 0x7) as u8;
        self.rb()
            .lay_en()
            .write(|w| unsafe { w.input_lay_en().bits(input_en) });
    }

    // ── Interrupt management ──────────────────────────────────────────────

    /// Read all interrupt status bits (INTR_ST).
    pub fn all_int_status(&self) -> IrqFlags {
        IrqFlags::from_bits_truncate(self.rb().intr_st().read().bits())
    }

    /// Clear the specified interrupt pending bits (INTR_CLR, write-1-to-clear).
    pub fn clear_int(&self, flags: IrqFlags) {
        // INTR_CLR is write-only in the PAC; write directly via the raw pointer.
        unsafe {
            self.rb().intr_clr().as_ptr().write_volatile(flags.bits());
        }
    }

    /// Enable (`enable = true`) or disable specific interrupt sources in
    /// INTR_MASK.
    pub fn mask_int_config(&self, flags: IrqFlags, enable: bool) {
        self.rb().intr_mask().modify(|r, w| {
            let cur = r.bits();
            let new_val = if enable {
                cur | flags.bits()
            } else {
                cur & !flags.bits()
            };
            unsafe { w.bits(new_val) }
        });
    }

    /// Disable all interrupt sources (write 0 to INTR_MASK).
    pub fn mask_all_int(&self) {
        self.rb().intr_mask().write(|w| unsafe { w.bits(0) });
    }

    // ── Auto-reload ───────────────────────────────────────────────────────

    /// Set which layers participate in auto-reload.
    pub fn auto_reload_set_layers(&self, layers: LayerMask) {
        let bits = (layers.bits() & 0xF) as u8;
        self.rb()
            .reload_cfg()
            .modify(|_, w| unsafe { w.lyr_reload_en().bits(bits) });
    }

    /// Clear all auto-reload layer selections (LYR_RELOAD_EN = 0).
    pub fn auto_reload_clear_all(&self) {
        self.rb()
            .reload_cfg()
            .modify(|_, w| unsafe { w.lyr_reload_en().bits(0) });
    }

    /// Enable or disable the auto-clear-after-loading feature (AUTO_CLR bit).
    pub fn auto_clear_cmd(&self, enable: bool) {
        self.rb()
            .reload_cfg()
            .modify(|_, w| w.auto_clr().bit(enable));
    }

    /// Mark the current register settings as valid for auto-reload (SET_VALID).
    pub fn set_valid(&self) {
        self.rb()
            .reload_cfg()
            .modify(|_, w| w.set_valid().set_bit());
    }

    /// Poll the SET_VALID bit.  Returns `false` when the hardware has consumed
    /// the settings and a new `set_valid()` call is permitted.
    pub fn get_valid(&self) -> bool {
        self.rb().reload_cfg().read().set_valid().bit_is_set()
    }

    // ── Linked-list ───────────────────────────────────────────────────────

    /// Write the linked-list pointer register (LLP).  Must be 4-byte aligned.
    pub fn set_llp(&self, lli: u32) {
        self.rb().llp().write(|w| unsafe { w.bits(lli) });
    }

    /// Set which layers are updated via the linked-list mechanism (LL_CFG).
    pub fn ll_set_layers(&self, layers: LayerMask) {
        let bits = (layers.bits() & 0xF) as u8;
        self.rb()
            .ll_cfg()
            .modify(|_, w| unsafe { w.layer_ll_en().bits(bits) });
    }

    // ── Line index ────────────────────────────────────────────────────────

    /// Set the line-index threshold for the LINE_OVER interrupt.
    pub fn set_line(&self, line: u32) {
        self.rb().line_index().write(|w| unsafe { w.bits(line) });
    }

    // ── Input layer initialisation ────────────────────────────────────────

    /// Initialise an input layer from a configuration struct.
    ///
    /// `id` selects Layer1, Layer2, or Layer3.  Only Layer1 supports non-zero
    /// rotation; a non-zero `angle` on Layer2/Layer3 is silently treated as 0.
    ///
    /// Mirrors `PPE_InitInputLayer` from the C driver.
    pub fn init_input_layer(&self, id: LayerId, cfg: &InputLayerConfig) {
        // Layer2 and Layer3 cannot rotate — force angle to 0.
        let angle = if id == LayerId::Layer1 { cfg.angle } else { 0 };

        let matrix = calculate_matrix(
            cfg.scale_x,
            cfg.scale_y,
            angle,
            cfg.pic_width,
            cfg.pic_height,
            cfg.win_min_x,
            cfg.win_min_y,
        );

        // Convert 2×3 sub-matrix to Q4 fixed-point (multiply by 16, truncate
        // to i32, then reinterpret bits as u16 — matches the C cast chain
        // `(int)(val * 16)` stored into a 16-bit register field).
        let e11 = ((matrix[0][0] * 16.0) as i32) as u16;
        let e12 = ((matrix[0][1] * 16.0) as i32) as u16;
        let e13 = ((matrix[0][2] * 16.0) as i32) as u16;
        let e21 = ((matrix[1][0] * 16.0) as i32) as u16;
        let e22 = ((matrix[1][1] * 16.0) as i32) as u16;
        let e23 = ((matrix[1][2] * 16.0) as i32) as u16;

        let pix_src_bit = cfg.pic_src == PixelSource::Dma;
        let interp_bit = cfg.interp == InterpType::Bilinear;

        macro_rules! init_layer {
            (
                $addr_l:ident,
                $pic_size:ident,
                $pic_cfg:ident,
                $const_pix:ident,
                $bus_cfg:ident,
                $win_min:ident,
                $win_max:ident,
                $key_min:ident,
                $key_max:ident,
                $me11:ident, $me12:ident, $me13:ident,
                $me21:ident, $me22:ident, $me23:ident,
                $f_format:ident, $f_pix_src:ident, $f_interp:ident,
                $f_key_mode:ident, $f_line_len:ident,
                $f_pic_w:ident, $f_pic_h:ident,
                $f_x_min:ident, $f_y_min:ident,
                $f_x_max:ident, $f_y_max:ident,
                $f_bgr_min:ident, $f_bgr_max:ident,
                $f_axcache:ident, $f_axsize:ident, $f_axlen:ident,
                $f_e11:ident, $f_e12:ident, $f_e13:ident,
                $f_e21:ident, $f_e22:ident, $f_e23:ident
            ) => {{
                let rb = self.rb();
                rb.$addr_l().write(|w| unsafe { w.bits(cfg.src_addr) });
                rb.$pic_size().write(|w| unsafe {
                    w.$f_pic_w()
                        .bits(cfg.pic_width)
                        .$f_pic_h()
                        .bits(cfg.pic_height)
                });
                rb.$pic_cfg().write(|w| unsafe {
                    w.$f_format()
                        .bits(cfg.format as u8)
                        .$f_pix_src()
                        .bit(pix_src_bit)
                        .$f_interp()
                        .bit(interp_bit)
                        .$f_key_mode()
                        .bits(cfg.key_mode as u8)
                        .$f_line_len()
                        .bits(cfg.line_len)
                });
                rb.$const_pix()
                    .write(|w| unsafe { w.bits(cfg.const_pixel) });
                // PSRAM path: enable read-buffer via AXCACHE=0x6, AXSIZE=0x2,
                // MAX_AXLEN_LOG=0.
                if cfg.src_addr >= PSRAM_BASE && cfg.src_addr < PSRAM_END {
                    rb.$bus_cfg().modify(|_, w| unsafe {
                        w.$f_axcache()
                            .bits(0x6)
                            .$f_axsize()
                            .bits(0x2)
                            .$f_axlen()
                            .bits(0)
                    });
                }
                rb.$win_min().write(|w| unsafe {
                    w.$f_x_min()
                        .bits(cfg.win_min_x)
                        .$f_y_min()
                        .bits(cfg.win_min_y)
                });
                rb.$win_max().write(|w| unsafe {
                    w.$f_x_max()
                        .bits(cfg.win_max_x)
                        .$f_y_max()
                        .bits(cfg.win_max_y)
                });
                rb.$key_min()
                    .write(|w| unsafe { w.$f_bgr_min().bits(cfg.key_min_bgr & 0x00FF_FFFF) });
                rb.$key_max()
                    .write(|w| unsafe { w.$f_bgr_max().bits(cfg.key_max_bgr & 0x00FF_FFFF) });
                rb.$me11().write(|w| unsafe { w.$f_e11().bits(e11) });
                rb.$me12().write(|w| unsafe { w.$f_e12().bits(e12) });
                rb.$me13().write(|w| unsafe { w.$f_e13().bits(e13) });
                rb.$me21().write(|w| unsafe { w.$f_e21().bits(e21) });
                rb.$me22().write(|w| unsafe { w.$f_e22().bits(e22) });
                rb.$me23().write(|w| unsafe { w.$f_e23().bits(e23) });
            }};
        }

        match id {
            LayerId::Layer1 => init_layer!(
                input_layer1_addr_l,
                input_layer1_pic_size,
                input_layer1_pic_cfg,
                input_layer1_const_pix,
                input_layer1_bus_cfg,
                input_layer1_win_min,
                input_layer1_win_max,
                input_layer1_key_min,
                input_layer1_key_max,
                input_layer1_matrix_e11,
                input_layer1_matrix_e12,
                input_layer1_matrix_e13,
                input_layer1_matrix_e21,
                input_layer1_matrix_e22,
                input_layer1_matrix_e23,
                // field names
                input_layer1_format,
                input_layer1_pix_src,
                input_layer1_interp,
                input_layer1_key_mode,
                input_layer1_line_len,
                input_layer1_pic_width,
                input_layer1_pic_height,
                input_layer1_x_min,
                input_layer1_y_min,
                input_layer1_x_max,
                input_layer1_y_max,
                input_layer1_bgr_min,
                input_layer1_bgr_max,
                input_layer1_axcache,
                input_layer1_axsize,
                input_layer1_max_axlen_log,
                input_layer1_matrix_e11,
                input_layer1_matrix_e12,
                input_layer1_matrix_e13,
                input_layer1_matrix_e21,
                input_layer1_matrix_e22,
                input_layer1_matrix_e23
            ),
            LayerId::Layer2 => init_layer!(
                input_layer2_addr_l,
                input_layer2_pic_size,
                input_layer2_pic_cfg,
                input_layer2_const_pix,
                input_layer2_bus_cfg,
                input_layer2_win_min,
                input_layer2_win_max,
                input_layer2_key_min,
                input_layer2_key_max,
                input_layer2_matrix_e11,
                input_layer2_matrix_e12,
                input_layer2_matrix_e13,
                input_layer2_matrix_e21,
                input_layer2_matrix_e22,
                input_layer2_matrix_e23,
                input_layer2_format,
                input_layer2_pix_src,
                input_layer2_interp,
                input_layer2_key_mode,
                input_layer2_line_len,
                input_layer2_pic_width,
                input_layer2_pic_height,
                input_layer2_x_min,
                input_layer2_y_min,
                input_layer2_x_max,
                input_layer2_y_max,
                input_layer2_bgr_min,
                input_layer2_bgr_max,
                input_layer2_axcache,
                input_layer2_axsize,
                input_layer2_max_axlen_log,
                input_layer2_matrix_e11,
                input_layer2_matrix_e12,
                input_layer2_matrix_e13,
                input_layer2_matrix_e21,
                input_layer2_matrix_e22,
                input_layer2_matrix_e23
            ),
            LayerId::Layer3 => init_layer!(
                input_layer3_addr_l,
                input_layer3_pic_size,
                input_layer3_pic_cfg,
                input_layer3_const_pix,
                input_layer3_bus_cfg,
                input_layer3_win_min,
                input_layer3_win_max,
                input_layer3_key_min,
                input_layer3_key_max,
                input_layer3_matrix_e11,
                input_layer3_matrix_e12,
                input_layer3_matrix_e13,
                input_layer3_matrix_e21,
                input_layer3_matrix_e22,
                input_layer3_matrix_e23,
                input_layer3_format,
                input_layer3_pix_src,
                input_layer3_interp,
                input_layer3_key_mode,
                input_layer3_line_len,
                input_layer3_pic_width,
                input_layer3_pic_height,
                input_layer3_x_min,
                input_layer3_y_min,
                input_layer3_x_max,
                input_layer3_y_max,
                input_layer3_bgr_min,
                input_layer3_bgr_max,
                input_layer3_axcache,
                input_layer3_axsize,
                input_layer3_max_axlen_log,
                input_layer3_matrix_e11,
                input_layer3_matrix_e12,
                input_layer3_matrix_e13,
                input_layer3_matrix_e21,
                input_layer3_matrix_e22,
                input_layer3_matrix_e23
            ),
        }
    }

    // ── Result layer initialisation ───────────────────────────────────────

    /// Initialise the result layer from a configuration struct.
    ///
    /// Mirrors `PPE_InitResultLayer` from the C driver.
    pub fn init_result_layer(&self, cfg: &ResultLayerConfig) {
        let rb = self.rb();

        // XOR enable/disable
        rb.xor_ctrl().modify(|_, w| w.xor().bit(cfg.xor_en));

        rb.result_layer_addr_l()
            .write(|w| unsafe { w.bits(cfg.src_addr) });

        rb.result_layer_pic_size().write(|w| unsafe {
            w.result_layer_pic_width()
                .bits(cfg.pic_width)
                .result_layer_pic_height()
                .bits(cfg.pic_height)
        });

        // bg_src: 0 = CONST, 1 = Layer1
        let bg_src_bit = cfg.bg_src == BgSource::Layer1;
        rb.result_layer_pic_cfg().write(|w| unsafe {
            w.result_layer_format()
                .bits(cfg.format as u8)
                .result_layer_bg_src()
                .bit(bg_src_bit)
                .result_layer_line_len()
                .bits(cfg.line_len)
        });

        rb.result_layer_const_bg()
            .write(|w| unsafe { w.bits(cfg.const_bg) });

        rb.result_layer_blk_size().write(|w| unsafe {
            w.result_layer_blk_width()
                .bits(cfg.blk_width)
                .result_layer_blk_height()
                .bits(cfg.blk_height)
        });
    }

    // ── XOR high-level helpers ────────────────────────────────────────────

    /// Start an XOR operation.
    ///
    /// `src1`, `src2`, `src3` are the three input buffer addresses; pass
    /// `src3 = 0` to use only layers 1 and 2.  `dst` is the output buffer.
    ///
    /// Caller must have already called `init_input_layer` and
    /// `init_result_layer` to configure geometry and format.
    ///
    /// Mirrors `PPE_XOR_process` from the C driver.
    pub fn xor_process(&self, src1: u32, src2: u32, src3: u32, dst: u32) {
        let rb = self.rb();

        rb.input_layer1_addr_l().write(|w| unsafe { w.bits(src1) });
        rb.input_layer2_addr_l().write(|w| unsafe { w.bits(src2) });
        rb.input_layer3_addr_l().write(|w| unsafe { w.bits(src3) });
        rb.result_layer_addr_l().write(|w| unsafe { w.bits(dst) });

        // Configure PSRAM read-buffer path for any PSRAM sources
        if (PSRAM_BASE..PSRAM_END).contains(&src1) {
            rb.input_layer1_bus_cfg().modify(|_, w| unsafe {
                w.input_layer1_axcache()
                    .bits(0x6)
                    .input_layer1_axsize()
                    .bits(0x2)
                    .input_layer1_max_axlen_log()
                    .bits(0)
            });
        }
        if (PSRAM_BASE..PSRAM_END).contains(&src2) {
            rb.input_layer2_bus_cfg().modify(|_, w| unsafe {
                w.input_layer2_axcache()
                    .bits(0x6)
                    .input_layer2_axsize()
                    .bits(0x2)
                    .input_layer2_max_axlen_log()
                    .bits(0)
            });
        }
        if (PSRAM_BASE..PSRAM_END).contains(&src3) {
            rb.input_layer3_bus_cfg().modify(|_, w| unsafe {
                w.input_layer3_axcache()
                    .bits(0x6)
                    .input_layer3_axsize()
                    .bits(0x2)
                    .input_layer3_max_axlen_log()
                    .bits(0)
            });
        }

        let layers = if src3 == 0 {
            LayerMask::LAYER1 | LayerMask::LAYER2
        } else {
            LayerMask::LAYER1 | LayerMask::LAYER2 | LayerMask::LAYER3
        };
        self.layer_enable(layers);
        self.cmd(RunState::Running);
    }

    /// Install or remove the XOR-done callback.
    ///
    /// - `Some(f)`: stores `f` and unmasks `IrqFlags::ALL_OVER`.
    /// - `None`   : clears the callback and masks `IrqFlags::ALL_OVER`.
    ///
    /// The hardware ISR must call `Ppe::xor_isr_handler()` to dispatch.
    ///
    /// Mirrors `PPE_XOR_Int_handler` from the C driver.
    pub fn set_xor_callback(&self, cb: Option<fn()>) {
        // Mask ALL_OVER while updating the pointer to prevent a torn ISR read.
        self.mask_int_config(IrqFlags::ALL_OVER, false);
        // SAFETY: single-core; interrupt is masked above.
        unsafe {
            *XOR_CALLBACK.0.get() = cb;
        }
        if cb.is_some() {
            self.mask_int_config(IrqFlags::ALL_OVER, true);
        }
    }

    /// Call from the PPE interrupt service routine.
    ///
    /// Checks for `ALL_OVER`, clears the bit, and invokes any registered
    /// callback.  Mirrors `PPE_XOR_INTHandler` from the C driver.
    pub fn xor_isr_handler() {
        // SAFETY: this is called from the ISR; no concurrent normal-mode access
        // to these registers occurs while the interrupt is active.
        let rb = unsafe { &*PpeReg::PTR };
        let raw = rb.intr_st().read().bits();
        if raw & IrqFlags::ALL_OVER.bits() != 0 {
            unsafe {
                rb.intr_clr()
                    .as_ptr()
                    .write_volatile(IrqFlags::ALL_OVER.bits());
            }
            if let Some(f) = unsafe { *XOR_CALLBACK.0.get() } {
                f();
            }
        }
    }
}
