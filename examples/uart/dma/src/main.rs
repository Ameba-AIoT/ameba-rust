#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use ameba_hal::dma::{Blocking, DmaChannel};
use ameba_hal::uart::{Uart, Uart0, UartConfig, UartDma};

use ameba_hal::println;
use ameba_hal::rcc;

const BUF_SIZE: usize = 256;

#[repr(align(32))]
struct Aligned256([u8; BUF_SIZE]);

static mut TX_BUF: Aligned256 = Aligned256([0u8; BUF_SIZE]);
static mut RX_BUF: Aligned256 = Aligned256([0u8; BUF_SIZE]);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    ameba_hal::init(ameba_hal::Config::default());
    println!("[UART DMA] start");

    rcc::enable(rcc::Peripheral::UART0);

    let uart = unsafe { Uart::<Uart0>::steal() };
    uart.init(&UartConfig::default());
    uart.loopback_cmd(true);

    // into_dma_async configures FCR DMA mode, burst sizes, and enables NVIC IRQs.
    let mut uart_dma: UartDma<Uart0, 0, 1> = uart
        .into_dma_async(unsafe { DmaChannel::<0, Blocking>::steal() }, unsafe {
            DmaChannel::<1, Blocking>::steal()
        });

    unsafe {
        let tx_ptr = core::ptr::addr_of_mut!(TX_BUF.0) as *mut u8;
        for i in 0..BUF_SIZE {
            tx_ptr
                .add(i)
                .write((i as u8).wrapping_mul(7).wrapping_add(0x5A));
        }
    }

    let tx_slice = unsafe {
        core::slice::from_raw_parts(core::ptr::addr_of!(TX_BUF.0) as *const u8, BUF_SIZE)
    };
    let rx_slice = unsafe {
        core::slice::from_raw_parts_mut(core::ptr::addr_of_mut!(RX_BUF.0) as *mut u8, BUF_SIZE)
    };

    match embassy_time::with_timeout(
        Duration::from_millis(500),
        uart_dma.dma_transfer(tx_slice, rx_slice),
    )
    .await
    {
        Ok(()) => {
            println!("[UART DMA] transfer done");
        }
        Err(_) => {
            println!("[FAIL] UART DMA: timeout");
            loop {
                Timer::after(Duration::from_secs(1)).await;
            }
        }
    }

    let mut all_ok = true;
    unsafe {
        let tx = core::slice::from_raw_parts(core::ptr::addr_of!(TX_BUF.0) as *const u8, BUF_SIZE);
        let rx = core::slice::from_raw_parts(core::ptr::addr_of!(RX_BUF.0) as *const u8, BUF_SIZE);
        for i in 0..BUF_SIZE {
            if tx[i] != rx[i] {
                println!(
                    "[FAIL] UART DMA: mismatch at [{}] expected=0x{:02X} got=0x{:02X}",
                    i, tx[i], rx[i]
                );
                all_ok = false;
                break;
            }
        }
    }

    if all_ok {
        println!("[PASS] UART DMA: 256 bytes loopback OK");
    }

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
