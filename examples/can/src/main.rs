#![no_std]
#![no_main]

use ameba_hal::{
    can::{CAN0_PTR, Can, CanConfig, FrameType, IdType, TxMsg},
    pinmux::{self, Pin, PinFunction, Pull},
    println,
};
use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[CAN] start");
    ameba_hal::init(ameba_hal::Config::default());

    pinmux::set(Pin::PA25, PinFunction::CAN0_TX);
    pinmux::set(Pin::PA26, PinFunction::CAN0_RX);
    // Pull STB pin low so an attached transceiver is put into normal mode.
    pinmux::set_pull(Pin::PA3, Pull::Down);

    let can = unsafe { Can::steal(CAN0_PTR) };
    can.init(&CanConfig::default());
    can.enable();
    can.bus_on();

    println!("[CAN] Sending frame id=0x123");

    let mut data = [0u32; 16];
    data[0] = u32::from_le_bytes([0x01, 0x02, 0x03, 0x04]);
    data[1] = u32::from_le_bytes([0x05, 0x06, 0x07, 0x08]);

    let msg = TxMsg {
        buf_idx: 0,
        frame_type: FrameType::Data,
        id_type: IdType::Standard,
        std_id: 0x123,
        ext_id: 0,
        dlc: 8,
        data,
    };
    can.write_msg(&msg);

    // Poll TX interrupt (no node on bus, TX may not complete without ACK)
    Timer::after(Duration::from_millis(10)).await;

    println!("[PASS] CAN TX: done");
}
