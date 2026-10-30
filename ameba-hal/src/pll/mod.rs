//! RTL8721F PLL driver — `#![no_std]`, PAC-based.
//!
//! Two PLL instances are exposed as distinct types via a phantom marker:
//!   - `Pll<SysPllMarker>`  — the system PLL (fractional-N SSC PLL)
//!   - `Pll<UsbPllMarker>`  — the USB PHY PLL (integer-N)
//!
//! Construct with `Pll::<SysPllMarker>::steal()` or `Pll::<UsbPllMarker>::steal()`.
//! All public methods are safe; the only `unsafe` boundary is `steal()`.

use core::marker::PhantomData;

use rtl8721f_pac::{lp_sys, pll_reg};

use crate::delay::Delay;
use embedded_hal::delay::DelayNs;

/// XTAL reference clock on RTL8721F — fixed 40 MHz.
///
/// SDK: `ameba_clk.h : XTAL_40M ((u32)40000000)`; ROM `XTAL_ClkGet()`
/// (`ameba_clk_rom.c`) simply `return XTAL_40M;`.
const XTAL_40M_HZ: u32 = 40_000_000;

// ---------------------------------------------------------------------------
// Inline PAC peripheral accessors — no raw address arithmetic
// ---------------------------------------------------------------------------

#[inline(always)]
fn pll() -> &'static pll_reg::RegisterBlock {
    // SAFETY: PLL_REG_BASE = 0x4080_A500; PAC Periph<pll_reg::RegisterBlock, 0x4080_a500>
    unsafe { &*(rtl8721f_pac::PllReg::PTR) }
}

#[inline(always)]
fn lp_sys() -> &'static lp_sys::RegisterBlock {
    // SAFETY: LP_SYS base = 0x4080_A200
    unsafe { &*(rtl8721f_pac::LpSys::PTR) }
}

// ---------------------------------------------------------------------------
// Marker types for type-state encoding
// ---------------------------------------------------------------------------

pub struct SysPllMarker;
pub struct UsbPllMarker;

// ---------------------------------------------------------------------------
// Public enumerations
// ---------------------------------------------------------------------------

/// Action for I2S clock-tune functions.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TuneAction {
    /// Reset to the nominal fractional code (no change).
    Auto,
    /// Increase PLL frequency by `ppm` parts-per-million.
    Faster,
    /// Decrease PLL frequency by `ppm` parts-per-million.
    Slower,
}

/// I2S audio reference clock selection.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum I2sClock {
    /// SYS_PLL / 4 → nominally 98.304 MHz output
    Pll98p304M,
    /// SYS_PLL / 8 → nominally 45.1584 MHz output
    Pll45p158M,
}

/// SPORT clock source (used by `set_sport_clk`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SportClkSrc {
    /// 40 MHz XTAL direct
    Xtal40M,
    /// System PLL output (divided)
    CpuPll,
}

// ---------------------------------------------------------------------------
// I2S clock source encoding for the PAC `cksl_i2s` 2-bit field:
//   x0 = XTAL  (0b00 or 0b10)
//   01 = USB_PLL
//   11 = SYS_PLL
// ---------------------------------------------------------------------------
const CKSL_I2S_XTAL: u8 = 0b00;
const CKSL_I2S_SYS_PLL: u8 = 0b11;

// SYS PLL pre-divider encoding: 01 = /2
const PREDIV_DIV2: u8 = 0x1;

// ---------------------------------------------------------------------------
// Private helper: xtal frequency
// ---------------------------------------------------------------------------

#[inline]
fn xtal_clk_hz() -> u32 {
    XTAL_40M_HZ
}

#[inline]
fn xtal_ready() -> bool {
    // SDK: ameba_clk_rom.c : XTAL_Check_READY — poll REG_LSYS_SYSTEM_CFG0
    // (offset 0x2A8) LSYS_BIT_XTAL_VALID (bit 29) with a 10000-iteration timeout.
    let mut timeout = 10_000u32;
    while timeout != 0 {
        if lp_sys().reg_system_cfg0().read().xtal_valid().bit_is_set() {
            return true;
        }
        timeout -= 1;
    }
    false
}

// ---------------------------------------------------------------------------
// Private SYS-PLL helpers
// ---------------------------------------------------------------------------

/// Returns the SYS PLL input frequency after the pre-divider (Hz).
fn sys_pll_freq_input() -> u32 {
    let prediv = pll().reg_pll_ctrl0().read().reg_prediv().bits() as u32;
    xtal_clk_hz() >> prediv
}

/// Writes ncode/fcode and pulses OC_EN to load them into the SSC engine.
fn sys_pll_set_div_reg(ncode: u8, fcode: u16) {
    let p = pll();

    // bypass_pi must be cleared when fcode != 0 so the PI interpolator runs
    if fcode != 0 {
        p.reg_ssc_ctrl3()
            .modify(|_, w| w.reg_bypass_pi().clear_bit());
    } else {
        p.reg_ssc_ctrl3().modify(|_, w| w.reg_bypass_pi().set_bit());
    }

    p.reg_ssc_ctrl0().modify(|_, w| unsafe {
        w.ncode_t_scpu().bits(ncode);
        w.fcode_t_scpu().bits(fcode)
    });

    // OC_EN 0→1 toggle loads the new N/F codes
    p.reg_ssc_ctrl2().modify(|_, w| w.oc_en().set_bit());
    p.reg_ssc_ctrl2().modify(|_, w| w.oc_en().clear_bit());

    Delay::new().delay_us(20);
}

fn sys_pll_enable() {
    if !xtal_ready() {
        return;
    }
    let p = pll();

    // 1. 3.3 V power
    p.reg_ssc_ctrl4().modify(|_, w| w.pwc_uahv_pll().set_bit());
    // 2. 0.9 V power
    p.reg_ssc_ctrl4().modify(|_, w| w.pwc_ualv_pll().set_bit());
    // 3. SSC Fen (global reset release + PLL reset release)
    p.reg_ssc_ctrl4().modify(|_, w| w.fen_pll().set_bit());
    // 4. Enable PLL CMU
    p.reg_pll_ctrl0().modify(|_, w| w.reg_cmu_en().set_bit());
    // 5. Load N/F codes via OC_EN toggle
    p.reg_ssc_ctrl2().modify(|_, w| w.oc_en().set_bit());
    p.reg_ssc_ctrl2().modify(|_, w| w.oc_en().clear_bit());

    // 6. Spin until sys_pll_valid asserts
    while !lp_sys()
        .reg_system_cfg0()
        .read()
        .sys_pll_valid()
        .bit_is_set()
    {}
}

fn sys_pll_disable() {
    let p = pll();
    p.reg_pll_ctrl0().modify(|_, w| w.reg_cmu_en().clear_bit());
    p.reg_ssc_ctrl4().modify(|_, w| w.fen_pll().clear_bit());
    p.reg_ssc_ctrl4()
        .modify(|_, w| w.pwc_ualv_pll().clear_bit());
    p.reg_ssc_ctrl4()
        .modify(|_, w| w.pwc_uahv_pll().clear_bit());
}

/// Configures the pre-divider and N/F codes for the requested output frequency.
fn sys_pll_set_freq(freq_hz: u32) {
    // Fix pre-divider to /2 (XTAL 40 MHz → 20 MHz reference)
    pll()
        .reg_pll_ctrl0()
        .modify(|_, w| unsafe { w.reg_prediv().bits(PREDIV_DIV2) });

    let freq_input = sys_pll_freq_input();
    let ncode = (freq_hz / freq_input).saturating_sub(2) as u8;

    let fraction = freq_hz - (ncode as u32 + 2) * freq_input;
    // fcode = round(fraction / freq_input * 2^14)
    let fcode = ((fraction as f32 / freq_input as f32) * (1u32 << 14) as f32 + 0.5f32) as u16;

    sys_pll_set_div_reg(ncode, fcode);
}

fn sys_pll_get_freq() -> u32 {
    let r = pll().reg_ssc_ctrl0().read();
    let ncode = r.ncode_t_scpu().bits() as u32;
    let fcode = r.fcode_t_scpu().bits() as u32;
    let freq_input = sys_pll_freq_input();

    let div_decimal = fcode / 16384 * 100_000;
    let div_integer = (ncode + 2) * freq_input;
    div_decimal + div_integer
}

// ---------------------------------------------------------------------------
// Private USB-PLL helpers
// ---------------------------------------------------------------------------

/// Writes the integer divider for the USB PLL (freq_hz = (div+2) * xtal).
fn usb_pll_set_div_reg(freq_hz: u32) {
    let xtal = xtal_clk_hz();
    let div = (freq_hz / xtal).saturating_sub(2) as u8;
    pll()
        .reg_upll_ctrl1()
        .modify(|_, w| unsafe { w.upll_n_pll_top().bits(div) });
}

fn usb_pll_enable() {
    if !xtal_ready() {
        return;
    }
    let p = pll();

    // 1. USB aphy power cut enable
    p.reg_upll_ctrl0().modify(|_, w| w.pwc_uapc_en().set_bit());
    Delay::new().delay_us(2);

    // 2. USB aphy BG power
    p.reg_upll_ctrl0().modify(|_, w| w.pwc_uabg_en().set_bit());
    Delay::new().delay_us(10);

    // 3. Release USB aphy ISO
    p.reg_upll_ctrl0()
        .modify(|_, w| w.iso_usba_en().clear_bit());
    Delay::new().delay_us(10);

    // 4. Release USB aphy reset
    p.reg_upll_ctrl0().modify(|_, w| w.usb_aphy_en().set_bit());

    // 5. Enable PLL
    p.reg_upll_ctrl0().modify(|_, w| w.sys_pll_en().set_bit());

    // 6. Spin until usb_pll_valid asserts
    while !lp_sys()
        .reg_system_cfg0()
        .read()
        .usb_pll_valid()
        .bit_is_set()
    {}
}

fn usb_pll_disable() {
    let p = pll();
    p.reg_upll_ctrl0()
        .modify(|_, w| w.usb_aphy_en().clear_bit());
    p.reg_upll_ctrl0().modify(|_, w| w.sys_pll_en().clear_bit());
    p.reg_upll_ctrl0().modify(|_, w| w.iso_usba_en().set_bit());
    p.reg_upll_ctrl0()
        .modify(|_, w| w.pwc_uabg_en().clear_bit());
    p.reg_upll_ctrl0()
        .modify(|_, w| w.pwc_uapc_en().clear_bit());
}

fn usb_pll_set_freq(freq_hz: u32) {
    usb_pll_set_div_reg(freq_hz);
    Delay::new().delay_us(20);
}

fn usb_pll_get_freq() -> u32 {
    let div = pll().reg_upll_ctrl1().read().upll_n_pll_top().bits() as u32;
    (div + 2) * xtal_clk_hz()
}

// ---------------------------------------------------------------------------
// I2S / SPORT clock helpers (lp_sys PAC — RCC_* macros reimplemented)
// ---------------------------------------------------------------------------

/// Sets the I2S clock source to SYS_PLL and programs the fractional divider.
/// Equivalent to `RCC_PeriphClockSourceSet(I2S, SYS_PLL)` +
///              `RCC_PeriphClockDividerSet(SYS_PLL_I2S, div)` +
///              `RCC_PeriphClockDividerFENSet(SYS_PLL_I2S, ENABLE)`.
fn i2s_use_sys_pll(div: u8) {
    let ls = lp_sys();
    // Clock source: cksl_i2s = 0b11 → SYS_PLL
    ls.reg_cksl_grp0()
        .modify(|_, w| unsafe { w.cksl_i2s().bits(CKSL_I2S_SYS_PLL) });
    // Divider: register holds (div - 1)
    ls.reg_ckd_sys_pll_grp1()
        .modify(|_, w| unsafe { w.ckd_sys_pll_i2s().bits(div.saturating_sub(1)) });
    // Enable the divider output
    ls.reg_ckd_fen_sys_pll()
        .modify(|_, w| w.fen_sys_pll_i2s().set_bit());
}

/// Sets the I2S clock source to XTAL.
fn i2s_use_xtal() {
    lp_sys()
        .reg_cksl_grp0()
        .modify(|_, w| unsafe { w.cksl_i2s().bits(CKSL_I2S_XTAL) });
}

// ---------------------------------------------------------------------------
// Shared tune helper used by both I2S tune functions
// ---------------------------------------------------------------------------

fn sys_pll_tune(ppm: f32, action: TuneAction) -> f32 {
    let r = pll().reg_ssc_ctrl0().read();
    let ncode = r.ncode_t_scpu().bits() as u32;
    let fcode = r.fcode_t_scpu().bits() as u32;

    // step = 1 ppm expressed in fcode units
    let step = 1_000_000.0_f32 / (fcode as f32 + (ncode + 2) as f32 * 16384.0_f32);

    let fcode_new = match action {
        TuneAction::Faster => fcode.saturating_add((ppm / step + 0.5_f32) as u32),
        TuneAction::Slower => fcode.saturating_sub((ppm / step + 0.5_f32) as u32),
        TuneAction::Auto => fcode,
    };

    let real_ppm = (fcode_new as f32 - fcode as f32) * step;
    sys_pll_set_div_reg(ncode as u8, fcode_new as u16);
    real_ppm
}

// ---------------------------------------------------------------------------
// PLL handle — phantom-typed on SysPllMarker / UsbPllMarker
// ---------------------------------------------------------------------------

/// RTL8721F PLL peripheral handle.
///
/// Phantom parameter `K` selects which PLL this handle controls.
/// Construct via `Pll::<SysPllMarker>::steal()` or
/// `Pll::<UsbPllMarker>::steal()`.
pub struct Pll<K> {
    _kind: PhantomData<K>,
}

// ---------------------------------------------------------------------------
// Common methods available on both PLL types
// ---------------------------------------------------------------------------

impl<K> Pll<K> {
    /// Steal a handle to this PLL.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other live `Pll<K>` instance is
    /// concurrently mutating the same PLL registers.
    pub unsafe fn steal() -> Self {
        Self { _kind: PhantomData }
    }
}

// ---------------------------------------------------------------------------
// SYS PLL — methods
// ---------------------------------------------------------------------------

impl Pll<SysPllMarker> {
    /// Power up and lock the system PLL.
    ///
    /// Spins until `sys_pll_valid` asserts. Returns immediately if the
    /// 40 MHz XTAL is not yet stable.
    pub fn enable(&mut self) {
        sys_pll_enable();
    }

    /// Power down the system PLL.
    pub fn disable(&mut self) {
        sys_pll_disable();
    }

    /// Program the system PLL output frequency.
    ///
    /// `freq_hz` is the desired VCO output in Hz (e.g. `393_216_000` for
    /// the 98.304 MHz I2S chain or `600_000_000` for 600 MHz CPU).
    pub fn set_freq(&mut self, freq_hz: u32) {
        sys_pll_set_freq(freq_hz);
    }

    /// Read back the system PLL output frequency (Hz).
    pub fn get_freq(&self) -> u32 {
        sys_pll_get_freq()
    }

    /// Adjust the PLL fractional code by `ppm` parts-per-million.
    ///
    /// This tunes the 98.304 MHz I2S output.  Returns the actual ppm
    /// applied (may differ from the requested value due to code rounding).
    ///
    /// `ppm` must be ≤ 1000.
    pub fn i2s_98p304m_tune(&mut self, ppm: f32, action: TuneAction) -> f32 {
        // ppm <= 1000 is a hardware constraint; silent clamp
        let ppm = if ppm > 1000.0_f32 { 1000.0_f32 } else { ppm };
        sys_pll_tune(ppm, action)
    }

    /// Adjust the PLL fractional code by `ppm` parts-per-million.
    ///
    /// This tunes the 45.1584 MHz I2S output.  Returns the actual ppm
    /// applied.
    ///
    /// `ppm` must be ≤ 1000.
    pub fn i2s_45p158m_tune(&mut self, ppm: f32, action: TuneAction) -> f32 {
        let ppm = if ppm > 1000.0_f32 { 1000.0_f32 } else { ppm };
        sys_pll_tune(ppm, action)
    }

    /// Configure the I2S MCLK supply.
    ///
    /// Routes the system PLL through the appropriate integer divider to
    /// produce the requested nominal MCLK and enables the divider FEN.
    ///
    /// | Variant          | Divider | Nominal output |
    /// |---|---|---|
    /// | `Pll98p304M` | /4 | 98.304 MHz |
    /// | `Pll45p158M` | /8 | 45.158 MHz |
    pub fn set_i2s_clk(&mut self, clk: I2sClock) {
        match clk {
            I2sClock::Pll98p304M => i2s_use_sys_pll(4),
            I2sClock::Pll45p158M => i2s_use_sys_pll(8),
        }
    }

    /// Configure the I2S MCLK for a specific index (Zephyr compatibility).
    ///
    /// Only `index == 0` is supported; the call is silently ignored for
    /// any other index.
    pub fn set_i2s_clk_div(&mut self, index: u32, clk: I2sClock) {
        if index == 0 {
            self.set_i2s_clk(clk);
        }
    }

    /// Switch the SPORT (audio serializer) clock source.
    pub fn set_sport_clk(&mut self, src: SportClkSrc) {
        match src {
            SportClkSrc::Xtal40M => i2s_use_xtal(),
            SportClkSrc::CpuPll => i2s_use_sys_pll(1),
        }
    }

    /// Placeholder — spread-spectrum configuration (not yet implemented
    /// in the C reference either).
    pub fn set_ssc(&mut self) {
        // intentional no-op: TO DO in C reference
    }
}

// ---------------------------------------------------------------------------
// USB PLL — methods
// ---------------------------------------------------------------------------

impl Pll<UsbPllMarker> {
    /// Power up and lock the USB PLL.
    ///
    /// Spins until `usb_pll_valid` asserts. Returns immediately if the
    /// 40 MHz XTAL is not yet stable.
    pub fn enable(&mut self) {
        usb_pll_enable();
    }

    /// Power down the USB PLL.
    pub fn disable(&mut self) {
        usb_pll_disable();
    }

    /// Program the USB PLL output frequency.
    ///
    /// `freq_hz` is the desired integer-N output in Hz
    /// (e.g. `480_000_000` for 480 MHz USB HS).
    pub fn set_freq(&mut self, freq_hz: u32) {
        usb_pll_set_freq(freq_hz);
    }

    /// Read back the USB PLL output frequency (Hz).
    pub fn get_freq(&self) -> u32 {
        usb_pll_get_freq()
    }
}
