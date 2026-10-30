//! RTL8721F Ethernet (FEMAC) driver — `#![no_std]` Rust translation.
//!
//! Translated from C SDK ethernet driver (`ameba_ethernet.c`).
//!
//! ## Skipped functions
//! - `Ethernet_UpdateTXDESCAndSend` — calls `DCache_Clean` (cache management, skip).
//! - `Ethernet_GetRXPktInfo`        — calls `DCache_Invalidate` (cache management, skip).
//!
//! ## PHY operations
//! PHY driver callbacks are expressed via the [`PhyOps`] trait. The caller
//! supplies a concrete implementation; no C function-pointer structs are used.

use bitflags::bitflags;
use core::sync::atomic::{Ordering, fence};
use rtl8721f_pac::{RmiiReg, rmii_reg};

// ---------------------------------------------------------------------------
// ROM / external C symbols
// ---------------------------------------------------------------------------

use crate::delay::Delay;
use embedded_hal::delay::DelayNs;

// ---------------------------------------------------------------------------
// RMII MAC register block accessor
// ---------------------------------------------------------------------------

#[inline(always)]
fn eth() -> &'static rmii_reg::RegisterBlock {
    unsafe { &*RmiiReg::PTR }
}

// ---------------------------------------------------------------------------
// Hardware constants (mirror of C header defines used in this driver)
// ---------------------------------------------------------------------------

// ISR bits [0:15] — RW1C, live in the lower half of ETH_ISR_AND_IMR
const ISR_ROK: u32 = 1 << 0;
const ISR_CNT_WRAP: u32 = 1 << 1;
const ISR_RER_RUNT: u32 = 1 << 2;
const ISR_RER_OVF: u32 = 1 << 4;
const ISR_RDU1: u32 = 1 << 5;
const ISR_TOK_TI: u32 = 1 << 6;
const ISR_TER: u32 = 1 << 7;
const ISR_LINKCHG: u32 = 1 << 8;
const ISR_TDU: u32 = 1 << 9;
const ISR_SWINT: u32 = 1 << 10;
const ISR_RDU2: u32 = 1 << 11;
const ISR_RDU3: u32 = 1 << 12;
const ISR_RDU4: u32 = 1 << 13;
const ISR_RDU5: u32 = 1 << 14;
const ISR_RDU6: u32 = 1 << 15;

// IMR bits [16:31] — normal RW fields
const IMR_ROK: u32 = 1 << 16;
const IMR_RER_RUNT: u32 = 1 << 18;
const IMR_RER_OVF: u32 = 1 << 20;
const IMR_TOK_TI: u32 = 1 << 22;
const IMR_TER: u32 = 1 << 23;
const IMR_LINKCHG: u32 = 1 << 24;

// MSR register fields (raw bit positions)
const MSR_REFCLK_ON: u32 = 1 << 0;
const MSR_RMII2MII_EN: u32 = 1 << 7;
const MSR_FORCE_SPD_MODE: u32 = 1 << 10;
const MSR_FORCEDFULLDUP: u32 = 1 << 19;
const MSR_NWCOMPLETE: u32 = 1 << 21;
const MSR_LINKB: u32 = 1 << 26;
const MSR_TXFCE: u32 = 1 << 29;
const MSR_RXFCE: u32 = 1 << 30;
const MSR_FORCE_SPD_MASK: u32 = 0x3 << 16;

// CR register fields
const CR_RST: u32 = 1 << 0;
const CR_RXVLAN: u32 = 1 << 2;
const CR_RXJUMBO: u32 = 1 << 3;

// MIIAR register fields
const MIIAR_FLAG: u32 = 1 << 31;
const MIIAR_PHYADDR_SHIFT: u32 = 26;
const MIIAR_REGADDR_SHIFT: u32 = 16;
const MIIAR_DISABLE_AUTO_POLL: u32 = 1 << 22;
const MIIAR_MDIO_BUSY: u32 = 1 << 25;
const MIIAR_DATA_MASK: u32 = 0xFFFF;

// VLAN_REG fields
const VLAN_STAG_PID_SHIFT: u32 = 16;
const VLAN_STAG_PID_MASK: u32 = 0xFFFF << 16;
const VLAN_TDSC_TYPE_BIT: u32 = 1 << 15;

// EEE CR1 fields
const EEE_CR1_TXTIMER_EN: u32 = 1 << 8;
const EEE_CR1_RXTIMER_EN: u32 = 1 << 9;
const EEE_CR1_REQ_SET0: u32 = 1 << 10;
const EEE_CR1_REQ_SET1: u32 = 1 << 11;
const EEE_CR1_REQ_SET2: u32 = 1 << 12;
const EEE_CR1_WAKE_SET0: u32 = 1 << 13;
const EEE_CR1_WAKE_SET1: u32 = 1 << 14;
const EEE_CR1_PAUSEFLAG: u32 = 1 << 16;
const EEE_CR1_RX_STS: u32 = 1 << 17;
const EEE_CR1_TX_STS: u32 = 1 << 18;
const EEE_CR1_STS: u32 = 1 << 19;
const EEE_CR1_EN_EEE_RX: u32 = 1 << 26;
const EEE_CR1_EN_EEE_TX: u32 = 1 << 27;
const EEE_CR1_EN_EEE: u32 = 1 << 28;
const EEE_CR1_TIMER_UNIT_100_MASK: u32 = 0xF; // bits[3:0]
const EEE_CR1_SLEEP_ALL_MASK: u32 = EEE_CR1_REQ_SET0 | EEE_CR1_REQ_SET1 | EEE_CR1_REQ_SET2;
const EEE_CR1_WAKE_ALL_MASK: u32 = EEE_CR1_WAKE_SET0 | EEE_CR1_WAKE_SET1;

// EEE CR3 timer shift positions (100M timers)
const EEE_CR3_TP_SHIFT: u32 = 0;
const EEE_CR3_TD_SHIFT: u32 = 8;
const EEE_CR3_TR_SHIFT: u32 = 16;
const EEE_CR3_TW_SHIFT: u32 = 24;

// EEE CR4 TX threshold for 100M (bits[15:0])
const EEE_CR4_TX_THR_100_MASK: u32 = 0xFFFF;

// ETHER_IO_CMD bits
const IO_CMD_SHORTDESFORMAT: u32 = 1 << 30;
const IO_CMD_RXFTH_SHIFT: u32 = 11;
const IO_CMD_TSH_SHIFT: u32 = 19;
const IO_CMD_TE: u32 = 1 << 4;
const IO_CMD_RE: u32 = 1 << 5;
const IO_CMD_TXFN1ST: u32 = 1 << 0;

// IO_CMD1 bits
const IO_CMD1_DSC_FORMAT_EXTRA_SHIFT: u32 = 28;
const IO_CMD1_EN_4GB: u32 = 1 << 24;
const IO_CMD1_RXRING1: u32 = 1 << 16;

// TCR field positions
const TCR_LBK_MASK: u32 = 0x3 << 8;
const TCR_IFG_MASK: u32 = 0x7 << 10;
const TCR_LBK_SHIFT: u32 = 8;
const TCR_IFG_SHIFT: u32 = 10;

// RCR bits
const RCR_AAP: u32 = 1 << 0;
const RCR_APM: u32 = 1 << 1;
const RCR_AM: u32 = 1 << 2;
const RCR_AB: u32 = 1 << 3;
const RCR_AR: u32 = 1 << 4;
const RCR_AER: u32 = 1 << 5;

// DMA descriptor threshold macros (ETHRNTRXCPU_DES_NUM1 register)
#[inline(always)]
fn rx_pse_des_thres_off_7_0(x: u32) -> u32 {
    (x & 0xFF) << 8
}
#[inline(always)]
fn rx_pse_des_thres_on_7_0(x: u32) -> u32 {
    (x & 0xFF) << 16
}
#[inline(always)]
fn rx_pse_des_thres_on_11_8(x: u32) -> u32 {
    x & 0xF
}
#[inline(always)]
fn ethrntrxcpu_des_num_7_0(x: u32) -> u32 {
    (x & 0xFF) << 24
}
#[inline(always)]
fn ethrntrxcpu_des_num_11_8(x: u32) -> u32 {
    (x & 0xF) << 4
}
#[inline(always)]
fn rxringsize_1_low(x: u32) -> u32 {
    (x & 0xFF) << 8
}
#[inline(always)]
fn dsc_format_extra(x: u32) -> u32 {
    (x & 0x7) << IO_CMD1_DSC_FORMAT_EXTRA_SHIFT
}
#[inline(always)]
fn rxfth(x: u32) -> u32 {
    (x & 0x3) << IO_CMD_RXFTH_SHIFT
}
#[inline(always)]
fn tsh(x: u32) -> u32 {
    (x & 0x3) << IO_CMD_TSH_SHIFT
}
#[inline(always)]
fn msr_force_spd(x: u32) -> u32 {
    (x & 0x3) << 16
}
#[inline(always)]
fn msr_refclk_phase(x: u32) -> u32 {
    (x & 0x1) << 1
}
#[inline(always)]
fn msr_forcedfulldup(x: u32) -> u32 {
    (x & 0x1) << 19
}
#[inline(always)]
fn tcr_lbk(x: u32) -> u32 {
    (x & 0x3) << TCR_LBK_SHIFT
}
#[inline(always)]
fn tcr_ifg(x: u32) -> u32 {
    (x & 0x7) << TCR_IFG_SHIFT
}
#[inline(always)]
fn phyaddress(x: u32) -> u32 {
    (x & 0x1F) << MIIAR_PHYADDR_SHIFT
}
#[inline(always)]
fn regaddr(x: u32) -> u32 {
    (x & 0x1F) << MIIAR_REGADDR_SHIFT
}

// MDIO wait time (us)
const MDIO_WAIT_TIME: u32 = 64;

// ---------------------------------------------------------------------------
// isr_and_imr RW1C helper — ISR bits [0:15] have no PAC writer field,
// so clearing them requires write_volatile to the register address.
// ---------------------------------------------------------------------------

/// Write a value directly to ETH_ISR_AND_IMR using write_volatile.
/// Required because the ISR [0:15] bits are RW1C and not writable via PAC.
#[inline(always)]
unsafe fn isr_and_imr_write_volatile(val: u32) {
    unsafe {
        eth().isr_and_imr().as_ptr().write_volatile(val);
    }
}

/// Read current value of ETH_ISR_AND_IMR.
#[inline(always)]
fn isr_and_imr_read() -> u32 {
    eth().isr_and_imr().read().bits()
}

// ---------------------------------------------------------------------------
// Public event bitflags
// ---------------------------------------------------------------------------

bitflags! {
    /// Software-level Ethernet interrupt events.
    /// Maps ISR/IMR hardware bits to logical event flags.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct EthEvent: u32 {
        /// Packet received successfully (ROK).
        const RX_DONE    = 1 << 0;
        /// Packet transmitted successfully (TOK/TI).
        const TX_DONE    = 1 << 1;
        /// Link status changed.
        const LINK_CHG   = 1 << 2;
        /// RX overflow or runt error.
        const RX_ERROR   = 1 << 3;
        /// TX error.
        const TX_ERROR   = 1 << 4;
        /// RX descriptor unavailable, ring 1.
        const RDU_RING1  = 1 << 5;
        /// RX descriptor unavailable, ring 2.
        const RDU_RING2  = 1 << 6;
        /// RX descriptor unavailable, ring 3.
        const RDU_RING3  = 1 << 7;
        /// RX descriptor unavailable, ring 4.
        const RDU_RING4  = 1 << 8;
        /// RX descriptor unavailable, ring 5.
        const RDU_RING5  = 1 << 9;
        /// RX descriptor unavailable, ring 6.
        const RDU_RING6  = 1 << 10;
    }
}

// ---------------------------------------------------------------------------
// EEE status flags
// ---------------------------------------------------------------------------

bitflags! {
    /// EEE real-time status bits returned by [`Ethernet::eee_get_status`].
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct EeeStatus: u32 {
        /// TX is in LPI mode.
        const TX_STS    = EEE_CR1_TX_STS;
        /// RX is in LPI mode.
        const RX_STS    = EEE_CR1_RX_STS;
        /// Both TX and RX are in LPI mode simultaneously.
        const STS       = EEE_CR1_STS;
        /// LPI request triggered by RX Pause frame.
        const PAUSEFLAG = EEE_CR1_PAUSEFLAG;
    }
}

// ---------------------------------------------------------------------------
// EEE sleep/wake policy
// ---------------------------------------------------------------------------

/// EEE auto-sleep policy (enter LPI).  Mirrors `ETH_EEE_SleepPolicy`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EeeSleepPolicy {
    /// Enter LPI based on low traffic rate (threshold based).
    LowTraffic,
    /// Enter LPI when TX queue is empty (recommended default).
    QueueEmpty,
    /// Enter LPI when flow-control Pause frame is received.
    OnPause,
}

/// EEE auto-wake policy (exit LPI).  Mirrors `ETH_EEE_WakePolicy`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EeeWakePolicy {
    /// Wake on any new TX data.
    AnyData,
    /// Wake only on high-priority TX data.
    HighPrio,
}

// ---------------------------------------------------------------------------
// EEE configuration structure
// ---------------------------------------------------------------------------

/// EEE (IEEE 802.3az) configuration.  Mirrors `ETH_EEE_InitTypeDef`.
#[derive(Clone, Debug)]
pub struct EeeConfig {
    /// Enable MAC TX LPI.
    pub enable_tx: bool,
    /// Enable MAC RX LPI.
    pub enable_rx: bool,
    /// Wake-up time in us (T_wakeup, 100M mode, unit = 1 us).
    pub tw_wake_time: u8,
    /// LPI interval in us (T_tr, 100M mode, unit = 1 us).
    pub tr_lpi_interval: u8,
    /// Decision delay (T_td, 100M mode, unit = 8 us).
    pub td_tx_delay: u8,
    /// Pause time in us (T_pause, 100M mode, unit = 1 us).
    pub tp_pause_time: u8,
    /// TX traffic threshold in bytes (100M mode).
    pub tx_traffic_thresh: u16,
    /// Auto-sleep policy.
    pub sleep_policy: EeeSleepPolicy,
    /// Auto-wake policy.
    pub wake_policy: EeeWakePolicy,
}

impl EeeConfig {
    /// Return an `EeeConfig` filled with IEEE 802.3az-recommended defaults.
    pub fn default_100m() -> Self {
        EeeConfig {
            enable_tx: true,
            enable_rx: true,
            tw_wake_time: 30,
            tr_lpi_interval: 22,
            td_tx_delay: 5,
            tp_pause_time: 10,
            tx_traffic_thresh: 0x7D,
            sleep_policy: EeeSleepPolicy::QueueEmpty,
            wake_policy: EeeWakePolicy::AnyData,
        }
    }
}

// ---------------------------------------------------------------------------
// MAC configuration enumerations
// ---------------------------------------------------------------------------

/// MAC operating mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MacMode {
    Mac = 0,
    Phy = 1,
}

/// Loopback mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoopbackMode {
    None = 0x0,
    R2TExt = 0x1,
    T2RInt = 0x3,
}

/// InterFrame Gap selection.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IfgTime {
    Ifg0 = 0,
    Ifg1,
    Ifg2,
    Ifg3,
    Ifg4,
    Ifg5,
    Ifg6,
    Ifg7,
}

/// Reference clock direction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RefclkDir {
    /// PHY drives the reference clock to MAC.
    Phy2Mac = 0,
    /// MAC drives the reference clock to PHY.
    Mac2Phy = 1,
}

/// Reference clock sampling edge.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RefclkPhase {
    SampledOnFalling = 0,
    SampledOnRising = 1,
}

/// Link speed selection.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EthSpeed {
    Speed100M = 0,
    Speed10M = 1,
}

/// Duplex mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EthDuplex {
    Half = 0,
    Full = 1,
}

/// RX packet filter mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PktFilterMode {
    DefaultUnicast = 0,
    WithMulticast = 1,
    StrictUnicast = 2,
    Promiscuous = 3,
    DiagnosticAll = 4,
}

/// TX DMA FIFO threshold.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TxThreshold {
    B128 = 0,
    B256 = 1,
    B512 = 2,
    B1024 = 3,
}

/// RX DMA FIFO threshold.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RxThreshold {
    B1024 = 0,
    B128 = 1,
    B256 = 2,
    B512 = 3,
}

/// TX/RX DMA trigger level.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TriggerLevel {
    Pkts1 = 0,
    Pkts4 = 1,
    Pkts8 = 2,
    Pkts12 = 3,
    Pkts16 = 4,
    Pkts20 = 5,
    Pkts24 = 6,
    Pkts28 = 7,
}

/// VLAN tag type for TX descriptor command.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VlanType {
    CTag = 0,
    STag = 1,
}

/// VLAN stripping on RX.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VlanStrip {
    Disable = 0,
    Enable = 1,
}

// ---------------------------------------------------------------------------
// DMA descriptor layouts (repr(C) for DMA hardware compatibility)
// ---------------------------------------------------------------------------

/// FEMAC TX descriptor (5 × 32-bit words, 20 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TxDesc {
    /// DW1: command/status (OWN, EOR, FS, LS, IPCS, L4CS, CRC, size, ...).
    pub dw1: u32,
    /// DW2: buffer address.
    pub addr: u32,
    /// DW3: VLAN/PPPoE tag command.
    pub dw2: u32,
    /// DW4: destination port / reserved.
    pub dw3: u32,
    /// DW5: large send / reserved.
    pub dw4: u32,
}

/// FEMAC RX descriptor (4 × 32-bit words, 16 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RxDesc {
    /// DW1: status (OWN, EOR, FS, LS, CRCERR, length).
    pub dw1: u32,
    /// DW2: buffer address.
    pub addr: u32,
    /// DW3: extended status (VLAN, PON ID, ...).
    pub dw2: u32,
    /// DW4: extended status 2.
    pub dw3: u32,
}

// TX descriptor DW1 bit constants
const TX_OWN: u32 = 1 << 31;
const TX_EOR: u32 = 1 << 30;
const TX_FS: u32 = 1 << 29;
const TX_LS: u32 = 1 << 28;
const TX_IPCS: u32 = 1 << 27;
const TX_L4CS: u32 = 1 << 26;
const TX_CRC: u32 = 1 << 23;
const TX_SIZE_MASK: u32 = 0x1FFFF;

#[inline(always)]
fn tx_size(len: u32) -> u32 {
    len & TX_SIZE_MASK
}

// TX DW2 (VLAN action / tag)
const TX_W2_VLAN_ACT_SHIFT: u32 = 25;
const TX_W2_VLAN_TAG_MASK: u32 = 0xFFFF;

#[inline(always)]
fn tx_w2_vlan_act(x: u32) -> u32 {
    (x & 0x3) << TX_W2_VLAN_ACT_SHIFT
}
#[inline(always)]
fn tx_w2_vlan_tag(x: u32) -> u32 {
    x & TX_W2_VLAN_TAG_MASK
}

// RX descriptor DW1 constants
const RX_OWN: u32 = 1 << 31;
const RX_EOR: u32 = 1 << 30;
const RX_FS: u32 = 1 << 29;
const RX_LS: u32 = 1 << 28;
const RX_CRCERR: u32 = 1 << 27;
const RX_LEN_MASK: u32 = 0x0FFF;

#[inline(always)]
fn rx_len(dw1: u32) -> u32 {
    dw1 & RX_LEN_MASK
}

// RX DW2 (extended status)
const RX_W2_CTAVA: u32 = 1 << 16;
const RX_W2_CVLAN_MASK: u32 = 0xFFFF;

// ---------------------------------------------------------------------------
// Packet metadata
// ---------------------------------------------------------------------------

/// Packet metadata used for TX/RX descriptor preparation.
/// Only the fields accessed by the translated functions are included;
/// the full C struct has more fields for PPPoE / STRM_ID / CPU-tag.
#[derive(Default, Clone, Debug)]
pub struct PktMeta {
    /// Packet length in bytes.
    pub pkt_len: u32,
    /// Non-zero if a C-VLAN tag is present.
    pub vlan_valid: u8,
    /// VLAN TCI value (if `vlan_valid != 0`).
    pub vlan_tci: u16,
    /// VLAN action for TX (0=nop, 1=add, 2=remove, 3=remark).
    pub vlan_act: u8,
    /// RX CRC error flag.
    pub rx_crc_err: u8,
}

// ---------------------------------------------------------------------------
// PHY trait
// ---------------------------------------------------------------------------

/// PHY driver operations.  Implementors provide hardware-specific PHY logic.
/// Mirrors the C `struct eth_phy_ops` function-pointer table.
pub trait PhyOps {
    /// Initialise the PHY (check ID, basic setup).
    fn init(&mut self) -> Result<(), i32> {
        Ok(())
    }
    /// Software reset.
    fn reset(&mut self) -> Result<(), i32> {
        Ok(())
    }
    /// Configure reference clock direction.
    fn cfg_refclock(&mut self, _dir: RefclkDir) -> Result<(), i32> {
        Ok(())
    }
    /// Enable / disable EEE in the PHY.
    fn cfg_eee(&mut self, _enable: bool) -> Result<(), i32> {
        Ok(())
    }
    /// Configure link parameters (auto-neg enable, speed, duplex, advertise).
    fn cfg_link(
        &mut self,
        _autoneg: bool,
        _speed: EthSpeed,
        _duplex: EthDuplex,
    ) -> Result<(), i32> {
        Ok(())
    }
    /// Restart auto-negotiation.
    fn autoneg_restart(&mut self) -> Result<(), i32> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Main Ethernet init structure
// ---------------------------------------------------------------------------

/// Ethernet MAC configuration and runtime state.  Mirrors `ETH_InitTypeDef`.
pub struct EthInit<'a, P: PhyOps> {
    // PHY
    pub phy: &'a mut P,
    // MAC settings
    pub mac_mode: MacMode,
    pub loopback: LoopbackMode,
    pub ifg_time: IfgTime,
    pub refclk_dir: RefclkDir,
    pub refclk_phase: RefclkPhase,
    pub rx_jumbo: bool,
    pub pkt_filter: PktFilterMode,
    pub eee_enable: bool,
    pub auto_nego: bool,
    pub speed: EthSpeed,
    pub duplex: EthDuplex,
    // VLAN
    pub vlan_rx_strip: VlanStrip,
    pub vlan_tx_type: VlanType,
    pub vlan_stag_pid: u16,
    // EEE (only consulted when `eee_enable` is true)
    pub eee_config: EeeConfig,
    // DMA / buffer
    pub tx_threshold: TxThreshold,
    pub rx_threshold: RxThreshold,
    pub tx_trigger: TriggerLevel,
    pub rx_trigger: TriggerLevel,
    pub int_mask_and_status: u32,
    pub mac_addr: [u8; 6],
    // Descriptor rings (raw slices — caller manages memory)
    pub tx_desc: &'a mut [TxDesc],
    pub rx_desc: &'a mut [RxDesc],
    // Packet buffers
    pub tx_pkt_buf: *mut u8,
    pub rx_pkt_buf: *mut u8,
    pub tx_buf_size: u16,
    pub rx_buf_size: u16,
    // Runtime counters (updated by driver)
    pub tx_current: u8,
    pub rx_current: u8,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Driver error codes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EthError {
    BadArg,
    Timeout,
    Busy,
    Fail,
}

// ---------------------------------------------------------------------------
// Main Ethernet driver struct
// ---------------------------------------------------------------------------

/// RTL8721F Ethernet MAC driver handle.
///
/// Constructed with [`Ethernet::steal`].  A single global instance is
/// expected (the hardware is a singleton).
pub struct Ethernet {
    _private: (),
}

impl Ethernet {
    /// Construct a driver handle by stealing the hardware peripheral.
    ///
    /// # Safety
    /// Caller must ensure this is the only `Ethernet` instance and that the
    /// hardware is not accessed concurrently.
    pub unsafe fn steal() -> Self {
        Ethernet { _private: () }
    }

    // -----------------------------------------------------------------------
    // EEE (Energy Efficient Ethernet)
    // -----------------------------------------------------------------------

    /// Initialise the EEE subsystem from an [`EeeConfig`].
    ///
    /// Mirrors `Ethernet_EEE_Init`.
    pub fn eee_init(&self, cfg: &EeeConfig) {
        let eth = eth();

        if !cfg.enable_tx && !cfg.enable_rx {
            // Disable all EEE bits
            let cur = eth.eee_cr1().read().bits();
            unsafe {
                eth.eee_cr1().as_ptr().write_volatile(
                    cur & !(EEE_CR1_EN_EEE | EEE_CR1_EN_EEE_TX | EEE_CR1_EN_EEE_RX),
                );
            }
            return;
        }

        // 1. Read current CR1, clear timer unit bits, set 1us/8us units for 100M
        let mut cr1 = eth.eee_cr1().read().bits();
        cr1 &= !EEE_CR1_TIMER_UNIT_100_MASK; // bits[3:0]: unit_100_2(1:0) | unit_100_1(3:2)
        // unit_100_1 = 0 (1us), unit_100_2 = 0 (8us) → value 0x00

        // 2. Write CR3: timers for 100M (Tw, Tr, Td, Tp)
        let cr3 = ((cfg.tw_wake_time as u32) << EEE_CR3_TW_SHIFT)
            | ((cfg.tr_lpi_interval as u32) << EEE_CR3_TR_SHIFT)
            | ((cfg.td_tx_delay as u32) << EEE_CR3_TD_SHIFT)
            | ((cfg.tp_pause_time as u32) << EEE_CR3_TP_SHIFT);
        unsafe {
            eth.eee_cr3().as_ptr().write_volatile(cr3);
        }

        // 3. Write CR4: TX byte threshold for 100M (bits[15:0])
        let cur_cr4 = eth.eee_cr4().read().bits();
        let cr4 = (cur_cr4 & !EEE_CR4_TX_THR_100_MASK)
            | (cfg.tx_traffic_thresh as u32 & EEE_CR4_TX_THR_100_MASK);
        unsafe {
            eth.eee_cr4().as_ptr().write_volatile(cr4);
        }

        // 4. Apply sleep / wake policies
        cr1 &= !EEE_CR1_SLEEP_ALL_MASK;
        cr1 &= !EEE_CR1_WAKE_ALL_MASK;
        cr1 |= sleep_policy_bits(cfg.sleep_policy);
        cr1 |= wake_policy_bits(cfg.wake_policy);

        // 5. Clear enable bits, write config first
        cr1 &= !(EEE_CR1_EN_EEE | EEE_CR1_EN_EEE_TX | EEE_CR1_EN_EEE_RX);
        unsafe {
            eth.eee_cr1().as_ptr().write_volatile(cr1);
        }

        // 6. Set per-direction enables + global enable + timers
        if cfg.enable_tx {
            cr1 |= EEE_CR1_EN_EEE_TX;
        }
        if cfg.enable_rx {
            cr1 |= EEE_CR1_EN_EEE_RX;
        }
        cr1 |= EEE_CR1_EN_EEE | EEE_CR1_TXTIMER_EN | EEE_CR1_RXTIMER_EN;
        unsafe {
            eth.eee_cr1().as_ptr().write_volatile(cr1);
        }
    }

    /// Return real-time EEE status flags.
    ///
    /// Mirrors `Ethernet_EEE_GetStatus`.
    pub fn eee_get_status(&self) -> EeeStatus {
        let val = eth().eee_cr1().read().bits();
        EeeStatus::from_bits_retain(val & EeeStatus::all().bits())
    }

    /// Read cumulative LPI save times.
    ///
    /// Mirrors `Ethernet_EEE_GetSaveTime`.
    /// Returns `(tx_lpi_us, rx_lpi_us)`.
    pub fn eee_get_save_time(&self) -> (u32, u32) {
        let eth = eth();
        let tx = eth.eee_lpi_tm1_reg().read().bits() & 0x3FFFFF;
        let rx = eth.eee_lpi_tmo_reg().read().bits() & 0x3FFFFF;
        (tx, rx)
    }

    // -----------------------------------------------------------------------
    // Interrupt management
    // -----------------------------------------------------------------------

    /// Clear all pending ISR status bits (write-1-to-clear).
    ///
    /// Mirrors `Ethernet_ClearAllINT`.
    pub fn clear_all_int(&self) {
        let all_isr = ISR_ROK
            | ISR_RER_OVF
            | ISR_RER_RUNT
            | ISR_TOK_TI
            | ISR_TER
            | ISR_LINKCHG
            | ISR_CNT_WRAP
            | ISR_SWINT
            | ISR_TDU
            | ISR_RDU1
            | ISR_RDU2
            | ISR_RDU3
            | ISR_RDU4
            | ISR_RDU5
            | ISR_RDU6;
        // To clear RW1C bits: write the current IMR upper half ORed with ISR bits to set.
        let cur = isr_and_imr_read();
        unsafe {
            isr_and_imr_write_volatile(cur | all_isr);
        }
    }

    /// Return bitmask of currently pending and enabled interrupt events.
    ///
    /// Mirrors `Ethernet_GetPendingINT`.
    pub fn get_pending_int(&self) -> EthEvent {
        let raw = isr_and_imr_read();
        let mut ev = EthEvent::empty();

        if (raw & ISR_ROK) != 0 && (raw & IMR_ROK) != 0 {
            ev |= EthEvent::RX_DONE;
        }
        if (raw & ISR_TOK_TI) != 0 && (raw & IMR_TOK_TI) != 0 {
            ev |= EthEvent::TX_DONE;
        }
        if (raw & ISR_LINKCHG) != 0 && (raw & IMR_LINKCHG) != 0 {
            ev |= EthEvent::LINK_CHG;
        }
        if ((raw & ISR_RER_OVF) != 0 && (raw & IMR_RER_OVF) != 0)
            || ((raw & ISR_RER_RUNT) != 0 && (raw & IMR_RER_RUNT) != 0)
        {
            ev |= EthEvent::RX_ERROR;
        }
        if (raw & ISR_TER) != 0 && (raw & IMR_TER) != 0 {
            ev |= EthEvent::TX_ERROR;
        }
        ev
    }

    /// Clear specific interrupt events (write-1-to-clear the ISR bits).
    ///
    /// Mirrors `Ethernet_ClearINT`.
    pub fn clear_int(&self, events: EthEvent) {
        let mut mask: u32 = 0;
        if events.contains(EthEvent::RX_DONE) {
            mask |= ISR_ROK;
        }
        if events.contains(EthEvent::TX_DONE) {
            mask |= ISR_TOK_TI;
        }
        if events.contains(EthEvent::LINK_CHG) {
            mask |= ISR_LINKCHG;
        }
        if events.contains(EthEvent::RX_ERROR) {
            mask |= ISR_RER_OVF | ISR_RER_RUNT;
        }
        if events.contains(EthEvent::TX_ERROR) {
            mask |= ISR_TER;
        }

        if mask != 0 {
            let cur = isr_and_imr_read();
            unsafe {
                isr_and_imr_write_volatile(cur | mask);
            }
        }
    }

    // -----------------------------------------------------------------------
    // Descriptor / address configuration
    // -----------------------------------------------------------------------

    /// Set the start address of TX and RX descriptor rings.
    ///
    /// Mirrors `Ethernet_SetDescAddr`.
    /// Both descriptor arrays must be cache-line aligned (32-byte); this
    /// function asserts the alignment requirement via a debug assertion.
    pub fn set_desc_addr(&self, tx_desc: *const TxDesc, rx_desc: *const RxDesc) {
        debug_assert_eq!(
            tx_desc as usize & 0x1F,
            0,
            "TX desc must be 32-byte aligned"
        );
        debug_assert_eq!(
            rx_desc as usize & 0x1F,
            0,
            "RX desc must be 32-byte aligned"
        );
        let eth = eth();
        eth.tx_fdp1().write(|w| unsafe { w.bits(tx_desc as u32) });
        eth.rx_fdp1().write(|w| unsafe { w.bits(rx_desc as u32) });
    }

    /// Set the Ethernet MAC address.
    ///
    /// Mirrors `Ethernet_SetMacAddr`.
    pub fn set_mac_addr(&self, mac: &[u8; 6]) {
        let eth = eth();
        let idr0 = ((mac[0] as u32) << 24)
            | ((mac[1] as u32) << 16)
            | ((mac[2] as u32) << 8)
            | (mac[3] as u32);
        let idr4 = ((mac[4] as u32) << 24) | ((mac[5] as u32) << 16);
        eth.idr0().write(|w| unsafe { w.bits(idr0) });
        eth.idr4().write(|w| unsafe { w.bits(idr4) });
    }

    /// Set the reference clock direction (MAC→PHY or PHY→MAC).
    ///
    /// Mirrors `Ethernet_SetRefclkDirec`.
    pub fn set_refclk_direc(&self, dir: RefclkDir) {
        let eth = eth();
        let cur = eth.msr().read().bits();
        let new = if dir == RefclkDir::Mac2Phy {
            cur | MSR_REFCLK_ON
        } else {
            cur & !MSR_REFCLK_ON
        };
        unsafe {
            eth.msr().as_ptr().write_volatile(new);
        }
    }

    /// Configure the given pin as an external 50 MHz RMII reference clock input.
    ///
    /// Mirrors `Ethernet_UseExtClk`.
    pub fn use_ext_clk(&self, pin: crate::pinmux::Pin) {
        crate::pinmux::set(pin, crate::pinmux::PinFunction::EXT_CLK50M_IN);
    }

    /// Enable or disable MDIO auto-polling.
    ///
    /// Mirrors `Ethernet_AutoPolling`.
    pub fn auto_polling(&self, enable: bool) {
        let eth = eth();
        let cur = eth.miiar().read().bits();
        let new = if enable {
            cur & !MIIAR_DISABLE_AUTO_POLL
        } else {
            cur | MIIAR_DISABLE_AUTO_POLL
        };
        unsafe {
            eth.miiar().as_ptr().write_volatile(new);
        }
    }

    // -----------------------------------------------------------------------
    // TX / RX descriptor helpers
    // -----------------------------------------------------------------------

    /// Return the TX packet buffer address for the current TX descriptor,
    /// or `None` if the DMA still owns the descriptor.
    ///
    /// If the descriptor is DMA-owned, pulses the TOK_TI ISR bit to wake
    /// the DMA engine.
    ///
    /// Mirrors `Ethernet_GetTXPktInfo`.
    pub fn get_tx_pkt_info(&self, init: &EthInit<'_, impl PhyOps>) -> Option<*mut u8> {
        let idx = init.tx_current as usize;
        if idx >= init.tx_desc.len() {
            return None;
        }
        let dw1 = unsafe { core::ptr::read_volatile(&init.tx_desc[idx].dw1) };
        if (dw1 & TX_OWN) == 0 {
            let addr = unsafe { core::ptr::read_volatile(&init.tx_desc[idx].addr) };
            Some(addr as *mut u8)
        } else {
            // Poke DMA to continue processing
            let cur = isr_and_imr_read();
            unsafe {
                isr_and_imr_write_volatile(cur | ISR_TOK_TI);
            }
            None
        }
    }

    /// Return the current TX descriptor index back to DMA and advance the ring.
    ///
    /// The caller is responsible for: (1) filling the TX buffer, (2) calling
    /// this function, (3) NOT accessing the buffer after this call.
    ///
    /// Mirrors the core of `Ethernet_UpdateTXDESCAndSend` *without* DCache_Clean.
    /// The caller must perform DCache maintenance before invoking this function.
    ///
    /// Returns `Err(EthError::BadArg)` if the metadata packet length is invalid.
    pub fn update_tx_desc_and_send(
        &self,
        init: &mut EthInit<'_, impl PhyOps>,
        meta: &PktMeta,
    ) -> Result<(), EthError> {
        let tx_idx = init.tx_current as usize;
        let size = meta.pkt_len;

        if size == 0 || size > init.tx_buf_size as u32 {
            return Err(EthError::BadArg);
        }

        // Prepare DW1: IP/L4 checksum offload + CRC + FS + LS + size
        let mut cmd_sts = TX_IPCS | TX_L4CS | TX_CRC | TX_FS | TX_LS | tx_size(size);

        // DW2: VLAN command
        if meta.vlan_valid != 0 {
            let tci_swapped = ((meta.vlan_tci & 0x00FF) << 8) | ((meta.vlan_tci & 0xFF00) >> 8);
            unsafe {
                core::ptr::write_volatile(
                    &mut init.tx_desc[tx_idx].dw2,
                    tx_w2_vlan_act(meta.vlan_act as u32) | tx_w2_vlan_tag(tci_swapped as u32),
                );
            }
        } else {
            unsafe {
                core::ptr::write_volatile(&mut init.tx_desc[tx_idx].dw2, 0);
            }
        }

        // DW3, DW4: clear reserved
        unsafe {
            core::ptr::write_volatile(&mut init.tx_desc[tx_idx].dw3, 0);
            core::ptr::write_volatile(&mut init.tx_desc[tx_idx].dw4, 0);
        }

        // End-of-ring bit
        if tx_idx == init.tx_desc.len() - 1 {
            cmd_sts |= TX_EOR;
        }

        // Memory barrier before setting OWN bit
        fence(Ordering::Release);

        // Hand ownership to DMA
        unsafe {
            core::ptr::write_volatile(&mut init.tx_desc[tx_idx].dw1, cmd_sts | TX_OWN);
        }

        // Advance ring index
        let next = (tx_idx + 1) % init.tx_desc.len();
        init.tx_current = next as u8;

        // Trigger DMA
        let cur = isr_and_imr_read();
        unsafe {
            isr_and_imr_write_volatile(cur | ISR_TOK_TI);
        }
        let cur_io = eth().ether_io_cmd().read().bits();
        unsafe {
            eth()
                .ether_io_cmd()
                .as_ptr()
                .write_volatile(cur_io | IO_CMD_TXFN1ST);
        }

        Ok(())
    }

    /// Scan the RX descriptor ring for a complete frame (FS…LS) and return
    /// the packet buffer pointer and metadata.
    ///
    /// Returns `None` if no complete frame is available yet.
    ///
    /// Mirrors `Ethernet_GetRXPktInfo` *without* DCache_Invalidate.
    /// The caller must perform DCache maintenance after this returns `Some`.
    pub fn get_rx_pkt_info(
        &self,
        init: &EthInit<'_, impl PhyOps>,
        meta: &mut PktMeta,
    ) -> Option<*mut u8> {
        *meta = PktMeta::default();

        let start_idx = init.rx_current as usize;
        let mut scan_idx = start_idx;
        let mut total_len: u32 = 0;
        let mut buf: *mut u8 = core::ptr::null_mut();
        let mut is_fs = false;

        loop {
            let dw1 = unsafe { core::ptr::read_volatile(&init.rx_desc[scan_idx].dw1) };
            let dw2 = unsafe { core::ptr::read_volatile(&init.rx_desc[scan_idx].dw2) };

            // DMA still owns this descriptor
            if (dw1 & RX_OWN) != 0 {
                let cur = isr_and_imr_read();
                unsafe {
                    isr_and_imr_write_volatile(cur | ISR_ROK);
                }
                return None;
            }

            if (dw1 & RX_FS) != 0 {
                is_fs = true;
                let addr = unsafe { core::ptr::read_volatile(&init.rx_desc[scan_idx].addr) };
                buf = (addr + 2) as *mut u8;
            }

            total_len += rx_len(dw1);

            if (dw1 & RX_LS) != 0 {
                meta.pkt_len = total_len;

                if (dw2 & RX_W2_CTAVA) != 0 {
                    let raw_tci = (dw2 & RX_W2_CVLAN_MASK) as u16;
                    meta.vlan_valid = 1;
                    meta.vlan_tci = ((raw_tci & 0x00FF) << 8) | ((raw_tci & 0xFF00) >> 8);
                }

                if (dw1 & RX_CRCERR) != 0 {
                    meta.rx_crc_err = 1;
                }
                break;
            }

            scan_idx = (scan_idx + 1) % init.rx_desc.len();
            if scan_idx == start_idx {
                return None; // no LS found — ring error
            }
        }

        if is_fs && !buf.is_null() {
            Some(buf)
        } else {
            None
        }
    }

    /// Release one frame's descriptors (FS…LS) back to the DMA and advance the RX ring.
    ///
    /// Mirrors `Ethernet_UpdateRXDESC`.
    pub fn update_rx_desc(&self, init: &mut EthInit<'_, impl PhyOps>) {
        let mut curr = init.rx_current as usize;
        let rx_buf_size = init.rx_buf_size as u32;

        loop {
            let current_status = unsafe { core::ptr::read_volatile(&init.rx_desc[curr].dw1) };
            let is_last = (current_status & RX_LS) != 0;

            // Clear extended status words
            unsafe {
                core::ptr::write_volatile(&mut init.rx_desc[curr].dw2, 0);
                core::ptr::write_volatile(&mut init.rx_desc[curr].dw3, 0);
            }

            // Prepare new DW1: preserve EOR, set OWN + buffer size
            let new_dw1 = (current_status & RX_EOR) | RX_OWN | rx_buf_size;

            // Memory barrier before setting OWN bit
            fence(Ordering::Release);

            unsafe {
                core::ptr::write_volatile(&mut init.rx_desc[curr].dw1, new_dw1);
            }

            curr = (curr + 1) % init.rx_desc.len();

            if is_last || curr == init.rx_current as usize {
                break;
            }
        }

        init.rx_current = curr as u8;

        // Poke DMA
        let c = isr_and_imr_read();
        unsafe {
            isr_and_imr_write_volatile(c | ISR_ROK);
        }
    }

    // -----------------------------------------------------------------------
    // Link / speed query
    // -----------------------------------------------------------------------

    /// Return `true` if the link is up.
    ///
    /// Mirrors `Ethernet_GetLinkStatus`.
    pub fn get_link_status(&self) -> bool {
        let val = eth().msr().read().bits();
        (val & MSR_LINKB) == 0 // LINKB is inverted
    }

    /// Return current MAC link speed (bits `[28:27]` of MSR).
    ///
    /// 0 = 100 Mbps, 1 = 10 Mbps.
    ///
    /// Mirrors the speed read portion of `Ethernet_GetSpeedDuplex`.
    pub fn get_speed(&self) -> u32 {
        (eth().msr().read().bits() >> 27) & 0x3
    }

    /// Return current duplex mode (bit 22 of MSR).
    ///
    /// Mirrors the duplex read portion of `Ethernet_GetSpeedDuplex`.
    pub fn get_duplex(&self) -> u32 {
        (eth().msr().read().bits() >> 22) & 0x1
    }

    // -----------------------------------------------------------------------
    // MDIO (Clause 22)
    // -----------------------------------------------------------------------

    /// Read a PHY register via MDIO (Clause 22).
    ///
    /// Mirrors `Ethernet_ReadPhyReg`.
    pub fn read_phy_reg(&self, phy_addr: u8, reg_addr: u8) -> Result<u16, EthError> {
        if reg_addr > 31 {
            return Err(EthError::Fail);
        }
        wait_phy_idle(MDIO_WAIT_TIME).map_err(|_| EthError::Busy)?;

        // Initiate read: PHY addr + Reg addr, no FLAG bit
        let miiar_val = phyaddress(phy_addr as u32) | regaddr(reg_addr as u32);
        unsafe {
            eth().miiar().as_ptr().write_volatile(miiar_val);
        }

        wait_phy_idle(MDIO_WAIT_TIME).map_err(|_| EthError::Busy)?;

        let data = (eth().miiar().read().bits() & MIIAR_DATA_MASK) as u16;
        Ok(data)
    }

    /// Write a PHY register via MDIO (Clause 22).
    ///
    /// Mirrors `Ethernet_WritePhyReg`.
    pub fn write_phy_reg(&self, phy_addr: u8, reg_addr: u8, data: u16) -> Result<(), EthError> {
        if reg_addr > 31 {
            return Err(EthError::Fail);
        }
        wait_phy_idle(MDIO_WAIT_TIME).map_err(|_| EthError::Busy)?;

        let miiar_val = MIIAR_FLAG
            | MIIAR_DISABLE_AUTO_POLL
            | phyaddress(phy_addr as u32)
            | regaddr(reg_addr as u32)
            | (data as u32);
        unsafe {
            eth().miiar().as_ptr().write_volatile(miiar_val);
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Initialisation
    // -----------------------------------------------------------------------

    /// Initialise the Ethernet MAC, configure TX/RX DMA, apply PHY settings.
    ///
    /// Mirrors `Ethernet_Init`.
    ///
    /// # Errors
    /// - `EthError::BadArg`  — if descriptor slices are empty.
    /// - `EthError::Timeout` — if MAC reset or auto-negotiation timed out.
    pub fn init<P: PhyOps>(&self, init: &mut EthInit<'_, P>) -> Result<(), EthError> {
        if init.tx_desc.is_empty() || init.rx_desc.is_empty() {
            return Err(EthError::BadArg);
        }

        let eth = eth();

        self.auto_polling(false);
        Delay::new().delay_ms(50); // Reset MAC
        let cur_cr = eth.cr().read().bits();
        unsafe {
            eth.cr().as_ptr().write_volatile(cur_cr | CR_RST);
        }

        let mut timeout: u32 = 1000;
        loop {
            let cr_val = eth.cr().read().bits();
            if (cr_val & CR_RST) == 0 {
                break;
            }
            timeout = timeout.saturating_sub(1);
            if timeout == 0 {
                return Err(EthError::Timeout);
            }
            Delay::new().delay_us(10);
        }

        // PHY operations
        init.phy.init().map_err(|_| EthError::Fail)?;
        init.phy.reset().map_err(|_| EthError::Fail)?;
        init.phy
            .cfg_refclock(init.refclk_dir)
            .map_err(|_| EthError::Fail)?;

        if init.eee_enable {
            init.phy.cfg_eee(true).map_err(|_| EthError::Fail)?;
        }

        // Auto-negotiation or forced speed/duplex
        if init.auto_nego {
            init.phy
                .cfg_link(true, init.speed, init.duplex)
                .map_err(|_| EthError::Fail)?;
        } else {
            // Force speed
            let cur_msr = eth.msr().read().bits();
            let new_msr = (cur_msr & !MSR_FORCE_SPD_MASK)
                | MSR_FORCE_SPD_MODE
                | msr_force_spd(init.speed as u32);
            unsafe {
                eth.msr().as_ptr().write_volatile(new_msr);
            }

            // Force duplex
            let cur_msr = eth.msr().read().bits();
            let new_msr = (cur_msr & !MSR_FORCEDFULLDUP) | msr_forcedfulldup(init.duplex as u32);
            unsafe {
                eth.msr().as_ptr().write_volatile(new_msr);
            }

            init.phy
                .cfg_link(false, init.speed, init.duplex)
                .map_err(|_| EthError::Fail)?;
        }

        if init.auto_nego {
            let _ = init.phy.autoneg_restart(); // non-fatal
        }

        // TX settings
        let mut msr = eth.msr().read().bits();
        msr |= MSR_RMII2MII_EN;
        unsafe {
            eth.msr().as_ptr().write_volatile(msr);
        }

        self.set_refclk_direc(init.refclk_dir);

        let mut msr = eth.msr().read().bits();
        msr |= MSR_TXFCE | MSR_RXFCE;
        msr |= msr_refclk_phase(init.refclk_phase as u32);
        unsafe {
            eth.msr().as_ptr().write_volatile(msr);
        }

        // TCR: IFG + Loopback
        let mut tcr = eth.tcr().read().bits();
        tcr = (tcr & !TCR_IFG_MASK) | tcr_ifg(init.ifg_time as u32);
        tcr = (tcr & !TCR_LBK_MASK) | tcr_lbk(init.loopback as u32);
        unsafe {
            eth.tcr().as_ptr().write_volatile(tcr);
        }

        // RX: MAC address
        self.set_mac_addr(&init.mac_addr);

        // VLAN configuration
        let cur_cr = eth.cr().read().bits();
        let new_cr = if init.vlan_rx_strip == VlanStrip::Enable {
            cur_cr | CR_RXVLAN
        } else {
            cur_cr & !CR_RXVLAN
        };
        unsafe {
            eth.cr().as_ptr().write_volatile(new_cr);
        }

        // VLAN_REG: S-Tag PID and TX descriptor VLAN type
        let mut vlan_reg = eth.vlan_reg().read().bits();
        vlan_reg &= !(VLAN_STAG_PID_MASK | VLAN_TDSC_TYPE_BIT);
        vlan_reg |= (init.vlan_stag_pid as u32) << VLAN_STAG_PID_SHIFT;
        if init.vlan_tx_type == VlanType::STag {
            vlan_reg |= VLAN_TDSC_TYPE_BIT;
        }
        unsafe {
            eth.vlan_reg().as_ptr().write_volatile(vlan_reg);
        }

        // Re-apply VLAN RX strip (matches C driver double-write)
        if init.vlan_rx_strip == VlanStrip::Enable {
            let cur = eth.cr().read().bits();
            unsafe {
                eth.cr().as_ptr().write_volatile(cur | CR_RXVLAN);
            }
        }

        // EEE
        if init.eee_enable {
            self.eee_init(&init.eee_config);
            let _ = init.phy.cfg_eee(true);
        }

        // Packet filter (RCR)
        let rcr_val: u32 = match init.pkt_filter {
            PktFilterMode::WithMulticast => RCR_APM | RCR_AB | RCR_AM,
            PktFilterMode::StrictUnicast => RCR_APM,
            PktFilterMode::Promiscuous => RCR_AAP | RCR_AB | RCR_AM,
            PktFilterMode::DiagnosticAll => RCR_AAP | RCR_AB | RCR_AM | RCR_AR | RCR_AER,
            PktFilterMode::DefaultUnicast => RCR_APM | RCR_AB | RCR_AM,
        };
        unsafe {
            eth.rcr().as_ptr().write_volatile(rcr_val);
        }

        // Jumbo frames
        if init.rx_jumbo {
            let cur = eth.cr().read().bits();
            unsafe {
                eth.cr().as_ptr().write_volatile(cur | CR_RXJUMBO);
            }
        }

        // DMA descriptor control (ETHRNTRXCPU_DES_NUM1)
        let rx_n = init.rx_desc.len() as u32;
        let des_num_reg = rx_pse_des_thres_off_7_0(0x01)
            | rx_pse_des_thres_on_11_8((rx_n - 2) >> 8)
            | rx_pse_des_thres_on_7_0(rx_n - 2)
            | ethrntrxcpu_des_num_11_8((rx_n - 1) >> 8)
            | ethrntrxcpu_des_num_7_0(rx_n - 1);
        unsafe {
            eth.ethrnt_rx_cpu_des_num1()
                .as_ptr()
                .write_volatile(des_num_reg);
        }

        let cur_ring = eth.rx_ring_size1().read().bits();
        unsafe {
            eth.rx_ring_size1()
                .as_ptr()
                .write_volatile(cur_ring | (rxringsize_1_low(rx_n - 1) as u16));
        }

        // I/O command registers
        unsafe {
            eth.io_cmd1()
                .as_ptr()
                .write_volatile(dsc_format_extra(0x3) | IO_CMD1_EN_4GB);
        }
        let io_cmd_val =
            rxfth(init.rx_threshold as u32) | tsh(init.tx_threshold as u32) | IO_CMD_SHORTDESFORMAT;
        unsafe {
            eth.ether_io_cmd().as_ptr().write_volatile(io_cmd_val);
        }

        // Set descriptor ring pointers
        eth.tx_fdp1()
            .write(|w| unsafe { w.bits(init.tx_desc.as_ptr() as u32) });
        eth.rx_fdp1()
            .write(|w| unsafe { w.bits(init.rx_desc.as_ptr() as u32) });

        // Initialise TX descriptors
        for (i, desc) in init.tx_desc.iter_mut().enumerate() {
            let buf_addr = unsafe { init.tx_pkt_buf.add(i * init.tx_buf_size as usize) } as u32;
            unsafe {
                core::ptr::write_volatile(&mut desc.dw1, 0);
                core::ptr::write_volatile(&mut desc.addr, buf_addr);
                core::ptr::write_volatile(&mut desc.dw2, 0);
                core::ptr::write_volatile(&mut desc.dw3, 0);
                core::ptr::write_volatile(&mut desc.dw4, 0);
            }
        }

        // Initialise RX descriptors
        let rx_desc_last = init.rx_desc.len().saturating_sub(1);
        for (i, desc) in init.rx_desc.iter_mut().enumerate() {
            let buf_addr = unsafe { init.rx_pkt_buf.add(i * init.rx_buf_size as usize) } as u32;
            let dw1 = if i == rx_desc_last {
                RX_OWN | RX_EOR | (init.rx_buf_size as u32)
            } else {
                RX_OWN | (init.rx_buf_size as u32)
            };
            unsafe {
                core::ptr::write_volatile(&mut desc.dw1, dw1);
                core::ptr::write_volatile(&mut desc.addr, buf_addr);
                core::ptr::write_volatile(&mut desc.dw2, 0);
                core::ptr::write_volatile(&mut desc.dw3, 0);
            }
        }

        // Enable TX and RX
        let cur_io = eth.ether_io_cmd().read().bits();
        unsafe {
            eth.ether_io_cmd()
                .as_ptr()
                .write_volatile(cur_io | IO_CMD_TE | IO_CMD_RE);
        }

        // Write ISR + IMR combined register
        unsafe {
            isr_and_imr_write_volatile(init.int_mask_and_status);
        }

        // Enable auto-polling
        self.auto_polling(true);

        // Enable RX ring 1
        let cur_cmd1 = eth.io_cmd1().read().bits();
        unsafe {
            eth.io_cmd1()
                .as_ptr()
                .write_volatile(cur_cmd1 | IO_CMD1_RXRING1);
        }

        // Wait for auto-negotiation completion
        if init.auto_nego {
            let mut timeout: u32 = 50000;
            loop {
                let msr_val = eth.msr().read().bits();
                if (msr_val & MSR_NWCOMPLETE) != 0 {
                    break;
                }
                timeout = timeout.saturating_sub(1);
                if timeout == 0 {
                    return Err(EthError::Timeout);
                }
            }
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Poll until MDIO bus is idle or `timeout_us` has elapsed.
fn wait_phy_idle(timeout_us: u32) -> Result<(), ()> {
    let start = crate::debugtimer::timestamp();
    loop {
        let elapsed = crate::debugtimer::timestamp().wrapping_sub(start);
        if elapsed > timeout_us {
            return Err(());
        }
        if (eth().miiar().read().bits() & MIIAR_MDIO_BUSY) == 0 {
            return Ok(());
        }
    }
}

/// Convert an [`EeeSleepPolicy`] enum to the corresponding CR1 bits.
fn sleep_policy_bits(p: EeeSleepPolicy) -> u32 {
    match p {
        EeeSleepPolicy::LowTraffic => EEE_CR1_REQ_SET0,
        EeeSleepPolicy::QueueEmpty => EEE_CR1_REQ_SET1,
        EeeSleepPolicy::OnPause => EEE_CR1_REQ_SET2,
    }
}

/// Convert an [`EeeWakePolicy`] enum to the corresponding CR1 bits.
fn wake_policy_bits(p: EeeWakePolicy) -> u32 {
    match p {
        EeeWakePolicy::AnyData => EEE_CR1_WAKE_SET0,
        EeeWakePolicy::HighPrio => EEE_CR1_WAKE_SET1,
    }
}
