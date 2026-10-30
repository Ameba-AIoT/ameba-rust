#![no_std]
#![no_main]

use ameba_hal::println;
use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use log::LevelFilter;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[LOGGER] start");
    ameba_hal::init(ameba_hal::Config::default());

    ameba_hal::logger::init(LevelFilter::Info);

    log::error!("error level");
    log::warn!("warn level");
    log::info!("info level");
    log::debug!("debug level -- must NOT appear (filtered out by LevelFilter::Info)");
    log::trace!("trace level -- must NOT appear (filtered out by LevelFilter::Info)");

    println!("[PASS] logger: OK");

    loop {
        cortex_m::asm::wfi();
    }
}
