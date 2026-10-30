#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embedded_hal::delay::DelayNs;

use ameba_hal::gpio::{GpioPin, Input, Output, Pin, Pull};
use ameba_hal::println;

// PA13 ↔ PA14 jumper required.
// PA14: output (driven HIGH/LOW alternately).
// PA13: input pull-up (mirrors PA14 via jumper).

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[GPIO basic] start");
    ameba_hal::init(ameba_hal::Config::default());
    let mut delay = ameba_hal::delay::Delay::new();

    let pa14 = GpioPin::<Output>::builder(Pin::PA14)
        .pull(Pull::NoPull)
        .output();
    let pa13 = GpioPin::<Input>::builder(Pin::PA13).pull(Pull::Up).input();

    // Start HIGH so first toggle goes LOW → levels visible from the start.
    pa14.set_high();

    let mut mismatches: u32 = 0;

    for i in 0u32..10 {
        pa14.toggle();
        delay.delay_ms(200);

        let out = pa14.read();
        let inp = pa13.read();

        println!(
            "[GPIO] [{}] PA14={} PA13={}",
            i,
            if out { 'H' } else { 'L' },
            if inp { 'H' } else { 'L' }
        );

        if out != inp {
            mismatches += 1;
        }
    }

    if mismatches == 0 {
        println!("[PASS] GPIO basic: OK");
    } else {
        println!("[FAIL] GPIO basic: {} mismatches", mismatches);
    }

    loop {
        delay.delay_ms(1000);
    }
}
