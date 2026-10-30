#![no_std]
#![no_main]

use core::ptr::addr_of_mut;
use core::sync::atomic::{AtomicU32, Ordering};

use ameba_rt as _;
use cortex_m as _;
use embedded_hal::delay::DelayNs;

use ameba_hal::gpio::{
    GpioEvent, GpioIrqHandler, GpioPin, IntPolarity, IntTrigger, Interrupt, Output, Pin, Pull,
};
use ameba_hal::println;

// PA13 ↔ PA14 jumper required.
// PA14: output (driven HIGH/LOW alternately).
// PA13: interrupt input pull-up, fires on falling edge (mirrors PA14 via jumper).

// User-registered raw ISR handler: runs directly in GPIO IRQ context, independent
// of the async/Waker path (contrast with `gpio/async`'s `wait_for_falling_edge`).
struct EdgeCounter {
    count: AtomicU32,
}

impl GpioIrqHandler for EdgeCounter {
    fn on_irq(&mut self, event: GpioEvent) {
        let n = self.count.fetch_add(1, Ordering::Relaxed) + 1;
        println!(
            "[GPIO-IRQ] edge {} (port={} pin={}, from ISR)",
            n, event.port, event.pin
        );
    }
}

static mut EDGE_COUNTER: EdgeCounter = EdgeCounter {
    count: AtomicU32::new(0),
};

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[GPIO interrupt] start");
    ameba_hal::init(ameba_hal::Config::default());
    let mut delay = ameba_hal::delay::Delay::new();

    let pa14 = GpioPin::<Output>::builder(Pin::PA14)
        .pull(Pull::NoPull)
        .output();
    let pa13 = GpioPin::<Interrupt>::builder(Pin::PA13)
        .pull(Pull::Up)
        .trigger(IntTrigger::Edge)
        .polarity(IntPolarity::ActiveLow)
        .interrupt();

    let handler = unsafe { &mut *addr_of_mut!(EDGE_COUNTER) };
    pa13.register_irq(handler);

    // Start HIGH so the first toggle is a falling edge.
    pa14.set_high();

    for i in 0u32..10 {
        pa14.toggle();
        delay.delay_ms(200);

        let count = unsafe { (*addr_of_mut!(EDGE_COUNTER)).count.load(Ordering::Relaxed) };
        println!(
            "[GPIO] [{}] PA14={} edge_count={}",
            i,
            if pa14.read() { 'H' } else { 'L' },
            count
        );
    }

    pa13.unregister_irq();

    // 10 toggles starting from HIGH → 5 falling edges expected.
    let count = unsafe { (*addr_of_mut!(EDGE_COUNTER)).count.load(Ordering::Relaxed) };
    if count == 5 {
        println!("[PASS] GPIO interrupt: 5 edges OK");
    } else {
        println!("[FAIL] GPIO interrupt: expected 5 edges, got {}", count);
    }

    loop {
        delay.delay_ms(1000);
    }
}
