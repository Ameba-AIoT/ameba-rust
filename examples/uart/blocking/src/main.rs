#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embedded_io::{Read, Write};

use ameba_hal::uart::{Uart, Uart0, UartConfig};

use ameba_hal::println;
use ameba_hal::rcc;

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[UART BLK] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::UART0);

    let mut uart = unsafe { Uart::<Uart0>::steal() };
    uart.init(&UartConfig::default());
    uart.loopback_cmd(true);

    let mut all_ok = true;
    for round in 1u8..=3 {
        let tx: [u8; 4] = [b'A' + round - 1, b'B' + round - 1, b'C' + round - 1, round];

        let _ = uart.write(&tx);

        // Drain RX loopback bytes - blocks on first byte, then reads available
        let mut rx = [0u8; 4];
        let mut got = 0usize;
        while got < 4 {
            match uart.read(&mut rx[got..]) {
                Ok(n) => got += n,
                Err(_) => break,
            }
        }

        println!(
            "[UART BLK] round {} sent:[{:02X}{:02X}{:02X}{:02X}] recv:[{:02X}{:02X}{:02X}{:02X}]",
            round, tx[0], tx[1], tx[2], tx[3], rx[0], rx[1], rx[2], rx[3]
        );

        if got < 4 || rx != tx {
            println!("[FAIL] UART blocking: round {} mismatch", round);
            all_ok = false;
        }
    }

    if all_ok {
        println!("[PASS] UART blocking: OK");
    }

    loop {
        cortex_m::asm::wfi();
    }
}
