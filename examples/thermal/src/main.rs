#![no_std]
#![no_main]

use ameba_hal::{
    println,
    thermal::{TM_INVALID_VALUE, Thermal, ThermalInitParams, raw_to_celsius},
};
use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[THERMAL] start");
    ameba_hal::init(ameba_hal::Config::default());

    let thermal = unsafe { Thermal::steal() };
    thermal.init(&ThermalInitParams::default());

    let mut all_pass = true;

    for _ in 0..5u32 {
        Timer::after(Duration::from_secs(1)).await;

        let raw = thermal.get_temp_result();
        if raw == TM_INVALID_VALUE {
            println!("[THERMAL] sensor not ready");
            all_pass = false;
            continue;
        }

        let temp_c = raw_to_celsius(raw);
        let temp_int = temp_c as i32;
        let frac = ((temp_c - temp_int as f32).abs() * 10.0) as u32;

        println!("[THERMAL] Temp: {}.{} C", temp_int, frac,);

        if !(-10..=110).contains(&temp_int) {
            println!("[FAIL] thermal: out of range: {} C", temp_int,);
            all_pass = false;
        }
    }

    if all_pass {
        println!("[PASS] thermal: OK");
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
