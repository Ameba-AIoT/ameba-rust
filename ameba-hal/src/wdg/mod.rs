//! System watchdog (`CPU0_S_WDG_DEV`) driver.
//!
//! SDK: `ameba_wdg.h` / `rom/rom_common/ameba_wdg.c`.
//!
//! This currently covers the secure system watchdog reachable from the KM4TZ core
//! (`Wdg`, `CPU0_S_WDG_DEV` / PAC `Wdg1RegS`). `CPU1_WDG_DEV` is KM4NS-only —
//! the SDK's own `ameba_vector_table.h` only exposes
//! `CPU0_S_WDG_IRQ`/`CPU0_NS_WDG_IRQ`/`IWDG_IRQ` under
//! `CONFIG_ARM_CORE_CM4_KM4TZ` (our core's build config) — and is out of
//! scope. Early-interrupt support (`WDG_CR.EIE`/`EICNT`) is also deferred;
//! hardware reset-on-timeout needs no ISR.
//!
//! `IWDG_DEV` is deliberately NOT exposed here. It is a single physical
//! peripheral (no secure/non-secure instance split, unlike the system
//! watchdog) that the NP (KM4NS) core's C firmware already owns
//! unconditionally at boot: `main/km4ns/src/main.c` : `app_IWDG_init()` sets
//! a 4095ms timeout and starts a perpetual 500ms refresh timer whenever the
//! `APBPeriph_IWDG` AON fuse bit is set — independent of KM4TZ's state, and
//! confirmed present on this board via the `"IWDG refresh thread Started!"`
//! boot log. Two consequences: (1) since NP keeps feeding it regardless of
//! whether KM4TZ is alive, IWDG can never actually detect a KM4TZ-only hang
//! — that job belongs to `Wdg` below; (2) a KM4TZ driver calling the same
//! magic-key unlock/`RELOAD`-write/re-lock sequence on `WDG_MKEYR`/`WDG_RLR`
//! would race NP's own independently-running sequence on the same registers,
//! for no corresponding safety benefit. Revisit only if a real cross-core
//! coordination mechanism (e.g. IPC-mediated single-owner handoff) is added.

use crate::rcc::{self, Peripheral};
use embedded_hal::delay::DelayNs;
use rtl8721f_pac::{Wdg1RegS, wdg1_reg_s};

/// Magic key: unlock `WDG_CR`/`WDG_RLR`/`WDG_WINR` for writes. SDK: `ameba_wdg.h` : `WDG_ACCESS_EN`.
const WDG_ACCESS_EN: u16 = 0x6969;
/// Magic key: re-lock register access. Raw literal in `ameba_wdg.c`'s `WDG_Init`/`WDG_Timeout`
/// (`WDG->WDG_MKEYR = 0xFFFF;`) — the SDK doesn't name this one.
const WDG_ACCESS_DIS: u16 = 0xFFFF;
/// Magic key: enable the watchdog function. SDK: `ameba_wdg.h` : `WDG_FUNC_EN`.
/// Once written, the SDK's own doc comment on `WDG_Enable` states it "can't Disabled by software".
const WDG_FUNC_EN: u16 = 0x3C3C;
/// Magic key: feed/refresh the counter. SDK: `ameba_wdg.h` : `WDG_REFRESH`.
const WDG_REFRESH: u16 = 0x5A5A;

/// No feed-window restriction (feed allowed at any point in the countdown).
/// SDK: `ameba_wdg.c` : `WDG_StructInit` default (`Window = 0xFFFF`), also the
/// hardware reset value of `WDG_WINR`.
const WDG_WINDOW_DISABLED: u16 = 0xFFFF;

/// Busy-poll iterations before giving up on `WDG_CR.RVU`/`EVU`.
/// SDK: `ameba_wdg.c` : `WDG_Wait_Busy` (`times > 1000`, 1 µs per iteration).
const WDG_WAIT_BUSY_TIMEOUT: u32 = 1000;

/// Prescaler for system watchdogs (as opposed to IWDG's `0x63`).
/// SDK: `ameba_wdg.c` : `WDG_Timeout`'s `else` branch (`IS_IWDG_PERIPH` false).
const WDG_PRESCALER: u8 = 0x1F;

fn regs() -> &'static wdg1_reg_s::RegisterBlock {
    unsafe { &*Wdg1RegS::PTR }
}

/// System watchdog driver (`CPU0_S_WDG_DEV` / secure `WDG1_REG_S`).
pub struct Wdg(());

impl Wdg {
    /// # Safety
    /// Only one instance may exist at a time.
    pub unsafe fn steal() -> Self {
        Self(())
    }

    /// Enable the peripheral clock and configure the reload timeout.
    ///
    /// `timeout_ms` is written directly into the 16-bit `WDG_RLR.RELOAD`
    /// field (clamped to 0xFFFF) — the SDK's own `WDG_Timeout(WDG, Timeout)`
    /// takes its `Timeout` parameter in ms and writes it in raw, unscaled
    /// (`WDG_RELOAD(x)` is a plain bit mask, no conversion).
    ///
    /// Mirrors `ameba_wdg.c` : `WDG_Init` with `EIMOD` disabled (no early
    /// interrupt) and the default `Window = 0xFFFF`.
    pub fn init(&self, timeout_ms: u32) {
        rcc::enable(Peripheral::WDG);
        Self::wait_busy();

        let regs = regs();
        regs.wdg_mkeyr()
            .write(|w| unsafe { w.mkey().bits(WDG_ACCESS_EN) });
        // SDK: ameba_wdg.c : WDG_Init, `else { WDG->WDG_CR = 0; }` branch (no early interrupt).
        unsafe { regs.wdg_cr().write_with_zero(|w| w) };
        regs.wdg_rlr().write(|w| unsafe {
            w.prer()
                .bits(WDG_PRESCALER)
                .reload()
                .bits(timeout_ms.min(0xFFFF) as u16)
        });
        regs.wdg_winr()
            .write(|w| unsafe { w.window().bits(WDG_WINDOW_DISABLED) });
        regs.wdg_mkeyr()
            .write(|w| unsafe { w.mkey().bits(WDG_ACCESS_DIS) });
    }

    /// Enable the watchdog function.
    ///
    /// # Warning
    /// This is **hardware-permanent**: the SDK exposes no magic key to
    /// disable a running watchdog (SDK: `ameba_wdg.c` : `WDG_Enable` doc
    /// comment — "Once enabled, the WDG can't Disabled by software").
    /// Only a chip reset clears it.
    pub fn enable(&self) {
        Self::wait_busy();
        regs()
            .wdg_mkeyr()
            .write(|w| unsafe { w.mkey().bits(WDG_FUNC_EN) });
    }

    /// Feed the watchdog, restarting its countdown from `WDG_RLR.RELOAD`.
    pub fn refresh(&self) {
        Self::wait_busy();
        regs()
            .wdg_mkeyr()
            .write(|w| unsafe { w.mkey().bits(WDG_REFRESH) });
    }

    /// Busy-poll `WDG_CR.RVU`/`EVU` until any in-flight hardware update
    /// completes, best-effort. Mirrors SDK `WDG_Wait_Busy`, which also
    /// gives up silently and proceeds after the iteration bound.
    fn wait_busy() {
        let mut delay = crate::delay::Delay::new();
        let mut timeout = WDG_WAIT_BUSY_TIMEOUT;
        while {
            let cr = regs().wdg_cr().read();
            cr.rvu().bit() || cr.evu().bit()
        } {
            timeout -= 1;
            if timeout == 0 {
                break;
            }
            delay.delay_us(1u32);
        }
    }
}
