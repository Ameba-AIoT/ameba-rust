#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_io_async::{Read, Write};

use ameba_hal::Async;
use ameba_hal::uart::{InterruptHandler, Uart, Uart0, UartConfig};

use ameba_hal::println;
use ameba_hal::rcc;

ameba_hal::bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<Uart0>;
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
async fn uart_async_test(mut uart: Uart<Uart0, Async>) {
    // Phase 1: 3-round echo.
    let mut all_ok = true;

    for round in 1u8..=3 {
        let tx: [u8; 4] = [b'A' + round - 1, b'B' + round - 1, b'C' + round - 1, round];

        let _ = uart.write(&tx).await;
        Timer::after(Duration::from_millis(5)).await;

        let mut rx = [0u8; 8];
        let result =
            embassy_time::with_timeout(Duration::from_millis(500), uart.read(&mut rx)).await;

        match result {
            Ok(Ok(count)) => {
                let ok = count >= 4 && rx[..4] == tx;
                println!(
                    "[UART ASYNC] round {} tx=[{:02X} {:02X} {:02X} {:02X}] rx=[{:02X} {:02X} {:02X} {:02X}] {}",
                    round,
                    tx[0],
                    tx[1],
                    tx[2],
                    tx[3],
                    rx[0],
                    rx[1],
                    rx[2],
                    rx[3],
                    if ok { "OK" } else { "FAIL" }
                );
                if !ok {
                    all_ok = false;
                }
            }
            _ => {
                println!("[UART ASYNC] round {} timeout", round);
                all_ok = false;
            }
        }
    }

    if all_ok {
        println!("[PASS] UART async: echo OK");
    } else {
        println!("[FAIL] UART async: mismatch or timeout");
    }

    // Phase 2: 256-byte concurrent write — verify executor is not blocked.
    Timer::after(Duration::from_millis(10)).await;

    let tx256 = [0x5Au8; 256];
    let start_hb = HB_COUNT.load(Ordering::Relaxed);
    println!("[UART] concurrent: 256B write start");

    let _ = uart.write(&tx256).await;

    let hb_seen = HB_COUNT.load(Ordering::Relaxed).wrapping_sub(start_hb);
    println!("[UART] heartbeats seen: {}", hb_seen);

    if hb_seen >= 2 {
        println!("[PASS] UART async: non-blocking write OK");
    } else {
        println!("[FAIL] UART async: executor blocked (hb={})", hb_seen);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    println!("[UART ASYNC] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::UART0);
    let uart = unsafe { Uart::<Uart0>::steal() };
    uart.init(&UartConfig::default());
    uart.loopback_cmd(true);

    let uart_async = uart.into_async(Irqs);
    spawner.spawn(heartbeat_task()).unwrap();
    spawner.spawn(uart_async_test(uart_async)).unwrap();
}
