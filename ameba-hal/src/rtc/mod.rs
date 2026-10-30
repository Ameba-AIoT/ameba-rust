//! RTL8721F RTC driver — idiomatic Rust, no_std, PAC-based.
//!
//! Covers: peripheral enable/clock-source, init, time get/set, alarm, wakeup
//! timer, day-threshold interrupt, daylight-saving adjustment, output config,
//! smooth calibration, and AON detection-interrupt helpers.
//!
//! # Usage sketch
//! ```ignore
//! // Mirror rtc_irq_init() in ameba_app_start.c — must follow this order:
//! rcc::enable(Peripheral::RTC);                          // RCC_PeriphClockCmd(APBPeriph_RTC_CLOCK, ENABLE)
//! let rtc = unsafe { Rtc::steal() };
//! rtc.enable(true);                                      // RTC_Enable(ENABLE) — sets fen_rtc
//! rtc.init(&RtcConfig::default());                       // RTC_Init()
//! rtc.set_time(TimeFormat::Bin, &TimeConfig::default()); // RTC_SetTime()
//! rtc.set_clock_source(ClockSource::Sdm32k);             // RTC_ClkSource_Select(SDM32K) — must come after fen_rtc is set
//! let t = rtc.get_time(TimeFormat::Bin);
//! ```

use bitflags::bitflags;
use rtl8721f_pac::{LpAon, RtcMisc, RtcReg, lp_aon, rtc_misc, rtc_reg};

// ─────────────────────────────────────────────────────────────────────────────
// ROM / extern-C bindings (private — never exposed to callers)
// ─────────────────────────────────────────────────────────────────────────────

use crate::delay::Delay;
use embedded_hal::delay::DelayNs;

// ─────────────────────────────────────────────────────────────────────────────
// Timeout constants (unchanged from C driver)
// ─────────────────────────────────────────────────────────────────────────────

const INITMODE_TIMEOUT: u32 = 0x0001_0000;
const SYNCHRO_TIMEOUT: u32 = 0x0008_0000;
const ALARMDIS_TIMEOUT: u32 = 0x0001_0000;
const WUTDIS_TIMEOUT: u32 = 0x0001_0000;
const RECALPF_TIMEOUT: u32 = 0x0001_0000;

/// Year offset: RTC hardware stores years since 1900.
const BASE_YEAR: u16 = 1900;

// ─────────────────────────────────────────────────────────────────────────────
// Public enumerations
// ─────────────────────────────────────────────────────────────────────────────

/// Time register format for `set_time` / `get_time` / `set_alarm` / `get_alarm`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    /// Binary (decimal) values.
    Bin,
    /// BCD-encoded values.
    Bcd,
}

/// RTC hour format stored in `RTC_CR.FMT`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HourFormat {
    /// 24-hour format.
    H24,
    /// 12-hour AM/PM format.
    H12,
}

/// AM or PM indicator in 12-hour mode.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AmPm {
    #[default]
    Am = 0,
    Pm = 1,
}

/// Clock source for the RTC.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClockSource {
    /// SDM 32 kHz oscillator (default when AON is powered).
    Sdm32k = 0,
    /// OSC131K divided by 4.
    Osc131kDiv4 = 1,
}

/// Daylight-saving hour adjustment direction.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DaylightSaving {
    /// Subtract one hour (winter time change) — writes `SUB1H`.
    Sub1h,
    /// Add one hour (summer time change) — writes `ADD1H`.
    Add1h,
}

/// Value to store in the `BKP` bit of `RTC_CR` as a daylight-saving memo.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    Reset = 0,
    Set = 1,
}

/// RTC output signal selection (`RTC_CR.OSEL`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OutputSel {
    Disabled = 0b00,
    Alarm = 0b01,
    ClkSpre = 0b10,
    ClkApre = 0b11,
}

/// Calibration sign (`RTC_CALIBR.DCS`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CalibSign {
    /// Positive calibration — increase time-update frequency.
    Positive = 0,
    /// Negative calibration — decrease time-update frequency.
    Negative = 1,
}

/// Calibration period (`RTC_CALIBR.CALP`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CalibPeriod {
    Min1 = 0,
    Min2 = 1,
    Min3 = 2,
    Min4 = 3,
    Min5 = 4,
    Min6 = 5,
    Min7 = 6,
    Min8 = 7,
}

// ─────────────────────────────────────────────────────────────────────────────
// bitflags — alarm mask bits
// ─────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// Alarm comparison mask (stored in `RTC_ALMR1L` bits 7/15/23).
    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct AlarmMask: u32 {
        /// Seconds always match (bit 7).
        const SECONDS = 1 << 7;
        /// Minutes always match (bit 15).
        const MINUTES = 1 << 15;
        /// Hours always match (bit 23).
        const HOURS   = 1 << 23;
        /// All time fields always match.
        const ALL     = Self::SECONDS.bits() | Self::MINUTES.bits() | Self::HOURS.bits();
    }
}

bitflags! {
    /// Day alarm mask (stored in `RTC_ALMR1H` bit 9).
    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct AlarmDayMask: u32 {
        /// Days always match (bit 9 of ALMR1H).
        const DAYS = 1 << 9;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Configuration structs
// ─────────────────────────────────────────────────────────────────────────────

/// Peripheral initialisation parameters (mirrors `RTC_InitTypeDef`).
#[derive(Clone, Copy)]
pub struct RtcConfig {
    pub hour_format: HourFormat,
    /// Asynchronous prescaler (7-bit): clk_apre = RTCCLK / (prediv_a + 1).
    pub prediv_a: u16,
    /// Synchronous prescaler (9-bit): clk_spre = clk_apre / (prediv_s + 1).
    pub prediv_s: u16,
    /// Day counter rollover threshold (9-bit binary, 0x1FF = 511).
    pub day_threshold: u16,
}

impl Default for RtcConfig {
    fn default() -> Self {
        RtcConfig {
            hour_format: HourFormat::H24,
            prediv_a: 0x7F, // 32768 / 128 = 256 Hz
            prediv_s: 0xFF, // 256 / 256 = 1 Hz
            day_threshold: 0x1FF,
        }
    }
}

/// Calendar time value (mirrors `RTC_TimeTypeDef`).
#[derive(Clone, Copy, Default)]
pub struct TimeConfig {
    pub am_pm: AmPm,
    /// Absolute year (e.g. 2024). Stored as offset from 1900 in hardware.
    pub year: u16,
    pub days: u16,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

/// Alarm configuration (mirrors `RTC_AlarmTypeDef`).
#[derive(Clone, Copy)]
pub struct AlarmConfig {
    pub time: TimeConfig,
    /// Bitmask for second/minute/hour comparison (see [`AlarmMask`]).
    pub mask: AlarmMask,
    /// Day comparison mask (see [`AlarmDayMask`]).
    pub day_mask: AlarmDayMask,
}

impl Default for AlarmConfig {
    fn default() -> Self {
        AlarmConfig {
            time: TimeConfig {
                seconds: 20,
                ..TimeConfig::default()
            },
            mask: AlarmMask::HOURS | AlarmMask::MINUTES,
            day_mask: AlarmDayMask::DAYS,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BCD helpers (module-private)
// ─────────────────────────────────────────────────────────────────────────────

fn byte_to_bcd2(mut val: u8) -> u8 {
    let mut hi: u8 = 0;
    while val >= 10 {
        hi += 1;
        val -= 10;
    }
    (hi << 4) | val
}

fn bcd2_to_byte(val: u8) -> u8 {
    let hi = (val >> 4) * 10;
    hi + (val & 0x0F)
}

// ─────────────────────────────────────────────────────────────────────────────
// PAC RegisterBlock accessors (module-private helpers)
// ─────────────────────────────────────────────────────────────────────────────

fn rtc_rb() -> &'static rtc_reg::RegisterBlock {
    unsafe { &*RtcReg::PTR }
}

fn misc_rb() -> &'static rtc_misc::RegisterBlock {
    unsafe { &*RtcMisc::PTR }
}

fn aon_rb() -> &'static lp_aon::RegisterBlock {
    unsafe { &*LpAon::PTR }
}

// ─────────────────────────────────────────────────────────────────────────────
// Write-protection sequence helpers
// ─────────────────────────────────────────────────────────────────────────────

fn wpr_unlock(rb: &rtc_reg::RegisterBlock) {
    rb.rtc_wpr().write(|w| unsafe { w.key().bits(0xCA) });
    rb.rtc_wpr().write(|w| unsafe { w.key().bits(0x53) });
}

fn wpr_lock(rb: &rtc_reg::RegisterBlock) {
    rb.rtc_wpr().write(|w| unsafe { w.key().bits(0xFF) });
}

// ─────────────────────────────────────────────────────────────────────────────
// Init-mode helpers (module-private)
// ─────────────────────────────────────────────────────────────────────────────

fn enter_init_mode(rb: &rtc_reg::RegisterBlock) -> bool {
    if rb.rtc_isr().read().initf().bit_is_set() {
        return true;
    }
    // Set INIT bit (the only W field in RTC_ISR).
    rb.rtc_isr().modify(|_, w| w.init().set_bit());

    let mut counter: u32 = 0;
    while counter < INITMODE_TIMEOUT {
        if rb.rtc_isr().read().initf().bit_is_set() {
            return true;
        }
        counter += 1;
    }
    rb.rtc_isr().read().initf().bit_is_set()
}

fn exit_init_mode(rb: &rtc_reg::RegisterBlock) {
    // Clear INIT bit via the W accessor.
    rb.rtc_isr().modify(|_, w| w.init().clear_bit());
}

// ─────────────────────────────────────────────────────────────────────────────
// Synchronisation helpers (module-private)
// ─────────────────────────────────────────────────────────────────────────────

fn wait_for_synchro(rb: &rtc_reg::RegisterBlock) -> bool {
    // When BYPSHAD is set, the shadow registers are bypassed; wait 125 µs
    // for the four 32.768 kHz cycles instead of polling RSF.
    if rb.rtc_cr().read().bypshad().bit_is_set() {
        Delay::new().delay_us(125);
        return true;
    }

    wpr_unlock(rb);

    // Set RSF (write-1-to-clear in hardware, but the PAC W impl doesn't expose
    // it — use write_volatile to set it so hardware clears and re-sets it).
    let isr_ptr = rb.rtc_isr().as_ptr();
    let current = rb.rtc_isr().read().bits();
    // Preserve INIT writable bit; set RSF bit (bit 5) to trigger re-sync.
    unsafe { isr_ptr.write_volatile(current | (1 << 5)) };

    let mut counter: u32 = 0;
    while counter < SYNCHRO_TIMEOUT {
        if rb.rtc_isr().read().rsf().bit_is_set() {
            break;
        }
        counter += 1;
    }
    let ok = rb.rtc_isr().read().rsf().bit_is_set();

    wpr_lock(rb);
    ok
}

fn wait_for_wut_synchro(rb: &rtc_reg::RegisterBlock) -> bool {
    wpr_unlock(rb);

    // Write 1 to WUTRSF bit (bit 1) to start the sync wait.
    let isr_ptr = rb.rtc_isr().as_ptr();
    let current = rb.rtc_isr().read().bits();
    unsafe { isr_ptr.write_volatile(current | (1 << 1)) };

    let mut counter: u32 = 0;
    while counter <= SYNCHRO_TIMEOUT {
        counter += 1;
        if rb.rtc_isr().read().wutrsf().bit_is_set() {
            break;
        }
    }
    let ok = rb.rtc_isr().read().wutrsf().bit_is_set();

    wpr_lock(rb);
    ok
}

// ─────────────────────────────────────────────────────────────────────────────
// Main driver struct
// ─────────────────────────────────────────────────────────────────────────────

/// RTC peripheral driver (RTL8721F, non-secure KM4NS domain).
///
/// Owns access to `rtc_reg`, `rtc_misc`, and the AON `lp_aon` register block
/// (for the detection-interrupt helpers).
pub struct Rtc {
    _priv: (),
}

impl Rtc {
    /// Construct the driver by claiming all three register blocks.
    ///
    /// # Safety
    /// Caller must ensure no other code accesses `rtc_reg`, `rtc_misc`, or the
    /// RTC-relevant fields of `lp_aon` for the lifetime of this instance.
    pub unsafe fn steal() -> Self {
        Rtc { _priv: () }
    }

    // ── Clock / enable ────────────────────────────────────────────────────────

    /// Enable or disable the RTC function via `rtc_misc.OSC131K_CTRL.fen_rtc`.
    ///
    /// Call `rcc::enable(Peripheral::RTC)` before this to open the APB clock
    /// gate (`bpc_rtc`). SDK: `RCC_PeriphClockCmd(APBPeriph_RTC_CLOCK, ENABLE)`
    /// must precede `RTC_Enable(ENABLE)` in `rtc_irq_init`.
    pub fn enable(&self, en: bool) {
        misc_rb().osc131k_ctrl().modify(|_, w| w.fen_rtc().bit(en));
    }

    /// Select the RTC clock source.
    ///
    /// Must be called **after** `enable(true)` (i.e. after `fen_rtc` is set).
    /// Default after AON power-on is `Osc131kDiv4`; call with `Sdm32k` for
    /// accurate timekeeping. SDK: `RTC_ClkSource_Select(SDM32K)` is the last
    /// step in `rtc_irq_init`, always after `RTC_Enable`.
    pub fn set_clock_source(&self, src: ClockSource) {
        misc_rb()
            .osc131k_state()
            .modify(|_, w| w.cksl_rtc().bit(src == ClockSource::Osc131kDiv4));
    }

    /// Read the raw value of `OSC131K_STATE`.
    pub fn osc131k_state(&self) -> u32 {
        misc_rb().osc131k_state().read().bits()
    }

    /// Write a raw value to `OSC131K_STATE`.
    pub fn set_osc131k_state(&self, val: u32) {
        misc_rb().osc131k_state().write(|w| unsafe { w.bits(val) });
    }

    // ── AON detection interrupt ───────────────────────────────────────────────

    /// Read the `rtc_det_intr` flag from the AON `REG_RTC_DET_CTRL1` register.
    pub fn get_det_intr(&self) -> bool {
        aon_rb()
            .reg_rtc_det_ctrl1()
            .read()
            .rtc_det_intr()
            .bit_is_set()
    }

    /// Clear `rtc_det_intr` (RW1C — write 1 to bit 20).
    ///
    /// The PAC `W` impl for `REG_RTC_DET_CTRL1` does not expose `rtc_det_intr`
    /// as a writable field, so `write_volatile` is used directly.
    pub fn clear_det_intr(&self) {
        let ptr = aon_rb().reg_rtc_det_ctrl1().as_ptr();
        // Bit 20 is the RW1C rtc_det_intr flag.
        unsafe { ptr.write_volatile(1 << 20) };
    }

    // ── Shadow bypass ─────────────────────────────────────────────────────────

    /// Enable or disable bypass-shadow mode. Returns `true` on success.
    pub fn set_bypass_shadow(&self, en: bool) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        let ok = if enter_init_mode(rb) {
            rb.rtc_cr().modify(|_, w| w.bypshad().bit(en));
            exit_init_mode(rb);
            wait_for_synchro(rb)
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    // ── Peripheral initialisation ─────────────────────────────────────────────

    /// Initialise the RTC peripheral. Returns `true` on success.
    pub fn init(&self, cfg: &RtcConfig) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        let ok = if enter_init_mode(rb) {
            // Prescaler.
            rb.rtc_prer().write(|w| unsafe {
                w.prediv_a().bits(cfg.prediv_a);
                w.prediv_s().bits(cfg.prediv_s)
            });

            // CR: fmt, day_thres, disable BYPSHAD and DOVTHIE.
            // day_thres (bits 31:23) has no PAC write accessor, so we build the
            // value from the current register and patch the field manually.
            {
                let cr_ptr = rb.rtc_cr().as_ptr();
                let cur = rb.rtc_cr().read().bits();
                // fmt = bit 7, bypshad = bit 3, dovthie = bit 16, day_thres = bits 31:23
                let fmt_bit: u32 = if cfg.hour_format == HourFormat::H12 {
                    1 << 7
                } else {
                    0
                };
                let val = (cur
                    & !(1 << 7)   // clear FMT
                    & !(1 << 3)   // clear BYPSHAD
                    & !(1 << 16)  // clear DOVTHIE
                    & !(0x1FF << 23)) // clear DAY_THRES
                    | fmt_bit
                    | ((cfg.day_threshold as u32 & 0x1FF) << 23);
                unsafe { cr_ptr.write_volatile(val) };
            }

            exit_init_mode(rb);
            wait_for_synchro(rb)
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    // ── Calendar time ─────────────────────────────────────────────────────────

    /// Set the current calendar time. Returns `true` on success.
    pub fn set_time(&self, fmt: TimeFormat, t: &TimeConfig) -> bool {
        let rb = rtc_rb();

        // Build BCD time register value.
        let (h_bcd, m_bcd, s_bcd) = match fmt {
            TimeFormat::Bin => (
                byte_to_bcd2(t.hours),
                byte_to_bcd2(t.minutes),
                byte_to_bcd2(t.seconds),
            ),
            TimeFormat::Bcd => (t.hours, t.minutes, t.seconds),
        };

        let h_tens = ((h_bcd >> 4) & 0x3) as u32;
        let h_units = (h_bcd & 0xF) as u32;
        let m_tens = ((m_bcd >> 4) & 0x7) as u32;
        let m_units = (m_bcd & 0xF) as u32;
        let s_tens = ((s_bcd >> 4) & 0x7) as u32;
        let s_units = (s_bcd & 0xF) as u32;

        wpr_unlock(rb);

        let ok = if enter_init_mode(rb) {
            // Write time register via write_volatile; RTC_TR is fully RW but
            // the PAC write method requires field-by-field access which is
            // cumbersome for this packed value — use modify with individual
            // fields instead.
            rb.rtc_tr().write(|w| unsafe {
                w.su().bits(s_units as u8);
                w.st().bits(s_tens as u8);
                w.mnu().bits(m_units as u8);
                w.mnt().bits(m_tens as u8);
                w.hu().bits(h_units as u8);
                w.ht().bits(h_tens as u8);
                w.pm().bit(t.am_pm == AmPm::Pm);
                w.day().bits(t.days)
            });

            let year_offset = t.year.saturating_sub(BASE_YEAR) as u8;
            rb.rtc_year()
                .modify(|_, w| unsafe { w.year().bits(year_offset) });

            exit_init_mode(rb);
            wait_for_synchro(rb)
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    /// Get the current calendar time.
    pub fn get_time(&self, fmt: TimeFormat) -> TimeConfig {
        let rb = rtc_rb();
        let tr = rb.rtc_tr().read();

        let (hours, minutes, seconds) = match fmt {
            TimeFormat::Bcd => (
                tr.hu().bits() | (tr.ht().bits() << 4),
                tr.mnu().bits() | (tr.mnt().bits() << 4),
                tr.su().bits() | (tr.st().bits() << 4),
            ),
            TimeFormat::Bin => (
                bcd2_to_byte(tr.hu().bits() | (tr.ht().bits() << 4)),
                bcd2_to_byte(tr.mnu().bits() | (tr.mnt().bits() << 4)),
                bcd2_to_byte(tr.su().bits() | (tr.st().bits() << 4)),
            ),
        };

        let am_pm = if tr.pm().bit_is_set() {
            AmPm::Pm
        } else {
            AmPm::Am
        };
        let days = tr.day().bits();
        let year_raw = rb.rtc_year().read().year().bits();

        TimeConfig {
            am_pm,
            year: (year_raw as u16) + BASE_YEAR,
            days,
            hours,
            minutes,
            seconds,
        }
    }

    /// Set only the day and year fields of the time register.
    pub fn set_day_year(&self, day: u16, year: u16) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        let ok = if enter_init_mode(rb) {
            rb.rtc_tr().modify(|_, w| unsafe { w.day().bits(day) });
            let year_offset = year.saturating_sub(BASE_YEAR) as u8;
            rb.rtc_year()
                .modify(|_, w| unsafe { w.year().bits(year_offset) });

            exit_init_mode(rb);
            wait_for_synchro(rb)
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    // ── Day-threshold interrupt ───────────────────────────────────────────────

    /// Clear the day-over-threshold pending interrupt.
    ///
    /// Preserves ALMF and WUTF; sets DOVTHF (bit 15) as RW1C.
    pub fn clear_day_int(&self) {
        let rb = rtc_rb();
        let isr_ptr = rb.rtc_isr().as_ptr();
        let isr = rb.rtc_isr().read().bits();

        // Clear ALMF (bit 8) and WUTF (bit 10) from the value we will write, so
        // we do not accidentally clear those flags.  Set DOVTHF (bit 15).
        let val = (isr & !(1 << 8) & !(1 << 10)) | (1 << 15);
        unsafe { isr_ptr.write_volatile(val) };

        let mut counter: u32 = 0;
        loop {
            if rb.rtc_isr().read().dovthf().bit_is_clear() {
                break;
            }
            if counter >= ALARMDIS_TIMEOUT {
                break;
            }
            counter += 1;
        }
    }

    /// Enable or disable the day-over-threshold interrupt. Returns `true` on success.
    pub fn set_day_int(&self, en: bool) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        let ok = if enter_init_mode(rb) {
            self.clear_day_int();
            rb.rtc_cr().modify(|_, w| w.dovthie().bit(en));
            exit_init_mode(rb);
            wait_for_synchro(rb)
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    /// Set the day counter rollover threshold. Returns `true` on success.
    pub fn set_day_threshold(&self, thres: u16) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        let ok = if enter_init_mode(rb) {
            // day_thres (bits 31:23) has no PAC write accessor — patch raw bits.
            {
                let cr_ptr = rb.rtc_cr().as_ptr();
                let cur = rb.rtc_cr().read().bits();
                let val = (cur & !(0x1FF << 23)) | ((thres as u32 & 0x1FF) << 23);
                unsafe { cr_ptr.write_volatile(val) };
            }
            exit_init_mode(rb);
            wait_for_synchro(rb)
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    /// Read the current day threshold from `RTC_CR`.
    pub fn day_threshold(&self) -> u16 {
        rtc_rb().rtc_cr().read().day_thres().bits()
    }

    // ── Alarm ─────────────────────────────────────────────────────────────────

    /// Configure the alarm registers. Alarm must be disabled first (call
    /// `alarm_cmd(false)` before this). Returns `true` on success.
    pub fn set_alarm(&self, fmt: TimeFormat, a: &AlarmConfig) -> bool {
        let rb = rtc_rb();

        let (h_bcd, m_bcd, s_bcd) = match fmt {
            TimeFormat::Bin => (
                byte_to_bcd2(a.time.hours),
                byte_to_bcd2(a.time.minutes),
                byte_to_bcd2(a.time.seconds),
            ),
            TimeFormat::Bcd => (a.time.hours, a.time.minutes, a.time.seconds),
        };

        // Build ALMR1L value with mask bits.
        let s_units = (s_bcd & 0xF) as u32;
        let s_tens = ((s_bcd >> 4) & 0x7) as u32;
        let m_units = (m_bcd & 0xF) as u32;
        let m_tens = ((m_bcd >> 4) & 0x7) as u32;
        let h_units = (h_bcd & 0xF) as u32;
        let h_tens = ((h_bcd >> 4) & 0x3) as u32;
        let pm_bit: u32 = (a.time.am_pm as u32) << 22;

        let almr1l_val: u32 = s_units
            | (s_tens << 4)
            | (m_units << 8)
            | (m_tens << 12)
            | (h_units << 16)
            | (h_tens << 20)
            | pm_bit
            | a.mask.bits();

        wpr_unlock(rb);

        // Disable alarm and wait for ALMWF.
        rb.rtc_cr().modify(|_, w| w.alme().clear_bit());
        let mut counter: u32 = 0;
        loop {
            if rb.rtc_isr().read().almwf().bit_is_set() {
                break;
            }
            if counter >= ALARMDIS_TIMEOUT {
                break;
            }
            counter += 1;
        }

        // Clear ALMF (RW1C, bit 8).
        {
            let isr_ptr = rb.rtc_isr().as_ptr();
            let isr = rb.rtc_isr().read().bits();
            let val = (isr & !(1 << 10) & !(1 << 15)) | (1 << 8);
            unsafe { isr_ptr.write_volatile(val) };
        }

        let ok = if rb.rtc_isr().read().almwf().bit_is_set() {
            rb.rtc_almr1l().write(|w| unsafe { w.bits(almr1l_val) });

            let almr1h_val: u32 = (a.time.days as u32) | a.day_mask.bits();
            rb.rtc_almr1h().write(|w| unsafe { w.bits(almr1h_val) });
            true
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    /// Get the current alarm configuration.
    pub fn get_alarm(&self, fmt: TimeFormat) -> AlarmConfig {
        let rb = rtc_rb();
        let l = rb.rtc_almr1l().read();
        let h = rb.rtc_almr1h().read();

        let h_bcd = l.alr_hu().bits() | (l.alr_ht().bits() << 4);
        let m_bcd = l.alr_mnu().bits() | (l.alr_mnt().bits() << 4);
        let s_bcd = l.alr_su().bits() | (l.alr_st().bits() << 4);

        let (hours, minutes, seconds) = match fmt {
            TimeFormat::Bcd => (h_bcd, m_bcd, s_bcd),
            TimeFormat::Bin => (
                bcd2_to_byte(h_bcd),
                bcd2_to_byte(m_bcd),
                bcd2_to_byte(s_bcd),
            ),
        };

        let am_pm = if l.alr_pm().bit_is_set() {
            AmPm::Pm
        } else {
            AmPm::Am
        };

        let mut mask = AlarmMask::empty();
        if l.msk0().bit_is_set() {
            mask |= AlarmMask::SECONDS;
        }
        if l.msk1().bit_is_set() {
            mask |= AlarmMask::MINUTES;
        }
        if l.msk2().bit_is_set() {
            mask |= AlarmMask::HOURS;
        }

        let mut day_mask = AlarmDayMask::empty();
        if h.msk3().bit_is_set() {
            day_mask |= AlarmDayMask::DAYS;
        }

        AlarmConfig {
            time: TimeConfig {
                am_pm,
                year: BASE_YEAR,
                days: h.alr_day().bits(),
                hours,
                minutes,
                seconds,
            },
            mask,
            day_mask,
        }
    }

    /// Enable or disable the alarm (and its interrupt).
    pub fn alarm_cmd(&self, en: bool) {
        let rb = rtc_rb();
        wpr_unlock(rb);

        // Set ALMF before enabling — clear it before disabling.
        {
            let isr_ptr = rb.rtc_isr().as_ptr();
            let isr = rb.rtc_isr().read().bits();
            let val = (isr & !(1 << 10) & !(1 << 15)) | (1 << 8);
            unsafe { isr_ptr.write_volatile(val) };
        }

        if en {
            rb.rtc_cr().modify(|_, w| {
                w.alme().set_bit();
                w.almie().set_bit()
            });
            wait_for_synchro(rb);
        } else {
            rb.rtc_cr().modify(|_, w| {
                w.alme().clear_bit();
                w.almie().clear_bit()
            });

            let mut counter: u32 = 0;
            loop {
                let isr = rb.rtc_isr().read();
                let cr = rb.rtc_cr().read();
                if isr.almwf().bit_is_set() && cr.alme().bit_is_clear() {
                    break;
                }
                if counter >= ALARMDIS_TIMEOUT {
                    break;
                }
                counter += 1;
            }
        }

        wpr_lock(rb);
    }

    /// Clear the alarm pending interrupt (preserves DOVTHF and WUTF).
    pub fn clear_alarm(&self) {
        let rb = rtc_rb();
        let isr_ptr = rb.rtc_isr().as_ptr();
        let isr = rb.rtc_isr().read().bits();

        // Preserve DOVTHF (bit 15) and WUTF (bit 10); set ALMF (bit 8).
        let val = (isr & !(1 << 15) & !(1 << 10)) | (1 << 8);
        unsafe { isr_ptr.write_volatile(val) };

        let mut counter: u32 = 0;
        loop {
            if rb.rtc_isr().read().almf().bit_is_clear() {
                break;
            }
            if counter >= ALARMDIS_TIMEOUT {
                break;
            }
            counter += 1;
        }
    }

    // ── Wakeup timer ──────────────────────────────────────────────────────────

    /// Configure the wakeup auto-reload value. Returns `true` on success.
    ///
    /// Wakeup timer must be disabled before calling (`wakeup_cmd(false)`).
    pub fn set_wakeup(&self, range: u32) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        rb.rtc_cr().modify(|_, w| w.wute().clear_bit());

        let mut counter: u32 = 0;
        loop {
            counter += 1;
            if rb.rtc_isr().read().wutwf().bit_is_set() {
                break;
            }
            if counter > WUTDIS_TIMEOUT {
                break;
            }
        }

        // Clear WUTF (bit 10) — RW1C.
        {
            let isr_ptr = rb.rtc_isr().as_ptr();
            let isr = rb.rtc_isr().read().bits();
            let val = (isr & !(1 << 8) & !(1 << 15)) | (1 << 10);
            unsafe { isr_ptr.write_volatile(val) };
        }

        let ok = if rb.rtc_isr().read().wutwf().bit_is_set() {
            rb.rtc_wutr()
                .write(|w| unsafe { w.wut().bits(range & 0x0001_FFFF) });
            true
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    /// Read the current wakeup auto-reload value.
    pub fn wakeup(&self) -> u32 {
        rtc_rb().rtc_wutr().read().wut().bits()
    }

    /// Enable or disable the wakeup timer (and its interrupt).
    ///
    /// When enabling, this blocks for several seconds waiting for the WUTE
    /// shadow register to synchronise.
    pub fn wakeup_cmd(&self, en: bool) {
        let rb = rtc_rb();
        wpr_unlock(rb);

        // Clear WUTF before any state change.
        {
            let isr_ptr = rb.rtc_isr().as_ptr();
            let isr = rb.rtc_isr().read().bits();
            let val = (isr & !(1 << 8) & !(1 << 15)) | (1 << 10);
            unsafe { isr_ptr.write_volatile(val) };
        }

        if en {
            rb.rtc_cr().modify(|_, w| {
                w.wute().set_bit();
                w.wutie().set_bit()
            });
            wait_for_wut_synchro(rb);
        } else {
            rb.rtc_cr().modify(|_, w| {
                w.wute().clear_bit();
                w.wutie().clear_bit()
            });

            let mut counter: u32 = 0;
            loop {
                counter += 1;
                let isr = rb.rtc_isr().read();
                let cr = rb.rtc_cr().read();
                if isr.wutwf().bit_is_set() && cr.wute().bit_is_clear() {
                    break;
                }
                if counter > WUTDIS_TIMEOUT {
                    break;
                }
            }
        }

        wpr_lock(rb);
    }

    /// Clear the wakeup timer pending interrupt (preserves DOVTHF and ALMF).
    pub fn clear_wakeup(&self) {
        let rb = rtc_rb();
        let isr_ptr = rb.rtc_isr().as_ptr();
        let isr = rb.rtc_isr().read().bits();

        // Preserve DOVTHF (bit 15) and ALMF (bit 8); set WUTF (bit 10).
        let val = (isr & !(1 << 15) & !(1 << 8)) | (1 << 10);
        unsafe { isr_ptr.write_volatile(val) };

        let mut counter: u32 = 0;
        loop {
            counter += 1;
            if rb.rtc_isr().read().wutf().bit_is_clear() {
                break;
            }
            if counter > WUTDIS_TIMEOUT {
                break;
            }
        }
    }

    // ── Daylight saving ───────────────────────────────────────────────────────

    /// Apply a daylight-saving adjustment and record it in the `BKP` bit.
    /// Returns `true` on success.
    pub fn daylight_saving(&self, op: DaylightSaving, store: StoreOp) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        let ok = if enter_init_mode(rb) {
            rb.rtc_cr().modify(|_, w| {
                w.bkp().bit(store == StoreOp::Set);
                match op {
                    DaylightSaving::Add1h => w.add1h().set_bit(),
                    DaylightSaving::Sub1h => w.sub1h().set_bit(),
                }
            });
            exit_init_mode(rb);
            wait_for_synchro(rb)
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    /// Read the `BKP` (daylight-saving memo) bit from `RTC_CR`.
    pub fn store_operation(&self) -> StoreOp {
        if rtc_rb().rtc_cr().read().bkp().bit_is_set() {
            StoreOp::Set
        } else {
            StoreOp::Reset
        }
    }

    // ── Output configuration ──────────────────────────────────────────────────

    /// Configure which signal is routed to the RTC_OUT pin.
    /// Returns `true` on success.
    pub fn set_output(&self, out: OutputSel) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        let ok = if enter_init_mode(rb) {
            rb.rtc_cr()
                .modify(|_, w| unsafe { w.osel().bits(out as u8) });
            exit_init_mode(rb);
            wait_for_synchro(rb)
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }

    // ── Smooth calibration ────────────────────────────────────────────────────

    /// Configure or disable the RTC smooth calibration.
    ///
    /// - `en = false`  → disable calibration (clears DCE in `RTC_CALIBR`)
    /// - `en = true`   → set `dc`, `sign`, `period`, enable DCE
    ///
    /// Returns `true` on success.
    ///
    /// `val` is the 7-bit calibration value (0–127 ppm magnitude).
    pub fn set_calib(&self, sign: CalibSign, val: u8, period: CalibPeriod, en: bool) -> bool {
        let rb = rtc_rb();
        wpr_unlock(rb);

        // Wait for any in-progress recalibration to complete.
        let mut counter: u32 = 0;
        loop {
            if rb.rtc_isr().read().recalpf().bit_is_clear() {
                break;
            }
            if counter >= RECALPF_TIMEOUT {
                break;
            }
            counter += 1;
        }

        let ok = if rb.rtc_isr().read().recalpf().bit_is_clear() {
            if !en {
                // Disable: clear DCE bit only.
                rb.rtc_calibr().modify(|_, w| w.dce().clear_bit());
            } else {
                // RTC_CALIBR write must be atomic — use write (not modify) so
                // RECALPF is asserted once and hardware sees a single update.
                rb.rtc_calibr().write(|w| unsafe {
                    w.dc().bits(val & 0x7F);
                    w.dcs().bit(sign == CalibSign::Negative);
                    w.dce().set_bit();
                    w.calp().bits(period as u8)
                });
            }
            true
        } else {
            false
        };

        wpr_lock(rb);
        ok
    }
}
