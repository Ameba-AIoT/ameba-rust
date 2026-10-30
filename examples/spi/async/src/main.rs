#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use ameba_hal::println;
use ameba_hal::spi::{InterruptHandler, Role, Spi, Spi0, SpiConfig};
use ameba_hal::{Async, Config};
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

// SPI0: PA7(CLK), PA8(MOSI), PA9(MISO), PA10(CS0). MOSI↔MISO jumper for loopback.

ameba_hal::bind_interrupts!(struct Irqs {
    SPI0_IRQ => InterruptHandler<Spi0>;
});

static HB_COUNT: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn heartbeat_task() {
    loop {
        Timer::after(Duration::from_millis(5)).await;
        let n = HB_COUNT.fetch_add(1, Ordering::Release) + 1;
        println!("[HB] {}", n);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    println!("[SPI async] start");
    ameba_hal::init(Config::default());

    rcc::enable(rcc::Peripheral::SPI0);
    pinmux::set(Pin::PA7, PinFunction::SPI0);
    pinmux::set(Pin::PA8, PinFunction::SPI0);
    pinmux::set(Pin::PA9, PinFunction::SPI0);
    pinmux::set(Pin::PA10, PinFunction::SPI0);
    pinmux::set_pull(Pin::PA10, Pull::Up);

    let spi = unsafe { Spi::<Spi0>::steal() };
    spi.set_role(Role::Master);

    // Slow clock (10 kHz) so 16 bytes ≈ 12.8 ms → 2+ heartbeats at 5 ms interval.
    let cfg = SpiConfig {
        clock_divider: 24000,
        ..SpiConfig::default()
    };
    spi.init(&cfg);
    spi.set_slave_enable(0);

    while spi.is_rx_not_empty() {
        let _ = spi.read_data();
    }
    while spi.is_busy() {}

    let mut spi_async: Spi<Spi0, Async> = spi.into_async(Irqs);

    spawner.spawn(heartbeat_task()).unwrap();
    Timer::after(Duration::from_millis(1)).await;

    let tx: [u8; 16] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    let mut rx = [0u8; 16];

    let hb_before = HB_COUNT.load(Ordering::Acquire);
    spi_async.transfer_async(&mut rx, &tx).await;
    let hb_during = HB_COUNT.load(Ordering::Acquire).wrapping_sub(hb_before);

    println!("[SPI] heartbeats during transfer: {}", hb_during);

    if hb_during < 2 {
        println!("[FAIL] SPI async: executor blocked (hb={})", hb_during);
        loop {
            Timer::after(Duration::from_millis(1000)).await;
        }
    }

    let mut mismatch: Option<usize> = None;
    for (i, (&t, &r)) in tx.iter().zip(rx.iter()).enumerate() {
        if t != r {
            mismatch = Some(i);
            break;
        }
    }

    match mismatch {
        Some(i) => {
            println!(
                "[FAIL] SPI async: mismatch at [{}] expected=0x{:02X} got=0x{:02X}",
                i, tx[i], rx[i]
            );
        }
        None => println!("[PASS] SPI async: loopback + non-blocking OK"),
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
