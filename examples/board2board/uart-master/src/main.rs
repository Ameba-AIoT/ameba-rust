#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_io_async::{Read, Write};

use ameba_hal::uart::{InterruptHandler, Uart, Uart0, UartConfig};

use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

// UART0 wired board-to-board (cross-connect):
//   A.PA23(TX) -> B.PA24(RX);  B.PA23(TX) -> A.PA24(RX); common ground.
const ROUNDS: u8 = 8;

ameba_hal::bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<Uart0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[UART-B2B master] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::UART0);
    pinmux::set(Pin::PA23, PinFunction::UART0_TXD);
    pinmux::set(Pin::PA24, PinFunction::UART0_RXD);
    // RX idles high; default pinmap pull is DOWN, so force pull-up.
    pinmux::set_pull(Pin::PA24, Pull::Up);

    let uart = unsafe { Uart::<Uart0>::steal() };
    uart.init(&UartConfig::default());
    let mut uart = uart.into_async(Irqs);

    // Give the slave board time to boot and arm its read loop.
    Timer::after(Duration::from_millis(1000)).await;

    let mut all_ok = true;
    for round in 1u8..=ROUNDS {
        let tx: [u8; 4] = [0x10 + round, 0x20 + round, 0x30 + round, round];

        let _ = uart.write(&tx).await;

        // Read the full 4-byte echo (a single `read` may return before all
        // bytes arrive at 115200, so accumulate with read_exact).
        let mut rx = [0u8; 4];
        let result =
            embassy_time::with_timeout(Duration::from_millis(500), uart.read_exact(&mut rx)).await;

        let got_full = matches!(result, Ok(Ok(())));
        let ok = got_full && rx == tx;
        println!(
            "[UART-B2B] round {} sent: [{:02X} {:02X} {:02X} {:02X}] recv: [{:02X} {:02X} {:02X} {:02X}] {}",
            round,
            tx[0],
            tx[1],
            tx[2],
            tx[3],
            rx[0],
            rx[1],
            rx[2],
            rx[3],
            if ok {
                "OK"
            } else if got_full {
                "MISMATCH"
            } else {
                "TIMEOUT"
            }
        );
        if !ok {
            all_ok = false;
        }

        Timer::after(Duration::from_millis(50)).await;
    }

    if all_ok {
        println!("[PASS] UART b2b: 8 rounds OK");
    } else {
        println!("[FAIL] UART b2b: mismatch or timeout");
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
