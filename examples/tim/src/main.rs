#![no_std]
#![no_main]

use core::ptr::addr_of_mut;
use core::sync::atomic::{AtomicU32, Ordering};

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use ameba_hal::bind_interrupts;
use ameba_hal::pac::Timer0Reg;
use ameba_hal::println;
use ameba_hal::tim::{self, TimFlag, TimIrqEvent, TimIrqHandler, TimIt};

// ── IRQ bookkeeping ────────────────────────────────────────────────────────

static IRQ_COUNT: AtomicU32 = AtomicU32::new(0);

bind_interrupts!(struct Irqs {
    TIMER0_IRQ => tim::InterruptHandler<Timer0Reg>;
});

struct Timer0Handler;

impl TimIrqHandler for Timer0Handler {
    fn on_irq(&mut self, event: TimIrqEvent) {
        if event.flags.contains(TimFlag::UIF) {
            IRQ_COUNT.fetch_add(1, Ordering::Release);
        }
    }
}

static mut TIMER0_HANDLER: Timer0Handler = Timer0Handler;

// ── Test parameters ────────────────────────────────────────────────────────

// Basic timer (Timer0-3) clock: SDM32K = 32768 Hz.
// Period register = ticks - 1.  1 s → ARR = 32768 - 1 = 32767.
// SDK: example_raw_gtimer.c  GTIMER_PERIOD = 1000 * 32768 / 1000 - 1
const PERIOD_1S: u32 = 32767;
const TEST_SECONDS: u32 = 5;

// ── Entry point ────────────────────────────────────────────────────────────

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[TIM] hardware timer test start");
    ameba_hal::init(ameba_hal::Config::default());

    // SAFETY: single-use, no other code touches Timer0 in this example.
    let pac = unsafe { Timer0Reg::steal() };
    let tim = tim::TimBuilder::new(pac).period(PERIOD_1S).init();
    tim.register_irq(unsafe { &mut *addr_of_mut!(TIMER0_HANDLER) }, Irqs);

    tim.enable_interrupt(TimIt::UIE);
    tim.start();
    println!("[TIM] timer0 started (32kHz clock, 1s period)");

    for expected in 1u32..=TEST_SECONDS {
        Timer::after(Duration::from_millis(1100)).await;
        let count = IRQ_COUNT.load(Ordering::Acquire);
        println!("[TIM] irq_count={} expected={}", count, expected);
    }

    tim.stop();

    let count = IRQ_COUNT.load(Ordering::Acquire);
    if (TEST_SECONDS - 1..=TEST_SECONDS + 1).contains(&count) {
        println!("[PASS] timer0: irq_count={}", count);
    } else {
        println!(
            "[FAIL] timer0: irq_count={} (expected ~{})",
            count, TEST_SECONDS
        );
    }
}
