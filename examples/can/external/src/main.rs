#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embedded_hal::delay::DelayNs;

use ameba_hal::can::{
    BitTiming, CAN0_PTR, Can, CanConfig, FrameType, IdType, RxMsg, TxMsg, WorkMode,
};
use ameba_hal::delay::Delay;
use ameba_hal::pinmux::{self, Pin, PinFunction, Pull};
use ameba_hal::println;
use ameba_hal::rcc::{self, Peripheral};

const CAN0_TX: Pin = Pin::PA25;
const CAN0_RX: Pin = Pin::PA26;
const CAN0_STB: Pin = Pin::PA3;

const TX_BUF: u8 = 0;
const RX_BUF: u8 = 15;
const BIT_RATE: u32 = match option_env!("CAN_BIT_RATE") {
    Some(s) => parse_rate(s),
    None => 1_000_000,
};

/// Parse `CAN_BIT_RATE` (plain decimal bps, e.g. `500000`); a malformed value
/// fails the build.
const fn parse_rate(s: &str) -> u32 {
    let b = s.as_bytes();
    assert!(!b.is_empty(), "CAN_BIT_RATE is empty");
    let mut n = 0u32;
    let mut i = 0;
    while i < b.len() {
        assert!(
            b[i].is_ascii_digit(),
            "CAN_BIT_RATE must be plain decimal digits, e.g. 500000"
        );
        n = match n.checked_mul(10) {
            Some(v) => match v.checked_add((b[i] - b'0') as u32) {
                Some(v) => v,
                None => panic!("CAN_BIT_RATE overflows u32"),
            },
            None => panic!("CAN_BIT_RATE overflows u32"),
        };
        i += 1;
    }
    n
}
const PERIOD_MS: u32 = 500;
const REPORT_EVERY: u32 = 4;
const PERIODIC_ID: u16 = 0x321;
const ECHO_ID_OFFSET: u32 = 1;

const RAM_BUFFER_MAP: [u32; 17] = [
    0x0, 0x6, 0xc, 0x12, 0x18, 0x1e, 0x24, 0x2a, 0x30, 0x36, 0x3c, 0x42, 0x48, 0x4e, 0x54, 0x5a,
    0x60,
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

/// Send one frame and wait for TX done. `false` means nobody ACKed (or the
/// bit timing is wrong): with `auto_retx` off the frame is sent once.
fn send(can: &Can, delay: &mut Delay, msg: &TxMsg) -> bool {
    can.write_msg(msg);
    let mut done = false;
    for _ in 0..1000 {
        if can.msgbuf_tx_done(TX_BUF as usize) {
            done = true;
            break;
        }
        delay.delay_us(10);
    }
    can.clear_msgbuf_tx_done(TX_BUF as usize);
    done
}

/// Request bus-on and wait up to 10 ms for the controller to join the bus.
fn bus_on(can: &Can, delay: &mut Delay) -> bool {
    can.bus_on();
    for _ in 0..1000 {
        if can.bus_status() {
            return true;
        }
        delay.delay_us(10);
    }
    false
}

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("[CAN external] start: CAN0 @ {} bps, classic CAN", BIT_RATE);
    ameba_hal::init(ameba_hal::Config::default());
    let mut delay = Delay::new();

    Can::core_clock_set();
    rcc::enable(Peripheral::CAN0);
    pinmux::set(CAN0_TX, PinFunction::CAN0_TX);
    pinmux::set(CAN0_RX, PinFunction::CAN0_RX);
    pinmux::set_pull(CAN0_STB, Pull::Down);

    let mut timing = BitTiming::default();
    if Can::calc_bit_timing(BIT_RATE, &mut timing).is_err() {
        println!("[FAIL] CAN external: no bit timing for {} bps", BIT_RATE);
        loop {
            cortex_m::asm::wfi();
        }
    }
    let can = unsafe { Can::steal(CAN0_PTR) };
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
    can.clear_err_status(0xFFFF_FFFF);
    can.enable();
    if !bus_on(&can, &mut delay) {
        println!("[FAIL] CAN external: bus-on timeout");
        loop {
            cortex_m::asm::wfi();
        }
    }
    can.set_rx_msg_buf(&empty_rx(RX_BUF));
    println!(
        "[CAN] bus on, periodic id=0x{:X} every {} ms, echo id+{}",
        PERIODIC_ID, PERIOD_MS, ECHO_ID_OFFSET
    );

    let mut seq: u8 = 0;
    let mut tx_ok = 0u32;
    let mut tx_fail = 0u32;
    let mut rx_cnt = 0u32;
    let mut echo_fail = 0u32;
    let mut tick = 0u32;
    let mut recoveries = 0u32;

    loop {
        // Repeated bit errors (e.g. a bit-rate mismatch) drive the controller
        // bus-off; it stays off until bus-on is requested again.
        if !can.bus_status() {
            can.bus_off();
            let ok = bus_on(&can, &mut delay);
            recoveries += 1;
            println!(
                "[CAN] bus-off, recovery {}: {}",
                recoveries,
                if ok { "bus on" } else { "failed" }
            );
        }

        // Periodic frame: data[0] is a sequence number, the rest a fixed pattern.
        let d = [seq, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
        let tx = TxMsg {
            buf_idx: TX_BUF,
            frame_type: FrameType::Data,
            id_type: IdType::Standard,
            std_id: PERIODIC_ID,
            ext_id: 0,
            dlc: 8,
            data: pack(&d),
        };
        if send(&can, &mut delay, &tx) {
            tx_ok += 1;
        } else {
            tx_fail += 1;
            can.clear_err_status(0xFFFF_FFFF);
            can.clear_tx_msgbuf_err(0xFFFF);
        }
        seq = seq.wrapping_add(1);

        // Sleep in 1 ms steps and echo whatever shows up.
        for _ in 0..PERIOD_MS {
            if can.msgbuf_rx_done(RX_BUF as usize) {
                can.clear_msgbuf_rx_done(RX_BUF as usize);
                let mut m = empty_rx(RX_BUF);
                can.read_msg(&mut m);
                rx_cnt += 1;
                let id = match m.id_type {
                    IdType::Standard => m.std_id as u32,
                    IdType::Extended => m.ext_id,
                };
                let data = unpack(&m.data);
                let len = (m.dlc as usize).min(8);
                println!(
                    "[CAN] rx {} {} id=0x{:X} dlc={} data={:02X?}",
                    if m.id_type == IdType::Extended {
                        "ext"
                    } else {
                        "std"
                    },
                    if m.frame_type == FrameType::Remote {
                        "rtr"
                    } else {
                        "data"
                    },
                    id,
                    m.dlc as u32,
                    &data[..len],
                );
                let (std_id, ext_id) = match m.id_type {
                    IdType::Standard => (((id + ECHO_ID_OFFSET) & 0x7FF) as u16, 0),
                    IdType::Extended => (0, (id + ECHO_ID_OFFSET) & 0x1FFF_FFFF),
                };
                let echo = TxMsg {
                    buf_idx: TX_BUF,
                    frame_type: m.frame_type,
                    id_type: m.id_type,
                    std_id,
                    ext_id,
                    dlc: m.dlc,
                    data: m.data,
                };
                if !send(&can, &mut delay, &echo) {
                    echo_fail += 1;
                    can.clear_err_status(0xFFFF_FFFF);
                    can.clear_tx_msgbuf_err(0xFFFF);
                }
            }
            delay.delay_ms(1);
        }

        tick += 1;
        if tick.is_multiple_of(REPORT_EVERY) {
            println!(
                "[CAN] stats: tx_ok={} tx_fail={} rx={} echo_fail={} TEC={} REC={} err=0x{:08X}",
                tx_ok,
                tx_fail,
                rx_cnt,
                echo_fail,
                can.tx_err_count() as u32,
                can.rx_err_count() as u32,
                can.err_status(),
            );
        }
    }
}
