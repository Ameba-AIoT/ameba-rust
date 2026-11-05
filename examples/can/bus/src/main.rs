#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embedded_hal::delay::DelayNs;

use ameba_hal::can::{
    BitTiming, CAN0_PTR, CAN1_PTR, Can, CanConfig, FrameType, IdType, RxMsg, TxMsg, WorkMode,
};
use ameba_hal::delay::Delay;
use ameba_hal::pinmux::{self, Pin, PinFunction, Pull};
use ameba_hal::println;
use ameba_hal::rcc::{self, Peripheral};

// CAN0 and CAN1 on one real bus: each controller drives its own on-board
// transceiver and the two CAN_H/CAN_L pairs are wired together.
// SDK: example_can_ext.h : CAN0_TX .. CAN1_STB (raw_can_trx board pins).
const CAN0_TX: Pin = Pin::PA25;
const CAN0_RX: Pin = Pin::PA26;
const CAN0_STB: Pin = Pin::PA3;
const CAN1_TX: Pin = Pin::PB17;
const CAN1_RX: Pin = Pin::PB18;
const CAN1_STB: Pin = Pin::PB19;

const TX_BUF: u8 = 0;
// A single RX buffer with all masks "don't care" accepts every frame
// (std/ext, data/remote).
const RX_BUF: u8 = 15;

const BIT_RATE: u32 = 1_000_000;
const DONE_TIMEOUT_US: u32 = 10_000;

// SDK: example_raw_can_trx.c : can_ram_buffer_map (16 base addresses + end).
const RAM_BUFFER_MAP: [u32; 17] = [
    0x0, 0x6, 0xc, 0x12, 0x18, 0x1e, 0x24, 0x2a, 0x30, 0x36, 0x3c, 0x42, 0x48, 0x4e, 0x54, 0x5a,
    0x60,
];

#[derive(Clone, Copy)]
struct Frame {
    id_type: IdType,
    frame_type: FrameType,
    id: u32,
    dlc: u8,
    data: [u8; 8],
}

// Four frame shapes per direction; ID and payload are salted with the
// direction so a stale buffer never matches.
fn frame(shape: usize, salt: u8) -> Frame {
    let mut data = [0u8; 8];
    for (i, b) in data.iter_mut().enumerate() {
        *b = salt.wrapping_mul(0x11) ^ (i as u8).wrapping_mul(0x25) ^ shape as u8;
    }
    match shape {
        0 => Frame {
            id_type: IdType::Standard,
            frame_type: FrameType::Data,
            id: 0x100 | salt as u32,
            dlc: 8,
            data,
        },
        1 => Frame {
            id_type: IdType::Extended,
            frame_type: FrameType::Data,
            id: 0x1ABC_0000 | (salt as u32) << 8 | 0x01,
            dlc: 5,
            data,
        },
        2 => Frame {
            id_type: IdType::Standard,
            frame_type: FrameType::Data,
            id: 0x700 | salt as u32,
            dlc: 0,
            data,
        },
        _ => Frame {
            id_type: IdType::Standard,
            frame_type: FrameType::Remote,
            id: 0x400 | salt as u32,
            dlc: 4,
            data,
        },
    }
}

fn pack(bytes: &[u8; 8]) -> [u32; 16] {
    let mut words = [0u32; 16];
    words[0] = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    words[1] = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    words
}

fn unpack(words: &[u32; 16]) -> [u8; 8] {
    let lo = words[0].to_le_bytes();
    let hi = words[1].to_le_bytes();
    [lo[0], lo[1], lo[2], lo[3], hi[0], hi[1], hi[2], hi[3]]
}

fn empty_rx(buf_idx: u8) -> RxMsg {
    RxMsg {
        buf_idx,
        frame_type: FrameType::Data,
        id_type: IdType::Standard,
        std_id: 0,
        ext_id: 0,
        dlc: 0,
        rx_lost: false,
        timestamp: 0,
        id_mask: 0,
        ide_mask: false,
        rtr_mask: false,
        data: [0; 16],
    }
}

fn wait_done(delay: &mut Delay, done: impl Fn() -> bool) -> bool {
    for _ in 0..DONE_TIMEOUT_US / 10 {
        if done() {
            return true;
        }
        delay.delay_us(10);
    }
    done()
}

fn to_tx(f: &Frame) -> TxMsg {
    let (std_id, ext_id) = match f.id_type {
        IdType::Standard => (f.id as u16, 0),
        IdType::Extended => (0, f.id),
    };
    TxMsg {
        buf_idx: TX_BUF,
        frame_type: f.frame_type,
        id_type: f.id_type,
        std_id,
        ext_id,
        dlc: f.dlc,
        data: pack(&f.data),
    }
}

/// Bring one controller onto the bus at `timing` with a catch-all RX buffer.
fn start(can: &Can, delay: &mut Delay, timing: BitTiming) -> bool {
    can.bus_off();
    can.disable();
    can.init(&CanConfig {
        rx_fifo_en: false,
        work_mode: WorkMode::Normal,
        timing,
        ..CanConfig::default()
    });
    can.ram_buffer_map(&RAM_BUFFER_MAP);
    can.tx_msgbuf_int_config(0xFFFF, true);
    can.rx_msgbuf_int_config(0xFFFF, true);
    can.clear_tx_err_count();
    can.clear_rx_err_count();
    can.clear_err_status(0xFFFF_FFFF);
    can.clear_tx_msgbuf_err(0xFFFF);
    can.clear_tx_done(0xFFFF);
    can.clear_rx_done(0xFFFF);
    can.enable();
    can.bus_on();
    if !wait_done(delay, || can.bus_status()) {
        return false;
    }
    can.set_rx_msg_buf(&empty_rx(RX_BUF));
    true
}

fn send(tx: &Can, delay: &mut Delay, f: &Frame) -> bool {
    tx.write_msg(&to_tx(f));
    let ok = wait_done(delay, || tx.msgbuf_tx_done(TX_BUF as usize));
    tx.clear_msgbuf_tx_done(TX_BUF as usize);
    ok
}

/// Send `f` from `tx`, receive it on `rx`, and compare every field.
fn transfer(tx: &Can, rx: &Can, delay: &mut Delay, dir: &str, f: &Frame) -> bool {
    if !send(tx, delay, f) {
        println!(
            "[CAN]   {} id=0x{:X}: TX timeout (TEC={} err=0x{:08X})",
            dir,
            f.id,
            tx.tx_err_count() as u32,
            tx.err_status(),
        );
        return false;
    }
    if !wait_done(delay, || rx.msgbuf_rx_done(RX_BUF as usize)) {
        println!(
            "[CAN]   {} id=0x{:X}: RX timeout (REC={} err=0x{:08X})",
            dir,
            f.id,
            rx.rx_err_count() as u32,
            rx.err_status(),
        );
        return false;
    }
    rx.clear_msgbuf_rx_done(RX_BUF as usize);

    let mut m = empty_rx(RX_BUF);
    rx.read_msg(&mut m);
    let rx_id = match m.id_type {
        IdType::Standard => m.std_id as u32,
        IdType::Extended => m.ext_id,
    };
    let got = unpack(&m.data);
    let len = f.dlc as usize;
    let data_ok = f.frame_type == FrameType::Remote || got[..len] == f.data[..len];
    let ok = m.id_type == f.id_type
        && m.frame_type == f.frame_type
        && rx_id == f.id
        && m.dlc == f.dlc
        && data_ok;
    if !ok {
        println!(
            "[CAN]   {} MISMATCH: sent id=0x{:X} dlc={} data={:02X?} / got id=0x{:X} dlc={} data={:02X?}",
            dir,
            f.id,
            f.dlc as u32,
            &f.data[..len],
            rx_id,
            m.dlc as u32,
            &got[..len],
        );
    }
    ok
}

fn halt() -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[CAN bus] start");
    ameba_hal::init(ameba_hal::Config::default());
    let mut delay = Delay::new();

    // Clock source before ungating, then pinmux; STB low puts both
    // transceivers into normal mode.
    Can::core_clock_set();
    rcc::enable(Peripheral::CAN0);
    rcc::enable(Peripheral::CAN1);
    pinmux::set(CAN0_TX, PinFunction::CAN0_TX);
    pinmux::set(CAN0_RX, PinFunction::CAN0_RX);
    pinmux::set(CAN1_TX, PinFunction::CAN1_TX);
    pinmux::set(CAN1_RX, PinFunction::CAN1_RX);
    pinmux::set_pull(CAN0_STB, Pull::Down);
    pinmux::set_pull(CAN1_STB, Pull::Down);

    let can0 = unsafe { Can::steal(CAN0_PTR) };
    let can1 = unsafe { Can::steal(CAN1_PTR) };

    let mut timing = BitTiming::default();
    if Can::calc_bit_timing(BIT_RATE, &mut timing).is_err() {
        println!("[FAIL] CAN bus: no bit timing for {} bps", BIT_RATE);
        halt();
    }
    if !start(&can0, &mut delay, timing) || !start(&can1, &mut delay, timing) {
        println!("[FAIL] CAN bus: bus-on timeout");
        halt();
    }
    println!("[CAN] {} bps: bus on", BIT_RATE);

    let mut ok01 = 0u32;
    let mut ok10 = 0u32;
    for shape in 0..4 {
        let salt = shape as u8;
        if transfer(&can0, &can1, &mut delay, "CAN0->CAN1", &frame(shape, salt)) {
            ok01 += 1;
        }
        if transfer(
            &can1,
            &can0,
            &mut delay,
            "CAN1->CAN0",
            &frame(shape, salt | 0x08),
        ) {
            ok10 += 1;
        }
    }
    let errs = [
        can0.tx_err_count(),
        can0.rx_err_count(),
        can1.tx_err_count(),
        can1.rx_err_count(),
    ];
    println!(
        "[CAN] CAN0->CAN1 {}/4, CAN1->CAN0 {}/4, TEC0={} REC0={} TEC1={} REC1={}",
        ok01, ok10, errs[0] as u32, errs[1] as u32, errs[2] as u32, errs[3] as u32,
    );

    if ok01 == 4 && ok10 == 4 && errs == [0; 4] {
        println!("[PASS] CAN bus: 8 frames OK, TEC=REC=0");
    } else {
        println!("[FAIL] CAN bus: {}/4 + {}/4 frames OK", ok01, ok10);
    }

    halt()
}
