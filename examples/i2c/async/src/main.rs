#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, with_timeout};
use embedded_hal_async::i2c::I2c as _;

use ameba_hal::Async;
use ameba_hal::i2c::{self, Config, I2c, I2c0};
use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

ameba_hal::bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2c0>;
});

static HB_COUNT: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn heartbeat_task() {
    let mut n = 0u32;
    loop {
        HB_COUNT.fetch_add(1, Ordering::Relaxed);
        println!("[HB] {}", n);
        n += 1;
        Timer::after(Duration::from_millis(5)).await;
    }
}

#[embassy_executor::task]
async fn i2c_task(mut i2c: I2c<I2c0, i2c::Enabled, Async>) {
    // Phase 1: NACK error handling — write to address 0x55 (no device present).
    let result = with_timeout(Duration::from_millis(500), i2c.write(0x55u8, &[0x42u8])).await;
    match result {
        Ok(Err(_)) => println!("[PASS] I2C async: NACK OK"),
        Ok(Ok(())) => println!("[FAIL] I2C async: unexpected ACK"),
        Err(_) => println!("[FAIL] I2C async: timeout (>500ms)"),
    }

    // Phase 2: 30 writes + heartbeat — verify executor is not blocked.
    let start_hb = HB_COUNT.load(Ordering::Relaxed);
    println!("[I2C] concurrent: 30 writes start");

    for _ in 0..30u32 {
        let _ = i2c.write(0x55, &[0xA5]).await;
        Timer::after(Duration::from_millis(1)).await;
    }

    let hb_seen = HB_COUNT.load(Ordering::Relaxed).wrapping_sub(start_hb);
    println!("[I2C] heartbeats seen: {}", hb_seen);

    if hb_seen >= 2 {
        println!("[PASS] I2C async: non-blocking OK");
    } else {
        println!("[FAIL] I2C async: executor blocked (hb={})", hb_seen);
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    println!("[I2C ASYNC] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::I2C0);
    pinmux::set(Pin::PA26, PinFunction::I2C0_SDA);
    pinmux::set(Pin::PA25, PinFunction::I2C0_SCL);
    pinmux::set_pull(Pin::PA26, Pull::Up);
    pinmux::set_pull(Pin::PA25, Pull::Up);

    let i2c = unsafe { I2c::<I2c0, _>::steal().init(Config::default()).enable() };
    let i2c_async = i2c.into_async(Irqs);

    spawner.spawn(heartbeat_task()).unwrap();
    spawner.spawn(i2c_task(i2c_async)).unwrap();
}
