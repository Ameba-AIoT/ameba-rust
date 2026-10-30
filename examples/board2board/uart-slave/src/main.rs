#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embedded_io_async::{Read, Write};

use ameba_hal::uart::{InterruptHandler, Uart, Uart0, UartConfig};

use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

// UART0 wired board-to-board (cross-connect):
//   A.PA23(TX) -> B.PA24(RX);  B.PA23(TX) -> A.PA24(RX); common ground.

ameba_hal::bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<Uart0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[UART-B2B slave] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::UART0);
    pinmux::set(Pin::PA23, PinFunction::UART0_TXD);
    pinmux::set(Pin::PA24, PinFunction::UART0_RXD);
    // RX idles high; default pinmap pull is DOWN, so force pull-up.
    pinmux::set_pull(Pin::PA24, Pull::Up);

    let uart = unsafe { Uart::<Uart0>::steal() };
    uart.init(&UartConfig::default());
    let mut uart = uart.into_async(Irqs);

    let mut round: u32 = 0;
    loop {
        // Block until a full 4-byte frame arrives, then echo it back verbatim.
        let mut buf = [0u8; 4];
        if uart.read_exact(&mut buf).await.is_err() {
            continue;
        }
        let _ = uart.write(&buf).await;
        round += 1;

        println!(
            "[UART-B2B] round {} echo: [{:02X} {:02X} {:02X} {:02X}]",
            round, buf[0], buf[1], buf[2], buf[3]
        );
    }
}
