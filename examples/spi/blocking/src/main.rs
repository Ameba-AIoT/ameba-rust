#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use ameba_hal::spi::{Role, Spi, Spi0, SpiConfig};

use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

// SPI0 on RTL8721F: PA_7(CLK), PA_8(MOSI), PA_9(MISO), PA_10(CS0)
// hw: MOSI(PA_8) ↔ MISO(PA_9) short wire required for loopback

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[SPI BLK] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::SPI0);
    pinmux::set(Pin::PA7, PinFunction::SPI0);
    pinmux::set(Pin::PA8, PinFunction::SPI0);
    pinmux::set(Pin::PA9, PinFunction::SPI0);
    pinmux::set(Pin::PA10, PinFunction::SPI0);
    pinmux::set_pull(Pin::PA10, Pull::Up);

    let spi = unsafe { Spi::<Spi0>::steal() };

    // Set master role BEFORE init (default power-on is slave).
    spi.set_role(Role::Master);

    let cfg = SpiConfig::default();
    spi.init(&cfg);
    spi.set_slave_enable(0);

    // Drain any stale RX FIFO data.
    while spi.is_rx_not_empty() {
        let _ = spi.read_data();
    }
    // Wait for peripheral to be idle before starting transfer.
    while spi.is_busy() {}

    let tx: [u8; 16] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10,
    ];
    let mut rx = [0u8; 16];

    // Write all bytes to TX FIFO (< 64 entries, no overflow risk).
    for &b in tx.iter() {
        spi.write_data(b as u32);
    }
    // Wait until TX FIFO drains and peripheral is idle.
    while spi.is_busy() {}

    // Poll until 16 bytes appear in RX FIFO (timeout ~500ms).
    let mut received = 0usize;
    let mut timeout = 500_000u32;
    while received < 16 && timeout > 0 {
        if spi.is_rx_not_empty() {
            rx[received] = spi.read_data() as u8;
            received += 1;
        } else {
            timeout -= 1;
        }
    }

    if received < 16 {
        println!("[FAIL] SPI blocking: timeout after {} bytes", received);
        loop {
            cortex_m::asm::wfi();
        }
    }

    // Compare.
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
                "[FAIL] SPI blocking: mismatch at [{}] expected=0x{:02X} got=0x{:02X}",
                i, tx[i], rx[i]
            );
        }
        None => println!("[PASS] SPI blocking: OK"),
    }

    loop {
        cortex_m::asm::wfi();
    }
}
