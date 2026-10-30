#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use ameba_hal::dma::{Blocking, DmaChannel};
use ameba_hal::spi::{Role, Spi, Spi0, SpiConfig, SpiDma};

use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

const BUF_SIZE: usize = 1024;

#[repr(align(32))]
#[allow(dead_code)]
struct AlignedBuf([u8; BUF_SIZE]);

static mut TX_BUF: AlignedBuf = AlignedBuf([0u8; BUF_SIZE]);
static mut RX_BUF: AlignedBuf = AlignedBuf([0u8; BUF_SIZE]);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());
    println!("[SPI DMA] start");

    rcc::enable(rcc::Peripheral::SPI0);
    pinmux::set(Pin::PA7, PinFunction::SPI0);
    pinmux::set(Pin::PA8, PinFunction::SPI0);
    pinmux::set(Pin::PA9, PinFunction::SPI0);
    pinmux::set(Pin::PA10, PinFunction::SPI0);
    pinmux::set_pull(Pin::PA10, Pull::Up);

    let spi = unsafe { Spi::<Spi0>::steal() };
    spi.set_role(Role::Master);
    spi.init(&SpiConfig::default());
    spi.set_slave_enable(0);

    // into_dma_async takes ownership of both DMA channels, selects HS IDs, enables NVIC.
    let mut spi_dma: SpiDma<Spi0, 0, 1> = spi
        .into_dma_async(unsafe { DmaChannel::<0, Blocking>::steal() }, unsafe {
            DmaChannel::<1, Blocking>::steal()
        });

    let tx_ptr = core::ptr::addr_of_mut!(TX_BUF) as *mut u8;
    let rx_ptr = core::ptr::addr_of_mut!(RX_BUF) as *mut u8;
    unsafe {
        for i in 0..BUF_SIZE {
            tx_ptr
                .add(i)
                .write((i as u8).wrapping_mul(7).wrapping_add(0x5A));
        }
    }

    let tx_slice = unsafe { core::slice::from_raw_parts(tx_ptr as *const u8, BUF_SIZE) };
    let rx_slice = unsafe { core::slice::from_raw_parts_mut(rx_ptr, BUF_SIZE) };

    match embassy_time::with_timeout(
        Duration::from_millis(500),
        spi_dma.transfer_async(rx_slice, tx_slice),
    )
    .await
    {
        Ok(()) => {
            println!("[SPI DMA] transfer done");
        }
        Err(_) => {
            println!("[FAIL] SPI DMA: timeout");
            loop {
                Timer::after(Duration::from_millis(1000)).await;
            }
        }
    }

    let mut all_ok = true;
    for i in 0..BUF_SIZE {
        let expected = unsafe { tx_ptr.add(i).read() };
        let got = unsafe { rx_ptr.add(i).read() };
        if expected != got {
            println!(
                "[FAIL] SPI DMA: mismatch at [{}] expected=0x{:02X} got=0x{:02X}",
                i, expected, got
            );
            all_ok = false;
            break;
        }
    }

    if all_ok {
        println!("[PASS] SPI DMA: 1024 bytes loopback OK");
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
