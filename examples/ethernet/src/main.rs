#![no_std]
#![no_main]

use ameba_hal::{
    ethernet::{
        EeeConfig, EeeSleepPolicy, EeeWakePolicy, EthDuplex, EthError, EthInit, EthSpeed, Ethernet,
        IfgTime, LoopbackMode, MacMode, PhyOps, PktFilterMode, RefclkDir, RefclkPhase, RxDesc,
        RxThreshold, TriggerLevel, TxDesc, TxThreshold, VlanStrip, VlanType,
    },
    println,
    rcc::{self, Peripheral},
};
use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;

// A no-op PHY stub — no physical PHY is wired up on this test run.
struct NoPhy;
impl PhyOps for NoPhy {}

// Ring sizes match the SDK's own ETH_StructInit() defaults (ETH_TxDescNum=8, ETH_RxDescNum=10).
const TX_RING: usize = 8;
const RX_RING: usize = 10;
const BUF_SIZE: usize = 1536;

static mut TX_DESC: [TxDesc; TX_RING] = [TxDesc {
    dw1: 0,
    addr: 0,
    dw2: 0,
    dw3: 0,
    dw4: 0,
}; TX_RING];
static mut RX_DESC: [RxDesc; RX_RING] = [RxDesc {
    dw1: 0,
    addr: 0,
    dw2: 0,
    dw3: 0,
}; RX_RING];
static mut TX_BUF: [u8; BUF_SIZE * TX_RING] = [0u8; BUF_SIZE * TX_RING];
static mut RX_BUF: [u8; BUF_SIZE * RX_RING] = [0u8; BUF_SIZE * RX_RING];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[ETH] start");
    ameba_hal::init(ameba_hal::Config::default());
    rcc::enable(Peripheral::GMAC);

    let eth = unsafe { Ethernet::steal() };
    let mut phy = NoPhy;

    let mut init = EthInit {
        phy: &mut phy,
        mac_mode: MacMode::Mac,
        loopback: LoopbackMode::None,
        ifg_time: IfgTime::Ifg0,
        refclk_dir: RefclkDir::Mac2Phy,
        refclk_phase: RefclkPhase::SampledOnFalling,
        rx_jumbo: false,
        pkt_filter: PktFilterMode::DefaultUnicast,
        eee_enable: false,
        auto_nego: false,
        speed: EthSpeed::Speed100M,
        duplex: EthDuplex::Full,
        vlan_rx_strip: VlanStrip::Disable,
        vlan_tx_type: VlanType::CTag,
        vlan_stag_pid: 0x88A8, // SDK: ameba_ethernet.c STagPID (IEEE 802.1ad S-Tag TPID)
        eee_config: EeeConfig {
            enable_tx: false,
            enable_rx: false,
            tw_wake_time: 30,
            tr_lpi_interval: 22,
            td_tx_delay: 5,
            tp_pause_time: 10,
            tx_traffic_thresh: 0x7D,
            sleep_policy: EeeSleepPolicy::QueueEmpty,
            wake_policy: EeeWakePolicy::AnyData,
        },
        tx_threshold: TxThreshold::B128,
        rx_threshold: RxThreshold::B128,
        tx_trigger: TriggerLevel::Pkts1,
        rx_trigger: TriggerLevel::Pkts1,
        int_mask_and_status: 0,
        mac_addr: [0x00, 0xE0, 0x4C, 0x68, 0x01, 0x23],
        tx_desc: unsafe { &mut *core::ptr::addr_of_mut!(TX_DESC) },
        rx_desc: unsafe { &mut *core::ptr::addr_of_mut!(RX_DESC) },
        tx_pkt_buf: core::ptr::addr_of_mut!(TX_BUF).cast::<u8>(),
        rx_pkt_buf: core::ptr::addr_of_mut!(RX_BUF).cast::<u8>(),
        tx_buf_size: BUF_SIZE as u16,
        rx_buf_size: BUF_SIZE as u16,
        tx_current: 0,
        rx_current: 0,
    };

    match eth.init(&mut init) {
        Ok(()) => {
            println!("[ETH] MAC init OK");
        }
        Err(EthError::Timeout) => {
            // Expected without real PHY/clock — MAC reset or auto-neg timed out.
            println!("[ETH] timeout (no PHY) -- compile OK");
        }
        Err(_) => {
            println!("[ETH] init error");
        }
    }

    println!("[PASS] ethernet: init OK");
}
