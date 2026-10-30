use crate::pac::Interrupt;
#[cfg(feature = "time-driver-tim4")]
use crate::pac::Timer4Reg;
#[cfg(feature = "time-driver-tim5")]
use crate::pac::Timer5Reg;
#[cfg(feature = "time-driver-tim6")]
use crate::pac::Timer6Reg;
#[cfg(feature = "time-driver-tim7")]
use crate::pac::Timer7Reg;
use crate::pac::timer0_reg;
use crate::tim::{TimBuilder, TimInstance, TimIt};
use core::cell::Cell;
use core::sync::atomic::{AtomicU32, Ordering};
use critical_section::Mutex;
use embassy_time_driver::{AlarmHandle, Driver, time_driver_impl};

// ── Feature flag validation ───────────────────────────────────────────────────
// Exactly one time-driver-timN feature must be enabled (default: time-driver-tim4).
//
// RTL8721F PWM timer allocation:
//   Timer4 (PWM0) — reserved when time-driver-tim4 is active (default)
//   Timer5 (PWM1) — reserved when time-driver-tim5 is active
//   Timer6 (PWM2) — reserved when time-driver-tim6 is active
//   Timer7 (PWM3) — reserved when time-driver-tim7 is active
// The selected timer must not be instantiated via TimBuilder.

#[cfg(not(any(
    feature = "time-driver-tim4",
    feature = "time-driver-tim5",
    feature = "time-driver-tim6",
    feature = "time-driver-tim7",
)))]
compile_error!("ameba-hal: exactly one time-driver-timN feature must be enabled");

#[cfg(all(feature = "time-driver-tim4", feature = "time-driver-tim5"))]
compile_error!("ameba-hal: time-driver-tim4 and time-driver-tim5 cannot both be enabled");
#[cfg(all(feature = "time-driver-tim4", feature = "time-driver-tim6"))]
compile_error!("ameba-hal: time-driver-tim4 and time-driver-tim6 cannot both be enabled");
#[cfg(all(feature = "time-driver-tim4", feature = "time-driver-tim7"))]
compile_error!("ameba-hal: time-driver-tim4 and time-driver-tim7 cannot both be enabled");
#[cfg(all(feature = "time-driver-tim5", feature = "time-driver-tim6"))]
compile_error!("ameba-hal: time-driver-tim5 and time-driver-tim6 cannot both be enabled");
#[cfg(all(feature = "time-driver-tim5", feature = "time-driver-tim7"))]
compile_error!("ameba-hal: time-driver-tim5 and time-driver-tim7 cannot both be enabled");
#[cfg(all(feature = "time-driver-tim6", feature = "time-driver-tim7"))]
compile_error!("ameba-hal: time-driver-tim6 and time-driver-tim7 cannot both be enabled");

// Compile-time type alias for the cfg-selected embassy-time timer.
#[cfg(feature = "time-driver-tim4")]
type TdTimer = Timer4Reg;
#[cfg(feature = "time-driver-tim5")]
type TdTimer = Timer5Reg;
#[cfg(feature = "time-driver-tim6")]
type TdTimer = Timer6Reg;
#[cfg(feature = "time-driver-tim7")]
type TdTimer = Timer7Reg;

// NVIC interrupt line (Interrupt enum needed by cortex_m::NVIC::unmask).
#[cfg(feature = "time-driver-tim4")]
const TD_IRQ: Interrupt = Interrupt::TIMER4_IRQ;
#[cfg(feature = "time-driver-tim5")]
const TD_IRQ: Interrupt = Interrupt::TIMER5_IRQ;
#[cfg(feature = "time-driver-tim6")]
const TD_IRQ: Interrupt = Interrupt::TIMER6_IRQ;
#[cfg(feature = "time-driver-tim7")]
const TD_IRQ: Interrupt = Interrupt::TIMER7_IRQ;

// ── Timer input clock group (SDK: ameba_pwmtimer.h) ──────────────────────────
// The general-purpose timers split into two clock groups:
//   TIM0–TIM3       → 32 kHz  (SDK: TIMER_TICK_US = 31, "32 kHz clock")
//   TIM4–TIM8 (PWM) → 40 MHz  (SDK macro: IS_TIM_40M_TIM)
// This driver is cfg-locked to TIM4/5/6/7 — all in the 40 MHz group — so PSC=39
// divides 40 MHz to exactly 1 MHz. If a future non-40 MHz timer (e.g. TIM0–3) is
// ever added as a time-driver option, this const is left UNDEFINED for that cfg
// so the build fails ("cannot find value TD_TIMER_CLK_HZ"), forcing the source
// clock to be re-verified rather than silently inheriting 40 MHz.
#[cfg(any(
    feature = "time-driver-tim4",
    feature = "time-driver-tim5",
    feature = "time-driver-tim6",
    feature = "time-driver-tim7",
))]
const TD_TIMER_CLK_HZ: u32 = 40_000_000;

// PSC = 39: 40 MHz / (39+1) = 1 MHz timer clock (1 µs/tick).
// SDK: ameba_pwm_test.c : TIM_Prescaler = 39
const PSC_1MHZ: u16 = 39;

// Pin the PSC ⇔ 40 MHz ⇔ 1 MHz coupling: the resulting tick MUST be 1 MHz to
// match embassy-time's `tick-hz-1_000_000` feature (see ameba-hal Cargo.toml and
// every example). A wrong clock group would otherwise produce silently-scaled
// delays with no compile error. Break the build if they ever drift apart.
const _: () = assert!(
    TD_TIMER_CLK_HZ / (PSC_1MHZ as u32 + 1) == 1_000_000,
    "ameba-hal: timer_clk / (PSC+1) must equal 1 MHz to match tick-hz-1_000_000",
);
// 16-bit free-running up-counter; overflow every 65536 µs.
const ARR_MAX: u16 = 0xFFFF;
// tim_sr is read-only in the PAC; W1C clears require raw write.
// Bit position from PAC: timer0_reg::tim_sr — CC0IF = Bit 1.
const TIM_SR_CC0IF: u32 = 1 << 1;

// Upper bits of the µs counter: incremented on every UIE (16-bit overflow).
static OVERFLOW_COUNT: AtomicU32 = AtomicU32::new(0);

#[inline]
fn td_regs() -> &'static timer0_reg::RegisterBlock {
    TdTimer::regs()
}

fn now_inner() -> u64 {
    // Double-read guards against an overflow ISR preempting between hi1 and hi2.
    loop {
        let hi1 = OVERFLOW_COUNT.load(Ordering::Acquire);
        let lo = td_regs().tim_cnt().read().cnt().bits() as u64;
        let hi2 = OVERFLOW_COUNT.load(Ordering::Acquire);
        if hi1 == hi2 {
            return ((hi1 as u64) << 16) | lo;
        }
    }
}

#[derive(Copy, Clone)]
struct AlarmState {
    timestamp: u64,
    callback: fn(*mut ()),
    ctx: *mut (),
    armed: bool,
}

// SAFETY: single-core target; AlarmState is never sent across real threads.
unsafe impl Send for AlarmState {}

struct TimerDriver {
    alarm: Mutex<Cell<AlarmState>>,
}

// SAFETY: single-core target; all mutable access is guarded by critical section.
unsafe impl Send for TimerDriver {}
unsafe impl Sync for TimerDriver {}

impl TimerDriver {
    const fn new() -> Self {
        Self {
            alarm: Mutex::new(Cell::new(AlarmState {
                timestamp: u64::MAX,
                callback: |_| {},
                ctx: core::ptr::null_mut(),
                armed: false,
            })),
        }
    }

    // Try to arm CCR0 hardware compare for `timestamp`.
    // Returns false if the timestamp was already past (caller must return false from
    // set_alarm so embassy fires the callback immediately).
    // Must be called within a critical section.
    fn try_arm_cc0(timestamp: u64) -> bool {
        let regs = td_regs();
        let current_overflow = OVERFLOW_COUNT.load(Ordering::Acquire);
        let target_overflow = (timestamp >> 16) as u32;

        if current_overflow != target_overflow {
            return true;
        }

        let target_lo = (timestamp & 0xFFFF) as u16;
        // CC0E (bit 24) enables the compare channel; without it CC0IF never fires.
        // SDK: ameba_pwmtimer.h : TIM_BIT_CCxE
        unsafe {
            regs.tim_ccr0()
                .write(|w| w.ccr0().bits(target_lo).cc0e().set_bit())
        };
        unsafe { regs.tim_sr().as_ptr().write_volatile(TIM_SR_CC0IF) };
        regs.tim_dier().modify(|_, w| w.cc0ie().set_bit());

        // Re-read now() after arming: if CNT already passed target_lo the compare
        // will not fire until the next wrap (~65 ms later).
        let now = now_inner();
        if now >= timestamp {
            regs.tim_dier().modify(|_, w| w.cc0ie().clear_bit());
            unsafe { regs.tim_sr().as_ptr().write_volatile(TIM_SR_CC0IF) };
            return false;
        }

        true
    }
}

impl Driver for TimerDriver {
    fn now(&self) -> u64 {
        now_inner()
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        Some(unsafe { AlarmHandle::new(0) })
    }

    fn set_alarm_callback(&self, _alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        critical_section::with(|cs| {
            let cell = self.alarm.borrow(cs);
            let mut s = cell.get();
            s.callback = callback;
            s.ctx = ctx;
            cell.set(s);
        });
    }

    fn set_alarm(&self, _alarm: AlarmHandle, timestamp: u64) -> bool {
        critical_section::with(|cs| {
            let now = now_inner();
            if now >= timestamp {
                return false;
            }

            let cell = self.alarm.borrow(cs);
            let mut s = cell.get();
            s.timestamp = timestamp;
            s.armed = true;
            cell.set(s);

            if !Self::try_arm_cc0(timestamp) {
                let mut s = cell.get();
                s.armed = false;
                cell.set(s);
                return false;
            }

            true
        })
    }
}

time_driver_impl!(static DRIVER: TimerDriver = TimerDriver::new());

/// Initialize the selected PWM timer as the embassy-time µs driver.
///
/// Called automatically by `ameba_hal::init()` when `config.sys_timer = true`.
/// SysTick must already be running before this is called (see `sys_timer::init()`).
///
/// Mirrors the SDK pattern: RCC_PeriphClockCmd → RTIM_TimeBaseInit →
/// RTIM_INTConfig(TIM_IT_Update) → RTIM_Cmd(ENABLE).
/// SDK: ameba_pwm_test.c : Timer_Pwm_Test ("init" branch)
pub(crate) fn init() {
    // TimBuilder::init() calls rcc::enable() (= RCC_PeriphClockCmd) and
    // RTIM_TimeBaseInit (PSC/ARR/UG/ug_done/SR-clear) internally.
    let tim = TimBuilder::new(unsafe { TdTimer::steal() })
        .prescaler(PSC_1MHZ)
        .period(ARR_MAX as u32)
        .init();
    // = RTIM_INTConfig(TIM_IT_Update, ENABLE)
    tim.enable_interrupt(TimIt::UIE);
    // SAFETY: ISR vector for TD_IRQ is defined below as the sole handler.
    unsafe { cortex_m::peripheral::NVIC::unmask(TD_IRQ) };
    // = RTIM_Cmd(ENABLE)
    tim.start();
}

fn on_interrupt() {
    let regs = td_regs();
    let sr = regs.tim_sr().read();
    unsafe { regs.tim_sr().as_ptr().write_volatile(sr.bits()) };

    if sr.uif().bit_is_set() {
        let new_overflow = OVERFLOW_COUNT.fetch_add(1, Ordering::Release) + 1;

        critical_section::with(|cs| {
            let cell = DRIVER.alarm.borrow(cs);
            let state = cell.get();
            if !state.armed {
                return;
            }
            let target_overflow = (state.timestamp >> 16) as u32;
            if target_overflow == new_overflow {
                let target_lo = (state.timestamp & 0xFFFF) as u16;
                // CC0E (bit 24) enables the compare channel.
                // SDK: ameba_pwmtimer.h : TIM_BIT_CCxE
                unsafe {
                    regs.tim_ccr0()
                        .write(|w| w.ccr0().bits(target_lo).cc0e().set_bit())
                };
                unsafe { regs.tim_sr().as_ptr().write_volatile(TIM_SR_CC0IF) };
                regs.tim_dier().modify(|_, w| w.cc0ie().set_bit());
            } else if target_overflow < new_overflow {
                cell.set(AlarmState {
                    armed: false,
                    ..state
                });
                (state.callback)(state.ctx);
            }
        });
    }

    if sr.cc0if().bit_is_set() {
        regs.tim_dier().modify(|_, w| w.cc0ie().clear_bit());

        let now = now_inner();
        critical_section::with(|cs| {
            let cell = DRIVER.alarm.borrow(cs);
            let state = cell.get();
            if state.armed && now >= state.timestamp {
                cell.set(AlarmState {
                    armed: false,
                    ..state
                });
                (state.callback)(state.ctx);
            }
        });
    }
}

// ISR vector name must match device.x. One function per possible timer selection.
#[cfg(feature = "time-driver-tim4")]
#[unsafe(no_mangle)]
unsafe extern "C" fn TIMER4_IRQ() {
    on_interrupt();
}
#[cfg(feature = "time-driver-tim5")]
#[unsafe(no_mangle)]
unsafe extern "C" fn TIMER5_IRQ() {
    on_interrupt();
}
#[cfg(feature = "time-driver-tim6")]
#[unsafe(no_mangle)]
unsafe extern "C" fn TIMER6_IRQ() {
    on_interrupt();
}
#[cfg(feature = "time-driver-tim7")]
#[unsafe(no_mangle)]
unsafe extern "C" fn TIMER7_IRQ() {
    on_interrupt();
}
