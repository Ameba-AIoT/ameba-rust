#![no_std]
#![no_main]

use core::ptr::addr_of_mut;
use core::sync::atomic::{AtomicU32, Ordering};

use ameba_rt as _;
use cortex_m as _;
use embedded_hal::delay::DelayNs;
use embedded_io::Write;

use ameba_hal::println;
use ameba_hal::rcc;
use ameba_hal::uart::{
    InterruptHandler, UART0_PTR, Uart, Uart0, UartConfig, UartIrqEvent, UartIrqHandler,
    read_available,
};

// No external wiring — UART0 internal loopback (`loopback_cmd(true)`).
//
// Demonstrates a user-registered raw ISR callback (`Uart::register_irq`),
// independent of the async/Waker path used by `uart/async`. `RxCounter::on_irq`
// runs directly in the UART0 IRQ context: it drains the Rx FIFO itself
// (`read_available`, never blocks) and prints — contrast with the main loop's
// own `[UART BLK]`-style logging, which only ever calls `write`.

ameba_hal::bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<Uart0>;
});

struct RxCounter {
    bytes: AtomicU32,
}

impl UartIrqHandler for RxCounter {
    fn on_irq(&mut self, event: UartIrqEvent) {
        if event.rx_error {
            println!("[UART-IRQ] rx error lsr=0x{:04X} (from ISR)", event.lsr);
            return;
        }
        let mut buf = [0u8; 16];
        let n = unsafe { read_available(UART0_PTR, &mut buf) };
        self.bytes.fetch_add(n as u32, Ordering::Relaxed);
        println!("[UART-IRQ] rx {} byte(s): {:02X?} (from ISR)", n, &buf[..n]);
    }
}

static mut RX_COUNTER: RxCounter = RxCounter {
    bytes: AtomicU32::new(0),
};

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[UART interrupt] start");
    ameba_hal::init(ameba_hal::Config::default());
    let mut delay = ameba_hal::delay::Delay::new();

    rcc::enable(rcc::Peripheral::UART0);

    let mut uart = unsafe { Uart::<Uart0>::steal() };
    uart.init(&UartConfig::default());
    uart.loopback_cmd(true);

    let handler = unsafe { &mut *addr_of_mut!(RX_COUNTER) };
    uart.register_irq(handler, Irqs);

    let mut sent: u32 = 0;
    for round in 1u8..=5 {
        let tx: [u8; 4] = [b'A' + round - 1, b'B' + round - 1, b'C' + round - 1, round];
        let _ = uart.write(&tx);
        sent += tx.len() as u32;
        println!("[UART] [{}] sent {:02X?}", round, tx);
        delay.delay_ms(50);
    }

    uart.unregister_irq();

    let received = unsafe { (*addr_of_mut!(RX_COUNTER)).bytes.load(Ordering::Relaxed) };
    if received == sent {
        println!(
            "[PASS] UART interrupt: {} bytes echoed via ISR OK",
            received
        );
    } else {
        println!("[FAIL] UART interrupt: sent {} received {}", sent, received);
    }

    loop {
        delay.delay_ms(1000);
    }
}
