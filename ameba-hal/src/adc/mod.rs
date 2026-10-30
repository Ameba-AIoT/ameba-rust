//! RTL8721F ADC driver — idiomatic Rust, no_std, PAC-based.
//!
//! # Hardware summary
//! - Base address  : 0x4081_2000 (non-secure)
//! - Channels      : 11 regular (CH0–CH10) + dummy cycle slot (0x0F)
//! - Resolution    : 16-bit
//! - Modes         : software-trigger, automatic, timer-trigger, comparator-assist
//! - CLK source    : 4 MHz core clock
//!
//! # Usage sketch
//! ```ignore
//! let adc = unsafe { Adc::steal() };
//! adc.init(&AdcInitParams::default());
//! adc.cmd(true);
//! adc.sw_trig_cmd(true);
//! while !adc.readable() {}
//! let entry = adc.read();
//! ```

use bitflags::bitflags;
use rtl8721f_pac::{
    AdcReg, PllReg, Timer0Reg, Timer1Reg, Timer2Reg, Timer3Reg, Timer4Reg, Timer5Reg, Timer6Reg,
    Timer7Reg, adc_reg, timer0_reg,
};

use crate::delay::Delay;
use embedded_hal::delay::DelayNs;

/// Register block for the ADC external-trigger RTIM selected at runtime.
///
/// Timers 0–3 are basic RTIMs, 4–7 are PWM/CCM RTIMs; all share the
/// `timer0_reg` register layout, so the base is derived from the PAC `::PTR`
/// of the matching `TimerNReg` peripheral — never a hand-written address.
#[inline]
fn trig_timer_regs(idx: usize) -> &'static timer0_reg::RegisterBlock {
    let base = match idx & 0x07 {
        0 => Timer0Reg::PTR as usize,
        1 => Timer1Reg::PTR as usize,
        2 => Timer2Reg::PTR as usize,
        3 => Timer3Reg::PTR as usize,
        4 => Timer4Reg::PTR as usize,
        5 => Timer5Reg::PTR as usize,
        6 => Timer6Reg::PTR as usize,
        _ => Timer7Reg::PTR as usize,
    };
    // SAFETY: every TimerNReg shares the timer0_reg layout; single-owner access
    // is the caller's responsibility via the `Adc` handle contract.
    unsafe { &*(base as *const timer0_reg::RegisterBlock) }
}

// ────────────────────────────────────────────────────────────────────────────
// EFuse-backed calibration (Vref selection + normal-channel voltage curve)
// ────────────────────────────────────────────────────────────────────────────

use core::sync::atomic::{AtomicBool, AtomicI32, Ordering};

/// OTP address of the per-chip Vref selection trim.
const VREF_SEL_ADDR: u32 = 0x7EC; // SDK: ameba_adc.h : VREF_SEL_ADDR
/// OTP base address of the 6-byte normal-channel voltage calibration block.
const NORM_VOL_ADDR: u32 = 0x7DA; // SDK: ameba_adc.h : NORM_VOL_ADDR

fn pll_reg() -> &'static rtl8721f_pac::pll_reg::RegisterBlock {
    // SAFETY: PLL_REG_BASE = 0x4080_A500; single-owner access guarded by VREF_INIT once-flag.
    unsafe { &*PllReg::PTR }
}

/// Cached normal-channel calibration coefficients, read once from EFuse.
static CAL_INIT: AtomicBool = AtomicBool::new(false);
static CAL_A: AtomicI32 = AtomicI32::new(0);
static CAL_B: AtomicI32 = AtomicI32::new(0);
static CAL_C: AtomicI32 = AtomicI32::new(0);
/// Whether the Vref trim has been applied (mirrors C `vref_init_done`).
static VREF_INIT: AtomicBool = AtomicBool::new(false);

/// Read a single OTP byte; returns `0xFF` (blank) on read failure.
fn otp_read8(addr: u32) -> u8 {
    crate::otp::read8(addr)
}

/// Apply the per-chip Vref trim from EFuse (mirrors the Vref block of C `ADC_Init`).
///
/// Reads `VREF_SEL_ADDR[2:0]`: `0x3` selects 0.875V, anything else 0.850V.
/// Runs at most once; subsequent calls are no-ops.
fn init_vref() {
    if VREF_INIT.swap(true, Ordering::Relaxed) {
        return;
    }
    let vref_sel = (otp_read8(VREF_SEL_ADDR) & 0x7) as u32;
    let sel = if vref_sel == 0x3 { 0x3u32 } else { 0x2u32 };
    unsafe {
        pll_reg()
            .reg_lpad0()
            .modify(|_, w| w.vref_sel().bits(sel as u8));
    }
}

/// Read and cache the normal-channel voltage calibration from EFuse
/// (mirrors C `ADC_InitCalPara`).  Blank (`0xFFFF`) fields fall back to the
/// factory defaults used by the C driver.
fn init_cal_para() {
    if CAL_INIT.load(Ordering::Relaxed) {
        return;
    }
    let mut buf = [0u8; 6];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = otp_read8(NORM_VOL_ADDR + i as u32);
    }
    let mut k_a = ((buf[1] as u16) << 8) | buf[0] as u16;
    let mut k_b = ((buf[3] as u16) << 8) | buf[2] as u16;
    let mut k_c = ((buf[5] as u16) << 8) | buf[4] as u16;
    if k_a == 0xFFFF {
        k_a = 0xFED9; // SDK: ameba_adc.c : ADC_InitCalPara
    }
    if k_b == 0xFFFF {
        k_b = 0x6D6C; // SDK: ameba_adc.c : ADC_InitCalPara
    }
    if k_c == 0xFFFF {
        k_c = 0xFD1D; // SDK: ameba_adc.c : ADC_InitCalPara
    }
    // ka, kc are signed (two's complement in bit15); kb is unsigned 16-bit.
    let ka = if k_a & 0x8000 != 0 {
        -(0x1_0000i32 - k_a as i32)
    } else {
        (k_a & 0x7FFF) as i32
    };
    let kb = k_b as i32;
    let kc = if k_c & 0x8000 != 0 {
        -(0x1_0000i32 - k_c as i32)
    } else {
        (k_c & 0x7FFF) as i32
    };
    CAL_A.store(ka, Ordering::Relaxed);
    CAL_B.store(kb, Ordering::Relaxed);
    CAL_C.store(kc, Ordering::Relaxed);
    CAL_INIT.store(true, Ordering::Relaxed);
}

// ────────────────────────────────────────────────────────────────────────────
// ADC register block accessor
// ────────────────────────────────────────────────────────────────────────────

#[inline(always)]
fn regs() -> &'static adc_reg::RegisterBlock {
    // SAFETY: AdcReg::PTR is the PAC-derived pointer for the ADC peripheral on
    // RTL8721F.  Access serialisation is the caller's responsibility via the
    // `Adc` handle contract.
    unsafe { &*AdcReg::PTR }
}

// ────────────────────────────────────────────────────────────────────────────
// bitflags — interrupt enable / status bits
// ────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// ADC interrupt bits — used for both the `ADC_INTR_CTRL` (enable) and
    /// `ADC_INTR_STS` / `ADC_INTR_RAW_STS` (status) registers.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct AdcInt: u32 {
        /// Conversion list end.
        const CVLIST_END    = 1 << 0;
        /// Single conversion end.
        const CV_END        = 1 << 1;
        /// Particular-channel conversion end (see `ADC_IT_CHNO_CON`).
        const CHCV_END      = 1 << 2;
        /// FIFO full threshold reached.
        const FIFO_FULL     = 1 << 3;
        /// FIFO overflow.
        const FIFO_OVER     = 1 << 4;
        /// FIFO empty.
        const FIFO_EMPTY    = 1 << 5;
        /// Data overwritten.
        const DAT_OVW       = 1 << 6;
        /// Error state.
        const ERR           = 1 << 7;
        /// Comparator match — channel 0.
        const COMP_CH0      = 1 << 8;
        /// Comparator match — channel 1.
        const COMP_CH1      = 1 << 9;
        /// Comparator match — channel 2.
        const COMP_CH2      = 1 << 10;
        /// Comparator match — channel 3.
        const COMP_CH3      = 1 << 11;
        /// Comparator match — channel 4.
        const COMP_CH4      = 1 << 12;
        /// Comparator match — channel 5.
        const COMP_CH5      = 1 << 13;
        /// Comparator match — channel 6.
        const COMP_CH6      = 1 << 14;
        /// Comparator match — channel 7.
        const COMP_CH7      = 1 << 15;
        /// Comparator match — channel 8.
        const COMP_CH8      = 1 << 16;
        /// Comparator match — channel 9.
        const COMP_CH9      = 1 << 17;
        /// Comparator match — channel 10.
        const COMP_CH10     = 1 << 18;
        /// All comparator-channel bits (CH0..CH10).
        const COMP_ALL      = 0x0007_FF00;
        /// All status bits.
        const ALL           = 0x0007_FFFF;
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Type-safe enumerations
// ────────────────────────────────────────────────────────────────────────────

/// ADC operating mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcOpMode {
    /// Software trigger — call `Adc::sw_trig_cmd(true)` for each conversion.
    SoftwareTrigger = 0,
    /// Automatic mode — `Adc::auto_csw_cmd(true)` starts continuous scan.
    Automatic = 1,
    /// Timer-trigger mode — an RTIM fires the conversion periodically.
    TimerTrigger = 2,
    /// Comparator-assist mode.
    ComparatorAssist = 3,
}

/// ADC channel identifier (4-bit register field).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcChannel {
    Ch0 = 0x00,
    Ch1 = 0x01,
    Ch2 = 0x02,
    Ch3 = 0x03,
    Ch4 = 0x04,
    Ch5 = 0x05,
    Ch6 = 0x06,
    Ch7 = 0x07,
    Ch8 = 0x08,  // internal
    Ch9 = 0x09,  // internal
    Ch10 = 0x0A, // internal
    Ch14 = 0x0E, // internal
    DummyCycle = 0x0F,
}

impl AdcChannel {
    /// Convert a raw nibble value (0–15) to `AdcChannel`.
    /// Returns `None` for reserved slots (0x0B–0x0D).
    pub fn from_raw(v: u8) -> Option<Self> {
        match v & 0x0F {
            0x00 => Some(Self::Ch0),
            0x01 => Some(Self::Ch1),
            0x02 => Some(Self::Ch2),
            0x03 => Some(Self::Ch3),
            0x04 => Some(Self::Ch4),
            0x05 => Some(Self::Ch5),
            0x06 => Some(Self::Ch6),
            0x07 => Some(Self::Ch7),
            0x08 => Some(Self::Ch8),
            0x09 => Some(Self::Ch9),
            0x0A => Some(Self::Ch10),
            0x0E => Some(Self::Ch14),
            0x0F => Some(Self::DummyCycle),
            _ => None,
        }
    }
}

/// Threshold comparison criterion for `Adc::set_comp`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CompCriteria {
    /// Match when `Vin < THL`.
    SmallerThanThl = 0,
    /// Match when `Vin > THH`.
    GreaterThanThh = 1,
    /// Match when `THL <= Vin <= THH`.
    WithinThlAndThh = 2,
    /// Match when `Vin < THL || Vin > THH`.
    OutsideThlAndThh = 3,
}

/// Comparator interrupt detection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompDetMode {
    /// Level: interrupt fires continuously while criterion holds.
    Level,
    /// Edge: interrupt fires once on the rising edge of a criterion match.
    Edge,
}

/// Oversample ratio for `Adc::set_oversample`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OsRatio {
    X2 = 0,
    X4 = 1,
    X8 = 2,
    X16 = 3,
    X32 = 4,
    X64 = 5,
    X128 = 6,
    X256 = 7,
}

/// Oversample right-shift amount for `Adc::set_oversample`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OsShift {
    None = 0,
    Shift1 = 1,
    Shift2 = 2,
    Shift3 = 3,
    Shift4 = 4,
    Shift5 = 5,
    Shift6 = 6,
    Shift7 = 7,
}

/// Oversample sequencing mode for `Adc::set_oversample`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsMode {
    /// Staggered: oversampling conversions spread across the channel scan.
    Staggered,
    /// Regular: all oversampling conversions done back-to-back.
    Regular,
}

// ────────────────────────────────────────────────────────────────────────────
// Initialisation parameters
// ────────────────────────────────────────────────────────────────────────────

/// ADC initialisation parameters — Rust equivalent of `ADC_InitTypeDef`.
pub struct AdcInitParams {
    /// Operating mode.
    pub op_mode: AdcOpMode,
    /// FIFO-full interrupt threshold level (0 → 1 entry, 15 → 16 entries).
    pub rx_threshold_level: u8,
    /// Channel conversion list.  Only entries `[0..cvlist_len]` are used.
    pub cvlist: [u8; 16],
    /// Number of valid entries in `cvlist` (1–16).
    pub cvlist_len: u8,
    /// Particular channel for `CHCV_END` interrupt.  0xFF = disabled.
    pub special_ch: u8,
    /// Sample period in µs (1–0x400).
    pub sample_period_us: u16,
}

impl Default for AdcInitParams {
    /// Mirrors `ADC_StructInit()`: SW-trigger, 12 channels (0–11) in order,
    /// 6 µs sample period.
    fn default() -> Self {
        let mut cvlist = [0u8; 16];
        for (i, slot) in cvlist.iter_mut().enumerate().take(12) {
            *slot = i as u8;
        }
        Self {
            op_mode: AdcOpMode::SoftwareTrigger,
            rx_threshold_level: 0,
            cvlist,
            cvlist_len: 12,
            special_ch: 0xFF,
            sample_period_us: 6,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Decoded FIFO entry
// ────────────────────────────────────────────────────────────────────────────

/// A single FIFO word decoded from `ADC_DATA_GLOBAL`.
///
/// Bits `[19:16]` hold the channel-list slot index; bits `[15:0]` hold the 16-bit
/// ADC conversion result.
#[derive(Clone, Copy, Debug)]
pub struct AdcFifoEntry {
    /// Channel-list slot index that produced this sample.
    pub channel_idx: u8,
    /// 16-bit conversion result.
    pub data: u16,
}

impl AdcFifoEntry {
    #[inline]
    fn from_raw(word: u32) -> Self {
        Self {
            channel_idx: ((word >> 16) & 0x0F) as u8,
            data: (word & 0xFFFF) as u16,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Adc driver struct
// ────────────────────────────────────────────────────────────────────────────

/// ADC peripheral driver handle.
///
/// Obtain one with `unsafe { Adc::steal() }`.  No ownership token is tracked
/// in the type; callers must guarantee exclusive access.
pub struct Adc(());

impl Adc {
    /// Obtain an `Adc` handle by stealing the peripheral singleton.
    ///
    /// # Safety
    /// The caller must ensure no other code concurrently accesses the
    /// ADC register block via `AdcReg::PTR`.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self(())
    }

    // ── register block shorthand ──────────────────────────────────────────

    #[inline(always)]
    fn r(&self) -> &'static adc_reg::RegisterBlock {
        regs()
    }

    // ── core API ──────────────────────────────────────────────────────────

    /// Initialise the ADC peripheral from `params`.
    ///
    /// Steps (mirrors `ADC_Init()`):
    /// 1. Disable ADC, clear interrupt enables, clear interrupt status, clear FIFO.
    /// 2. Set `ADC_CONF`: op-mode and cvlist-length.
    /// 3. Set per-channel sample period via `ADC_CLK_HCNTx_DIV`.
    /// 4. Set `ADC_CLK_LCNT0_DIV` = 2.
    /// 5. Program `ADC_CHSW_LIST_0` / `ADC_CHSW_LIST_1`.
    /// 6. Optionally configure the particular-channel interrupt.
    /// 7. Set `ADC_FULL_LVL`.
    ///
    /// This also applies the per-chip Vref trim from EFuse (once), mirroring the
    /// Vref block of the C `ADC_Init`: without it every conversion is mis-scaled
    /// (or garbage). It writes only the `vref_sel` field of `PLL_REG->REG_LPAD0`.
    pub fn init(&self, p: &AdcInitParams) {
        let r = self.r();
        let cvlist_len = p.cvlist_len.clamp(1, 16) as usize;

        // 0. Apply the EFuse Vref trim (once) before any conversion.
        init_vref();

        // 1. Disable + clear.
        self.cmd(false);
        unsafe {
            r.reg_adc_intr_ctrl().write_with_zero(|w| w);
        }
        self.int_clear();
        self.clear_fifo();

        // 2. ADC_CONF: op-mode + cvlist-length.
        r.reg_adc_conf().modify(|_, w| unsafe {
            w.bit_adc_op_mod()
                .bits(p.op_mode as u8)
                .bit_adc_cvlist_len()
                .bits((cvlist_len - 1) as u8)
        });

        // 3. Per-channel HCnt: hcnt = period_us * 4 - 3 (matches C driver).
        let hcnt_val = ((p.sample_period_us as u32) * 4).saturating_sub(3) as u16;
        for i in 0..cvlist_len {
            let ch = p.cvlist[i] as usize;
            self.write_hcnt(ch, hcnt_val);
        }

        // 4. LCnt = 2.
        r.reg_adc_clk_lcnt0_div()
            .write(|w| unsafe { w.bit_adc_clk_lcnt().bits(2) });

        // 5. Channel switch lists.
        self.program_chsw_list(&p.cvlist, cvlist_len);

        // 6. Particular-channel interrupt.
        if p.special_ch < 16 {
            self.int_config(AdcInt::CHCV_END, true);
            r.reg_adc_it_chno_con()
                .write(|w| unsafe { w.bit_adc_it_chno_con().bits(p.special_ch & 0x0F) });
        }

        // 7. FIFO full threshold.
        r.reg_adc_full_lvl()
            .write(|w| unsafe { w.bit_adc_full_lvl().bits(p.rx_threshold_level & 0x3F) });
    }

    /// Enable or disable the ADC.
    ///
    /// Disabling also clears the FIFO (silicon errata noted in C driver).
    pub fn cmd(&self, enable: bool) {
        let r = self.r();
        if enable {
            r.reg_adc_conf().modify(|_, w| w.bit_adc_enable().set_bit());
        } else {
            r.reg_adc_conf()
                .modify(|_, w| w.bit_adc_enable().clear_bit());
            self.clear_fifo();
        }
    }

    /// Set the sample clock period for a single channel in microseconds.
    ///
    /// `period_us` should be in 1–0x40F.  The current LCNT value is read from
    /// hardware to compute the correct HCnt divisor.
    pub fn set_chan_clk(&self, channel: AdcChannel, period_us: u16) {
        let r = self.r();
        let ch = channel as usize;
        let lcnt = r.reg_adc_clk_lcnt0_div().read().bit_adc_clk_lcnt().bits() as u16;
        let clk_cnt = period_us.saturating_mul(4);
        // hcnt = clk_cnt - 1 - lcnt
        let hcnt = clk_cnt.saturating_sub(1).saturating_sub(lcnt) & 0x0FFF;
        self.write_hcnt(ch, hcnt);
    }

    /// Enable or disable a set of interrupt sources.
    ///
    /// `bits` selects which `AdcInt` flags to enable or disable.
    pub fn int_config(&self, bits: AdcInt, enable: bool) {
        let r = self.r();
        let cur = r.reg_adc_intr_ctrl().read().bits();
        let new = if enable {
            cur | bits.bits()
        } else {
            cur & !bits.bits()
        };
        r.reg_adc_intr_ctrl().write(|w| unsafe { w.bits(new) });
    }

    /// Clear all ADC interrupt pending bits (RW1C register).
    ///
    /// `ADC_INTR_STS` is write-1-to-clear.  The PAC marks this register as
    /// read-only in its type system (no `Writable` impl), so we perform the
    /// clear write via `write_volatile` on the raw address.
    pub fn int_clear(&self) {
        // Safety: ADC_INTR_STS is at a known, fixed hardware address.
        // Writing 1s to RW1C bits clears them; writing 0s has no effect.
        unsafe {
            self.r()
                .reg_adc_intr_sts()
                .as_ptr()
                .write_volatile(AdcInt::ALL.bits());
        }
    }

    /// Clear specific ADC interrupt pending bits (RW1C).
    pub fn int_clear_pending(&self, bits: AdcInt) {
        unsafe {
            self.r()
                .reg_adc_intr_sts()
                .as_ptr()
                .write_volatile(bits.bits());
        }
    }

    /// Read the masked interrupt status register (`ADC_INTR_STS`).
    pub fn get_isr(&self) -> AdcInt {
        AdcInt::from_bits_truncate(self.r().reg_adc_intr_sts().read().bits())
    }

    /// Read the raw (unmasked) interrupt status register (`ADC_INTR_RAW_STS`).
    pub fn get_raw_isr(&self) -> AdcInt {
        AdcInt::from_bits_truncate(self.r().reg_adc_intr_raw_sts().read().bits())
    }

    /// Set the channel conversion list and program its length into `ADC_CONF`.
    ///
    /// `chan_list` must have 1–16 entries, each a valid channel raw index (0–15).
    pub fn set_ch_list(&self, chan_list: &[u8]) {
        let len = chan_list.len().clamp(1, 16);
        self.r()
            .reg_adc_conf()
            .modify(|_, w| unsafe { w.bit_adc_cvlist_len().bits((len - 1) as u8) });
        self.program_chsw_list(chan_list, len);
    }

    /// Return the number of valid entries currently in the receive FIFO.
    pub fn get_rx_count(&self) -> u32 {
        self.r().reg_adc_flr().read().bit_adc_flr().bits() as u32
    }

    /// Return the last ADC channel used.
    pub fn get_last_chan(&self) -> u8 {
        (self.r().reg_adc_last_ch().read().bits() & 0x0F) as u8
    }

    /// Read the 2-bit comparison result for the given channel from `ADC_COMP_STS`.
    pub fn get_comp_status(&self, channel: AdcChannel) -> u8 {
        let ch = channel as u32;
        let raw = self.r().reg_adc_comp_sts().read().bits();
        ((raw >> (ch * 2)) & 0x3) as u8
    }

    /// Set the comparator detection mode (level vs. edge).
    pub fn set_comp_mode(&self, mode: CompDetMode) {
        let r = self.r();
        let cur = r.reg_adc_intr_ctrl().read().bits();
        let new = match mode {
            CompDetMode::Level => cur & !(1u32 << 31),
            CompDetMode::Edge => cur | (1u32 << 31),
        };
        r.reg_adc_intr_ctrl().write(|w| unsafe { w.bits(new) });
    }

    /// Set the comparison thresholds and criteria for a channel.
    ///
    /// Also enables the corresponding per-channel comparator interrupt.
    pub fn set_comp(
        &self,
        channel: AdcChannel,
        thresh_high: u16,
        thresh_low: u16,
        criteria: CompCriteria,
    ) {
        let ch = channel as usize;
        self.write_comp_th(ch, thresh_high, thresh_low);
        self.write_comp_ctrl(ch, criteria as u8);
        // Enable the comparator interrupt for this channel (bit 8 + ch).
        let comp_bit = AdcInt::from_bits_truncate(1 << (8 + ch));
        self.int_config(comp_bit, true);
    }

    /// Return `true` if the FIFO has at least one entry ready to read.
    pub fn readable(&self) -> bool {
        !self
            .r()
            .reg_adc_busy_sts()
            .read()
            .bit_adc_fifo_empty()
            .bit()
    }

    /// Pop one raw 20-bit FIFO word (`channel-idx[19:16]` | `data[15:0]`).
    ///
    /// Masks to 20 bits, matching `ADC_ID_AND_DATA()` in the C driver.
    pub fn read_raw(&self) -> u32 {
        self.r().reg_adc_data_global().read().bits() & 0x000F_FFFF
    }

    /// Pop one decoded FIFO entry.
    pub fn read(&self) -> AdcFifoEntry {
        AdcFifoEntry::from_raw(self.read_raw())
    }

    /// Convert a normal-channel raw sample to millivolts (mirrors C `ADC_GetVoltage`).
    ///
    /// Uses the per-chip calibration curve read from EFuse on first use:
    /// `mv = (ka·d² >> 26) + (kb·d >> 15) + (kc >> 6)`.
    /// Valid for normal channels (CH0–CH6, CH8–CH10); not for internal channels.
    pub fn get_voltage(&self, chan_data: u32) -> i32 {
        init_cal_para();
        let ka = CAL_A.load(Ordering::Relaxed) as i64;
        let kb = CAL_B.load(Ordering::Relaxed) as i64;
        let kc = CAL_C.load(Ordering::Relaxed);
        let d = chan_data as i64;
        (((ka * d * d) >> 26) + ((kb * d) >> 15) + ((kc >> 6) as i64)) as i32
    }

    /// Read `buf.len()` samples in automatic mode.
    ///
    /// Clears the FIFO, enables auto-channel-switch, polls until all samples
    /// are collected, then disables auto-channel-switch.
    pub fn receive_buf(&self, buf: &mut [u32]) {
        self.clear_fifo();
        self.auto_csw_cmd(true);
        for slot in buf.iter_mut() {
            while !self.readable() {}
            *slot = self.read_raw();
        }
        self.auto_csw_cmd(false);
    }

    /// Clear the ADC receive FIFO.
    ///
    /// Sets `ADC_CLR_FIFO`, waits until `ADC_FLR` reports zero entries,
    /// then clears the bit.
    pub fn clear_fifo(&self) {
        let r = self.r();
        r.reg_adc_clr_fifo()
            .write(|w| w.bit_adc_clr_fifo().set_bit());
        while r.reg_adc_flr().read().bit_adc_flr().bits() != 0 {}
        r.reg_adc_clr_fifo()
            .write(|w| w.bit_adc_clr_fifo().clear_bit());
    }

    /// Return the raw `ADC_BUSY_STS` register value.
    pub fn get_status_raw(&self) -> u32 {
        self.r().reg_adc_busy_sts().read().bits()
    }

    /// Return `true` if the ADC is currently busy processing a conversion.
    pub fn is_busy(&self) -> bool {
        self.r().reg_adc_busy_sts().read().bit_adc_busy_sts().bit()
    }

    /// Control the software-trigger bit (`ADC_SW_TRIG`) in SW-trigger mode.
    ///
    /// `true` enables the analog mux and starts a conversion (309 µs delay).
    /// `false` disables it (129 µs delay).
    pub fn sw_trig_cmd(&self, enable: bool) {
        let r = self.r();
        if enable {
            r.reg_adc_sw_trig().write(|w| w.bit_adc_sw_trig().set_bit());
            Delay::new().delay_us(309);
        } else {
            r.reg_adc_sw_trig()
                .write(|w| w.bit_adc_sw_trig().clear_bit());
            Delay::new().delay_us(129);
        }
    }

    /// Control the automatic channel-switch (`ADC_AUTO_CSW_CTRL`).
    ///
    /// `true` starts a continuous scan from the first channel in the list
    /// (309 µs power-on delay).  `false` terminates it (129 µs delay).
    pub fn auto_csw_cmd(&self, enable: bool) {
        let r = self.r();
        if enable {
            r.reg_adc_auto_csw_ctrl()
                .write(|w| w.bit_adc_autosw_en().set_bit());
            Delay::new().delay_us(309);
        } else {
            r.reg_adc_auto_csw_ctrl()
                .write(|w| w.bit_adc_autosw_en().clear_bit());
            Delay::new().delay_us(129);
        }
    }

    /// Configure and start/stop timer-trigger ADC mode.
    ///
    /// `tim_idx`  — which basic RTIM to use (0–7).
    /// `period_ms` — trigger period in ms (1–131071).
    /// `enable`   — `true` to start, `false` to stop.
    ///
    /// When `enable` is `true` this configures the selected RTIM as a periodic
    /// trigger source (equivalent to the SDK `RTIM_TimeBaseStructInit` +
    /// `RTIM_TimeBaseInit` + `RTIM_Cmd(ENABLE)` sequence, no IRQ callback) and
    /// starts it; when `false` it stops the timer (`RTIM_Cmd(DISABLE)`).
    ///
    /// SDK reference: `ameba_tim_rom.c` (RTIM_TimeBaseInit / RTIM_Cmd).
    pub fn timer_trig_cmd(&self, tim_idx: u8, period_ms: u32, enable: bool) {
        let tim_idx = (tim_idx & 0x07) as usize;
        self.r()
            .reg_adc_ext_trig_timer_sel()
            .write(|w| unsafe { w.bit_adc_ext_trig_timer_sel().bits(tim_idx as u8) });

        let regs = trig_timer_regs(tim_idx);

        if enable {
            // Convert ms to 32.768 kHz tick count.
            let period_ticks = period_ms.saturating_mul(32768) / 1000;

            // RTIM_TimeBaseInit reset sequence — stop the counter first.
            if regs.tim_en().read().cen().bit_is_set() {
                regs.tim_en().write(|w| w.cnt_stop().set_bit());
                while regs.tim_en().read().cen().bit_is_set() {}
            }
            // No IRQ callback for the ADC trigger → DIER stays disabled.
            regs.tim_dier().write(|w| unsafe { w.bits(0) });
            // RTIM_INTClear: SR is W1C but the PAC marks it read-only.
            let sr = regs.tim_sr().read().bits();
            unsafe { regs.tim_sr().as_ptr().write_volatile(sr) };
            for _ in 0..4000u32 {
                if regs.tim_sr().read().bits() & 0x00FF_FFFF == 0 {
                    break;
                }
            }

            // ARR: basic timers (0–3) hold a full 32-bit count (TIMBasic_CNT);
            // CCM/PWM timers (4–7) hold 16 bits (TIM_CNT). The PAC `arr` field is
            // only 16-bit, so write the register raw to preserve the basic-timer
            // high word.
            let arr = if tim_idx < 4 {
                period_ticks
            } else {
                period_ticks & 0xFFFF
            };
            unsafe { regs.tim_arr().as_ptr().write_volatile(arr) };

            // Prescaler: RTIM_TimeBaseStructInit default is 0; basic timers ignore
            // PSC and the SDK forces 0 for them regardless.
            regs.tim_psc().write(|w| unsafe { w.psc().bits(0) });

            // CR from RTIM_TimeBaseStructInit defaults: ARRProtection=ENABLE
            // (ARPE=1), UpdateSource=Overflow (URS=1), UpdateEvent=ENABLE (UDIS=0).
            regs.tim_cr()
                .modify(|_, w| w.arpe().set_bit().urs().set_bit().udis().clear_bit());

            // Trigger UG to load PSC/ARR and reset the counter.
            regs.tim_egr().write(|w| w.ug().set_bit());
            while !regs.tim_sr().read().ug_done().bit_is_set() {}
            let sr = regs.tim_sr().read().bits();
            unsafe { regs.tim_sr().as_ptr().write_volatile(sr) };

            // RTIM_Cmd(ENABLE).
            if !regs.tim_en().read().cen().bit_is_set() {
                regs.tim_en().write(|w| w.cnt_start().set_bit());
                while !regs.tim_en().read().cen().bit_is_set() {}
            }
        } else {
            // RTIM_Cmd(DISABLE).
            if regs.tim_en().read().cen().bit_is_set() {
                regs.tim_en().write(|w| w.cnt_stop().set_bit());
                while regs.tim_en().read().cen().bit_is_set() {}
            }
        }
    }

    /// Enable or disable the oversampling engine.
    pub fn oversample_cmd(&self, enable: bool) {
        self.r().reg_adc_oversample().modify(|_, w| {
            if enable {
                w.bit_adc_os_en().set_bit()
            } else {
                w.bit_adc_os_en().clear_bit()
            }
        });
    }

    /// Configure oversampling parameters.
    ///
    /// Call `oversample_cmd(true)` to activate after configuring.
    pub fn set_oversample(&self, shift: OsShift, ratio: OsRatio, mode: OsMode) {
        self.r().reg_adc_oversample().modify(|_, w| unsafe {
            let w = w.bit_adc_osf().bits(shift as u8);
            let w = w.bit_adc_osr().bits(ratio as u8);
            match mode {
                OsMode::Regular => w.bit_ov_mode().set_bit(),
                OsMode::Staggered => w.bit_ov_mode().clear_bit(),
            }
        });
    }

    /// Set the ADC dummy cycle time and CapTouch-to-ADC settle time.
    ///
    /// `dummy_us`  — dummy channel time in µs (`DummyCnt = dummy_us * 4`).
    /// `settle_us` — settle time in µs (`SettleCnt = settle_us * 4`, max 3).
    pub fn set_dummy_cycle(&self, dummy_us: u32, settle_us: u8) {
        let r = self.r();
        let dummy_cnt = dummy_us.saturating_mul(4) & 0x01FF_FFFF;
        let settle_cnt = ((settle_us as u32).saturating_mul(4)) & 0x0F;

        r.reg_adc_dummy_cnt()
            .modify(|_, w| unsafe { w.bit_adc_dummy_chn_scnt().bits(dummy_cnt) });

        r.reg_adc_settle()
            .modify(|_, w| unsafe { w.bit_settle_ctc_to_adc().bits(settle_cnt as u8) });
    }

    /// Keep ADC power on even after a conversion-triggered shutdown.
    pub fn keep_power_cmd(&self, enable: bool) {
        self.r().reg_adc_pwr_ctrl().modify(|_, w| {
            if enable {
                w.bit_adc_pwr_keep().set_bit()
            } else {
                w.bit_adc_pwr_keep().clear_bit()
            }
        });
    }

    // ── private helpers ───────────────────────────────────────────────────

    /// Write the HCnt clock divider for one channel.
    ///
    /// The PAC exposes a distinct per-channel field name for each register
    /// (e.g. `bit_adc_clk_hcnt0`, `bit_adc_clk_hcnt1`, …) rather than a
    /// generic indexed accessor.
    fn write_hcnt(&self, channel: usize, hcnt: u16) {
        let r = regs();
        let v = hcnt & 0x0FFF;
        match channel {
            0 => r
                .reg_adc_clk_hcnt0_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt0().bits(v) }),
            1 => r
                .reg_adc_clk_hcnt1_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt1().bits(v) }),
            2 => r
                .reg_adc_clk_hcnt2_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt2().bits(v) }),
            3 => r
                .reg_adc_clk_hcnt3_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt3().bits(v) }),
            4 => r
                .reg_adc_clk_hcnt4_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt4().bits(v) }),
            5 => r
                .reg_adc_clk_hcnt5_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt5().bits(v) }),
            6 => r
                .reg_adc_clk_hcnt6_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt6().bits(v) }),
            7 => r
                .reg_adc_clk_hcnt7_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt7().bits(v) }),
            8 => r
                .reg_adc_clk_hcnt8_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt8().bits(v) }),
            9 => r
                .reg_adc_clk_hcnt9_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt9().bits(v) }),
            10 => r
                .reg_adc_clk_hcnt10_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt10().bits(v) }),
            11 => r
                .reg_adc_clk_hcnt11_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt11().bits(v) }),
            12 => r
                .reg_adc_clk_hcnt12_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt12().bits(v) }),
            13 => r
                .reg_adc_clk_hcnt13_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt13().bits(v) }),
            14 => r
                .reg_adc_clk_hcnt14_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt14().bits(v) }),
            15 => r
                .reg_adc_clk_hcnt15_div()
                .modify(|_, w| unsafe { w.bit_adc_clk_hcnt15().bits(v) }),
            _ => 0,
        };
    }

    /// Program `ADC_CHSW_LIST_0` and `ADC_CHSW_LIST_1` from a slice.
    ///
    /// Each 4-bit slot in `LIST_0` holds channels 0–7; `LIST_1` holds 8–15.
    /// Builds both 32-bit words before writing to avoid extra reads.
    fn program_chsw_list(&self, list: &[u8], len: usize) {
        let r = self.r();
        let mut word0: u32 = 0;
        let mut word1: u32 = 0;
        for (idx, &item) in list[..len].iter().enumerate() {
            let ch = (item & 0x0F) as u32;
            if idx < 8 {
                word0 |= ch << (idx * 4);
            } else {
                word1 |= ch << ((idx - 8) * 4);
            }
        }
        r.reg_adc_chsw_list_0().write(|w| unsafe { w.bits(word0) });
        r.reg_adc_chsw_list_1().write(|w| unsafe { w.bits(word1) });
    }

    /// Write the comparison thresholds for channel `ch` (0–10).
    ///
    /// Each register has channel-specific PAC field names.
    fn write_comp_th(&self, ch: usize, high: u16, low: u16) {
        let r = regs();
        match ch {
            0 => r.reg_adc_comp_th_ch0().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch0()
                    .bits(high)
                    .bit_adc_comp_th_l_ch0()
                    .bits(low)
            }),
            1 => r.reg_adc_comp_th_ch1().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch1()
                    .bits(high)
                    .bit_adc_comp_th_l_ch1()
                    .bits(low)
            }),
            2 => r.reg_adc_comp_th_ch2().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch2()
                    .bits(high)
                    .bit_adc_comp_th_l_ch2()
                    .bits(low)
            }),
            3 => r.reg_adc_comp_th_ch3().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch3()
                    .bits(high)
                    .bit_adc_comp_th_l_ch3()
                    .bits(low)
            }),
            4 => r.reg_adc_comp_th_ch4().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch4()
                    .bits(high)
                    .bit_adc_comp_th_l_ch4()
                    .bits(low)
            }),
            5 => r.reg_adc_comp_th_ch5().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch5()
                    .bits(high)
                    .bit_adc_comp_th_l_ch5()
                    .bits(low)
            }),
            6 => r.reg_adc_comp_th_ch6().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch6()
                    .bits(high)
                    .bit_adc_comp_th_l_ch6()
                    .bits(low)
            }),
            7 => r.reg_adc_comp_th_ch7().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch7()
                    .bits(high)
                    .bit_adc_comp_th_l_ch7()
                    .bits(low)
            }),
            8 => r.reg_adc_comp_th_ch8().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch8()
                    .bits(high)
                    .bit_adc_comp_th_l_ch8()
                    .bits(low)
            }),
            9 => r.reg_adc_comp_th_ch9().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch9()
                    .bits(high)
                    .bit_adc_comp_th_l_ch9()
                    .bits(low)
            }),
            10 => r.reg_adc_comp_th_ch10().write(|w| unsafe {
                w.bit_adc_comp_th_h_ch10()
                    .bits(high)
                    .bit_adc_comp_th_l_ch10()
                    .bits(low)
            }),
            _ => 0,
        };
    }

    /// Write the 2-bit comparison criteria for channel `ch` into `ADC_COMP_CTRL`.
    fn write_comp_ctrl(&self, ch: usize, val: u8) {
        let r = self.r();
        match ch {
            0 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch0().bits(val) }),
            1 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch1().bits(val) }),
            2 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch2().bits(val) }),
            3 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch3().bits(val) }),
            4 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch4().bits(val) }),
            5 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch5().bits(val) }),
            6 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch6().bits(val) }),
            7 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch7().bits(val) }),
            8 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch8().bits(val) }),
            9 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch9().bits(val) }),
            10 => r
                .reg_adc_comp_ctrl()
                .modify(|_, w| unsafe { w.bit_adc_comp_ctrl_ch10().bits(val) }),
            _ => 0,
        };
    }
}
