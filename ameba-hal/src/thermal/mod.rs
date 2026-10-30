use bitflags::bitflags;
use rtl8721f_pac::{ThermalReg, thermal_reg};

// ────────────────────────────────────────────────────────────────────────────
// Constants
// ────────────────────────────────────────────────────────────────────────────

const THERMAL_PTR: *const thermal_reg::RegisterBlock = ThermalReg::PTR;

const SYSCFG_CUT_VERSION_A: u8 = 0;
const TM_CALIB_GAIN: u32 = 0x0834_D000; // SDK: ameba_thermal.h : TM_CALIB_GAIN
const TM_CALIB_OFFSET: u32 = 0x0037_0E00; // SDK: ameba_thermal.h : TM_CALIB_OFFSET

/// Value returned by `get_temp_result` / `get_max_temp` / `get_min_temp`
/// when the measurement is not yet valid.
pub const TM_INVALID_VALUE: u32 = 0x0008_0000; // SDK: ameba_thermal.h : TM_INVALID_VALUE

// ────────────────────────────────────────────────────────────────────────────
// Public enums / bitflags
// ────────────────────────────────────────────────────────────────────────────

/// Down-sample rate for the thermal ADC output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DownSampRate {
    Dsr1 = 0,
    Dsr2 = 1,
    Dsr4 = 2,
    Dsr8 = 3,
    Dsr16 = 4,
    Dsr32 = 5,
    Dsr64 = 6,
    Dsr128 = 7,
    Dsr256 = 8,
    Dsr512 = 9,
}

/// ADC clock divider selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcClkDiv {
    Div64,
    Div128,
}

bitflags! {
    /// Thermal interrupt flags (both IMR and ISR use the same bit positions).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ThermalInt: u8 {
        const HIGH_WARN = 0b01;
        const LOW_WARN  = 0b10;
    }
}

/// Thermal sensor initialisation parameters.
pub struct ThermalInitParams {
    pub down_samp_rate: DownSampRate,
    pub adc_clk_div: AdcClkDiv,
    /// High over-temperature protect threshold (valid 0x46–0x8C).
    pub high_protect_threshold: u16,
    /// High warning threshold.
    pub high_warn_threshold: u16,
    /// Low warning threshold.
    pub low_warn_threshold: u16,
    /// Auto sampling time period (8-bit field in TM_TIMER).
    pub time_period: u8,
}

impl Default for ThermalInitParams {
    fn default() -> Self {
        ThermalInitParams {
            down_samp_rate: DownSampRate::Dsr512,
            adc_clk_div: AdcClkDiv::Div128,
            high_protect_threshold: 0x7D,
            high_warn_threshold: 0x69,
            low_warn_threshold: 0x1D8,
            time_period: 0x0F,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Driver struct
// ────────────────────────────────────────────────────────────────────────────

/// Thermal meter driver.
///
/// Uses the Non-Secure aperture (0x4081_6000) from the svd2rust PAC per the
/// HAL-wide S/NS convention (see `crate::lib`).
pub struct Thermal {
    ptr: *const thermal_reg::RegisterBlock,
}

// SAFETY: embedded single-CPU target; `Thermal` has exclusive ownership of its peripheral.
unsafe impl Send for Thermal {}

impl Thermal {
    /// # Safety
    /// Only one `Thermal` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        Thermal { ptr: THERMAL_PTR }
    }

    fn regs(&self) -> &'static thermal_reg::RegisterBlock {
        unsafe { &*self.ptr }
    }

    // ── Power / program-enable ───────────────────────────────────────────────

    /// Set the TM_PWR write-enable password.
    ///
    /// Must be called with `true` before writing protected fields, and `false`
    /// after. Mirrors `TM_PwrProgCmd`.
    pub fn set_prog_enabled(&self, enabled: bool) {
        const TM_PROG_PWR: u8 = 0x69; // SDK: ameba_thermal.h : TM_PROG_PWR
        self.regs()
            .tm_ctrl()
            .modify(|_, w| unsafe { w.tm_pwr().bits(if enabled { TM_PROG_PWR } else { 0 }) });
    }

    // ── Enable / disable ────────────────────────────────────────────────────

    /// Enable or disable the thermal peripheral. Mirrors `TM_Cmd`.
    pub fn cmd(&self, enable: bool) {
        self.set_prog_enabled(true);
        if enable {
            self.regs()
                .tm_ctrl()
                .modify(|_, w| w.tm_powcut().set_bit().tm_pow().set_bit());
            self.regs()
                .tm_th_ctrl()
                .modify(|_, w| w.iso_thm().clear_bit());
            self.regs().tm_ctrl().modify(|_, w| w.tm_rstb().set_bit());
        } else {
            self.regs().tm_ctrl().modify(|_, w| {
                w.tm_rstb()
                    .clear_bit()
                    .tm_pow()
                    .clear_bit()
                    .tm_powcut()
                    .clear_bit()
            });
        }
        self.set_prog_enabled(false);
    }

    // ── Initialise ──────────────────────────────────────────────────────────

    /// Initialise and enable the thermal peripheral. Mirrors `TM_Init`.
    pub fn init(&self, p: &ThermalInitParams) {
        self.set_interrupt(ThermalInt::HIGH_WARN | ThermalInt::LOW_WARN, false);
        self.cmd(false);

        self.regs().tm_ctrl().modify(|_, w| unsafe {
            w.tm_osr()
                .bits(p.down_samp_rate as u8)
                .tm_adccksel()
                .bit(p.adc_clk_div == AdcClkDiv::Div128)
        });

        self.clear_max_temp();
        self.clear_min_temp();

        self.regs()
            .tm_timer()
            .modify(|_, w| unsafe { w.time_period().bits(p.time_period) });

        self.cmd(true);
        self.clear_interrupt(ThermalInt::all());

        // Apply factory calibration for chip version A
        if crate::otp::chip_version() == SYSCFG_CUT_VERSION_A {
            self.regs()
                .tm_gain()
                .write(|w| unsafe { w.tm_a().bits(TM_CALIB_GAIN) });
            self.regs()
                .tm_offset()
                .write(|w| unsafe { w.tm_b().bits(TM_CALIB_OFFSET) });
        }

        self.set_high_protect_threshold(p.high_protect_threshold, true);
        self.set_high_warn_threshold(p.high_warn_threshold, true);
        self.set_low_warn_threshold(p.low_warn_threshold, true);

        self.set_interrupt(ThermalInt::HIGH_WARN | ThermalInt::LOW_WARN, true);
    }

    // ── Interrupt control ────────────────────────────────────────────────────

    /// Enable or disable thermal interrupt sources. Mirrors `TM_INTConfig`.
    pub fn set_interrupt(&self, flags: ThermalInt, enabled: bool) {
        self.regs().tm_intr_ctrl().modify(|_, w| {
            let w = if flags.contains(ThermalInt::HIGH_WARN) {
                w.imr_tm_high_wt().bit(enabled)
            } else {
                w
            };
            if flags.contains(ThermalInt::LOW_WARN) {
                w.imr_tm_low_wt().bit(enabled)
            } else {
                w
            }
        });
    }

    /// Clear thermal interrupt pending flags (RW1C). Mirrors `TM_INTClear` /
    /// `TM_INTClearPendingBits`.
    pub fn clear_interrupt(&self, flags: ThermalInt) {
        unsafe {
            self.regs()
                .tm_intr_sts()
                .as_ptr()
                .write_volatile(flags.bits() as u32);
        }
    }

    /// Read current interrupt status flags. Mirrors `TM_GetISR`.
    pub fn get_interrupt(&self) -> ThermalInt {
        let r = self.regs().tm_intr_sts().read();
        let mut f = ThermalInt::empty();
        if r.isr_tm_high_wt().bit() {
            f |= ThermalInt::HIGH_WARN;
        }
        if r.isr_tm_low_wt().bit() {
            f |= ThermalInt::LOW_WARN;
        }
        f
    }

    // ── Threshold configuration ──────────────────────────────────────────────

    /// Configure the over-temperature protect threshold.
    ///
    /// Valid range: 0x46–0x8C. Mirrors `TM_HighPtConfig`.
    pub fn set_high_protect_threshold(&self, threshold: u16, enabled: bool) {
        self.set_prog_enabled(true);
        if enabled {
            self.regs().tm_th_ctrl().modify(|_, w| unsafe {
                w.tm_high_pt_thr()
                    .bits(threshold)
                    .tm_highcmp_pt_en()
                    .set_bit()
            });
        } else {
            self.regs()
                .tm_th_ctrl()
                .modify(|_, w| w.tm_highcmp_pt_en().clear_bit());
        }
        self.set_prog_enabled(false);
    }

    /// Configure the high-temperature warning threshold. Mirrors `TM_HighWtConfig`.
    pub fn set_high_warn_threshold(&self, threshold: u16, enabled: bool) {
        if enabled {
            self.regs().tm_th_ctrl().modify(|_, w| unsafe {
                w.tm_high_wt_thr()
                    .bits(threshold)
                    .tm_highcmp_wt_en()
                    .set_bit()
            });
        } else {
            self.regs()
                .tm_th_ctrl()
                .modify(|_, w| w.tm_highcmp_wt_en().clear_bit());
        }
    }

    /// Configure the low-temperature warning threshold. Mirrors `TM_LowWtConfig`.
    pub fn set_low_warn_threshold(&self, threshold: u16, enabled: bool) {
        if enabled {
            self.regs().tm_th_ctrl().modify(|_, w| unsafe {
                w.tm_low_wt_thr()
                    .bits(threshold)
                    .tm_lowcmp_wt_en()
                    .set_bit()
            });
        } else {
            self.regs()
                .tm_th_ctrl()
                .modify(|_, w| w.tm_lowcmp_wt_en().clear_bit());
        }
    }

    // ── Max/min temperature records ──────────────────────────────────────────

    /// Clear the recorded maximum temperature. Mirrors `TM_MaxTempClr`.
    pub fn clear_max_temp(&self) {
        self.regs()
            .tm_max_ctrl()
            .modify(|_, w| w.tm_max_clr().set_bit());
    }

    /// Clear the recorded minimum temperature. Mirrors `TM_MinTempClr`.
    pub fn clear_min_temp(&self) {
        self.regs()
            .tm_min_ctrl()
            .modify(|_, w| w.tm_min_clr().set_bit());
    }

    // ── Data polling and result reads ────────────────────────────────────────

    /// Poll until the thermal data latch completes.
    ///
    /// Returns `true` on success, `false` on timeout. Mirrors `TM_PollDataValid`.
    pub fn poll_data_valid(&self) -> bool {
        let mut timeout: u32 = 1_000_000;
        while self.regs().tm_ctrl().read().tm_en_latch().bit() {
            timeout -= 1;
            if timeout == 0 {
                return false;
            }
        }
        true
    }

    /// Read the current temperature result (19-bit raw).
    ///
    /// Returns `TM_INVALID_VALUE` if the data is not yet valid.
    /// Mirrors `TM_GetTempResult`.
    pub fn get_temp_result(&self) -> u32 {
        if !self.poll_data_valid() {
            return TM_INVALID_VALUE;
        }
        self.regs().tm_result().read().tm_out().bits()
    }

    /// Read the power-on temperature (19-bit raw). Mirrors `TM_GetPowOnTemp`.
    pub fn get_pow_on_temp(&self) -> u32 {
        if !self.poll_data_valid() {
            return TM_INVALID_VALUE;
        }
        self.regs().tm_out_pwr_on().read().temp_out_poweron().bits()
    }

    /// Read the maximum recorded temperature (19-bit raw). Mirrors `TM_GetMaxTemp`.
    pub fn get_max_temp(&self) -> u32 {
        if !self.poll_data_valid() {
            return TM_INVALID_VALUE;
        }
        self.regs().tm_max_ctrl().read().tm_max().bits()
    }

    /// Read the minimum recorded temperature (19-bit raw). Mirrors `TM_GetMinTemp`.
    pub fn get_min_temp(&self) -> u32 {
        if !self.poll_data_valid() {
            return TM_INVALID_VALUE;
        }
        self.regs().tm_min_ctrl().read().tm_min().bits()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Temperature conversion (pure math, no register access)
// ────────────────────────────────────────────────────────────────────────────

const TM_NEGATIVE_MIN: u32 = 0x40000; // SDK: ameba_thermal.h : TM_NEGATIVE_MIN
const TM_NEGATIVE_MAX: u32 = 0x7FFFF; // SDK: ameba_thermal.h : TM_NEGATIVE_MAX

/// Convert a 19-bit raw thermal result to degrees Celsius. Mirrors `TM_GetCdegree`.
pub fn raw_to_celsius(data: u32) -> f32 {
    if (TM_NEGATIVE_MIN..=TM_NEGATIVE_MAX).contains(&data) {
        let temp = TM_NEGATIVE_MAX + 1 - data;
        let integer = (temp / 1024) as f32;
        let decimal = (temp & 0x3FF) as f32 / 1024.0;
        -(integer + decimal)
    } else {
        let integer = (data / 1024) as f32;
        let decimal = (data & 0x3FF) as f32 / 1024.0;
        integer + decimal
    }
}

/// Convert a 19-bit raw thermal result to degrees Fahrenheit. Mirrors `TM_GetFdegree`.
pub fn raw_to_fahrenheit(data: u32) -> f32 {
    raw_to_celsius(data) * 1.8 + 32.0
}
