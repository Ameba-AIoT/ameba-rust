#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::println;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());
    spawner.spawn(hello_task()).unwrap();
}

#[embassy_executor::task]
async fn hello_task() {
    loop {
        println!("Hello World");
        Timer::after(Duration::from_millis(1000)).await;
    }
}
