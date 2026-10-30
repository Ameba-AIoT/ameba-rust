#![no_std]
#![no_main]

use core::ptr::addr_of_mut;
use core::sync::atomic::{AtomicU32, Ordering};

use ameba_rt as _;
use cortex_m as _;
use embedded_hal::delay::DelayNs;

use ameba_hal::pinmux::{self, Pin, PinFunction, Pull};
use ameba_hal::println;
use ameba_hal::rcc;
use ameba_hal::spi::{
    InterruptHandler, Role, Spi, Spi0, SpiConfig, SpiInstance, SpiIrqEvent, SpiIrqHandler,
    read_available,
};

// SPI0 on RTL8721F: PA_7(CLK), PA_8(MOSI), PA_9(MISO), PA_10(CS0)
// hw: MOSI(PA_8) <-> MISO(PA_9) short wire required for loopback
//
// Demonstrates a user-registered raw ISR callback (`Spi::register_irq` /
// `SpiIrqHandler`), independent of the async/Waker path used by `spi/async`.
// `RxCounter::on_irq` runs directly in the SPI0 IRQ context: it drains the RX
// FIFO itself (`read_available`, never blocks) and prints — contrast with the
// main loop's own `[SPI]`-style logging, which only ever calls `write_data`.

ameba_hal::bind_interrupts!(struct Irqs {
    SPI0_IRQ => InterruptHandler<Spi0>;
});

struct RxCounter {
    bytes: AtomicU32,
}

impl SpiIrqHandler for RxCounter {
    fn on_irq(&mut self, event: SpiIrqEvent) {
        if event.error {
            println!("[SPI-IRQ] error isr=0x{:08X} (from ISR)", event.isr);
            return;
        }
        let mut buf = [0u8; 16];
        let n = unsafe { read_available(Spi0::PTR, &mut buf) };
        self.bytes.fetch_add(n as u32, Ordering::Relaxed);
        println!("[SPI-IRQ] rx {} byte(s): {:02X?} (from ISR)", n, &buf[..n]);
    }
}

static mut RX_COUNTER: RxCounter = RxCounter {
    bytes: AtomicU32::new(0),
};

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[SPI interrupt] start");
    ameba_hal::init(ameba_hal::Config::default());
    let mut delay = ameba_hal::delay::Delay::new();

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

    // Drain any stale RX FIFO data before registering the callback.
    while spi.is_rx_not_empty() {
        let _ = spi.read_data();
    }
    while spi.is_busy() {}

    let handler = unsafe { &mut *addr_of_mut!(RX_COUNTER) };
    spi.register_irq(handler, Irqs);

    let mut sent: u32 = 0;
    for round in 1u8..=5 {
        let tx: [u8; 4] = [b'A' + round - 1, b'B' + round - 1, b'C' + round - 1, round];
        for &b in tx.iter() {
            spi.write_data(b as u32);
        }
        sent += tx.len() as u32;
        println!("[SPI] [{}] sent {:02X?}", round, tx);
        delay.delay_ms(50);
    }

    // Let the ISR drain the last round's echoed bytes before tearing down.
    delay.delay_ms(20);
    spi.unregister_irq();

    let received = unsafe { (*addr_of_mut!(RX_COUNTER)).bytes.load(Ordering::Relaxed) };
    if received == sent {
        println!("[PASS] SPI interrupt: {} bytes echoed via ISR OK", received);
    } else {
        println!("[FAIL] SPI interrupt: sent {} received {}", sent, received);
    }

    loop {
        delay.delay_ms(1000);
    }
}
