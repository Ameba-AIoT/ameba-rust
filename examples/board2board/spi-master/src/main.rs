#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use ameba_hal::Async;
use ameba_hal::println;
use ameba_hal::spi::{InterruptHandler, Role, Spi, Spi0, SpiConfig};
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

// SPI0 pins wired board-to-board (straight through, same pin names):
//   PA7 SCLK, PA8 MOSI, PA9 MISO, PA10 CS.
const LEN: usize = 16;

ameba_hal::bind_interrupts!(struct Irqs {
    SPI0_IRQ => InterruptHandler<Spi0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[SPI-B2B master] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::SPI0);
    pinmux::set(Pin::PA7, PinFunction::SPI0);
    pinmux::set(Pin::PA8, PinFunction::SPI0);
    pinmux::set(Pin::PA9, PinFunction::SPI0);
    pinmux::set(Pin::PA10, PinFunction::SPI0);
    pinmux::set_pull(Pin::PA10, Pull::Up);

    let spi = unsafe { Spi::<Spi0>::steal() };
    spi.set_role(Role::Master);
    // ~240 kHz: reliable over jumper wires between two boards.
    let cfg = SpiConfig {
        clock_divider: 1000,
        ..SpiConfig::default()
    };
    spi.init(&cfg);
    spi.set_slave_enable(0);

    while spi.is_rx_not_empty() {
        let _ = spi.read_data();
    }
    while spi.is_busy() {}

    let mut spi_async: Spi<Spi0, Async> = spi.into_async(Irqs);

    // Give the slave board time to boot and arm transfer_async().
    Timer::after(Duration::from_millis(1000)).await;

    let mut tx = [0u8; LEN];
    for (i, b) in tx.iter_mut().enumerate() {
        *b = (i as u8) + 1; // [0x01..0x10]
    }
    let mut rx = [0u8; LEN];

    // Full-duplex: clock out `tx`, simultaneously receive the slave's preloaded data.
    spi_async.transfer_async(&mut rx, &tx).await;

    println!(
        "[SPI-B2B] master rx: {:02x} {:02x} {:02x} ... {:02x} {:02x}",
        rx[0], rx[1], rx[2], rx[14], rx[15]
    );

    // Slave preloaded [0xA0..0xAF]; verify the master received exactly that.
    let mut ok = true;
    for (i, &got) in rx.iter().enumerate() {
        let expect = 0xA0u8 + i as u8;
        if got != expect {
            println!(
                "[FAIL] SPI b2b: mismatch at [{}] expected=0x{:02x} got=0x{:02x}",
                i, expect, got
            );
            ok = false;
            break;
        }
    }
    if ok {
        println!("[PASS] SPI b2b: full-duplex OK");
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
