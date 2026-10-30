#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;

use ameba_hal::i2c::{self, Config, I2c, I2c0, Role};

use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};
const SLAVE_ADDR: u8 = 0x23; // application-chosen 7-bit slave address (not a chip constant)
const LEN: usize = 32;

ameba_hal::bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2c0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[I2C-B2B slave] start addr=0x23");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::I2C0);
    pinmux::set(Pin::PA26, PinFunction::I2C0_SDA);
    pinmux::set(Pin::PA25, PinFunction::I2C0_SCL);
    pinmux::set_pull(Pin::PA26, Pull::Up);
    pinmux::set_pull(Pin::PA25, Pull::Up);

    let cfg = Config {
        role: Role::Slave,
        ack_addr: SLAVE_ADDR as u16,
        ..Default::default()
    };

    let i2c = unsafe { I2c::<I2c0, _>::steal().init(cfg).enable() };
    let slave = i2c.into_async_slave(SLAVE_ADDR, Irqs);

    loop {
        // Master write → Slave read.
        let mut buf = [0u8; LEN];
        let n = slave.recv_async(&mut buf).await;
        {
            println!(
                "[I2C-B2B] slave recv {}B: {:02x} {:02x} {:02x} ... {:02x} {:02x}",
                n, buf[0], buf[1], buf[2], buf[30], buf[31],
            );
        }

        // Master read → Slave write (echo back what was received).
        let sent = slave.send_async(&buf).await;
        {
            println!("[I2C-B2B] slave sent {}B echo", sent,);
        }
    }
}
