//! RTL8721F clock driver — `#![no_std]` Rust port of `ameba_clk.c`.
//!
//! # Functions converted
//! - [`sdm32k_enable`]
//! - [`sdm32k_timer_cal_enable`]
//! - [`sdm32k_cal_factor_modify`]
//! - [`osc_cal_result_get`]
//! - [`osc131_r_set`]
//! - [`osc131k_calibration`]
//! - [`osc131k_first_pon`], [`osc131k_set_first_pon`], [`chip_type_is_asic`],
//!   [`osc131k_cold_boot_init`] — additions not in `ameba_clk.c` itself;
//!   port the `first_pon`-gated calibration call out of the SDK's
//!   `RTC_DET_IRQ` handler (`ameba_app_start.c : rtc_irq_init`) into a
//!   plain function callable from `chip_pre_init()`.
//! - [`osc4m_r_set`]
//! - [`osc4m_vcm_set`]
//! - [`osc4m_calibration`]
//! - [`osc4m_init`]
//! - [`xtal_init`]
//! - [`xtal_aack`]
//!
//! # Functions skipped
//! `_PERI_ClkGet`, `USB_PLL_ClkGet`, `SYS_PLL_ClkGet`, `SHPERI_ClkGet`,
//! `HPERI_ClkGet`, `PSRAMC_ClkGet`, `VO_ClkGet`, `CPU_ClkGet`,
//! `CPU_Clk_Check`, `PLL_ClkSrcGet`, `RCC_PeriphClockEnableChk` —
//! all require `RRAM_DEV->clk_info_bk` (retained-RAM struct, no PAC
//! equivalent) or IPC semaphores / inter-core RPC.

// ---------------------------------------------------------------------------
// ROM / external C symbols
// ---------------------------------------------------------------------------

use crate::delay::Delay;
use embedded_hal::delay::DelayNs;

// ---------------------------------------------------------------------------
// PAC peripheral block references
// ---------------------------------------------------------------------------

use rtl8721f_pac::{LdoReg, LpSys, PllReg, RtcMisc, SdmReg, XtalReg};
use rtl8721f_pac::{ldo_reg, lp_sys, pll_reg, rtc_misc, sdm_reg, xtal_reg};

#[inline(always)]
fn sdm() -> &'static sdm_reg::RegisterBlock {
    unsafe { &*SdmReg::PTR }
}

#[inline(always)]
fn pll() -> &'static pll_reg::RegisterBlock {
    unsafe { &*PllReg::PTR }
}

#[inline(always)]
fn rtc_misc() -> &'static rtc_misc::RegisterBlock {
    unsafe { &*RtcMisc::PTR }
}

#[inline(always)]
fn ldo() -> &'static ldo_reg::RegisterBlock {
    unsafe { &*LdoReg::PTR }
}

#[inline(always)]
fn lp_sys() -> &'static lp_sys::RegisterBlock {
    unsafe { &*LpSys::PTR }
}

#[inline(always)]
fn xtal() -> &'static xtal_reg::RegisterBlock {
    unsafe { &*XtalReg::PTR }
}

// ---------------------------------------------------------------------------
// Type-safe enums
// ---------------------------------------------------------------------------

/// Clock source selector for [`osc_cal_result_get`].
///
/// Maps to the 2-bit `ck_cal_sel` field of `PLL_REG.REG_CLK_CALC`.
/// The reference clock is always the XTAL.
/// SDK: ameba_clk.h : OSC4M_CAL_CLK / AON100K_CAL_CLK / AON128K_CAL_CLK
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CalClk {
    /// 4 MHz internal oscillator (OSC4M). Selector value 0.
    Osc4m = 0,
    /// AON 100 K oscillator. Selector value 1.
    Aon100k = 1,
    /// AON 128 K oscillator (SDM 32 K path). Selector value 2.
    Aon128k = 2,
}

// ---------------------------------------------------------------------------
// Constants (from C headers)
// ---------------------------------------------------------------------------

/// SDM timer calibration interval — 1-minute countdown value.
/// SDK: sysreg_sdm.h : SDM_TIEMRCAL_INTERVAL_1MIN
const SDM_TIEMRCAL_INTERVAL_1MIN: u32 = 0x001e_0000;

/// Default observed 128 K cycle count for SDM user-defined mode.
/// SDK: ameba_clk.h : SDM_OBS_CYC_DEFAULT
const SDM_OBS_CYC_DEFAULT: u16 = 1024;

/// Modified reference cycle count: default 312500 + 3 = 312503.
/// SDK: ameba_clk.h : SDM_OBS_REF_CYC_MODIFY
const SDM_OBS_REF_CYC_MODIFY: u32 = 312503;

/// Default XTAL period value for SDM user-defined mode (ns × scale).
/// SDK: ameba_clk.h : SDM_XTAL_PERIOD_DEFAULT
const SDM_XTAL_PERIOD_DEFAULT: u32 = 102400;

/// Bit 10 of `LDO_REG.REG_DEBUG.adbg_regu` — OSC OK status flag.
/// SDK: sysreg_ldo.h : LDO_DBG_OSC_OK
const LDO_DBG_OSC_OK: u32 = 1 << 10;

/// `ANAPAR_XTAL_MODE_DEC_ON_1.xtal_mode_dec` value for HP mode (0b11 = 3).
/// SDK: ameba_clk.h : XTAL_HP_MODE
const XTAL_HP_MODE: u8 = 3;

// ---------------------------------------------------------------------------
// SDM32K functions
// ---------------------------------------------------------------------------

/// Enable the 32 K SDM clock in default (non-user-defined) mode.
///
/// Sets the timer timeout to 1 minute, enables SDM, asserts reset,
/// enables always-calibration and timer-triggered calibration, and
/// switches to default mode (clears `sdm_mod_sel`).
///
/// Corresponds to `SDM32K_Enable()` in `ameba_clk.c`.
pub fn sdm32k_enable() {
    let sdm = sdm();

    sdm.sdm_timeout()
        .modify(|_, w| unsafe { w.timeout_cnt().bits(SDM_TIEMRCAL_INTERVAL_1MIN) });

    sdm.sdm_ctrl0().modify(|_, w| {
        w.sdm_en()
            .set_bit()
            .sdm_rst()
            .set_bit()
            .always_cal_en()
            .set_bit()
            .timer_cal_en()
            .set_bit()
            // Switch to default mode (clear user-defined mode select).
            .sdm_mod_sel()
            .clear_bit()
    });
}

/// Enable or disable the SDM 32 K timer-triggered auto-calibration.
///
/// `true` enables timer calibration; `false` disables it.
///
/// Corresponds to `SDM32K_TimerCalEnable()` in `ameba_clk.c`.
pub fn sdm32k_timer_cal_enable(enable: bool) {
    sdm().sdm_ctrl0().modify(|_, w| {
        if enable {
            w.timer_cal_en().set_bit()
        } else {
            w.timer_cal_en().clear_bit()
        }
    });
}

/// Modify the SDM 32 K calibration factor (user-defined parameter mode).
///
/// When `enable` is `true`, loads the modified reference cycle count
/// (`SDM_OBS_REF_CYC_MODIFY` = 312 503, i.e. default 312 500 + 3) and
/// switches to user-defined mode.  When `false`, switches back to
/// default mode.
///
/// Corresponds to `SDM32K_CalFactorModify()` in `ameba_clk.c`.
pub fn sdm32k_cal_factor_modify(enable: bool) {
    let sdm = sdm();
    if enable {
        sdm.sdm_ctrl1()
            .modify(|_, w| unsafe { w.sdm_obs_cyc().bits(SDM_OBS_CYC_DEFAULT) });
        sdm.sdm_ctrl2()
            .modify(|_, w| unsafe { w.sdm_obs_ref_cyc().bits(SDM_OBS_REF_CYC_MODIFY) });
        sdm.sdm_ctrl3()
            .modify(|_, w| unsafe { w.xtal_period().bits(SDM_XTAL_PERIOD_DEFAULT) });
        sdm.sdm_ctrl0().modify(|_, w| w.sdm_mod_sel().set_bit());
    } else {
        sdm.sdm_ctrl0().modify(|_, w| w.sdm_mod_sel().clear_bit());
    }
}

// ---------------------------------------------------------------------------
// OSC calibration helper
// ---------------------------------------------------------------------------

/// Start a hardware clock calibration and return the raw 16-bit result.
///
/// Selects `cal_clk` as the clock under test (reference is always XTAL),
/// asserts `ck_cal_start`, polls until the hardware auto-clears it
/// (calibration done), then returns `ck_cal_rpt` widened to `u32`.
///
/// Corresponds to `OSC_CalResult_Get()` in `ameba_clk.c`.
pub fn osc_cal_result_get(cal_clk: CalClk) -> u32 {
    let pll = pll();

    // Start calibration: load selector and set start bit.
    pll.reg_clk_calc()
        .modify(|_, w| unsafe { w.ck_cal_sel().bits(cal_clk as u8).ck_cal_start().set_bit() });

    // Poll until the hardware clears the start bit (calibration complete).
    while pll.reg_clk_calc().read().ck_cal_start().bit_is_set() {}

    // Return 16-bit calibration result, zero-extended to u32.
    pll.reg_clk_calc().read().ck_cal_rpt().bits() as u32
}

// ---------------------------------------------------------------------------
// OSC131K (AON 131.072 K) functions
// ---------------------------------------------------------------------------

/// Set or clear bits in the OSC131K RCAL (resistance calibration) field.
///
/// `setbit` and `clearbit` are masks within the 6-bit `rcal` field
/// (bits 0–5 of `OSC131K_CTRL`).  The function reads the current register,
/// applies the set/clear masks to the `rcal` value, writes it back, then
/// delays 1 ms for the oscillator to stabilise.
///
/// Corresponds to `OSC131_R_Set()` in `ameba_clk.c`.
pub fn osc131_r_set(setbit: u32, clearbit: u32) {
    let osc = rtc_misc();
    let current = osc.osc131k_ctrl().read().rcal().bits() as u32;
    let new_rcal = (current | setbit) & !clearbit;
    osc.osc131k_ctrl()
        .modify(|_, w| unsafe { w.rcal().bits((new_rcal & 0x3f) as u8) });
    // 1 ms for oscillator to stabilise.
    Delay::new().delay_ms(1);
}

/// Calibrate the AON 131.072 K oscillator using binary search on `rcal`.
///
/// Performs 6 iterations of MSB-first binary search over the 6-bit `rcal`
/// field.  Each step measures calibration against the XTAL reference
/// (`AON128K_CAL_CLK`) and checks whether the result is above or below
/// the target (2441 XTAL cycles ≈ 8 × 40 MHz / 131.072 kHz).  After
/// all iterations the best `rcal` found is applied if the last step was
/// not already optimal.
///
/// Returns `true` on success (PPM within `ppm_limit`); panics if calibration
/// fails, mirroring the SDK, whose `assert_param(0)` reaches
/// `io_assert_failed()` and spins in `for (;;)`.
///
/// Blocking with no timeout, also mirroring the SDK: the `LDO_DEBUG.OSC_OK`
/// and `OSC131K_POW.POW_32KOSC` waits and the `REG_CLK_CALC` completion poll
/// in [`osc_cal_result_get`] all spin until the hardware responds. Since this
/// runs synchronously during boot (the SDK runs it from `RTC_DET_IRQ`, whose
/// handler would spin just the same), an OSC131K/XTAL that never comes up
/// stalls the boot rather than surfacing an error.
///
/// Corresponds to `OSC131K_Calibration()` in `ameba_clk.c`.
pub fn osc131k_calibration(ppm_limit: u32) -> bool {
    // target = 8 × 40 MHz / 131.072 kHz = 2441 XTAL cycles.
    // SDK: ameba_clk.c : OSC131K_Calibration
    let target_40m_counter: u32 = 2441;

    let osc = rtc_misc();
    let ldo_r = ldo();

    // Wait until LDO reports OSC OK (bit 10 of REG_DEBUG).
    while (ldo_r.reg_debug().read().adbg_regu().bits() & LDO_DBG_OSC_OK) == 0 {}

    // Clear RCAL calibration field before the binary search.
    osc.osc131k_ctrl()
        .modify(|_, w| unsafe { w.rcal().bits(0) });

    // Wait for the oscillator power-on confirmation (POW_32KOSC bit).
    while !osc.osc131k_pow().read().pow_32kosc().bit_is_set() {}

    let mut clearbit: u32 = 0;
    let mut delta: u32 = 0;
    let mut min_delta: u32 = 0xFFFF_FFFF;
    let mut min_delta_r: u32 = 0;

    // Binary search over bits 5..=0 of RCAL (MSB first).
    for cal_n in 1u32..=6 {
        let rsel_bit_index = 6 - cal_n;
        osc131_r_set(1 << rsel_bit_index, clearbit);

        let temp = osc_cal_result_get(CalClk::Aon128k);

        if temp < target_40m_counter {
            delta = target_40m_counter - temp;
            clearbit = 1 << rsel_bit_index;
        } else {
            delta = temp - target_40m_counter;
            clearbit = 0;
        }

        if delta < min_delta {
            min_delta = delta;
            // Capture only the rcal field.
            min_delta_r = osc.osc131k_ctrl().read().rcal().bits() as u32;
        }
    }

    // If the final iteration was not the global minimum, restore the best rcal.
    if delta > min_delta {
        osc.osc131k_ctrl()
            .modify(|_, w| unsafe { w.rcal().bits((min_delta_r & 0x3f) as u8) });
        // 1 ms stabilisation.
        Delay::new().delay_ms(1);

        let temp = osc_cal_result_get(CalClk::Aon128k);
        if temp < target_40m_counter {
            delta = target_40m_counter - temp;
        } else {
            delta = temp - target_40m_counter;
        }
    }

    let cur_ppm = delta * 1_000_000 / target_40m_counter;

    if cur_ppm >= ppm_limit {
        panic!(
            "[CAL131K] calibration FAILED: PPM {} >= limit {}",
            cur_ppm, ppm_limit
        );
    }

    true
}

/// Has the AON `first_pon` latch already been set?
///
/// `first_pon` (`RTC_MISC.OSC131K_STATE` bit 0) is hardware-latched (`R/WH`):
/// once SW writes 1 it stays 1 "until next reset" per the register description,
/// so it survives a reflash and the warm resets used during development. Used
/// to run [`osc131k_calibration`] at most once per reset cycle.
///
/// Note the SDK's `RTC_ClkSource_Select()` writes this register with `&=`
/// (`ameba_rtc.c`), so selecting `SDM32K` (0) there clears the whole register
/// including this latch. This port does not reproduce that write.
///
/// SDK: `ameba_rtc.c : Get_OSC131_STATE()` & `RTC_BIT_FIRST_PON`;
/// bit description in `ameba_osc131k.h`.
pub fn osc131k_first_pon() -> bool {
    rtc_misc().osc131k_state().read().first_pon().bit_is_set()
}

/// Latch the AON `first_pon` bit (write-1, hardware holds it until reset).
///
/// Corresponds to `Set_OSC131_STATE(Get_OSC131_STATE() | RTC_BIT_FIRST_PON)`
/// in `ameba_rtc.c`.
pub fn osc131k_set_first_pon() {
    rtc_misc()
        .osc131k_state()
        .modify(|_, w| w.first_pon().set_bit());
}

/// Is the chip an ASIC (as opposed to FPGA/PALADIUM/RTL-sim)?
///
/// Reads `LSYS.REG_SYSTEM_CFG0.hw_chip_type` (00 = ASIC). OSC131K/OSC4M
/// calibration only run on real ASIC silicon — FPGA/emulation clocks don't
/// need (and can't usefully do) hardware trim.
///
/// Corresponds to `SYSCFG_CHIPType_Get() == CHIP_TYPE_ASIC_POSTSIM` in
/// `ameba_syscfg_rom.c` / `sysreg_lsys.h`.
pub fn chip_type_is_asic() -> bool {
    lp_sys().reg_system_cfg0().read().hw_chip_type().bits() == 0
}

/// Cold-boot OSC131K calibration, gated by the `first_pon` hardware latch.
///
/// Enables the RTC clock gate and the `fen_rtc` function-enable bit
/// (idempotent — safe alongside a later `rcc::enable(Peripheral::RTC)` /
/// `Rtc::enable(true)` call by an RTC driver), then, on the first boot of a
/// reset cycle, latches [`osc131k_first_pon`] so this never runs again until
/// the next reset and — on ASIC silicon only — runs [`osc131k_calibration`]
/// (PPM limit 30000 = 3%, matching `app_start()`'s own call).
///
/// The calibration measurement does not depend on the RTC clock source:
/// [`osc_cal_result_get`] counts the OSC131K oscillator itself against the
/// XTAL reference via `PLL.REG_CLK_CALC`, independent of `cksl_rtc`. (The SDK
/// likewise calls `RTC_ClkSource_Select()` only *after* calibrating.)
///
/// `fen_rtc` must be set before calibration: [`osc131k_calibration`] polls
/// `LDO_DEBUG.OSC_OK` and `OSC131K_POW.POW_32KOSC`, both gated by `fen_rtc`
/// powering the OSC131K analog block — without it those bits never assert
/// and the calibration polling loops spin forever. This mirrors the SDK,
/// where `rtc_irq_init()` runs `app_rtc_init()` (whose first action is
/// `RTC_Enable(ENABLE)`) before `OSC131K_Calibration()`.
///
/// Deliberately does not touch `RTC_Init`/`RTC_SetTime`/`RTC_ClkSource_Select`
/// — those belong to the RTC driver's own init sequence.
///
/// The SDK performs this same check inside the `RTC_DET_IRQ` handler
/// (`rtc_irq_init()` in `ameba_app_start.c`), which additionally handles
/// `app_rtc_init()` and switches the RTC clock source before returning.
/// This port only reproduces the calibration half of that handler and runs
/// it once, synchronously, during boot rather than from an interrupt.
///
/// Corresponds to the `first_pon`/`OSC131K_Calibration` portion of
/// `rtc_irq_init()` in `ameba_app_start.c`.
pub fn osc131k_cold_boot_init() {
    crate::rcc::enable(crate::rcc::Peripheral::RTC);
    rtc_misc()
        .osc131k_ctrl()
        .modify(|_, w| w.fen_rtc().set_bit());

    if !osc131k_first_pon() {
        // Latch `first_pon` *before* calibrating, matching the SDK order in
        // `rtc_irq_init()` (Set_OSC131_STATE then OSC131K_Calibration). This
        // ordering matters on failure: `osc131k_calibration` hangs forever when
        // the trim is out of spec (as does the SDK — `assert_param(0)` ends in
        // `for (;;)`). With the latch already set, a marginal chip hangs once
        // and the next warm reset skips calibration and boots (uncalibrated but
        // alive); latching afterwards would never be reached, hanging on every
        // boot instead.
        osc131k_set_first_pon();

        if chip_type_is_asic() {
            osc131k_calibration(30_000);
        }
    }
}

// ---------------------------------------------------------------------------
// OSC4M functions
// ---------------------------------------------------------------------------

/// Set or clear bits in the 4 MHz OSC `freq_r_sel` (8-bit) field.
///
/// Reads the current `freq_r_sel` value, applies `setbit` / `clearbit`
/// masks, writes the result back, then delays 2 µs for the oscillator
/// to stabilise.
///
/// Corresponds to `OSC4M_R_Set()` in `ameba_clk.c`.
pub fn osc4m_r_set(setbit: u32, clearbit: u32) {
    let ldo_r = ldo();
    let current = ldo_r.reg_4m_osc_ctrl1().read().freq_r_sel().bits() as u32;
    let new_val = (current | setbit) & !clearbit;
    ldo_r
        .reg_4m_osc_ctrl1()
        .modify(|_, w| unsafe { w.freq_r_sel().bits((new_val & 0xff) as u8) });
    Delay::new().delay_us(2);
}

/// Set both `vcm_sel_l` and `vcm_sel_h` fields to the same `value` (4 bits).
///
/// Delays 20 µs before writing (settling), 20 µs after writing, and a
/// further 2 µs (final stabilisation).
///
/// Corresponds to `OSC4M_VCM_Set()` in `ameba_clk.c`.
pub fn osc4m_vcm_set(value: u8) {
    let ldo_r = ldo();
    Delay::new().delay_us(20);
    ldo_r.reg_4m_osc_ctrl1().modify(|_, w| unsafe {
        w.vcm_sel_l()
            .bits(value & 0xf)
            .vcm_sel_h()
            .bits(value & 0xf)
    });
    Delay::new().delay_us(20);
    Delay::new().delay_us(2);
}

/// Calibrate the 4 MHz internal oscillator.
///
/// **Phase 1 — VCM sweep** (`cal_n` = 5..=9): sweeps `vcm_sel_l` /
/// `vcm_sel_h` from 5 to 9 to minimise the frequency error relative to
/// the XTAL reference.  The best VCM setting found is applied before
/// moving to phase 2 (unless an exact match is found earlier).
///
/// **Phase 2 — R_SEL binary search** (`cal_n` = 1..=8, MSB first):
/// binary-searches the 8-bit `freq_r_sel` field to further reduce error.
///
/// Target counter = 320 XTAL cycles (32 × 40 MHz / 4 MHz).
///
/// Returns `true` on success; panics if `cur_ppm >= ppm_limit`.
///
/// Corresponds to `OSC4M_Calibration()` in `ameba_clk.c`.
pub fn osc4m_calibration(ppm_limit: u32) -> bool {
    // target = 32 × 40 MHz / 4 MHz = 320 XTAL cycles.
    // SDK: ameba_clk.c : OSC4M_Calibration
    let target_40m_counter: u32 = 320;

    let ldo_r = ldo();

    // Step 1: Power on the 4 MHz OSC.
    ldo_r
        .reg_rf_en_dummy()
        .modify(|_, w| w.reg_pow_4mosc().set_bit());
    Delay::new().delay_us(20);

    // Step 2: Clear VCM calibration fields (vcm_sel_l = vcm_sel_h = 0).
    ldo_r
        .reg_4m_osc_ctrl1()
        .modify(|_, w| unsafe { w.vcm_sel_l().bits(0).vcm_sel_h().bits(0) });
    Delay::new().delay_us(20);

    let mut delta: u32 = 0;
    let mut min_delta: u32 = 0xFFFF_FFFF;
    // Packed snapshot: bits[7:0]=freq_r_sel, bits[11:8]=vcm_sel_l,
    // bits[15:12]=vcm_sel_h.  Mirrors the C `min_delta_r &=
    // (VCM_SEL_L | VCM_SEL_H | FREQ_R_SEL)` capture.
    let mut min_delta_r: u32 = 0;
    let mut cal_end = false;

    // -----------------------------------------------------------------------
    // Phase 1: VCM sweep (cal_n = 5..=9)
    // -----------------------------------------------------------------------
    'vcm_loop: for cal_n in 5u8..=9 {
        osc4m_vcm_set(cal_n);

        let temp = osc_cal_result_get(CalClk::Osc4m);

        if temp < target_40m_counter {
            delta = target_40m_counter - temp;
        } else if temp > target_40m_counter {
            delta = temp - target_40m_counter;
        } else {
            delta = 0;
            cal_end = true;
            break 'vcm_loop;
        }

        if delta < min_delta {
            min_delta = delta;
            Delay::new().delay_us(20);
            let snap = ldo_r.reg_4m_osc_ctrl1().read();
            min_delta_r = (snap.freq_r_sel().bits() as u32)
                | ((snap.vcm_sel_l().bits() as u32) << 8)
                | ((snap.vcm_sel_h().bits() as u32) << 12);
        }
    }

    if !cal_end {
        // Apply the best VCM setting if the last iteration was not optimal.
        if delta > min_delta {
            let best_vcm_l = ((min_delta_r >> 8) & 0xf) as u8;
            let best_vcm_h = ((min_delta_r >> 12) & 0xf) as u8;
            ldo_r.reg_4m_osc_ctrl1().modify(|_, w| unsafe {
                w.vcm_sel_l().bits(best_vcm_l).vcm_sel_h().bits(best_vcm_h)
            });
            Delay::new().delay_us(20);
        }

        // Clear freq_r_sel before the R_SEL binary search.
        ldo_r
            .reg_4m_osc_ctrl1()
            .modify(|_, w| unsafe { w.freq_r_sel().bits(0) });
        Delay::new().delay_us(20);

        // Reset tracking variables for phase 2.
        min_delta = 0xFFFF_FFFF;
        let mut clearbit: u32 = 0;

        // -------------------------------------------------------------------
        // Phase 2: R_SEL binary search (cal_n = 1..=8, MSB first)
        // -------------------------------------------------------------------
        'r_loop: for cal_n in 1u32..=8 {
            let rsel_bit_index = 8 - cal_n;
            osc4m_r_set(1 << rsel_bit_index, clearbit);

            let temp = osc_cal_result_get(CalClk::Osc4m);

            if temp < target_40m_counter {
                delta = target_40m_counter - temp;
                clearbit = 0;
            } else if temp > target_40m_counter {
                delta = temp - target_40m_counter;
                clearbit = 1 << rsel_bit_index;
            } else {
                delta = 0;
                cal_end = true;
                break 'r_loop;
            }

            if delta < min_delta {
                min_delta = delta;
                Delay::new().delay_us(20);
                min_delta_r = ldo_r.reg_4m_osc_ctrl1().read().freq_r_sel().bits() as u32;
            }
        }

        // Restore the best freq_r_sel if the last iteration was not optimal.
        if !cal_end && delta > min_delta {
            ldo_r
                .reg_4m_osc_ctrl1()
                .modify(|_, w| unsafe { w.freq_r_sel().bits((min_delta_r & 0xff) as u8) });
            // 2 µs stabilisation.
            Delay::new().delay_us(2);

            let temp = osc_cal_result_get(CalClk::Osc4m);
            if temp < target_40m_counter {
                delta = target_40m_counter - temp;
            } else {
                delta = temp - target_40m_counter;
            }
        }
    }

    // Final PPM check (mirrors the `cal_end:` label in C).
    let cur_ppm = delta * 1_000_000 / target_40m_counter;

    if cur_ppm >= ppm_limit {
        panic!(
            "[CAL4M] calibration FAILED: PPM {} >= limit {}",
            cur_ppm, ppm_limit
        );
    }

    true
}

/// No-operation OSC4M initialisation stub.
///
/// Corresponds to `OSC4M_Init()` in `ameba_clk.c` (empty body).
#[inline(always)]
pub fn osc4m_init() {}

// ---------------------------------------------------------------------------
// XTAL functions
// ---------------------------------------------------------------------------

/// Initialise the XTAL USB drive strength.
///
/// Sets `xtal_drv_usb` to 0x1 in `ANAPAR_XTAL_OFF_0` to reduce the
/// 2440 MHz spur observed by the RF receiver.
///
/// SDK: `ameba_clk.c : XTAL_INIT()` — `temp |= XTAL_DRV_USB(0x1)`, where
/// `sysreg_xtal.h : XTAL_DRV_USB(x) = (x & 0x3) << 13` is a 2-bit field at
/// bit 13 of `XTAL_ANAPAR_XTAL_OFF_0`; value `0x1` matches the SDK exactly
/// (not `0x3`/"large current" — see the register's own R/W comment).
pub fn xtal_init() {
    xtal()
        .anapar_xtal_off_0()
        .modify(|_, w| unsafe { w.xtal_drv_usb().bits(0x1) });
}

/// Run the XTAL Automatic Amplitude Calibration (AACK) sequence.
///
/// Sequence (mirrors `XTAL_AACK()` in `ameba_clk.c`):
///
/// 1. Read `ANAPAR_XTAL_MODE_DEC_ON_1.xtal_mode_dec` and assert HP mode
///    (value 0b11 = 3) in debug builds.
/// 2. Enable the DIGI drive clock by setting
///    `ANAPAR_XTAL_ON_1.en_xtal_drv_digi`.
/// 3. Toggle the AACK module reset: clear then set
///    `XTAL_FEN.fen_xaack`.
/// 4. Set `ANAPAR_XTAL_AAC_ON_1.pk_start_ar` and `pk_end_ar` both to
///    0x3 (single GM-value set across all peak levels).
/// 5. Set `ANAPAR_XTAL_ON_2.aac_mode` to 0x3 (use `XTAL_SC_XI/XO` caps).
/// 6. Set `ANAPAR_XTAL_OFF_0.en_xtal_aac_pkdet`; delay 50 µs.
/// 7. Set `ANAPAR_XTAL_AAC.en_xtal_aac_digi`; delay 50 µs.
/// 8. Set bit 4 of `ANAPAR_XTAL_OFF_0` (mapped to `xqsel_rf_awake` in
///    the PAC — this is the hardware bit for `XTAL_BIT_EN_XTAL_AAC_GM`
///    as used in the C source `XTAL_ANAPAR_XTAL_OFF_0 |= XTAL_BIT_EN_XTAL_AAC_GM`);
///    delay 50 µs.
/// 9. Spin-wait until that bit reads back as set, re-asserting it if it
///    is found clear.
///
/// Corresponds to `XTAL_AACK()` in `ameba_clk.c`.
pub fn xtal_aack() {
    let xtal_r = xtal();

    // 1. Check XTAL mode; warn in debug builds if not HP.
    let xtal_mode = xtal_r
        .anapar_xtal_mode_dec_on_1()
        .read()
        .xtal_mode_dec()
        .bits();
    debug_assert_eq!(xtal_mode, XTAL_HP_MODE, "XTAL is not in HP mode");
    // Suppress unused-variable warning in release builds.
    let _ = xtal_mode;

    // 2. Open clock for AAC: enable DIGI drive.
    xtal_r
        .anapar_xtal_on_1()
        .modify(|_, w| w.en_xtal_drv_digi().set_bit());

    // 3. Reset AACK module (clear then set fen_xaack).
    xtal_r.xtal_fen().modify(|_, w| w.fen_xaack().clear_bit());
    xtal_r.xtal_fen().modify(|_, w| w.fen_xaack().set_bit());

    // 4. Configure AACK to calibrate a single set of GM values for all
    //    peak levels (pk_start_ar = pk_end_ar = 0x3 = field max).
    //    SDK: sysreg_xtal.h : XTAL_MASK_PK_START_AR, XTAL_MASK_PK_END_AR
    xtal_r
        .anapar_xtal_aac_on_1()
        .modify(|_, w| unsafe { w.pk_start_ar().bits(0x3).pk_end_ar().bits(0x3) });

    // 5. Set AAC_MODE = 0x3 (use XTAL_SC_XI/XO caps).
    //    SDK: sysreg_xtal.h : XTAL_MASK_AAC_MODE
    xtal_r
        .anapar_xtal_on_2()
        .modify(|_, w| unsafe { w.aac_mode().bits(0x3) });

    // 6. Enable AAC peak detector; wait 50 µs.
    xtal_r
        .anapar_xtal_off_0()
        .modify(|_, w| w.en_xtal_aac_pkdet().set_bit());
    Delay::new().delay_us(50);

    // 7. Enable AAC digital circuit; wait 50 µs.
    xtal_r
        .anapar_xtal_aac()
        .modify(|_, w| w.en_xtal_aac_digi().set_bit());
    Delay::new().delay_us(50);

    // 8. Enable AAC GM control.
    //
    // The C macro XTAL_BIT_EN_XTAL_AAC_GM is applied to the register
    // XTAL_ANAPAR_XTAL_OFF_0 (not ANAPAR_XTAL_ON_2).  In the PAC the
    // field at that bit position in ANAPAR_XTAL_OFF_0 is `xqsel_rf_awake`
    // (bit 4).  The write faithfully replicates what the C code does.
    xtal_r
        .anapar_xtal_off_0()
        .modify(|_, w| w.xqsel_rf_awake().set_bit());
    Delay::new().delay_us(50);

    // 9. Spin until bit 4 of ANAPAR_XTAL_OFF_0 reads back as set.
    //    If it is found clear, re-assert it (hardware may clear it
    //    transiently — matches the C loop body).
    loop {
        if xtal_r
            .anapar_xtal_off_0()
            .read()
            .xqsel_rf_awake()
            .bit_is_set()
        {
            break;
        }
        xtal_r
            .anapar_xtal_off_0()
            .modify(|_, w| w.xqsel_rf_awake().set_bit());
    }
}
