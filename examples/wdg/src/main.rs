#![no_std]
#![no_main]

use ameba_hal::{println, wdg::Wdg};
use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

const TIMEOUT_MS: u32 = 5000;
const REFRESH_INTERVAL_MS: u64 = 500;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[WDG] start");
    ameba_hal::init(ameba_hal::Config::default());

    let wdg = unsafe { Wdg::steal() };
    wdg.init(TIMEOUT_MS);
    wdg.enable();

    println!("[WDG] system watchdog enabled, timeout={}ms", TIMEOUT_MS);

    for i in 0..10u32 {
        Timer::after(Duration::from_millis(REFRESH_INTERVAL_MS)).await;
        wdg.refresh();
        println!("[WDG] refresh #{}", i);
    }

    println!("[PASS] wdg: OK");

    loop {
        Timer::after(Duration::from_millis(REFRESH_INTERVAL_MS)).await;
        wdg.refresh();
    }
}
