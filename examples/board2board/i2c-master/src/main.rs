#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_hal::i2c::I2c as _;

use ameba_hal::i2c::{Config, I2c, I2c0};

use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};
const SLAVE_ADDR: u8 = 0x23; // application-chosen 7-bit slave address (not a chip constant)
const LEN: usize = 32;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[I2C-B2B master] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::I2C0);
    pinmux::set(Pin::PA26, PinFunction::I2C0_SDA);
    pinmux::set(Pin::PA25, PinFunction::I2C0_SCL);
    pinmux::set_pull(Pin::PA26, Pull::Up);
    pinmux::set_pull(Pin::PA25, Pull::Up);

    let mut i2c = unsafe { I2c::<I2c0, _>::steal().init(Config::default()).enable() };

    // Give the slave board time to boot and arm recv_async().
    Timer::after(Duration::from_millis(1000)).await;

    let mut tx = [0u8; LEN];
    for (i, b) in tx.iter_mut().enumerate() {
        *b = (i as u8) + 1; // [0x01..0x20]
    }

    // Master write → Slave read.
    match i2c.write(SLAVE_ADDR, &tx) {
        Ok(()) => println!("[I2C-B2B] wrote 32B to 0x23"),
        Err(_) => {
            println!("[FAIL] I2C b2b: master write NACK/err");
            loop {
                Timer::after(Duration::from_millis(1000)).await;
            }
        }
    }

    // Let the slave transition from recv_async() to send_async().
    Timer::after(Duration::from_millis(100)).await;

    // Master read → Slave write (echo).
    let mut rx = [0u8; LEN];
    match i2c.read(SLAVE_ADDR, &mut rx) {
        Ok(()) => {}
        Err(_) => {
            println!("[FAIL] I2C b2b: master read err");
            loop {
                Timer::after(Duration::from_millis(1000)).await;
            }
        }
    }
    println!(
        "[I2C-B2B] read 32B: {:02x} {:02x} {:02x} ... {:02x} {:02x}",
        rx[0], rx[1], rx[2], rx[30], rx[31]
    );

    // Verify echo matches what we sent.
    let mut ok = true;
    for i in 0..LEN {
        if rx[i] != tx[i] {
            println!(
                "[FAIL] I2C b2b: mismatch at [{}] expected=0x{:02x} got=0x{:02x}",
                i, tx[i], rx[i]
            );
            ok = false;
            break;
        }
    }
    if ok {
        println!("[PASS] I2C b2b: write+read OK");
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
