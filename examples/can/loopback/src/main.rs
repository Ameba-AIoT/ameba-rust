#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embedded_hal::delay::DelayNs;

use ameba_hal::can::{
    BitTiming, CAN0_PTR, Can, CanConfig, FrameType, IdType, RxMsg, TxMsg, WorkMode,
};
use ameba_hal::println;
use ameba_hal::rcc::{self, Peripheral};

// CAN0 internal loopback: TX is tied back to RX inside the controller and the
// TX pin is held recessive, so no transceiver or wiring is needed.

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

struct Case {
    id_type: IdType,
    frame_type: FrameType,
    id: u32,
    dlc: u8,
    data: [u8; 8],
}

const CASES: [Case; 4] = [
    Case {
        id_type: IdType::Standard,
        frame_type: FrameType::Data,
        id: 0x123,
        dlc: 8,
        data: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    },
    Case {
        id_type: IdType::Extended,
        frame_type: FrameType::Data,
        id: 0x1ABC_DE01,
        dlc: 5,
        data: [0xA5, 0x5A, 0xFF, 0x00, 0x3C, 0, 0, 0],
    },
    Case {
        id_type: IdType::Standard,
        frame_type: FrameType::Data,
        id: 0x7FF,
        dlc: 0,
        data: [0; 8],
    },
    Case {
        id_type: IdType::Standard,
        frame_type: FrameType::Remote,
        id: 0x456,
        dlc: 4,
        data: [0; 8],
    },
];

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

fn wait_done(delay: &mut ameba_hal::delay::Delay, done: impl Fn() -> bool) -> bool {
    for _ in 0..DONE_TIMEOUT_US / 10 {
        if done() {
            return true;
        }
        delay.delay_us(10);
    }
    done()
}

fn run_case(can: &Can, delay: &mut ameba_hal::delay::Delay, n: usize, c: &Case) -> bool {
    let (std_id, ext_id) = match c.id_type {
        IdType::Standard => (c.id as u16, 0),
        IdType::Extended => (0, c.id),
    };
    let tx = TxMsg {
        buf_idx: TX_BUF,
        frame_type: c.frame_type,
        id_type: c.id_type,
        std_id,
        ext_id,
        dlc: c.dlc,
        data: pack(&c.data),
    };
    can.write_msg(&tx);

    if !wait_done(delay, || can.msgbuf_tx_done(TX_BUF as usize)) {
        println!(
            "[FAIL] CAN loopback: case {} TX timeout (TEC={} err=0x{:08X})",
            n as u32,
            can.tx_err_count() as u32,
            can.err_status(),
        );
        return false;
    }
    can.clear_msgbuf_tx_done(TX_BUF as usize);

    if !wait_done(delay, || can.msgbuf_rx_done(RX_BUF as usize)) {
        println!(
            "[FAIL] CAN loopback: case {} RX timeout (REC={} err=0x{:08X})",
            n as u32,
            can.rx_err_count() as u32,
            can.err_status(),
        );
        return false;
    }
    can.clear_msgbuf_rx_done(RX_BUF as usize);

    let mut rx = empty_rx(RX_BUF);
    can.read_msg(&mut rx);

    let rx_id = match rx.id_type {
        IdType::Standard => rx.std_id as u32,
        IdType::Extended => rx.ext_id,
    };
    let rx_bytes = unpack(&rx.data);
    let len = c.dlc as usize;
    let data_ok = c.frame_type == FrameType::Remote || rx_bytes[..len] == c.data[..len];
    let ok = rx.id_type == c.id_type
        && rx.frame_type == c.frame_type
        && rx_id == c.id
        && rx.dlc == c.dlc
        && data_ok;

    println!(
        "[CAN] case {}: {} {} id=0x{:X} dlc={} -> rx {} {} id=0x{:X} dlc={} data={:02X?} {}",
        n as u32,
        if c.id_type == IdType::Extended {
            "ext"
        } else {
            "std"
        },
        if c.frame_type == FrameType::Remote {
            "rtr"
        } else {
            "data"
        },
        c.id,
        c.dlc as u32,
        if rx.id_type == IdType::Extended {
            "ext"
        } else {
            "std"
        },
        if rx.frame_type == FrameType::Remote {
            "rtr"
        } else {
            "data"
        },
        rx_id,
        rx.dlc as u32,
        &rx_bytes[..len.min(8)],
        if ok { "OK" } else { "MISMATCH" },
    );
    ok
}

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[CAN loopback] start");
    ameba_hal::init(ameba_hal::Config::default());
    let mut delay = ameba_hal::delay::Delay::new();

    // Select the CAN clock source before ungating the controller.
    Can::core_clock_set();
    rcc::enable(Peripheral::CAN0);

    let mut timing = BitTiming::default();
    if Can::calc_bit_timing(BIT_RATE, &mut timing).is_err() {
        println!("[FAIL] CAN loopback: no bit timing for {} bps", BIT_RATE);
        loop {
            cortex_m::asm::wfi();
        }
    }
    println!(
        "[CAN] {} bps: brp={} prop={} ph1={} ph2={} sjw={}",
        BIT_RATE,
        timing.prescaler as u32,
        timing.prop_seg as u32,
        timing.phase_seg1 as u32,
        timing.phase_seg2 as u32,
        timing.sjw as u32,
    );

    let can = unsafe { Can::steal(CAN0_PTR) };
    can.bus_off();
    can.init(&CanConfig {
        rx_fifo_en: false,
        work_mode: WorkMode::InternalLoopback,
        timing,
        ..CanConfig::default()
    });
    can.ram_buffer_map(&RAM_BUFFER_MAP);
    can.tx_msgbuf_int_config(0xFFFF, true);
    can.rx_msgbuf_int_config(0xFFFF, true);
    can.enable();
    can.bus_on();

    if !wait_done(&mut delay, || can.bus_status()) {
        println!("[FAIL] CAN loopback: bus-on timeout");
        loop {
            cortex_m::asm::wfi();
        }
    }
    println!("[CAN] bus on");

    let mut rx_cfg = empty_rx(RX_BUF);
    rx_cfg.id_mask = 0; // all ID bits don't care
    can.set_rx_msg_buf(&rx_cfg);

    let mut passed = 0u32;
    for (n, c) in CASES.iter().enumerate() {
        if run_case(&can, &mut delay, n, c) {
            passed += 1;
        }
    }

    let tec = can.tx_err_count() as u32;
    let rec = can.rx_err_count() as u32;
    if passed == CASES.len() as u32 && tec == 0 && rec == 0 {
        println!("[PASS] CAN loopback: {} frames OK, TEC=0 REC=0", passed);
    } else {
        println!(
            "[FAIL] CAN loopback: {}/{} frames OK, TEC={} REC={}",
            passed,
            CASES.len() as u32,
            tec,
            rec,
        );
    }

    loop {
        cortex_m::asm::wfi();
    }
}
