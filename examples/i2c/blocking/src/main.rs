#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embedded_hal::i2c::I2c as _;

use ameba_hal::i2c::{Config, I2c, I2c0};

use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[I2C BLK] start");
    ameba_hal::init(ameba_hal::Config::default());

    rcc::enable(rcc::Peripheral::I2C0);
    pinmux::set(Pin::PA26, PinFunction::I2C0_SDA);
    pinmux::set(Pin::PA25, PinFunction::I2C0_SCL);
    pinmux::set_pull(Pin::PA26, Pull::Up);
    pinmux::set_pull(Pin::PA25, Pull::Up);

    let mut i2c = unsafe { I2c::<I2c0, _>::steal().init(Config::default()).enable() };

    // Write to non-existent device → expect NACK (Err)
    match i2c.write(0x55u8, &[0x42u8]) {
        Err(_) => {
            println!("[PASS] I2C blocking: NACK OK");
        }
        Ok(()) => {
            println!("[FAIL] I2C blocking: unexpected ACK");
        }
    }

    loop {
        cortex_m::asm::wfi();
    }
}
