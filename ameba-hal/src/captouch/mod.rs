//! RTL8721F CapTouch (CTC) driver — idiomatic Rust, no_std, PAC-based.
//!
//! # Hardware summary
//! - Base address: 0x4081_3000 (Capacitive Touch Controller)
//! - Channels: 9 (CH0–CH8)
//! - PAC type: `rtl8721f_pac::CtcReg`
//!
//! # Usage sketch
//! ```ignore
//! let mut cfg = CapTouchConfig::default();
//! cfg.channels[0] = ChannelConfig {
//!     enabled: true,
//!     diff_threshold: 0xf0,
//!     mbias_current: 0x200,
//!     etc_n_noise_thr: 100,
//!     etc_p_noise_thr: 100,
//! };
//! let ctc_dis = unsafe { CapTouch::steal() }.init(&cfg);
//! let ctc = ctc_dis.enable();
//! ctc.int_config(IntrFlags::TOUCH_PRESS_CH0, true);
//! ```

use bitflags::bitflags;
use rtl8721f_pac::{CtcReg, ctc_reg};

// ─────────────────────────────────────────────────────────────────────────────
// Private ROM / OTP bindings
// ─────────────────────────────────────────────────────────────────────────────

mod sys {
    pub const CTC_MBIAS_CAL_ADDR: u32 = 0x704; // SDK: ameba_captouch.h : CTC_MBIAS_CAL_ADDR
}

// ─────────────────────────────────────────────────────────────────────────────
// PAC register-block accessor
// ─────────────────────────────────────────────────────────────────────────────

#[inline(always)]
fn regs() -> &'static ctc_reg::RegisterBlock {
    // SAFETY: CtcReg::PTR is the PAC-derived pointer for the CapTouch peripheral.
    unsafe { &*CtcReg::PTR }
}

// ─────────────────────────────────────────────────────────────────────────────
// Interrupt flag set
// ─────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// Combined interrupt-enable / interrupt-status / interrupt-clear flags.
    ///
    /// Bit layout mirrors the INTERRUPT_ENABLE / INTERRUPT_STATUS_CLR registers:
    /// - touch_press_intr_en   → bits [8:0]  (one per channel)
    /// - touch_release_intr_en → bits [17:9] (one per channel)
    /// - over_p_noise_th       → bit 18
    /// - afifo_overflow        → bit 19
    /// - over_n_noise_th       → bit 20
    /// - afifo_overlvl         → bit 21
    /// - ctc_brk_by_adc        → bit 22
    /// - scan_end              → bit 23
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct IntrFlags: u32 {
        const TOUCH_PRESS_CH0   = 1 << 0;
        const TOUCH_PRESS_CH1   = 1 << 1;
        const TOUCH_PRESS_CH2   = 1 << 2;
        const TOUCH_PRESS_CH3   = 1 << 3;
        const TOUCH_PRESS_CH4   = 1 << 4;
        const TOUCH_PRESS_CH5   = 1 << 5;
        const TOUCH_PRESS_CH6   = 1 << 6;
        const TOUCH_PRESS_CH7   = 1 << 7;
        const TOUCH_PRESS_CH8   = 1 << 8;
        const TOUCH_RELEASE_CH0 = 1 << 9;
        const TOUCH_RELEASE_CH1 = 1 << 10;
        const TOUCH_RELEASE_CH2 = 1 << 11;
        const TOUCH_RELEASE_CH3 = 1 << 12;
        const TOUCH_RELEASE_CH4 = 1 << 13;
        const TOUCH_RELEASE_CH5 = 1 << 14;
        const TOUCH_RELEASE_CH6 = 1 << 15;
        const TOUCH_RELEASE_CH7 = 1 << 16;
        const TOUCH_RELEASE_CH8 = 1 << 17;
        const OVER_P_NOISE_TH   = 1 << 18;
        const AFIFO_OVERFLOW    = 1 << 19;
        const OVER_N_NOISE_TH   = 1 << 20;
        const AFIFO_OVERLVL     = 1 << 21;
        const CTC_BRK_BY_ADC    = 1 << 22;
        const SCAN_END          = 1 << 23;
        const ALL_PRESS         = 0x01ff;
        const ALL_RELEASE       = 0x01ff << 9;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Public type-safe enums
// ─────────────────────────────────────────────────────────────────────────────

/// Median filter coefficient selection (maps to `med_coef` PAC field).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MedCoef {
    /// Median filter disabled.
    Disabled = 0,
    /// Window size 3.
    Coef3 = 1,
    /// Window size 5.
    Coef5 = 2,
    /// Window size 7.
    Coef7 = 3,
}

/// Over-sample mode (`ov_mode` PAC field).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampMode {
    /// All conversions in regular (sequential) order.
    Regular,
    /// All conversions in staggered order.
    Staggered,
}

/// Trigger mode for CTC sampling (`ctc_trig_mode` PAC field).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrigMode {
    /// CTC triggered when a channel is enabled.
    ChEn,
    /// CTC triggered in dummy cycle only.
    Dummy,
}

/// Which noise threshold to query.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoiseType {
    Positive,
    Negative,
}

/// Debug-mode channel switch control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChSwitchMode {
    /// Auto-advance through enabled channels.
    Auto,
    /// Lock to one specific channel (manual mode).
    Manual(Channel),
}

/// Valid channel index (0–8).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Channel(u8);

impl Channel {
    /// Create a `Channel`, panicking if `idx > 8`.
    pub fn new(idx: u8) -> Self {
        assert!(idx <= 8, "channel index must be 0..=8");
        Channel(idx)
    }

    /// Create a `Channel` without bounds checking.
    ///
    /// # Safety
    /// `idx` must be in 0..=8.
    pub(crate) unsafe fn new_unchecked(idx: u8) -> Self {
        Channel(idx)
    }

    pub fn index(self) -> u8 {
        self.0
    }
}

/// Raw FIFO entry returned by `dbg_raw_data`.
#[derive(Clone, Copy, Debug)]
pub struct RawEntry {
    pub data: u16,
    pub ch: u8,
}

// ─────────────────────────────────────────────────────────────────────────────
// Configuration structures
// ─────────────────────────────────────────────────────────────────────────────

/// Per-channel configuration.
#[derive(Clone, Copy, Debug)]
pub struct ChannelConfig {
    pub enabled: bool,
    /// Differential touch threshold (0..=0xFFF).
    pub diff_threshold: u16,
    /// Mbias current code (raw 16-bit register value; use `compute_mbias_para` to derive from nA).
    pub mbias_current: u16,
    /// ETC negative noise threshold (0..=0xFFF).
    pub etc_n_noise_thr: u16,
    /// ETC positive noise threshold (0..=0xFFF).
    pub etc_p_noise_thr: u16,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        ChannelConfig {
            enabled: false,
            diff_threshold: 0xf0,
            mbias_current: 0x0200,
            etc_n_noise_thr: 100,
            etc_p_noise_thr: 100,
        }
    }
}

/// Top-level CapTouch configuration (builder-style via `Default` + field updates).
#[derive(Clone, Debug)]
pub struct CapTouchConfig {
    pub debounce_en: bool,
    /// Sample count exponent; actual samples = 2^(sample_cnt+2). Range 0–7.
    pub sample_cnt: u8,
    /// Scan interval in ms (0–0xFFF).
    pub scan_interval: u16,
    /// Sample period in µs (must produce ClkCnt in 4..0x200).
    pub sample_period_us: u16,
    /// ETC baseline update step (0–0xF).
    pub etc_step: u8,
    /// ETC baseline update weight factor exponent (0–0xF).
    pub etc_factor: u8,
    /// ETC scan interval (0–0x7F).
    pub etc_scan_interval: u8,
    pub med_coef: MedCoef,
    /// IIR normal coefficient (0–11).
    pub iir_coef_norm: u8,
    /// IIR accelerated coefficient (0–11).
    pub iir_coef_accel: u8,
    /// Per-channel configurations; indices 0..=8.
    pub channels: [ChannelConfig; 9],
}

impl Default for CapTouchConfig {
    fn default() -> Self {
        CapTouchConfig {
            debounce_en: true,
            sample_cnt: 6,
            scan_interval: 60,
            sample_period_us: 6,
            etc_step: 1,
            etc_factor: 4,
            etc_scan_interval: 3,
            med_coef: MedCoef::Coef3,
            iir_coef_norm: 5,
            iir_coef_accel: 3,
            channels: [ChannelConfig::default(); 9],
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Mbias calibration table (mirrors C `Mbias_Config[16]`)
// ─────────────────────────────────────────────────────────────────────────────

struct MbiasEntry {
    current_na: u32,
    /// Byte index within the 12-byte OTP calibration region.
    otp_offset: u8,
    otp_start: u8,
    otp_end: u8,
}

static MBIAS_TABLE: [MbiasEntry; 16] = [
    MbiasEntry {
        current_na: 64000,
        otp_offset: 0x0,
        otp_start: 0x0,
        otp_end: 0xA,
    },
    MbiasEntry {
        current_na: 32000,
        otp_offset: 0x2,
        otp_start: 0x0,
        otp_end: 0x9,
    },
    MbiasEntry {
        current_na: 32000,
        otp_offset: 0x4,
        otp_start: 0x0,
        otp_end: 0x9,
    },
    MbiasEntry {
        current_na: 16000,
        otp_offset: 0x6,
        otp_start: 0x0,
        otp_end: 0x8,
    },
    MbiasEntry {
        current_na: 8000,
        otp_offset: 0x8,
        otp_start: 0x0,
        otp_end: 0x7,
    },
    MbiasEntry {
        current_na: 8000,
        otp_offset: 0x8,
        otp_start: 0x0,
        otp_end: 0x7,
    },
    MbiasEntry {
        current_na: 4000,
        otp_offset: 0xA,
        otp_start: 0x0,
        otp_end: 0x6,
    },
    MbiasEntry {
        current_na: 2000,
        otp_offset: 0x2,
        otp_start: 0xA,
        otp_end: 0xF,
    },
    MbiasEntry {
        current_na: 2000,
        otp_offset: 0x2,
        otp_start: 0xA,
        otp_end: 0xF,
    },
    MbiasEntry {
        current_na: 1000,
        otp_offset: 0x4,
        otp_start: 0xA,
        otp_end: 0xF,
    },
    MbiasEntry {
        current_na: 500,
        otp_offset: 0x0,
        otp_start: 0xB,
        otp_end: 0xF,
    },
    MbiasEntry {
        current_na: 500,
        otp_offset: 0x0,
        otp_start: 0xB,
        otp_end: 0xF,
    },
    MbiasEntry {
        current_na: 250,
        otp_offset: 0x6,
        otp_start: 0x9,
        otp_end: 0xB,
    },
    MbiasEntry {
        current_na: 250,
        otp_offset: 0x6,
        otp_start: 0x9,
        otp_end: 0xB,
    },
    MbiasEntry {
        current_na: 250,
        otp_offset: 0x6,
        otp_start: 0x9,
        otp_end: 0xB,
    },
    MbiasEntry {
        current_na: 250,
        otp_offset: 0x6,
        otp_start: 0x9,
        otp_end: 0xB,
    },
];

// ─────────────────────────────────────────────────────────────────────────────
// Type-state markers
// ─────────────────────────────────────────────────────────────────────────────

pub struct Disabled;
pub struct Enabled;

// ─────────────────────────────────────────────────────────────────────────────
// Driver struct
// ─────────────────────────────────────────────────────────────────────────────

pub struct CapTouch<State> {
    _state: core::marker::PhantomData<State>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Disabled state: steal + init + enable
// ─────────────────────────────────────────────────────────────────────────────

impl CapTouch<Disabled> {
    /// Obtain a handle to the CTC peripheral by pointing at the fixed MMIO base.
    ///
    /// # Safety
    /// Must be called at most once; no other code may alias CTC registers
    /// concurrently for the lifetime of the returned handle.
    pub unsafe fn steal() -> Self {
        CapTouch {
            _state: core::marker::PhantomData,
        }
    }

    /// Apply `cfg` and return an initialized-but-disabled handle.
    ///
    /// Peripheral is left disabled after this call; call `enable()` to start
    /// scanning.
    pub fn init(self, cfg: &CapTouchConfig) -> CapTouch<Disabled> {
        let r = regs();

        // Disable CTC and clear all interrupts.
        r.ctc_ctrl().modify(|_, w| w.ct_enable().clear_bit());
        unsafe {
            r.interrupt_enable().write_with_zero(|w| w);
        }
        r.interrupt_all_clr()
            .modify(|_, w| w.intr_all_clr().set_bit());

        // Sample clock: HCNT = period_us * 4 - 4; LCNT0/1/2 = 2/3/4.
        // From C: CT_CTC_CLK_HCNT(CT_SamplePeriodUs * 4 - 4)
        let hcnt: u16 = (cfg.sample_period_us * 4).saturating_sub(4);
        r.ctc_div_hcnt()
            .modify(|_, w| unsafe { w.ctc_clk_hcnt().bits(hcnt) });
        // LCNT0=2, LCNT1=3, LCNT2=4 — initial clock divider setup.
        r.ctc_div_lcnt().modify(|_, w| unsafe {
            w.ctc_clk_lcnt0()
                .bits(0x2)
                .ctc_clk_lcnt1()
                .bits(0x3)
                .ctc_clk_lcnt2()
                .bits(0x4)
        });

        // Set baseline-init bit and optional debounce.
        r.ctc_ctrl().modify(|_, w| {
            let w = w.baseline_ini().set_bit();
            if cfg.debounce_en {
                w.debounce_en().set_bit()
            } else {
                w.debounce_en().clear_bit()
            }
        });

        // Scan period register: sleep_time_cnt [11:0] and sample_ave_ctrl [18:16].
        r.scan_period().modify(|_, w| unsafe {
            w.sleep_time_cnt()
                .bits(cfg.scan_interval)
                .sample_ave_ctrl()
                .bits(cfg.sample_cnt)
        });

        // ETC control: enable ETC, set scan interval, weight factor, update step.
        r.etc_ctrl().modify(|_, w| unsafe {
            w.etc_func_ctrl()
                .set_bit()
                .etc_scan_interval()
                .bits(cfg.etc_scan_interval)
                .baseline_wt_factor()
                .bits(cfg.etc_factor)
                .baseline_upd_step()
                .bits(cfg.etc_step)
        });

        // Debug mode register: enable auto channel switch (ch_switch_ctrl = 1).
        r.debug_mode_ctrl()
            .modify(|_, w| w.ch_switch_ctrl().set_bit());

        // Enable diode protect.
        r.ctc_analog().modify(|_, w| w.reg_diode().set_bit());

        // Median filter coefficient.
        r.ctc_filter_ctrl0()
            .modify(|_, w| unsafe { w.med_coef().bits(cfg.med_coef as u8) });

        // IIR filter: accelerated mode on, set both coefficients.
        r.ctc_filter_ctrl1().modify(|_, w| unsafe {
            w.iir_k_normal()
                .bits(cfg.iir_coef_norm)
                .iir_k_acc_en()
                .set_bit()
                .iir_k_acc()
                .bits(cfg.iir_coef_accel)
        });

        // Decimation = 0.
        r.ctc_filter_ctrl0()
            .modify(|_, w| unsafe { w.mdec().bits(0) });

        // Enable IIR filter and global filter.
        r.ctc_filter_ctrl0()
            .modify(|_, w| w.iir_en().set_bit().flt_en().set_bit());

        // Ave-median config: LCNT1=3, step=1 → LCNT0=2, LCNT2=4; enable med1.
        r.ctc_div_lcnt().modify(|_, w| unsafe {
            w.ctc_clk_lcnt0()
                .bits(0x2)
                .ctc_clk_lcnt1()
                .bits(0x3)
                .ctc_clk_lcnt2()
                .bits(0x4)
        });
        r.ctc_filter_ctrl0().modify(|_, w| w.med1_en().set_bit());

        // Configure each enabled channel.
        for (i, ch_cfg) in cfg.channels.iter().enumerate() {
            if ch_cfg.enabled {
                let ch = unsafe { Channel::new_unchecked(i as u8) };
                ch_set_enabled(ch, true);
                ch_set_diff_threshold(ch, ch_cfg.diff_threshold);
                ch_set_mbias(ch, ch_cfg.mbias_current);
                ch_set_noise_thresholds(ch, ch_cfg.etc_p_noise_thr, ch_cfg.etc_n_noise_thr);
            }
        }

        CapTouch {
            _state: core::marker::PhantomData,
        }
    }

    /// Enable the CTC peripheral, triggering baseline initialization and scan start.
    pub fn enable(self) -> CapTouch<Enabled> {
        regs()
            .ctc_ctrl()
            .modify(|_, w| w.ct_enable().set_bit().baseline_ini().set_bit());
        CapTouch {
            _state: core::marker::PhantomData,
        }
    }

    // ── Configuration methods valid only when peripheral is disabled ──────────

    /// Reconfigure the sample clock period (µs).
    pub fn set_clk(&self, period_us: u16) {
        let r = regs();
        let lcnt1 = r.ctc_div_lcnt().read().ctc_clk_lcnt1().bits();
        let clk_cnt = period_us * 4;
        // hcnt = clk_cnt - 1 - lcnt1
        let hcnt = clk_cnt.saturating_sub(1).saturating_sub(lcnt1);
        r.ctc_div_hcnt()
            .modify(|_, w| unsafe { w.ctc_clk_hcnt().bits(hcnt) });
    }

    /// Set per-channel Mbias current code.
    pub fn set_ch_mbias(&self, ch: Channel, mbias: u16) {
        ch_set_mbias(ch, mbias);
    }

    /// Set per-channel differential touch threshold.
    pub fn set_ch_diff_thres(&self, ch: Channel, threshold: u16) {
        ch_set_diff_threshold(ch, threshold);
    }

    /// Set per-channel negative noise threshold.
    pub fn set_n_noise_thres(&self, ch: Channel, threshold: u16) {
        let r = regs();
        ch_modify_noise(r, ch, |p, _n| (p, threshold));
    }

    /// Set per-channel positive noise threshold.
    pub fn set_p_noise_thres(&self, ch: Channel, threshold: u16) {
        let r = regs();
        ch_modify_noise(r, ch, |_p, n| (threshold, n));
    }

    /// Enable or disable the global filter.
    pub fn filter_cmd(&self, enable: bool) {
        regs().ctc_filter_ctrl0().modify(|_, w| {
            if enable {
                w.flt_en().set_bit()
            } else {
                w.flt_en().clear_bit()
            }
        });
    }

    /// Enable or disable the IIR filter.
    pub fn filter_iir_cmd(&self, enable: bool) {
        regs().ctc_filter_ctrl0().modify(|_, w| {
            if enable {
                w.iir_en().set_bit()
            } else {
                w.iir_en().clear_bit()
            }
        });
    }

    /// Configure IIR filter coefficients and optional accelerated mode.
    pub fn filter_iir_config(&self, accel_en: bool, coef_norm: u8, coef_accel: u8) {
        regs().ctc_filter_ctrl1().modify(|_, w| unsafe {
            let w = w.iir_k_normal().bits(coef_norm);
            if accel_en {
                w.iir_k_acc_en().set_bit().iir_k_acc().bits(coef_accel)
            } else {
                w.iir_k_acc_en().clear_bit()
            }
        });
    }

    /// Set the MDEC (decimation factor) value.
    pub fn set_mdec(&self, mdec: u16) {
        regs()
            .ctc_filter_ctrl0()
            .modify(|_, w| unsafe { w.mdec().bits(mdec) });
    }

    /// Compute the MDEC value required to reject a given AC mains frequency.
    pub fn get_mdec(&self, ac_freq_hz: u32) -> u32 {
        let r = regs();
        let hcnt = r.ctc_div_hcnt().read().ctc_clk_hcnt().bits() as u32;
        let lcnt1 = r.ctc_div_lcnt().read().ctc_clk_lcnt1().bits() as u32;
        let clk_cnt = hcnt + lcnt1;
        let sample_cnt = r.scan_period().read().sample_ave_ctrl().bits() as u32;
        // mdec = 4_000_000 / clk_cnt / (ac_freq * 2^(sample_cnt+2)) - 1
        4_000_000 / clk_cnt / (ac_freq_hz * (1u32 << (sample_cnt + 2))) - 1
    }

    /// Set the median filter coefficient.
    pub fn set_med_coef(&self, coef: MedCoef) {
        regs()
            .ctc_filter_ctrl0()
            .modify(|_, w| unsafe { w.med_coef().bits(coef as u8) });
    }

    /// Configure the averaged-median (LCNT0/1/2) filter.
    ///
    /// `val` is the LCNT1 value; `step` is the offset between LCNT0/1/2.
    /// When `step == 0`, the averaged-median filter is disabled.
    pub fn ave_med_config(&self, val: u16, step: u16) {
        regs().ctc_div_lcnt().modify(|_, w| unsafe {
            w.ctc_clk_lcnt0()
                .bits(val.saturating_sub(step))
                .ctc_clk_lcnt1()
                .bits(val)
                .ctc_clk_lcnt2()
                .bits(val + step)
        });
        regs().ctc_filter_ctrl0().modify(|_, w| {
            if step != 0 {
                w.med1_en().set_bit()
            } else {
                w.med1_en().clear_bit()
            }
        });
    }

    /// Configure over-sample mode and sample count exponent.
    pub fn oversamp_config(&self, mode: SampMode, samp_cnt: u8) {
        regs().ctc_ov_mode().modify(|_, w| {
            if mode == SampMode::Staggered {
                w.ov_mode().set_bit()
            } else {
                w.ov_mode().clear_bit()
            }
        });
        regs()
            .scan_period()
            .modify(|_, w| unsafe { w.sample_ave_ctrl().bits(samp_cnt) });
    }

    /// Set the staggered-mode wait time (in sample clocks).
    pub fn set_wait_time(&self, wait_time: u32) {
        regs()
            .ctc_ov_mode()
            .modify(|_, w| unsafe { w.ov_wait_timer_cnt().bits(wait_time) });
    }

    /// Set the CTC trigger mode.
    pub fn set_trig_mode(&self, mode: TrigMode) {
        regs().ctc_ov_mode().modify(|_, w| {
            if mode == TrigMode::Dummy {
                w.ctc_trig_mode().set_bit()
            } else {
                w.ctc_trig_mode().clear_bit()
            }
        });
    }

    /// Configure the RX DMA burst size field (`rxdma_msize`).
    pub fn rxdma_config(&self, burst_size: u8) {
        regs()
            .ctc_dma()
            .modify(|_, w| unsafe { w.rxdma_msize().bits(burst_size) });
    }

    /// Enable or disable RX DMA.
    pub fn rxdma_cmd(&self, enable: bool) {
        regs().ctc_dma().modify(|_, w| {
            if enable {
                w.rxdma_en().set_bit()
            } else {
                w.rxdma_en().clear_bit()
            }
        });
    }

    /// Enable or disable debug mode.
    pub fn dbg_cmd(&self, enable: bool) {
        if enable {
            regs()
                .debug_mode_ctrl()
                .modify(|_, w| w.debug_en().set_bit());
        } else {
            regs()
                .debug_mode_ctrl()
                .modify(|_, w| w.ch_switch_ctrl().set_bit().debug_en().clear_bit());
        }
    }

    /// Set the debug channel switch mode.
    pub fn dbg_channel(&self, mode: ChSwitchMode) {
        regs().debug_mode_ctrl().modify(|_, w| match mode {
            ChSwitchMode::Auto => w.ch_switch_ctrl().set_bit(),
            ChSwitchMode::Manual(ch) => unsafe {
                w.ch_switch_ctrl().clear_bit().ch_ctrl().bits(ch.0)
            },
        });
    }

    /// Enable or disable continuous debug sampling.
    pub fn dbg_cont_cmd(&self, enable: bool) {
        regs().debug_mode_ctrl().modify(|_, w| {
            if enable {
                w.continue_sample_en().set_bit()
            } else {
                w.continue_sample_en().clear_bit()
            }
        });
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Enabled state: runtime control and status reads
// ─────────────────────────────────────────────────────────────────────────────

impl CapTouch<Enabled> {
    /// Disable the CTC peripheral.
    pub fn disable(self) -> CapTouch<Disabled> {
        regs().ctc_ctrl().modify(|_, w| w.ct_enable().clear_bit());
        CapTouch {
            _state: core::marker::PhantomData,
        }
    }

    /// Enable or disable specific interrupt(s) by flag mask.
    pub fn int_config(&self, flags: IntrFlags, enable: bool) {
        let r = regs();
        let cur = r.interrupt_enable().read().bits();
        let new = if enable {
            cur | flags.bits()
        } else {
            cur & !flags.bits()
        };
        r.interrupt_enable().write(|w| unsafe { w.bits(new) });
    }

    /// Clear specific interrupt pending bits via INTERRUPT_STATUS_CLR (write-1-to-clear).
    pub fn clear_pending(&self, flags: IntrFlags) {
        let r = regs();
        let cur = r.interrupt_status_clr().read().bits();
        r.interrupt_status_clr()
            .write(|w| unsafe { w.bits(cur | flags.bits()) });
    }

    /// Clear all interrupts via INTERRUPT_ALL_CLR.
    pub fn clear_all_pending(&self) {
        regs()
            .interrupt_all_clr()
            .modify(|_, w| w.intr_all_clr().set_bit());
    }

    /// Read the masked interrupt status register.
    pub fn isr(&self) -> u32 {
        regs().interrupt_status().read().bits()
    }

    /// Read the raw (unmasked) interrupt status register.
    pub fn raw_isr(&self) -> u32 {
        regs().raw_interrupt_status().read().bits()
    }

    /// Return whether a given channel is currently enabled.
    pub fn ch_enabled(&self, ch: Channel) -> bool {
        ch_get_enabled(ch)
    }

    /// Enable or disable a single channel at runtime.
    pub fn ch_cmd(&self, ch: Channel, enable: bool) {
        ch_set_enabled(ch, enable);
    }

    /// Set the AFIFO over-level threshold (0–63).
    pub fn set_fifo_level(&self, level: u8) {
        regs()
            .raw_code_fifo_status()
            .modify(|_, w| unsafe { w.afifo_full_lvl().bits(level & 0x3f) });
    }

    /// Set scan interval (ms) while the peripheral is running.
    pub fn set_scan_interval(&self, interval_ms: u16) {
        regs()
            .scan_period()
            .modify(|_, w| unsafe { w.sleep_time_cnt().bits(interval_ms) });
    }

    /// Poll the debug raw data FIFO until a valid entry appears.
    ///
    /// Returns `None` if no valid entry appears within 200 000 iterations
    /// (equivalent to C's `CapTouch_DbgRawData` timeout returning `0xEAEA`).
    pub fn dbg_raw_data(&self) -> Option<RawEntry> {
        let r = regs();
        let mut loops: u32 = 0;
        loop {
            let v = r.raw_code_fifo_read().read();
            if v.afifo_rd_data_vld().bit_is_set() {
                return Some(RawEntry {
                    data: v.afifo_rd_data().bits(),
                    ch: v.afifo_rd_ch_num().bits(),
                });
            }
            loops += 1;
            if loops > 200_000 {
                return None;
            }
        }
    }

    /// Read the Mbias current code for a channel.
    pub fn ch_mbias(&self, ch: Channel) -> u16 {
        ch_get_mbias(ch)
    }

    /// Read the differential touch threshold for a channel.
    pub fn ch_diff_thres(&self, ch: Channel) -> u16 {
        ch_get_diff_threshold(ch)
    }

    /// Read the absolute touch threshold (HW-maintained) for a channel.
    pub fn ch_abs_thres(&self, ch: Channel) -> u16 {
        ch_get_abs_threshold(ch)
    }

    /// Read a noise threshold for a channel.
    pub fn noise_thres(&self, ch: Channel, noise: NoiseType) -> u16 {
        ch_get_noise_threshold(ch, noise)
    }

    /// Read the HW-maintained baseline for a channel.
    pub fn ch_baseline(&self, ch: Channel) -> u16 {
        ch_get_baseline(ch)
    }

    /// Read the averaged raw data for a channel.
    pub fn ch_ave_data(&self, ch: Channel) -> u16 {
        ch_get_ave_data(ch)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Mbias calibration helpers (free functions; no register access)
// ─────────────────────────────────────────────────────────────────────────────

/// Compute the 16-bit Mbias register code for `mbias_current_na` nA, using
/// calibration data from a 12-byte OTP buffer at addresses 0x704–0x70F.
///
/// Call `read_otp_mbias_cal()` first to obtain the buffer.
pub fn compute_mbias_para(otp_buf: &[u8; 12], mut mbias_current_na: u32) -> u16 {
    let mut currents = [0u32; 16];
    for (idx, entry) in MBIAS_TABLE.iter().enumerate() {
        let start = entry.otp_start as usize;
        let end = entry.otp_end as usize;
        let off = entry.otp_offset as usize;

        let diff: u32 = if start == 0 && end >= 8 {
            // Spans two bytes; upper bits in otp_buf[off+1].
            let hi_mask = (1u8 << (end - 7)).wrapping_sub(1);
            let hi = (otp_buf[off + 1] & hi_mask) as u32;
            let lo = otp_buf[off] as u32;
            (hi << 8) | lo
        } else if start >= 8 {
            // Entirely in the upper byte.
            let shift = start - 8;
            let width = end - start;
            let mask = (0xFFu8 >> (7 - width)) << shift;
            ((otp_buf[off + 1] & mask) >> shift) as u32
        } else {
            // Entirely in the lower byte.
            let width = end - start;
            let mask = (0xFFu8 >> (7 - width)) << start;
            ((otp_buf[off] & mask) >> start) as u32
        };

        // Multiply diff by 62.5 using integer arithmetic: diff * 125 / 2.
        let adj = diff * 125 / 2;
        currents[idx] = entry.current_na + adj;

        // If the sign bit of the calibration field is set, subtract the
        // two's-complement value.
        let sign_bit = 1u32 << (end - start);
        if diff & sign_bit != 0 {
            let neg_adj = (sign_bit << 1) * 125 / 2;
            currents[idx] = currents[idx].saturating_sub(neg_adj);
        }
    }

    let mut val: u16 = 0;
    for (idx, &cur) in currents.iter().enumerate() {
        if mbias_current_na >= cur {
            val |= 1u16 << (15 - idx);
            mbias_current_na -= cur;
        }
    }
    val
}

/// Read the 12-byte Mbias OTP calibration region via `crate::otp::read8`.
pub fn read_otp_mbias_cal() -> [u8; 12] {
    let mut buf = [0u8; 12];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = crate::otp::read8(sys::CTC_MBIAS_CAL_ADDR + i as u32);
    }
    buf
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-channel register helpers
//
// The PAC exposes each channel as a separate top-level accessor (ch0_ctrl,
// ch1_ctrl, …) because the SVD has no array type for channel registers.  We
// dispatch on the channel index using a macro so the match is inlined.
// ─────────────────────────────────────────────────────────────────────────────

/// Dispatch to the correct `chN_ctrl` register, binding the result to `$reg`.
macro_rules! with_ch_ctrl {
    ($ch:expr_2021, |$reg:ident| $body:expr_2021) => {
        match $ch.0 {
            0 => {
                let $reg = regs().ch0_ctrl();
                $body
            }
            1 => {
                let $reg = regs().ch1_ctrl();
                $body
            }
            2 => {
                let $reg = regs().ch2_ctrl();
                $body
            }
            3 => {
                let $reg = regs().ch3_ctrl();
                $body
            }
            4 => {
                let $reg = regs().ch4_ctrl();
                $body
            }
            5 => {
                let $reg = regs().ch5_ctrl();
                $body
            }
            6 => {
                let $reg = regs().ch6_ctrl();
                $body
            }
            7 => {
                let $reg = regs().ch7_ctrl();
                $body
            }
            _ => {
                let $reg = regs().ch8_ctrl();
                $body
            }
        }
    };
}

/// Dispatch to the correct `chN_noise_th` register.
macro_rules! with_ch_noise {
    ($ch:expr_2021, |$reg:ident| $body:expr_2021) => {
        match $ch.0 {
            0 => {
                let $reg = regs().ch0_noise_th();
                $body
            }
            1 => {
                let $reg = regs().ch1_noise_th();
                $body
            }
            2 => {
                let $reg = regs().ch2_noise_th();
                $body
            }
            3 => {
                let $reg = regs().ch3_noise_th();
                $body
            }
            4 => {
                let $reg = regs().ch4_noise_th();
                $body
            }
            5 => {
                let $reg = regs().ch5_noise_th();
                $body
            }
            6 => {
                let $reg = regs().ch6_noise_th();
                $body
            }
            7 => {
                let $reg = regs().ch7_noise_th();
                $body
            }
            _ => {
                let $reg = regs().ch8_noise_th();
                $body
            }
        }
    };
}

/// Dispatch to the correct `chN_mbias_ath` register.
macro_rules! with_ch_mbias {
    ($ch:expr_2021, |$reg:ident| $body:expr_2021) => {
        match $ch.0 {
            0 => {
                let $reg = regs().ch0_mbias_ath();
                $body
            }
            1 => {
                let $reg = regs().ch1_mbias_ath();
                $body
            }
            2 => {
                let $reg = regs().ch2_mbias_ath();
                $body
            }
            3 => {
                let $reg = regs().ch3_mbias_ath();
                $body
            }
            4 => {
                let $reg = regs().ch4_mbias_ath();
                $body
            }
            5 => {
                let $reg = regs().ch5_mbias_ath();
                $body
            }
            6 => {
                let $reg = regs().ch6_mbias_ath();
                $body
            }
            7 => {
                let $reg = regs().ch7_mbias_ath();
                $body
            }
            _ => {
                let $reg = regs().ch8_mbias_ath();
                $body
            }
        }
    };
}

/// Dispatch to the correct `chN_data_inf` register.
macro_rules! with_ch_data {
    ($ch:expr_2021, |$reg:ident| $body:expr_2021) => {
        match $ch.0 {
            0 => {
                let $reg = regs().ch0_data_inf();
                $body
            }
            1 => {
                let $reg = regs().ch1_data_inf();
                $body
            }
            2 => {
                let $reg = regs().ch2_data_inf();
                $body
            }
            3 => {
                let $reg = regs().ch3_data_inf();
                $body
            }
            4 => {
                let $reg = regs().ch4_data_inf();
                $body
            }
            5 => {
                let $reg = regs().ch5_data_inf();
                $body
            }
            6 => {
                let $reg = regs().ch6_data_inf();
                $body
            }
            7 => {
                let $reg = regs().ch7_data_inf();
                $body
            }
            _ => {
                let $reg = regs().ch8_data_inf();
                $body
            }
        }
    };
}

// Because all CHN_CTRL registers have the same bit layout but are different
// PAC types, we must use the raw `.bits()` reader/writer only for the
// CH_EN bit (bit 0) and named field writers for multi-bit fields.
// For ch_en (single bit) we use set_bit/clear_bit via .bits() approach
// since all registers share the same unnamed bit-0 position.  The
// `ch0_en`, `ch1_en`, … field names differ per register type, so we
// access that single bit via read().bits() & 1 and write via bit
// manipulation of the whole word — this is the idiomatic PAC path for
// single-bit fields when using polymorphic dispatch.

fn ch_get_enabled(ch: Channel) -> bool {
    with_ch_ctrl!(ch, |r| (r.read().bits() & 1) != 0)
}

fn ch_set_enabled(ch: Channel, en: bool) {
    with_ch_ctrl!(ch, |r| {
        let cur = r.read().bits();
        r.write(|w| unsafe { w.bits(if en { cur | 1 } else { cur & !1 }) });
    });
}

fn ch_set_diff_threshold(ch: Channel, th: u16) {
    // ch_d_touch_th occupies bits [27:16] in all CHN_CTRL registers.
    with_ch_ctrl!(ch, |r| {
        r.modify(|rd, w| unsafe {
            w.bits((rd.bits() & !(0x0fff_u32 << 16)) | ((th as u32 & 0x0fff) << 16))
        });
    });
}

fn ch_get_diff_threshold(ch: Channel) -> u16 {
    with_ch_ctrl!(ch, |r| ((r.read().bits() >> 16) & 0x0fff) as u16)
}

fn ch_get_baseline(ch: Channel) -> u16 {
    // ch_baseline occupies bits [15:4] in all CHN_CTRL registers.
    with_ch_ctrl!(ch, |r| ((r.read().bits() >> 4) & 0x0fff) as u16)
}

fn ch_set_mbias(ch: Channel, mbias: u16) {
    // ch_mbias occupies bits [15:0] in CHN_MBIAS_ATH.
    with_ch_mbias!(ch, |r| {
        r.modify(|rd, w| unsafe { w.bits((rd.bits() & !0xffff_u32) | (mbias as u32)) });
    });
}

fn ch_get_mbias(ch: Channel) -> u16 {
    with_ch_mbias!(ch, |r| (r.read().bits() & 0xffff) as u16)
}

fn ch_get_abs_threshold(ch: Channel) -> u16 {
    // ch_a_touch_th occupies bits [27:16] in CHN_MBIAS_ATH.
    with_ch_mbias!(ch, |r| ((r.read().bits() >> 16) & 0x0fff) as u16)
}

fn ch_set_noise_thresholds(ch: Channel, p_thr: u16, n_thr: u16) {
    // p_ent occupies bits [11:0], n_ent occupies bits [27:16] in CHN_NOISE_TH.
    with_ch_noise!(ch, |r| {
        r.modify(|_, w| unsafe {
            w.bits(((n_thr as u32 & 0x0fff) << 16) | (p_thr as u32 & 0x0fff))
        });
    });
}

fn ch_get_noise_threshold(ch: Channel, noise: NoiseType) -> u16 {
    with_ch_noise!(ch, |r| {
        let v = r.read().bits();
        match noise {
            NoiseType::Positive => (v & 0x0fff) as u16,
            NoiseType::Negative => ((v >> 16) & 0x0fff) as u16,
        }
    })
}

/// Modify one or both noise thresholds for `ch` via a closure.
/// The closure receives `(p_noise, n_noise)` and returns the updated values.
fn ch_modify_noise(_r: &ctc_reg::RegisterBlock, ch: Channel, f: impl Fn(u16, u16) -> (u16, u16)) {
    with_ch_noise!(ch, |r| {
        let v = r.read().bits();
        let p = (v & 0x0fff) as u16;
        let n = ((v >> 16) & 0x0fff) as u16;
        let (new_p, new_n) = f(p, n);
        r.modify(|_, w| unsafe {
            w.bits(((new_n as u32 & 0x0fff) << 16) | (new_p as u32 & 0x0fff))
        });
    });
}

fn ch_get_ave_data(ch: Channel) -> u16 {
    // ch_data_ave occupies bits [11:0] in CHN_DATA_INF.
    with_ch_data!(ch, |r| (r.read().bits() & 0x0fff) as u16)
}
