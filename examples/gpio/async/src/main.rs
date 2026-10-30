#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use ameba_hal::gpio::{GpioPin, Input, Output, Pin, Pull};
use ameba_hal::println;

// PA13 ↔ PA14 jumper required.

static TOGGLE_COUNT: AtomicU32 = AtomicU32::new(0);
static EDGE_COUNT: AtomicU32 = AtomicU32::new(0);

// Phase 1 helper: one-shot trigger — drives PA14 low after 100 ms.
#[embassy_executor::task]
async fn trigger_once(pa14: GpioPin<Output>) {
    Timer::after(Duration::from_millis(100)).await;
    pa14.set_low();
}

#[embassy_executor::task]
async fn toggle_task(pa14: GpioPin<Output>) {
    for _ in 0..20u32 {
        Timer::after(Duration::from_millis(500)).await;
        pa14.toggle();
        let n = TOGGLE_COUNT.fetch_add(1, Ordering::Release) + 1;
        println!("[GPIO] PA14 toggle {}", n);
    }
}

#[embassy_executor::task]
async fn edge_task(mut pa13: GpioPin<Input>) {
    loop {
        pa13.wait_for_any_edge().await;
        let n = EDGE_COUNT.fetch_add(1, Ordering::AcqRel) + 1;
        println!("[GPIO] edge {}", n);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    println!("[GPIO async] start");
    ameba_hal::init(ameba_hal::Config::default());

    // Phase 1: wait_for_falling_edge with 200 ms timeout.
    {
        let pa14 = GpioPin::<Output>::builder(Pin::PA14)
            .pull(Pull::NoPull)
            .output();
        pa14.set_high();
        let mut pa13 = GpioPin::<Input>::builder(Pin::PA13).pull(Pull::Up).input();
        spawner.spawn(trigger_once(pa14)).unwrap();
        match embassy_time::with_timeout(Duration::from_millis(200), pa13.wait_for_falling_edge())
            .await
        {
            Ok(()) => println!("[PASS] GPIO: falling edge detected"),
            Err(_) => println!("[FAIL] GPIO: timeout, no falling edge"),
        };
    }

    // Phase 2: wait_for_any_edge, 20 toggles × 500 ms = 10 s.
    Timer::after(Duration::from_millis(100)).await;

    let pa14 = GpioPin::<Output>::builder(Pin::PA14)
        .pull(Pull::NoPull)
        .output();
    let pa13 = GpioPin::<Input>::builder(Pin::PA13).pull(Pull::Up).input();
    pa14.set_high();

    spawner.spawn(toggle_task(pa14)).unwrap();
    spawner.spawn(edge_task(pa13)).unwrap();

    Timer::after(Duration::from_millis(10500)).await;

    let t = TOGGLE_COUNT.load(Ordering::Acquire);
    let e = EDGE_COUNT.load(Ordering::Acquire);

    if t >= 18 && e >= 18 {
        println!("[PASS] GPIO async: {} toggles {} edges OK", t, e);
    } else {
        println!(
            "[FAIL] GPIO async: {} toggles {} edges (need >=18 each)",
            t, e
        );
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
