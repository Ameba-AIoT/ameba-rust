#![no_std]
#![no_main]

use ameba_hal::{println, trng::Trng};
use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[TRNG] start");
    ameba_hal::init(ameba_hal::Config::default());

    let trng = unsafe { Trng::steal() };
    trng.init();

    let mut all_pass = true;
    let mut first: Option<u32> = None;
    let mut all_same = true;
    let mut success_reads: u32 = 0;

    for i in 0..10u32 {
        match trng.read_u32() {
            Some(value) => {
                println!("[TRNG] #{}: 0x{:08x}", i, value);
                success_reads += 1;
                match first {
                    None => first = Some(value),
                    Some(f) if value != f => all_same = false,
                    _ => {}
                }
            }
            None => {
                println!("[FAIL] trng: read_u32 returned None (APT failure or timeout)");
                all_pass = false;
            }
        }
        Timer::after(Duration::from_millis(50)).await;
    }

    if success_reads >= 2 && all_same {
        println!("[FAIL] trng: all reads returned the same value");
        all_pass = false;
    }

    if all_pass {
        println!("[PASS] trng: OK");
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
