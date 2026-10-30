//! Ameba Green2 LCDC driver — `no_std`, PAC-only register access.
//!
//! Supports both MCU parallel (8/9/16/18/24-bit) and RGB parallel
//! (6/8/16/18/24-bit) interfaces.  DMA and IO-mode operations are
//! provided; clock-tree management (`LCDC_RccEnable`) and GDMA/IPC/
//! rtos-dependent functions are intentionally omitted per the driver
//! conversion rules.

use bitflags::bitflags;
use rtl8721f_pac::{LcdcReg, lcdc_reg::RegisterBlock};

// ---------------------------------------------------------------------------
// Write timing constants (nanoseconds), matching the C source.
// ---------------------------------------------------------------------------

const LCDC_WRITE_CYCLE_PERIOD_NS: u32 = 66; // ~15 MHz write cycle
const LCDC_READ_CYCLE_PERIOD_NS: u32 = 160; // ~6 MHz read cycle

// MCU pre-command DWORD array length (4 DWORDs × 4 bytes = 16 bytes).
const LCDC_CMD_DWORD_MAX: usize = 4;

// ---------------------------------------------------------------------------
// Type-safe enumerations
// ---------------------------------------------------------------------------

/// MCU parallel bus width.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum McuIfWidth {
    Bit8 = 0x0,
    Bit16 = 0x1,
    Bit9 = 0x2,
    Bit18 = 0x3,
    Bit24 = 0x4,
}

/// RGB parallel bus width.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum RgbIfWidth {
    Bit6 = 0x8,
    Bit16 = 0x9,
    Bit18 = 0xA,
    Bit8 = 0xB,
    Bit24 = 0xC,
}

/// Input (frame-buffer) pixel format.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum InputFormat {
    Argb8888 = 0x0,
    Rgb888 = 0x1,
    Rgb565 = 0x2,
    Argb1555 = 0x3,
    Argb4444 = 0x4,
    Rgb666 = 0x5,
    Argb8666 = 0x6,
    Abgr8888 = 0x8,
    Bgr888 = 0x9,
    Bgr565 = 0xA,
}

/// Output (panel) pixel format.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum OutputFormat {
    Rgb888 = 0x0,
    Rgb565 = 0x1,
    Rgb666 = 0x2,
    Bgr888 = 0x8,
    Bgr565 = 0x9,
}

/// MCU RS (D/CX) polarity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum McuRsPolar {
    LowLevelCmd,
    HighLevelCmd,
}

/// MCU WR strobe active edge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum McuWrPolar {
    RisingEdgeFetch,
    FallingEdgeFetch,
}

/// MCU RD strobe active edge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum McuRdPolar {
    RisingEdgeFetch,
    FallingEdgeFetch,
}

/// MCU TE signal polarity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum McuTePolar {
    LowLevelActive,
    HighLevelActive,
}

/// MCU VSYNC polarity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum McuVsyncPolar {
    LowLevelActive,
    HighLevelActive,
}

/// MCU sync mode (IO or DMA).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum McuSyncMode {
    InternalClk = 0,
    Vsync = 1,
    Te = 2,
}

/// RGB DE/EN polarity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RgbEnPolar {
    LowLevelActive,
    HighLevelActive,
}

/// RGB HSYNC polarity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RgbHsPolar {
    LowLevelSync,
    HighLevelSync,
}

/// RGB VSYNC polarity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RgbVsPolar {
    LowLevelSync,
    HighLevelSync,
}

/// RGB DCLK active edge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RgbDclkEdge {
    RisingEdgeFetch,
    FallingEdgeFetch,
}

/// DMA burst size (unit: 64 bytes).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DmaBurstSize {
    X64 = 0,
    X128 = 1,
    X256 = 2,
}

/// DMA underflow output mode.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DmaUnderflowOutMode {
    LastData,
    ErrorData,
}

/// DMA underflow handling option.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DmaUnderflowOpt {
    /// Instantly output error/last data for the rest of the current frame.
    InstantErrorDataCurFrame = 0,
    /// Pause and wait, then output error/last data for the rest of the frame.
    PauseErrorDataCurFrame = 1,
    /// Instantly drop the current line and continue with the next.
    InstantDropLine = 2,
    /// Pause and wait, then drop the current line and continue.
    PauseDropLine = 3,
}

/// Image shift direction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShiftDir {
    Disable,
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

/// MCU run status.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum McuRunStatus {
    IoMode,
    DmaMode,
}

// ---------------------------------------------------------------------------
// bitflags for interrupt enable / status
// ---------------------------------------------------------------------------

bitflags! {
    /// LCDC interrupt sources (shared between enable and status registers).
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct IrqFlags: u32 {
        /// DMA FIFO underflow.
        const DMA_UNDERFLOW = 1 << 0;
        /// LCD refresh frame done.
        const FRAME_DONE    = 1 << 2;
        /// Line interrupt.
        const LINE          = 1 << 3;
        /// MCU IO write/read timeout.
        const IO_TIMEOUT    = 1 << 4;
        /// DMA frame start.
        const FRAME_START   = 1 << 5;
        /// Panel TE input.
        const PANEL_TE      = 1 << 6;
    }
}

// ---------------------------------------------------------------------------
// Configuration structs (builder / init pattern)
// ---------------------------------------------------------------------------

/// Panel pixel dimensions.
#[derive(Clone, Copy, Debug)]
pub struct PanelGeometry {
    pub width: u16,
    pub height: u16,
}

/// MCU interface polarity / timing flags.
#[derive(Clone, Copy, Debug)]
pub struct McuTiming {
    pub rs_polar: McuRsPolar,
    pub wr_polar: McuWrPolar,
    pub rd_polar: McuRdPolar,
    pub te_polar: McuTePolar,
    pub vsync_polar: McuVsyncPolar,
}

impl Default for McuTiming {
    fn default() -> Self {
        Self {
            rs_polar: McuRsPolar::LowLevelCmd,
            wr_polar: McuWrPolar::RisingEdgeFetch,
            rd_polar: McuRdPolar::RisingEdgeFetch,
            te_polar: McuTePolar::LowLevelActive,
            vsync_polar: McuVsyncPolar::LowLevelActive,
        }
    }
}

/// Full MCU interface initialisation configuration.
#[derive(Clone, Copy, Debug)]
pub struct McuConfig {
    pub if_width: McuIfWidth,
    pub geometry: PanelGeometry,
    pub input_fmt: InputFormat,
    pub output_fmt: OutputFormat,
    pub timing: McuTiming,
    /// LCDC system clock in Hz — used to derive write/read dividers.
    pub sys_clk_hz: u32,
}

impl Default for McuConfig {
    fn default() -> Self {
        Self {
            if_width: McuIfWidth::Bit8,
            geometry: PanelGeometry {
                width: 240,
                height: 320,
            },
            input_fmt: InputFormat::Rgb565,
            output_fmt: OutputFormat::Rgb565,
            timing: McuTiming::default(),
            sys_clk_hz: 200_000_000,
        }
    }
}

/// RGB synchronisation timing (in pixel-clocks / lines).
#[derive(Clone, Copy, Debug)]
pub struct RgbSyncTiming {
    pub vsw: u8,
    pub vbp: u8,
    pub vfp: u8,
    pub hsw: u8,
    pub hbp: u8,
    pub hfp: u8,
}

/// RGB signal polarity flags.
#[derive(Clone, Copy, Debug)]
pub struct RgbPolarFlags {
    pub en_polar: RgbEnPolar,
    pub hs_polar: RgbHsPolar,
    pub vs_polar: RgbVsPolar,
    pub dclk_edge: RgbDclkEdge,
}

impl Default for RgbPolarFlags {
    fn default() -> Self {
        Self {
            en_polar: RgbEnPolar::HighLevelActive,
            hs_polar: RgbHsPolar::LowLevelSync,
            vs_polar: RgbVsPolar::LowLevelSync,
            dclk_edge: RgbDclkEdge::FallingEdgeFetch,
        }
    }
}

/// Full RGB interface initialisation configuration.
#[derive(Clone, Copy, Debug)]
pub struct RgbConfig {
    pub if_width: RgbIfWidth,
    pub geometry: PanelGeometry,
    pub input_fmt: InputFormat,
    pub output_fmt: OutputFormat,
    /// Target refresh frequency in Hz.
    pub refresh_freq: u32,
    pub sync: RgbSyncTiming,
    pub polarity: RgbPolarFlags,
    /// LCDC system clock in Hz — used to derive DCLK divider.
    pub sys_clk_hz: u32,
}

impl Default for RgbConfig {
    fn default() -> Self {
        Self {
            if_width: RgbIfWidth::Bit24,
            geometry: PanelGeometry {
                width: 800,
                height: 480,
            },
            input_fmt: InputFormat::Rgb888,
            output_fmt: OutputFormat::Rgb888,
            refresh_freq: 35,
            sync: RgbSyncTiming {
                vsw: 1,
                vbp: 4,
                vfp: 6,
                hsw: 4,
                hbp: 40,
                hfp: 40,
            },
            polarity: RgbPolarFlags::default(),
            sys_clk_hz: 200_000_000,
        }
    }
}

/// MCU DMA mode configuration.
#[derive(Clone, Copy, Debug)]
pub struct McuDmaConfig {
    /// Use TE-based DMA trigger.
    pub te_mode: bool,
    /// Delay from TE signal to first transfer, in WR-pulse units (must be ≥ 5
    /// when `te_mode = true`).
    pub te_delay: u16,
    /// When `te_mode = false`: `true` → trigger-once DMA, `false` → auto DMA.
    pub trigger_dma: bool,
}

impl Default for McuDmaConfig {
    fn default() -> Self {
        Self {
            te_mode: false,
            te_delay: 5,
            trigger_dma: false,
        }
    }
}

/// Advanced DMA image-buffer descriptor (used by the slide/transition helper).
#[derive(Clone, Copy, Debug)]
pub struct DmaImageAdvance {
    pub img_addr_a: u32,
    pub img_addr_b: u32,
    /// Horizontal pixel offset (must be a multiple of 4; 0 = no offset).
    pub ofst_dot_x: u16,
    /// Vertical line offset (0 = no offset).
    pub ofst_line_y: u16,
}

// Sentinel: "this offset dimension is not active".
const SHIFT_OFST_INVALID: u16 = 0;

// ---------------------------------------------------------------------------
// The driver
// ---------------------------------------------------------------------------

/// Idiomatic Rust driver for the Ameba Green2 LCDC peripheral.
///
/// Obtain a handle with [`Lcdc::steal`]; all other public methods are safe.
pub struct Lcdc {
    rb: &'static RegisterBlock,
}

impl Lcdc {
    /// Construct a driver handle by stealing the peripheral pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure exclusive access to the LCDC hardware for the
    /// lifetime of this driver instance.
    pub unsafe fn steal() -> Self {
        unsafe {
            // LCDC_REG is mapped at 0x4010_0000 in rtl8721f-pac.
            let rb = &*LcdcReg::PTR;
            Self { rb }
        }
    }

    // ------------------------------------------------------------------
    // Enable / disable
    // ------------------------------------------------------------------

    /// Enable the LCDC.
    pub fn enable(&self) {
        // Clear instant-disable first, then assert the enable bit.
        self.rb
            .lcdc_ctrl()
            .modify(|_, w| w.lcdc_inst_dis().clear_bit().lcdc_en().set_bit());
    }

    /// Disable the LCDC instantly (resets internal state machine).
    pub fn disable(&self) {
        self.rb
            .lcdc_ctrl()
            .modify(|_, w| w.lcdc_inst_dis().set_bit());
    }

    // ------------------------------------------------------------------
    // MCU interface initialisation
    // ------------------------------------------------------------------

    /// Initialise the LCDC for MCU parallel IO mode.
    ///
    /// Programs interface width, colour formats, plane size, write/read
    /// timing and IO-timeout registers.  The controller is left disabled
    /// after this call; invoke [`enable`](Self::enable) to start operation.
    pub fn mcu_init(&self, cfg: &McuConfig) {
        self.disable();

        let t = &cfg.timing;

        // LCDC_MCU_CFG: polarity bits + IO mode + internal-clock sync.
        self.rb.lcdc_mcu_cfg().modify(|_, w| unsafe {
            w.mcu_io_mode_en()
                .set_bit()
                .mcu_sync_mode()
                .bits(McuSyncMode::InternalClk as u8)
                .rspl()
                .bit(t.rs_polar == McuRsPolar::HighLevelCmd)
                .wrpl()
                .bit(t.wr_polar == McuWrPolar::FallingEdgeFetch)
                .rdpl()
                .bit(t.rd_polar == McuRdPolar::FallingEdgeFetch)
                .tepl()
                .bit(t.te_polar == McuTePolar::HighLevelActive)
                .mcusypl()
                .bit(t.vsync_polar == McuVsyncPolar::HighLevelActive)
        });

        // LCDC_CTRL: interface mode field.
        self.rb
            .lcdc_ctrl()
            .modify(|_, w| unsafe { w.lcdc_if_mode().bits(cfg.if_width as u8) });

        // LCDC_PLANE_SIZE.
        self.rb.lcdc_plane_size().modify(|_, w| unsafe {
            w.imagewidth()
                .bits(cfg.geometry.width)
                .imageheight()
                .bits(cfg.geometry.height)
        });

        // LCDC_MCU_TIMING_CFG: write and read pulse widths.
        // wrdiv = sys_clk_MHz * write_period_ns / 1000 / 2 - 1
        // rddiv = sys_clk_MHz * read_period_ns  / 1000 / 2
        let mhz = cfg.sys_clk_hz / 1_000_000;
        let wrdiv = (mhz * LCDC_WRITE_CYCLE_PERIOD_NS / 1000 / 2).saturating_sub(1);
        let rddiv = mhz * LCDC_READ_CYCLE_PERIOD_NS / 1000 / 2;
        self.rb.lcdc_mcu_timing_cfg().modify(|_, w| unsafe {
            w.wrpulw()
                .bits(wrdiv as u16)
                .rdactw()
                .bits(rddiv as u16)
                .rdinactw()
                .bits(rddiv as u16)
        });

        // LCDC_MCU_IO_TO_CFG: IO-mode timeout counter.
        // Recommended: 16 * WRPULW + RDACTW + RDINACTW
        let timeout = (16 * wrdiv + rddiv + rddiv) as u16;
        self.rb
            .lcdc_mcu_io_to_cfg()
            .modify(|_, w| unsafe { w.io_timeout_num().bits(timeout) });

        // LCDC_IMG_CFG: input colour format.
        self.rb
            .lcdc_img_cfg()
            .modify(|_, w| unsafe { w.img_format().bits(cfg.input_fmt as u8) });

        // LCDC_COLOR_CFG: output colour format.
        self.rb
            .lcdc_color_cfg()
            .modify(|_, w| unsafe { w.color_format().bits(cfg.output_fmt as u8) });
    }

    /// Switch the MCU interface to direct IO (non-DMA) mode.
    pub fn mcu_set_io_mode(&self) {
        self.disable();
        self.rb.lcdc_mcu_cfg().modify(|_, w| unsafe {
            w.mcu_io_mode_en()
                .set_bit()
                .mcu_sync_mode()
                .bits(McuSyncMode::InternalClk as u8)
        });
        self.rb
            .lcdc_dma_mode_cfg()
            .modify(|_, w| w.dma_te_mode().clear_bit());
        self.enable();
    }

    /// Switch the MCU interface to DMA mode and configure the sync source.
    pub fn mcu_set_dma_mode(&self, dma_cfg: &McuDmaConfig) {
        // Clear TE mode first to prevent hang during transition.
        self.rb
            .lcdc_dma_mode_cfg()
            .modify(|_, w| w.dma_te_mode().clear_bit());

        if dma_cfg.te_mode {
            debug_assert!(dma_cfg.te_delay >= 5, "te_delay must be >= 5");
            self.rb.lcdc_mcu_cfg().modify(|_, w| unsafe {
                w.mcu_io_mode_en()
                    .clear_bit()
                    .mcu_sync_mode()
                    .bits(McuSyncMode::Te as u8)
                    .tedelay()
                    .bits(dma_cfg.te_delay.saturating_sub(5))
            });
            self.rb
                .lcdc_dma_mode_cfg()
                .modify(|_, w| w.dma_triger_mode().clear_bit().dma_te_mode().set_bit());
        } else {
            self.rb.lcdc_mcu_cfg().modify(|_, w| unsafe {
                w.mcu_io_mode_en()
                    .clear_bit()
                    .mcu_sync_mode()
                    .bits(McuSyncMode::Vsync as u8)
            });
            self.rb
                .lcdc_dma_mode_cfg()
                .modify(|_, w| w.dma_triger_mode().bit(dma_cfg.trigger_dma));
        }
    }

    // ------------------------------------------------------------------
    // RGB interface initialisation
    // ------------------------------------------------------------------

    /// Initialise the LCDC for RGB parallel (HV-mode) DMA operation.
    pub fn rgb_init(&self, cfg: &RgbConfig) {
        self.disable();

        let s = &cfg.sync;
        let p = &cfg.polarity;

        // LCDC_RGB_CFG: polarity bits + HV mode (0x01).
        self.rb.lcdc_rgb_cfg().modify(|_, w| unsafe {
            w.rgb_sync_mode()
                .bits(0x01)
                .dclkpl()
                .bit(p.dclk_edge == RgbDclkEdge::FallingEdgeFetch)
                .vspl()
                .bit(p.vs_polar == RgbVsPolar::HighLevelSync)
                .hspl()
                .bit(p.hs_polar == RgbHsPolar::HighLevelSync)
                .enpl()
                .bit(p.en_polar == RgbEnPolar::HighLevelActive)
        });

        // LCDC_CTRL: interface mode field.
        self.rb
            .lcdc_ctrl()
            .modify(|_, w| unsafe { w.lcdc_if_mode().bits(cfg.if_width as u8) });

        // LCDC_PLANE_SIZE.
        self.rb.lcdc_plane_size().modify(|_, w| unsafe {
            w.imagewidth()
                .bits(cfg.geometry.width)
                .imageheight()
                .bits(cfg.geometry.height)
        });

        // LCDC_RGB_VSYNC_CFG: vsw/vbp/vfp stored as (value − 1).
        self.rb.lcdc_rgb_vsync_cfg().modify(|_, w| unsafe {
            w.vsw()
                .bits(s.vsw.saturating_sub(1))
                .vbp()
                .bits(s.vbp.saturating_sub(1))
                .vfp()
                .bits(s.vfp.saturating_sub(1))
        });

        // LCDC_RGB_HSYNC_CFG: hsw/hbp/hfp stored as (value − 1).
        self.rb.lcdc_rgb_hsync_cfg().modify(|_, w| unsafe {
            w.hsw()
                .bits(s.hsw.saturating_sub(1))
                .hbp()
                .bits(s.hbp.saturating_sub(1))
                .hfp()
                .bits(s.hfp.saturating_sub(1))
        });

        // LCDC_DMA_MODE_CFG: auto DMA (not trigger mode).
        self.rb
            .lcdc_dma_mode_cfg()
            .modify(|_, w| w.dma_triger_mode().clear_bit());

        // LCDC_RGB_CLK_DIV: derive DCLK divider from target refresh rate.
        // Active horizontal pixels: for 6-bit and 8-bit buses each pixel
        // takes 3 clock cycles; all others take 1 cycle.
        let hblank: u32 = s.hsw as u32 + s.hbp as u32 + s.hfp as u32;
        let active_clks: u32 = match cfg.if_width {
            RgbIfWidth::Bit6 | RgbIfWidth::Bit8 => 3 * cfg.geometry.width as u32,
            _ => cfg.geometry.width as u32,
        };
        let htotal: u32 = hblank + active_clks;
        let vtotal: u32 = cfg.geometry.height as u32 + s.vsw as u32 + s.vbp as u32 + s.vfp as u32;
        let target_dclk = cfg.refresh_freq * htotal * vtotal;
        // Round down to even value; minimum divisor is 4 (PAC: reg = div/2 - 1).
        let mut ckd = cfg.sys_clk_hz / target_dclk.max(1) / 2 * 2;
        if ckd < 4 {
            ckd = 4;
        }
        self.rb
            .lcdc_rgb_clk_div()
            .modify(|_, w| unsafe { w.clkdiv().bits((ckd / 2 - 1) as u16) });

        // LCDC_IMG_CFG / LCDC_COLOR_CFG: colour formats.
        self.rb
            .lcdc_img_cfg()
            .modify(|_, w| unsafe { w.img_format().bits(cfg.input_fmt as u8) });
        self.rb
            .lcdc_color_cfg()
            .modify(|_, w| unsafe { w.color_format().bits(cfg.output_fmt as u8) });
    }

    /// Read the current RGB H/V sync status.
    ///
    /// Returns `(hs_status, vs_status)`.  Values follow the PAC encoding:
    ///   0 = SYNC, 1 = BP, 2 = FP, 3 = ACTIVE.
    pub fn rgb_get_sync_status(&self) -> (u8, u8) {
        let r = self.rb.lcdc_rgb_sync_status().read();
        (r.hsstatus().bits(), r.vsstatus().bits())
    }

    // ------------------------------------------------------------------
    // DMA image address / slide helpers
    // ------------------------------------------------------------------

    /// Set the DMA frame-buffer base address (single-buffer case).
    pub fn dma_set_image_addr(&self, addr_a: u32) {
        self.rb
            .lcdc_img_a_base_addr()
            .modify(|_, w| unsafe { w.img_a_base_addr().bits(addr_a) });
    }

    /// Configure both image-A / image-B addresses and the horizontal /
    /// vertical pixel offsets for the image-slide transition feature.
    ///
    /// Constraints:
    /// - `ofst_dot_x` must be a multiple of 4.
    /// - `ofst_dot_x` and `ofst_line_y` must not both be non-zero.
    pub fn dma_set_image_advance(&self, info: &DmaImageAdvance) {
        debug_assert!(
            info.ofst_dot_x.is_multiple_of(4),
            "ofst_dot_x must be a multiple of 4"
        );
        debug_assert!(
            !(info.ofst_dot_x != 0 && info.ofst_line_y != 0),
            "ofst_dot_x and ofst_line_y cannot both be non-zero"
        );
        self.rb
            .lcdc_img_a_base_addr()
            .modify(|_, w| unsafe { w.img_a_base_addr().bits(info.img_addr_a) });
        self.rb
            .lcdc_img_b_base_addr()
            .modify(|_, w| unsafe { w.img_b_base_addr().bits(info.img_addr_b) });
        self.rb.lcdc_img_slide().modify(|_, w| unsafe {
            w.img_hs_ofst()
                .bits(info.ofst_dot_x)
                .img_vs_ofst()
                .bits(info.ofst_line_y)
        });
    }

    /// Trigger one DMA frame transfer (for trigger mode).
    pub fn mcu_dma_trigger(&self) {
        self.rb
            .lcdc_dma_mode_cfg()
            .modify(|_, w| w.triger_onetime().set_bit());
    }

    /// Configure a directional image slide / transition.
    ///
    /// `buf_current` is the currently displayed frame buffer;
    /// `buf_target` is the incoming frame buffer.
    pub fn dma_image_shift(&self, buf_current: u32, buf_target: u32, dir: ShiftDir) {
        let img_w = self.rb.lcdc_plane_size().read().imagewidth().bits();
        let img_h = self.rb.lcdc_plane_size().read().imageheight().bits();

        let info = match dir {
            ShiftDir::Disable => DmaImageAdvance {
                img_addr_a: buf_current,
                img_addr_b: 0,
                ofst_dot_x: SHIFT_OFST_INVALID,
                ofst_line_y: SHIFT_OFST_INVALID,
            },
            ShiftDir::LeftToRight => DmaImageAdvance {
                img_addr_a: buf_target,
                img_addr_b: buf_current,
                ofst_dot_x: img_w,
                ofst_line_y: SHIFT_OFST_INVALID,
            },
            ShiftDir::RightToLeft => DmaImageAdvance {
                img_addr_a: buf_current,
                img_addr_b: buf_target,
                ofst_dot_x: 0,
                ofst_line_y: SHIFT_OFST_INVALID,
            },
            ShiftDir::TopToBottom => DmaImageAdvance {
                img_addr_a: buf_target,
                img_addr_b: buf_current,
                ofst_dot_x: SHIFT_OFST_INVALID,
                ofst_line_y: img_h,
            },
            ShiftDir::BottomToTop => DmaImageAdvance {
                img_addr_a: buf_current,
                img_addr_b: buf_target,
                ofst_dot_x: SHIFT_OFST_INVALID,
                ofst_line_y: 0,
            },
        };
        self.dma_set_image_advance(&info);
    }

    // ------------------------------------------------------------------
    // MCU IO-mode read / write
    // ------------------------------------------------------------------

    /// Write a data word to the MCU parallel bus (RS=0 → data frame).
    ///
    /// Only the lower 24 bits of `data` are sent.
    pub fn mcu_io_write_data(&self, data: u32) {
        // LCDC_MCU_IO_DATA is at base + 0x6C.
        // Bit 31 = 0 → data.  The PAC maps this register read-only, so we
        // use a raw volatile write (same as the C HAL).
        let addr = (self.rb as *const RegisterBlock as usize) + 0x6C;
        unsafe { core::ptr::write_volatile(addr as *mut u32, data & 0x00FF_FFFF) };
    }

    /// Write a command word to the MCU parallel bus (RS=1 → command frame).
    ///
    /// Only the lower 24 bits of `cmd` are sent.
    pub fn mcu_io_write_cmd(&self, cmd: u32) {
        let addr = (self.rb as *const RegisterBlock as usize) + 0x6C;
        unsafe { core::ptr::write_volatile(addr as *mut u32, (cmd & 0x00FF_FFFF) | (1 << 31)) };
    }

    /// Read a data word from the MCU parallel bus.
    pub fn mcu_io_read_data(&self) -> u32 {
        self.rb.lcdc_mcu_io_data().read().mcu_rw_data().bits()
    }

    /// Return whether the MCU interface is currently running in IO or DMA mode.
    pub fn mcu_get_run_status(&self) -> McuRunStatus {
        if self.rb.lcdc_mcu_cfg().read().mcu_io_mode_run().bit() {
            McuRunStatus::IoMode
        } else {
            McuRunStatus::DmaMode
        }
    }

    // ------------------------------------------------------------------
    // MCU pre-defined commands (sent before each DMA frame)
    // ------------------------------------------------------------------

    /// Programme up to 16 single-byte commands to send before each DMA frame.
    pub fn mcu_set_pre_cmd(&self, cmds: &[u8]) {
        debug_assert!(cmds.len() <= 16, "maximum 16 pre-defined commands");

        // Command bytes are stored at consecutive byte addresses starting at
        // LCDC_MCU_CMD_DWORD0_0 (offset 0xE0 from register-block base).
        let base = (self.rb as *const RegisterBlock as usize) + 0xE0;
        for (i, &byte) in cmds.iter().enumerate() {
            unsafe { core::ptr::write_volatile((base + i) as *mut u8, byte) };
        }

        self.rb
            .lcdc_mcu_cmd_wrnum()
            .modify(|_, w| unsafe { w.mcu_cmd_wrnum().bits(cmds.len() as u8) });
    }

    /// Clear all pre-defined commands and reset the command count to zero.
    pub fn mcu_reset_pre_cmd(&self) {
        let base = (self.rb as *const RegisterBlock as usize) + 0xE0;
        for i in 0..(LCDC_CMD_DWORD_MAX * 4) {
            unsafe { core::ptr::write_volatile((base + i) as *mut u8, 0x00) };
        }
        self.rb
            .lcdc_mcu_cmd_wrnum()
            .modify(|_, w| unsafe { w.mcu_cmd_wrnum().bits(0) });
    }

    // ------------------------------------------------------------------
    // Shadow reload
    // ------------------------------------------------------------------

    /// Request a shadow-register reload during the next vertical-blank period.
    pub fn shadow_reload(&self) {
        self.rb.lcdc_shw_rld_cfg().modify(|_, w| w.vbr().set_bit());
    }

    // ------------------------------------------------------------------
    // Panel size / colour format helpers
    // ------------------------------------------------------------------

    /// Reconfigure the panel pixel dimensions.
    pub fn set_panel_size(&self, width: u16, height: u16) {
        debug_assert!(width < 4095 && height < 4095);
        self.rb
            .lcdc_plane_size()
            .modify(|_, w| unsafe { w.imagewidth().bits(width).imageheight().bits(height) });
    }

    /// Reconfigure the output (panel) colour format.
    pub fn set_output_format(&self, fmt: OutputFormat) {
        self.rb
            .lcdc_color_cfg()
            .modify(|_, w| unsafe { w.color_format().bits(fmt as u8) });
    }

    /// Reconfigure the input (frame-buffer) colour format.
    pub fn set_input_format(&self, fmt: InputFormat) {
        self.rb
            .lcdc_img_cfg()
            .modify(|_, w| unsafe { w.img_format().bits(fmt as u8) });
    }

    // ------------------------------------------------------------------
    // DMA burst / underflow configuration
    // ------------------------------------------------------------------

    /// Configure the AXI DMA burst size.
    pub fn dma_set_burst_size(&self, size: DmaBurstSize) {
        self.rb
            .lcdc_shw_rld_cfg()
            .modify(|_, w| unsafe { w.dma_burst_opt().bits(size as u8) });
    }

    /// Configure the pixel output when a DMA underflow occurs.
    ///
    /// `error_data` must fit in 24 bits and is only used when `out_mode`
    /// is [`DmaUnderflowOutMode::ErrorData`].
    pub fn dma_set_underflow_out(&self, out_mode: DmaUnderflowOutMode, error_data: u32) {
        debug_assert!(error_data <= 0x00FF_FFFF);
        self.rb.lcdc_undflw_cfg().modify(|_, w| unsafe {
            match out_mode {
                DmaUnderflowOutMode::LastData => w.dma_un_mode().clear_bit(),
                DmaUnderflowOutMode::ErrorData => {
                    w.dma_un_mode().set_bit().errout_data().bits(error_data)
                }
            }
        });
    }

    /// Configure the recovery strategy when a DMA underflow occurs.
    ///
    /// `timeout_threshold` (12-bit) is only consulted for the `Pause*` variants.
    pub fn dma_set_underflow_opt(&self, opt: DmaUnderflowOpt, timeout_threshold: u16) {
        debug_assert!(timeout_threshold <= 0x0FFF);
        self.rb
            .lcdc_undflw_cfg()
            .modify(|_, w| unsafe { w.dma_udf_opt().bits(opt as u8) });
        match opt {
            DmaUnderflowOpt::PauseErrorDataCurFrame | DmaUnderflowOpt::PauseDropLine => {
                self.rb
                    .lcdc_dma_mode_cfg()
                    .modify(|_, w| unsafe { w.dma_udf_timeout().bits(timeout_threshold) });
            }
            _ => {}
        }
    }

    // ------------------------------------------------------------------
    // Interrupts
    // ------------------------------------------------------------------

    /// Enable the specified interrupt sources.
    pub fn irq_enable(&self, flags: IrqFlags) {
        self.rb
            .lcdc_irq_en()
            .modify(|r, w| unsafe { w.bits(r.bits() | flags.bits()) });
    }

    /// Disable the specified interrupt sources.
    pub fn irq_disable(&self, flags: IrqFlags) {
        self.rb
            .lcdc_irq_en()
            .modify(|r, w| unsafe { w.bits(r.bits() & !flags.bits()) });
    }

    /// Set the scan line at which the line interrupt fires.
    ///
    /// `line` must be less than the configured panel height.
    pub fn irq_set_line_pos(&self, line: u16) {
        debug_assert!(
            line < self.rb.lcdc_plane_size().read().imageheight().bits(),
            "line must be within panel height"
        );
        self.rb
            .lcdc_line_int_pos()
            .modify(|_, w| unsafe { w.line_int_pos().bits(line) });
    }

    /// Read the masked (enabled) interrupt status register.
    pub fn irq_get_status(&self) -> IrqFlags {
        IrqFlags::from_bits_truncate(self.rb.lcdc_irq_status().read().bits())
    }

    /// Read the raw (unmasked) interrupt status register.
    pub fn irq_get_raw_status(&self) -> IrqFlags {
        IrqFlags::from_bits_truncate(self.rb.lcdc_irq_raw().read().bits())
    }

    /// Clear all pending interrupts.
    pub fn irq_clear_all(&self) {
        // LCDC_IRQ_STATUS is write-1-to-clear and is read-only in the PAC,
        // so we use a raw volatile write.  Register offset = 0x24.
        let addr = (self.rb as *const RegisterBlock as usize) + 0x24;
        unsafe { core::ptr::write_volatile(addr as *mut u32, 0xFFFF_FFFF) };
    }

    /// Clear the specified interrupt flags.
    pub fn irq_clear(&self, flags: IrqFlags) {
        let addr = (self.rb as *const RegisterBlock as usize) + 0x24;
        unsafe { core::ptr::write_volatile(addr as *mut u32, flags.bits()) };
    }

    // ------------------------------------------------------------------
    // Status queries
    // ------------------------------------------------------------------

    /// Return the current DMA source addresses for image-A and image-B.
    pub fn get_image_addrs(&self) -> (u32, u32) {
        let a = self
            .rb
            .lcdc_img_a_base_addr()
            .read()
            .img_a_base_addr()
            .bits();
        let b = self
            .rb
            .lcdc_img_b_base_addr()
            .read()
            .img_b_base_addr()
            .bits();
        (a, b)
    }

    /// Return the current image-slide offsets `(hs_ofst, vs_ofst)`.
    pub fn get_image_offset(&self) -> (u16, u16) {
        let r = self.rb.lcdc_img_slide().read();
        (r.img_hs_ofst().bits(), r.img_vs_ofst().bits())
    }

    /// Return the current display scan position `(x, y)`.
    pub fn get_cur_pos(&self) -> (u16, u16) {
        let r = self.rb.lcdc_cur_pos_status().read();
        (r.cur_pos_x().bits(), r.cur_pos_y().bits())
    }

    // ------------------------------------------------------------------
    // Byte-swap (MCU 8-bit RGB565 DMA mode only)
    // ------------------------------------------------------------------

    /// Enable or disable byte-swap of `D[15:8]` / `D[7:0]` in MCU 8-bit mode.
    pub fn mcu_set_byte_swap(&self, enable: bool) {
        self.rb
            .lcdc_mcu_cfg()
            .modify(|_, w| w.dat_swap().bit(enable));
    }

    // ------------------------------------------------------------------
    // De-initialise
    // ------------------------------------------------------------------

    /// Instantly disable the LCDC, clear all pending interrupts, and mask
    /// all interrupt sources.
    pub fn deinit(&self) {
        self.disable();
        self.irq_clear_all();
        self.rb.lcdc_irq_en().modify(|_, w| unsafe { w.bits(0) });
    }
}
