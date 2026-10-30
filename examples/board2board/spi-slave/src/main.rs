#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;

use ameba_hal::spi::{InterruptHandler, Role, Spi, Spi0, SpiConfig, SpiSlaveAsync};

use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

// SPI0 pins wired board-to-board (straight through, same pin names):
//   PA7 SCLK, PA8 MOSI, PA9 MISO, PA10 CS. Master drives SCLK/CS.
const LEN: usize = 16;

ameba_hal::bind_interrupts!(struct Irqs {
    SPI0_IRQ => InterruptHandler<Spi0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[SPI-B2B slave] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::SPI0);
    pinmux::set(Pin::PA7, PinFunction::SPI0);
    pinmux::set(Pin::PA8, PinFunction::SPI0);
    pinmux::set(Pin::PA9, PinFunction::SPI0);
    pinmux::set(Pin::PA10, PinFunction::SPI0);
    pinmux::set_pull(Pin::PA10, Pull::Up);

    let spi = unsafe { Spi::<Spi0>::steal() };
    spi.set_role(Role::Slave);
    let cfg = SpiConfig {
        role: Role::Slave,
        ..SpiConfig::default()
    };
    spi.init(&cfg);

    while spi.is_rx_not_empty() {
        let _ = spi.read_data();
    }

    let mut slave: SpiSlaveAsync<Spi0> = spi.into_async_slave(Irqs);

    let mut round: u32 = 0;
    loop {
        let mut tx = [0u8; LEN];
        for (i, b) in tx.iter_mut().enumerate() {
            *b = 0xA0u8 + i as u8; // [0xA0..0xAF]
        }
        let mut rx = [0u8; LEN];

        // Preload `tx` onto MISO, wait for the master to clock LEN frames,
        // receive the master's data into `rx`.
        slave.transfer_async(&tx, &mut rx).await;
        round += 1;

        println!(
            "[SPI-B2B] slave rx (round {}): {:02x} {:02x} {:02x} ... {:02x} {:02x}",
            round, rx[0], rx[1], rx[2], rx[14], rx[15]
        );
    }
}
