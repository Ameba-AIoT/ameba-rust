//! Ameba SD Host Controller driver for RTL8721F.
//!
//! This is a `no_std`, PAC-only translation of `ameba_sd_host.c`.
//!
//! # Architecture note
//!
//! The SDIO peripheral at 0x4011_4000 is split into two address windows:
//!  - 0x4011_4A00..0x4011_4B8F  RTK wrapper registers (`rtl8721f_pac::sdio`)
//!  - 0x4011_4C00..0x4011_4CFF  SDIO Host Controller Specification registers
//!
//! The PAC `sdio::RegisterBlock` only describes the wrapper window.  The Host
//! Controller IP registers are accessed through raw pointer arithmetic from the
//! same peripheral base address, using the byte offsets documented in
//! `ameba_sd_host.h` (0xC00..0xCFE relative to base = 0x4011_4000).
//!
//! # RTOS dependency
//!
//! Functions that require `rtos_*`, `os_*`, `IPC_SEMTake`, `DCache_*`,
//! `pmu_acquire_wakelock`, or `InterruptRegister` are NOT translated.
//! The polling helpers `wait_resp` / `init` use `DelayUs`/`DelayMs`
//! from the ROM via the private `rom` module.

use core::ptr;

use bitflags::bitflags;

// ---------------------------------------------------------------------------
// ROM FFI (private)
// ---------------------------------------------------------------------------
use crate::delay::Delay;
use embedded_hal::delay::DelayNs;

// ---------------------------------------------------------------------------
// PAC peripheral steal — only the wrapper registers need the PAC type; the
// host controller IP registers are accessed through raw ptr arithmetic.
// ---------------------------------------------------------------------------
use rtl8721f_pac::Sdio as SdioPac;

// Host Controller IP register offsets from SDIO_BASE (byte offsets)
const OFF_SDMA_SYS_ADDR: usize = 0xC00;
const OFF_BLK_SZ: usize = 0xC04;
const OFF_BLK_CNT: usize = 0xC06;
const OFF_CMD_ARG1: usize = 0xC08;
const OFF_XFER_MODE: usize = 0xC0C;
const OFF_CMD: usize = 0xC0E;
const OFF_RESP0: usize = 0xC10;
const OFF_RESP1: usize = 0xC14;
const OFF_RESP2: usize = 0xC18;
const OFF_RESP3: usize = 0xC1C;
const OFF_BUFFER_PORT_DATA: usize = 0xC20;
const OFF_PRESENT_STATE: usize = 0xC24;
const OFF_HOST_CTL_1: usize = 0xC28; // u8
const OFF_PWR_CTL: usize = 0xC29; // u8
const OFF_CLK_CTL: usize = 0xC2C; // u16
const OFF_TIMEOUT_CTL: usize = 0xC2E; // u8
const OFF_SW_RST: usize = 0xC2F; // u8
const OFF_NORMAL_INT_STATUS: usize = 0xC30; // u16
const OFF_ERR_INT_STATUS: usize = 0xC32; // u16
const OFF_NORMAL_INT_STATUS_EN: usize = 0xC34; // u16
const OFF_ERR_INT_STATUS_EN: usize = 0xC36; // u16
const OFF_NORMAL_SIG_EN: usize = 0xC38; // u16
const OFF_ERR_SIG_EN: usize = 0xC3A; // u16
const OFF_HOST_CTL_2: usize = 0xC3E; // u16
const OFF_CAPABILITIES0: usize = 0xC40; // u32 RO
const OFF_SYS_ADDR0: usize = 0xC58; // u32

// RTK wrapper-window offsets (relative to SDIO_BASE) — SDK: ameba_sd_host.h : SDIOHOST_TypeDef
const OFF_SRAM_CTL: usize = 0xA00; // SDK: ameba_sd_host.h : SDIOHOST_SRAM_CTL
const OFF_WRAPPER_CTL: usize = 0xA10; // SDK: ameba_sd_host.h : SDIOHOST_CTL
const OFF_ISREN: usize = 0xA34; // SDK: ameba_sd_host.h : SDIOHOST_ISREN
const OFF_SD_SAMPLE_POINT_CTL: usize = 0xB85; // SDK: ameba_sd_host.h : SDIOHOST_SD_SAMPLE_POINT_CTL

// ---------------------------------------------------------------------------
// Helper inlines for typed MMIO access
// ---------------------------------------------------------------------------
#[inline(always)]
fn reg32(off: usize) -> *mut u32 {
    (SdioPac::PTR as usize + off) as *mut u32
}

#[inline(always)]
fn reg16(off: usize) -> *mut u16 {
    (SdioPac::PTR as usize + off) as *mut u16
}

#[inline(always)]
fn reg8(off: usize) -> *mut u8 {
    (SdioPac::PTR as usize + off) as *mut u8
}

// ---------------------------------------------------------------------------
// Configuration constants (from ameba_sd_host.h)
// ---------------------------------------------------------------------------
const SDIO_SRAM_CTL_CONFIG: u32 = 0x021F_0200;
const SDIO_IP_CTL_CONFIG: u32 = 0x0000_000B;
const SDIO_HOST_CTL_1_CONFIG: u8 = 0xC1;
const SDIO_TIMEOUT_CTL_CONFIG: u8 = 0x0E;
const SDIO_ISREN_CONFIG: u32 = 0x0000_0011;
const SDIO_NORMAL_INT_STATUS_EN_CONFIG: u16 = 0x01FB;
const SDIO_ERR_INT_STATUS_EN_CONFIG: u16 = 0x037F;
const SDIO_ERR_SIG_EN_CONFIG: u16 = 0x0380;
const SDIO_HOST_CTL2_CONFIG: u16 = 0x000A; // 1.8 V + SDR50

/// 3.3 V bus voltage (bits [3:1] = 0b111) | power-on (bit 0)
const PWR_CTL_3V3_ON: u8 = (0x7 << 1) | 0x1;

// ---------------------------------------------------------------------------
// Timeout defaults
// ---------------------------------------------------------------------------
const SDIO_CMD_TIMEOUT_US: u32 = 100_000;
const SDMMC_CMD_TIMEOUT: u32 = 5_000;
const SDMMC_STOP_TRANS_TIMEOUT: u32 = 500_000;
const SDMMC_ERASE_TIMEOUT: u32 = 250_000;
const SDMMC_DAT_TIMEOUT: u32 = 1_000_000;

/// Initialisation clock frequency (kHz).
const SD_INIT_400K: u32 = 400;

// ---------------------------------------------------------------------------
// Bitflags for Normal / Error interrupt status and signal-enable registers
// ---------------------------------------------------------------------------
bitflags! {
    /// Normal interrupt status bits.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct NormIntFlags: u16 {
        const CMD_COMPLETE      = 1 << 0;
        const XFER_COMPLETE     = 1 << 1;
        const BLK_GAP_EVENT     = 1 << 2;
        const DMA_INT           = 1 << 3;
        const BUFF_WRITE_READY  = 1 << 4;
        const BUFF_READ_READY   = 1 << 5;
        const CARD_INSERTION    = 1 << 6;
        const CARD_REMOVAL      = 1 << 7;
        const CARD_INT          = 1 << 8;
        const INT_A             = 1 << 9;
        const INT_B             = 1 << 10;
        const INT_C             = 1 << 11;
        const RETUNING_EVENT    = 1 << 12;
        const ERR_INT           = 1 << 15;
    }
}

bitflags! {
    /// Error interrupt status bits.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ErrIntFlags: u16 {
        const CMD_TIMEOUT_ERR   = 1 << 0;
        const CMD_CRC_ERR       = 1 << 1;
        const CMD_END_BIT_ERR   = 1 << 2;
        const CMD_IDX_ERR       = 1 << 3;
        const DATA_TIMEOUT_ERR  = 1 << 4;
        const DATA_CRC_ERR      = 1 << 5;
        const DATA_END_BIT_ERR  = 1 << 6;
        const CURRENT_LIMIT_ERR = 1 << 7;
        const AUTO_CMD_ERR      = 1 << 8;
        const ADMA_ERR          = 1 << 9;
        const TUNING_ERR        = 1 << 10;
        const VENDOR_ERR        = 0xF << 12;
    }
}

/// Mask for all command-path error bits.
const SDIO_CMD_ERR: ErrIntFlags = ErrIntFlags::from_bits_retain(
    ErrIntFlags::CMD_IDX_ERR.bits()
        | ErrIntFlags::CMD_END_BIT_ERR.bits()
        | ErrIntFlags::CMD_CRC_ERR.bits()
        | ErrIntFlags::CMD_TIMEOUT_ERR.bits(),
);

bitflags! {
    /// Present-state register bits (read-only).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PresentState: u32 {
        const CMD_INHIBIT_CMD   = 1 << 0;
        const CMD_INHIBIT_DAT   = 1 << 1;
        const DAT_LINE_ACTIVE   = 1 << 2;
        const WRITE_XFER_ACTIVE = 1 << 8;
        const READ_XFER_ACTIVE  = 1 << 9;
        const BUFF_WRITE_EN     = 1 << 10;
        const BUFF_READ_EN      = 1 << 11;
        const CARD_INSERTED     = 1 << 16;
        const CARD_STATE_STABLE = 1 << 17;
        const CD_PIN_LEVEL      = 1 << 18;
        const WP_SWITCH_PIN_LVL = 1 << 19;
        const DAT_LINE_SIG_LVL  = 0xF << 20;
        const CMD_LINE_SIG_LVL  = 1 << 24;
    }
}

// ---------------------------------------------------------------------------
// Type-safe enumerations
// ---------------------------------------------------------------------------

/// SD data bus width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BusWidth {
    OneBit,
    FourBit,
}

/// DMA mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaMode {
    /// Single-operation DMA.
    Sdma,
    /// Advanced DMA 2, 32-bit address.
    Adma2_32b,
}

/// Data transfer type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransType {
    SingleBlock,
    InfiniteBlock,
    MultiBlock,
    MultiBlockStop,
}

/// Transfer direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransDir {
    HostToCard = 0,
    CardToHost = 1,
}

/// Auto-command enable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoCmd {
    Disabled = 0,
    AutoCmd12 = 1,
    AutoCmd23 = 2,
}

/// Command type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdType {
    Normal = 0,
    Suspend = 1,
    Resume = 2,
    Abort = 3,
}

/// SDMMC response type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RespType {
    None,
    R1,
    R1b,
    R2,
    R3,
    R4,
    R5,
    R5b,
    R6,
    R7,
}

/// Whether a command carries a data phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataPresent {
    NoData,
    WithData,
}

/// Response register selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RespReg {
    Resp0,
    Resp1,
    Resp2,
    Resp3,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// SD host error codes (mirrors C `SD_ERROR_*` defines).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SdError {
    None = 0x0000_0000,
    CmdCrcFail = 0x0000_0001,
    DataCrcFail = 0x0000_0002,
    CmdRspTimeout = 0x0000_0004,
    DataTimeout = 0x0000_0008,
    AddrMisaligned = 0x0000_0040,
    BlockLenErr = 0x0000_0080,
    EraseSeqErr = 0x0000_0100,
    BadEraseParam = 0x0000_0200,
    WriteProt = 0x0000_0400,
    LockUnlockFailed = 0x0000_0800,
    ComCrcFailed = 0x0000_1000,
    IllegalCmd = 0x0000_2000,
    CardEccFailed = 0x0000_4000,
    CcErr = 0x0000_8000,
    GeneralUnknown = 0x0001_0000,
    StreamReadUnderrun = 0x0002_0000,
    StreamWriteOverrun = 0x0004_0000,
    CidCsdOverwrite = 0x0008_0000,
    WpEraseSkip = 0x0010_0000,
    CardEccDisabled = 0x0020_0000,
    EraseReset = 0x0040_0000,
    AkeSeqErr = 0x0080_0000,
    AddrOutOfRange = 0x0200_0000,
    Busy = 0x2000_0000,
    Timeout = 0x8000_0000,
}

pub type Result<T = ()> = core::result::Result<T, SdError>;

// ---------------------------------------------------------------------------
// Command / Data descriptor structs
// ---------------------------------------------------------------------------

/// Parameters for a single SDMMC command.
pub struct CmdInit {
    pub argument: u32,
    pub cmd_index: u8,
    pub cmd_type: CmdType,
    pub resp_type: RespType,
    pub data_present: DataPresent,
}

/// Parameters for a data transfer.
pub struct DataInit {
    pub trans_type: TransType,
    pub trans_dir: TransDir,
    pub auto_cmd: AutoCmd,
    pub dma_en: bool,
    pub block_size: u16,
    pub block_cnt: u16,
}

// ---------------------------------------------------------------------------
// Driver handle
// ---------------------------------------------------------------------------

/// SD Host Controller driver handle.
///
/// Owns the PAC `Sdio` token.  Obtain with [`SdHost::steal`].
pub struct SdHost {
    _pac: SdioPac,
}

impl SdHost {
    // -----------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------

    /// Construct an `SdHost` by stealing the PAC peripheral singleton.
    ///
    /// # Safety
    ///
    /// The caller must guarantee no other code concurrently accesses the SDIO
    /// peripheral.  This is the only `unsafe` public entry point.
    pub unsafe fn steal() -> Self {
        unsafe {
            Self {
                _pac: SdioPac::steal(),
            }
        }
    }

    // -----------------------------------------------------------------------
    // init  (C: SDIOH_Init)
    // -----------------------------------------------------------------------

    /// Initialise the SDIO host controller (equivalent to C `SDIOH_Init`).
    pub fn init(&mut self) -> Result {
        unsafe {
            // Set RTK wrapper registers
            ptr::write_volatile(reg32(OFF_SRAM_CTL), SDIO_SRAM_CTL_CONFIG);
            ptr::write_volatile(reg32(OFF_WRAPPER_CTL), SDIO_IP_CTL_CONFIG);

            // Host Control 1 and power
            ptr::write_volatile(reg8(OFF_HOST_CTL_1), SDIO_HOST_CTL_1_CONFIG);
            ptr::write_volatile(reg8(OFF_PWR_CTL), PWR_CTL_3V3_ON);

            // Byte-access: enable MCU_SEL, configure sample point, clear MCU_SEL
            let sram = ptr::read_volatile(reg32(OFF_SRAM_CTL));
            ptr::write_volatile(reg32(OFF_SRAM_CTL), sram | 0x1); // MCU_SEL = 1

            let sp = ptr::read_volatile(reg8(OFF_SD_SAMPLE_POINT_CTL));
            ptr::write_volatile(reg8(OFF_SD_SAMPLE_POINT_CTL), sp | (1 << 3)); // SD20_SAMP_POINT

            let sram = ptr::read_volatile(reg32(OFF_SRAM_CTL));
            ptr::write_volatile(reg32(OFF_SRAM_CTL), sram & !0x1); // MCU_SEL = 0

            // Turn on internal clock (bit 0 = INTERNAL_CLK_EN)
            ptr::write_volatile(reg16(OFF_CLK_CTL), 0x0001);

            // Wait for internal clock stable (bit 1 = INTERNAL_CLK_STABLE)
            let start = crate::debugtimer::timestamp();
            loop {
                if ptr::read_volatile(reg16(OFF_CLK_CTL)) & (1 << 1) != 0 {
                    break;
                }
                if is_timeout(start, SDIO_CMD_TIMEOUT_US) {
                    return Err(SdError::Timeout);
                }
            }

            self.config_clock(SD_INIT_400K)?;

            ptr::write_volatile(reg8(OFF_TIMEOUT_CTL), SDIO_TIMEOUT_CTL_CONFIG);
            ptr::write_volatile(reg32(OFF_ISREN), SDIO_ISREN_CONFIG);

            ptr::write_volatile(
                reg16(OFF_NORMAL_INT_STATUS_EN),
                SDIO_NORMAL_INT_STATUS_EN_CONFIG,
            );
            ptr::write_volatile(reg16(OFF_ERR_INT_STATUS_EN), SDIO_ERR_INT_STATUS_EN_CONFIG);

            // Normal signal enable: card-interrupt only
            ptr::write_volatile(reg16(OFF_NORMAL_SIG_EN), NormIntFlags::CARD_INT.bits());
            ptr::write_volatile(reg16(OFF_ERR_SIG_EN), SDIO_ERR_SIG_EN_CONFIG);

            // Clear all interrupts
            ptr::write_volatile(reg16(OFF_NORMAL_INT_STATUS), 0xFFFF);
            ptr::write_volatile(reg16(OFF_ERR_INT_STATUS), 0xFFFF);

            ptr::write_volatile(reg16(OFF_HOST_CTL_2), SDIO_HOST_CTL2_CONFIG);

            Delay::new().delay_ms(5); // regulator stable within 5 ms
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // reset_all  (C: SDIO_ResetAll)
    // -----------------------------------------------------------------------

    /// Perform a software-reset-all and poll for completion.
    pub fn reset_all(&mut self) -> Result {
        unsafe {
            let sw = ptr::read_volatile(reg8(OFF_SW_RST));
            ptr::write_volatile(reg8(OFF_SW_RST), sw | 0x01); // SW_RST_ALL

            let start = crate::debugtimer::timestamp();
            loop {
                if ptr::read_volatile(reg8(OFF_SW_RST)) == 0 {
                    break;
                }
                if is_timeout(start, SDIO_CMD_TIMEOUT_US) {
                    return Err(SdError::Timeout);
                }
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // check_state  (C: SDIO_CheckState)
    // -----------------------------------------------------------------------

    /// Verify that the SDIO bus is idle and a card is inserted and not write-protected.
    pub fn check_state(&self) -> Result {
        let s = self.get_present_state();

        let bus_idle = PresentState::CMD_LINE_SIG_LVL | PresentState::DAT_LINE_SIG_LVL;
        if !s.contains(bus_idle) {
            return Err(SdError::Busy);
        }
        if s.contains(PresentState::CMD_INHIBIT_CMD) {
            return Err(SdError::Busy);
        }
        if s.contains(PresentState::CMD_INHIBIT_DAT) {
            return Err(SdError::Busy);
        }
        if s.intersects(PresentState::WRITE_XFER_ACTIVE | PresentState::READ_XFER_ACTIVE) {
            return Err(SdError::Busy);
        }
        if !s.contains(PresentState::CARD_INSERTED) {
            return Err(SdError::GeneralUnknown);
        }
        if !s.contains(PresentState::WP_SWITCH_PIN_LVL) {
            return Err(SdError::WriteProt);
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // FIFO access  (C: SDIO_ReadFIFO / SDIO_WriteFIFO)
    // -----------------------------------------------------------------------

    /// Read one 32-bit word from the buffer data port (PIO mode).
    #[inline]
    pub fn read_fifo(&self) -> u32 {
        unsafe { ptr::read_volatile(reg32(OFF_BUFFER_PORT_DATA)) }
    }

    /// Write one 32-bit word to the buffer data port (PIO mode).
    #[inline]
    pub fn write_fifo(&mut self, data: u32) {
        unsafe { ptr::write_volatile(reg32(OFF_BUFFER_PORT_DATA), data) }
    }

    // -----------------------------------------------------------------------
    // Power control  (C: SDIO_PowerState_ON / _OFF / GetPowerState)
    // -----------------------------------------------------------------------

    /// Enable bus power.
    #[inline]
    pub fn power_on(&mut self) {
        unsafe {
            let v = ptr::read_volatile(reg8(OFF_PWR_CTL));
            ptr::write_volatile(reg8(OFF_PWR_CTL), v | 0x01);
        }
    }

    /// Disable bus power.
    #[inline]
    pub fn power_off(&mut self) {
        unsafe { ptr::write_volatile(reg8(OFF_PWR_CTL), 0x00) }
    }

    /// Returns `true` if bus power is currently on.
    #[inline]
    pub fn power_state(&self) -> bool {
        unsafe { ptr::read_volatile(reg8(OFF_PWR_CTL)) & 0x01 != 0 }
    }

    // -----------------------------------------------------------------------
    // config_clock  (C: SDIO_ConfigClock)
    // -----------------------------------------------------------------------

    /// Configure the SD clock to `clk_khz` kHz.
    pub fn config_clock(&mut self, clk_khz: u32) -> Result {
        if clk_khz == 0 {
            return Err(SdError::GeneralUnknown);
        }

        unsafe {
            // Disable SD clock (bit 2 = SDCLK_EN)
            let c = ptr::read_volatile(reg16(OFF_CLK_CTL));
            ptr::write_volatile(reg16(OFF_CLK_CTL), c & !(1 << 2));

            // High-speed mode: bit 2 of HOST_CTL_1 = HS_EN
            let h1 = ptr::read_volatile(reg8(OFF_HOST_CTL_1));
            if clk_khz > 25_000 {
                ptr::write_volatile(reg8(OFF_HOST_CTL_1), h1 | (1 << 2));
            } else {
                ptr::write_volatile(reg8(OFF_HOST_CTL_1), h1 & !(1 << 2));
            }

            // Base clock from capabilities bits [15:8], unit MHz; actual = half
            let cap0 = ptr::read_volatile(reg32(OFF_CAPABILITIES0));
            let base_clk_mhz = ((cap0 >> 8) & 0xFF) / 2;
            if base_clk_mhz == 0 {
                return Err(SdError::GeneralUnknown);
            }

            // ceil division
            let div_cnt = (base_clk_mhz * 1_000).div_ceil(clk_khz);
            if div_cnt == 0 {
                return Err(SdError::GeneralUnknown);
            }

            let div_select: u8 = if (div_cnt & (div_cnt - 1)) == 0 {
                // exact power-of-two
                (div_cnt / 2) as u8
            } else {
                let mut power: u32 = 1;
                while power < div_cnt {
                    power <<= 1;
                }
                if power > 256 {
                    return Err(SdError::GeneralUnknown);
                }
                (div_cnt / 2) as u8
            };

            // SDCLK_FREQ_SEL lives in bits [15:8]; preserve lower byte
            let mut temp = ptr::read_volatile(reg16(OFF_CLK_CTL)) & 0x00FF;
            temp |= (div_select as u16) << 8;
            ptr::write_volatile(reg16(OFF_CLK_CTL), temp);

            // Re-enable SD clock
            let c2 = ptr::read_volatile(reg16(OFF_CLK_CTL));
            ptr::write_volatile(reg16(OFF_CLK_CTL), c2 | (1 << 2));
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // config_bus_width  (C: SDIO_ConfigBusWidth)
    // -----------------------------------------------------------------------

    /// Configure the data bus width.
    pub fn config_bus_width(&mut self, width: BusWidth) -> Result {
        unsafe {
            let h1 = ptr::read_volatile(reg8(OFF_HOST_CTL_1));
            match width {
                BusWidth::OneBit => ptr::write_volatile(reg8(OFF_HOST_CTL_1), h1 & !(1 << 1)),
                BusWidth::FourBit => ptr::write_volatile(reg8(OFF_HOST_CTL_1), h1 | (1 << 1)),
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // config_dma  (C: SDIO_ConfigDMA)
    // -----------------------------------------------------------------------

    /// Configure DMA mode and provide the system address.
    ///
    /// 64-bit ADMA2 mode is not supported and returns `Err`.
    pub fn config_dma(&mut self, mode: DmaMode, addr: u32) -> Result {
        let cap0 = unsafe { ptr::read_volatile(reg32(OFF_CAPABILITIES0)) };

        unsafe {
            match mode {
                DmaMode::Sdma => {
                    // Bit 22 = SDMA_SUPPORT
                    if cap0 & (1 << 22) == 0 {
                        return Err(SdError::GeneralUnknown);
                    }
                    ptr::write_volatile(reg32(OFF_SDMA_SYS_ADDR), addr);
                    // DMA_SEL [4:3] in HOST_CTL_1 = 0b00
                    let h1 = ptr::read_volatile(reg8(OFF_HOST_CTL_1));
                    ptr::write_volatile(reg8(OFF_HOST_CTL_1), h1 & !(0x3 << 3));
                }
                DmaMode::Adma2_32b => {
                    // Bit 19 = ADMA2_SUPPORT
                    if cap0 & (1 << 19) == 0 {
                        return Err(SdError::GeneralUnknown);
                    }
                    ptr::write_volatile(reg32(OFF_SYS_ADDR0), addr);
                    // DMA_SEL [4:3] = 0b10
                    let h1 = ptr::read_volatile(reg8(OFF_HOST_CTL_1));
                    ptr::write_volatile(reg8(OFF_HOST_CTL_1), (h1 & !(0x3 << 3)) | (0x2 << 3));
                }
            }
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // config_data  (C: SDIO_ConfigData)
    // -----------------------------------------------------------------------

    /// Configure the data path before issuing a data command.
    pub fn config_data(&mut self, data: &DataInit) {
        unsafe {
            ptr::write_volatile(reg16(OFF_BLK_SZ), data.block_size);

            let dma: u16 = if data.dma_en { 1 } else { 0 };
            let dir: u16 = data.trans_dir as u16;
            let auto: u16 = data.auto_cmd as u16;

            let mut xfer: u16 = (dir << 4) | dma | (auto << 2);

            match data.trans_type {
                TransType::MultiBlock => {
                    xfer |= (1 << 5) | (1 << 1); // multi-blk | blk-cnt-en
                    ptr::write_volatile(reg16(OFF_BLK_CNT), data.block_cnt);
                }
                TransType::InfiniteBlock => {
                    xfer |= 1 << 5; // multi-blk
                    xfer &= !(1 << 1); // no blk-cnt
                }
                TransType::MultiBlockStop => {
                    xfer |= (1 << 5) | (1 << 1);
                    ptr::write_volatile(reg16(OFF_BLK_CNT), 0);
                }
                TransType::SingleBlock => {
                    xfer &= !(1 << 5);
                }
            }

            ptr::write_volatile(reg16(OFF_XFER_MODE), xfer);
        }
    }

    // -----------------------------------------------------------------------
    // send_command  (C: SDIO_SendCommand)
    // -----------------------------------------------------------------------

    /// Write an SDMMC command to the CMD register (does not poll for completion).
    pub fn send_command(&mut self, cmd: &CmdInit) {
        // SD Host Controller Spec response type field (bits [1:0] of CMD register)
        const RESP_NONE: u16 = 0b00;
        const RESP_136: u16 = 0b01;
        const RESP_48: u16 = 0b10;
        const RESP_48BUSY: u16 = 0b11;

        unsafe {
            ptr::write_volatile(reg32(OFF_CMD_ARG1), cmd.argument);

            let mut tmp: u16 = (cmd.cmd_index as u16) << 8;

            // CMD_TYPE bits [7:6]
            tmp = (tmp & !(0x3 << 6)) | ((cmd.cmd_type as u16) << 6);

            // DATA_PRESENT_SEL bit 5
            match cmd.data_present {
                DataPresent::WithData => tmp |= 1 << 5,
                DataPresent::NoData => tmp &= !(1 << 5),
            }

            // Response type, CRC/IDX check bits
            let (resp_bits, idx_chk, crc_chk): (u16, bool, bool) = match cmd.resp_type {
                RespType::None => (RESP_NONE, false, false),
                RespType::R2 => (RESP_136, false, true),
                RespType::R3 | RespType::R4 => (RESP_48, false, false),
                RespType::R1b | RespType::R5b => (RESP_48BUSY, true, true),
                _ => (RESP_48, true, true), // R1,R5,R6,R7
            };

            tmp = (tmp & !0x3) | resp_bits;
            if idx_chk {
                tmp |= 1 << 4;
            } else {
                tmp &= !(1 << 4);
            }
            if crc_chk {
                tmp |= 1 << 3;
            } else {
                tmp &= !(1 << 3);
            }

            ptr::write_volatile(reg16(OFF_CMD), tmp);
        }
    }

    // -----------------------------------------------------------------------
    // get_response  (C: SDIO_GetResponse)
    // -----------------------------------------------------------------------

    /// Read one of the four response registers.
    pub fn get_response(&self, reg: RespReg) -> u32 {
        let off = match reg {
            RespReg::Resp0 => OFF_RESP0,
            RespReg::Resp1 => OFF_RESP1,
            RespReg::Resp2 => OFF_RESP2,
            RespReg::Resp3 => OFF_RESP3,
        };
        unsafe { ptr::read_volatile(reg32(off)) }
    }

    // -----------------------------------------------------------------------
    // Status and interrupt helpers
    // -----------------------------------------------------------------------

    /// Read the present-state register.
    pub fn get_present_state(&self) -> PresentState {
        PresentState::from_bits_truncate(unsafe { ptr::read_volatile(reg32(OFF_PRESENT_STATE)) })
    }

    /// Read normal interrupt status.
    #[inline]
    pub fn get_norm_sts(&self) -> NormIntFlags {
        NormIntFlags::from_bits_truncate(unsafe {
            ptr::read_volatile(reg16(OFF_NORMAL_INT_STATUS))
        })
    }

    /// Read error interrupt status.
    #[inline]
    pub fn get_err_sts(&self) -> ErrIntFlags {
        ErrIntFlags::from_bits_truncate(unsafe { ptr::read_volatile(reg16(OFF_ERR_INT_STATUS)) })
    }

    /// Clear bits in the normal interrupt status register (W1C).
    #[inline]
    pub fn clear_norm_sts(&mut self, bits: NormIntFlags) {
        unsafe { ptr::write_volatile(reg16(OFF_NORMAL_INT_STATUS), bits.bits()) }
    }

    /// Clear bits in the error interrupt status register (W1C).
    #[inline]
    pub fn clear_err_sts(&mut self, bits: ErrIntFlags) {
        unsafe { ptr::write_volatile(reg16(OFF_ERR_INT_STATUS), bits.bits()) }
    }

    /// Enable or disable normal interrupt signal bits.
    pub fn config_norm_int_sig(&mut self, bits: NormIntFlags, enable: bool) {
        unsafe {
            let v = ptr::read_volatile(reg16(OFF_NORMAL_SIG_EN));
            ptr::write_volatile(
                reg16(OFF_NORMAL_SIG_EN),
                if enable {
                    v | bits.bits()
                } else {
                    v & !bits.bits()
                },
            );
        }
    }

    /// Enable or disable error interrupt signal bits.
    pub fn config_err_int_sig(&mut self, bits: ErrIntFlags, enable: bool) {
        unsafe {
            let v = ptr::read_volatile(reg16(OFF_ERR_SIG_EN));
            ptr::write_volatile(
                reg16(OFF_ERR_SIG_EN),
                if enable {
                    v | bits.bits()
                } else {
                    v & !bits.bits()
                },
            );
        }
    }

    /// Enable or disable normal interrupt status bits.
    pub fn config_norm_int_sts(&mut self, bits: NormIntFlags, enable: bool) {
        unsafe {
            let v = ptr::read_volatile(reg16(OFF_NORMAL_INT_STATUS_EN));
            ptr::write_volatile(
                reg16(OFF_NORMAL_INT_STATUS_EN),
                if enable {
                    v | bits.bits()
                } else {
                    v & !bits.bits()
                },
            );
        }
    }

    /// Enable or disable error interrupt status bits.
    pub fn config_err_int_sts(&mut self, bits: ErrIntFlags, enable: bool) {
        unsafe {
            let v = ptr::read_volatile(reg16(OFF_ERR_INT_STATUS_EN));
            ptr::write_volatile(
                reg16(OFF_ERR_INT_STATUS_EN),
                if enable {
                    v | bits.bits()
                } else {
                    v & !bits.bits()
                },
            );
        }
    }

    // -----------------------------------------------------------------------
    // wait_resp  (C: SDIO_WaitResp)
    // -----------------------------------------------------------------------

    /// Poll until a command completes (or errors) and check the response.
    ///
    /// `timeout_us` — maximum wait in microseconds (each loop iteration waits 1 µs).
    pub fn wait_resp(&mut self, resp_type: RespType, timeout_us: u32) -> Result {
        match resp_type {
            RespType::R1
            | RespType::R2
            | RespType::R3
            | RespType::R4
            | RespType::R5
            | RespType::R6
            | RespType::R7 => {
                let mut count: u32 = 0;
                let (norm, err) = loop {
                    let n = self.get_norm_sts();
                    let e = self.get_err_sts();
                    if n.contains(NormIntFlags::CMD_COMPLETE) || e.intersects(SDIO_CMD_ERR) {
                        break (n, e);
                    }
                    count += 1;
                    if count >= timeout_us {
                        return Err(SdError::Timeout);
                    }
                    Delay::new().delay_us(1);
                };

                if norm.contains(NormIntFlags::CMD_COMPLETE) {
                    self.clear_norm_sts(NormIntFlags::CMD_COMPLETE);
                }

                if err.intersects(SDIO_CMD_ERR) {
                    if matches!(resp_type, RespType::R7) {
                        Delay::new().delay_us(10);
                    }
                    self.clear_err_sts(SDIO_CMD_ERR);
                    return if err.contains(ErrIntFlags::CMD_TIMEOUT_ERR) {
                        Err(SdError::CmdRspTimeout)
                    } else {
                        Err(SdError::GeneralUnknown)
                    };
                }

                match resp_type {
                    RespType::R2 | RespType::R3 | RespType::R4 | RespType::R7 => Ok(()),
                    RespType::R1 => check_err_resp1(self.get_response(RespReg::Resp0)),
                    RespType::R5 => check_err_resp5(self.get_response(RespReg::Resp0)),
                    RespType::R6 => check_err_resp6(self.get_response(RespReg::Resp0)),
                    _ => Err(SdError::GeneralUnknown),
                }
            }

            RespType::R1b | RespType::R5b => {
                let wait_err = SDIO_CMD_ERR | ErrIntFlags::DATA_TIMEOUT_ERR;
                let mut count: u32 = 0;

                let (norm, err) = loop {
                    let n = self.get_norm_sts();
                    let e = self.get_err_sts();
                    if n.contains(NormIntFlags::XFER_COMPLETE) || e.intersects(wait_err) {
                        break (n, e);
                    }
                    count += 1;
                    if count >= timeout_us {
                        return Err(SdError::Timeout);
                    }
                    Delay::new().delay_us(1);
                };

                if norm.contains(NormIntFlags::CMD_COMPLETE) {
                    self.clear_norm_sts(NormIntFlags::CMD_COMPLETE);
                }
                if norm.contains(NormIntFlags::XFER_COMPLETE) {
                    self.clear_norm_sts(NormIntFlags::XFER_COMPLETE);
                }

                if !norm.contains(NormIntFlags::XFER_COMPLETE)
                    && err.contains(ErrIntFlags::DATA_TIMEOUT_ERR)
                {
                    return Err(SdError::Busy);
                }

                if err.intersects(SDIO_CMD_ERR) {
                    self.clear_err_sts(SDIO_CMD_ERR);
                    return if err.contains(ErrIntFlags::CMD_TIMEOUT_ERR) {
                        Err(SdError::CmdRspTimeout)
                    } else {
                        Err(SdError::GeneralUnknown)
                    };
                }

                let r0 = self.get_response(RespReg::Resp0);
                match resp_type {
                    RespType::R1b => check_err_resp1(r0),
                    RespType::R5b => check_err_resp5(r0),
                    _ => Err(SdError::GeneralUnknown),
                }
            }

            RespType::None => {
                let mut count: u32 = 0;
                loop {
                    if self.get_norm_sts().contains(NormIntFlags::CMD_COMPLETE) {
                        break;
                    }
                    count += 1;
                    if count >= timeout_us {
                        return Err(SdError::Timeout);
                    }
                    Delay::new().delay_us(1);
                }
                self.clear_norm_sts(NormIntFlags::CMD_COMPLETE);
                Ok(())
            }
        }
    }

    // -----------------------------------------------------------------------
    // High-level SDMMC / SDIO command helpers
    // -----------------------------------------------------------------------

    /// CMD0 — GO_IDLE_STATE
    pub fn cmd_go_idle_state(&mut self) -> Result {
        self.send_command(&CmdInit {
            argument: 0,
            cmd_index: 0,
            cmd_type: CmdType::Normal,
            resp_type: RespType::None,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::None, SDMMC_CMD_TIMEOUT)
    }

    /// CMD2 — ALL_SEND_CID
    pub fn cmd_send_cid(&mut self) -> Result {
        self.send_command(&CmdInit {
            argument: 0,
            cmd_index: 2,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R2,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R2, SDMMC_CMD_TIMEOUT)
    }

    /// CMD3 — SET_RELATIVE_ADDR
    pub fn cmd_set_rel_addr(&mut self) -> Result {
        self.send_command(&CmdInit {
            argument: 0,
            cmd_index: 3,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R6,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R6, SDMMC_CMD_TIMEOUT)
    }

    /// CMD5 — IO_SEND_OP_COND (SDIO)
    pub fn cmd_send_op_cond(&mut self, ocr: u32) -> Result {
        self.send_command(&CmdInit {
            argument: ocr,
            cmd_index: 5,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R4,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R4, SDMMC_CMD_TIMEOUT)
    }

    /// CMD6 — SWITCH (check or switch function)
    pub fn cmd_switch(&mut self, argument: u32) -> Result {
        self.send_command(&CmdInit {
            argument,
            cmd_index: 6,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::WithData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD7 — SELECT/DESELECT_CARD.  Address 0 deselects (R1); non-zero selects (R1b).
    pub fn cmd_sel_desel(&mut self, addr: u32) -> Result {
        let resp = if addr != 0 {
            RespType::R1b
        } else {
            RespType::R1
        };
        self.send_command(&CmdInit {
            argument: addr,
            cmd_index: 7,
            cmd_type: CmdType::Normal,
            resp_type: resp,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(resp, SDMMC_DAT_TIMEOUT)
    }

    /// CMD8 — SEND_IF_COND
    pub fn cmd_oper_cond(&mut self) -> Result {
        const CHECK_PATTERN: u32 = 0x0000_01AA;
        self.send_command(&CmdInit {
            argument: CHECK_PATTERN,
            cmd_index: 8,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R7,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R7, SDMMC_CMD_TIMEOUT)
    }

    /// CMD9 — SEND_CSD
    pub fn cmd_send_csd(&mut self, argument: u32) -> Result {
        self.send_command(&CmdInit {
            argument,
            cmd_index: 9,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R2,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R2, SDMMC_CMD_TIMEOUT)
    }

    /// CMD12 — STOP_TRANSMISSION
    pub fn cmd_stop_transfer(&mut self) -> Result {
        self.send_command(&CmdInit {
            argument: 0,
            cmd_index: 12,
            cmd_type: CmdType::Abort,
            resp_type: RespType::R1b,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1b, SDMMC_STOP_TRANS_TIMEOUT)
    }

    /// CMD13 — SEND_STATUS
    pub fn cmd_send_status(&mut self, argument: u32) -> Result {
        self.send_command(&CmdInit {
            argument,
            cmd_index: 13,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD16 — SET_BLOCKLEN
    pub fn cmd_block_length(&mut self, block_size: u32) -> Result {
        self.send_command(&CmdInit {
            argument: block_size,
            cmd_index: 16,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD17 — READ_SINGLE_BLOCK
    pub fn cmd_read_single_block(&mut self, read_addr: u32) -> Result {
        self.send_command(&CmdInit {
            argument: read_addr,
            cmd_index: 17,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::WithData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD18 — READ_MULTIPLE_BLOCK
    pub fn cmd_read_multi_block(&mut self, read_addr: u32) -> Result {
        self.send_command(&CmdInit {
            argument: read_addr,
            cmd_index: 18,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::WithData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD24 — WRITE_BLOCK
    pub fn cmd_write_single_block(&mut self, write_addr: u32) -> Result {
        self.send_command(&CmdInit {
            argument: write_addr,
            cmd_index: 24,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::WithData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD25 — WRITE_MULTIPLE_BLOCK
    pub fn cmd_write_multi_block(&mut self, write_addr: u32) -> Result {
        self.send_command(&CmdInit {
            argument: write_addr,
            cmd_index: 25,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::WithData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD32 — SD_ERASE_WR_BLK_START
    pub fn cmd_sd_erase_start(&mut self, start_addr: u32) -> Result {
        self.send_command(&CmdInit {
            argument: start_addr,
            cmd_index: 32,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD33 — SD_ERASE_WR_BLK_END
    pub fn cmd_sd_erase_end(&mut self, end_addr: u32) -> Result {
        self.send_command(&CmdInit {
            argument: end_addr,
            cmd_index: 33,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// CMD38 — ERASE.  `block_cnt` scales the per-block erase timeout.
    pub fn cmd_erase(&mut self, block_cnt: u32) -> Result {
        self.send_command(&CmdInit {
            argument: 0,
            cmd_index: 38,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1b,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1b, SDMMC_ERASE_TIMEOUT.saturating_mul(block_cnt))
    }

    /// CMD52 — IO_RW_DIRECT (SDIO)
    pub fn cmd_rw_direct(&mut self, argument: u32) -> Result {
        self.send_command(&CmdInit {
            argument,
            cmd_index: 52,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R5,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R5, SDMMC_CMD_TIMEOUT)
    }

    /// CMD53 — IO_RW_EXTENDED (SDIO)
    pub fn cmd_rw_extended(&mut self, argument: u32) -> Result {
        self.send_command(&CmdInit {
            argument,
            cmd_index: 53,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R5,
            data_present: DataPresent::WithData,
        });
        self.wait_resp(RespType::R5, SDMMC_CMD_TIMEOUT)
    }

    /// CMD55 — APP_CMD (precedes all ACMDs)
    pub fn cmd_app_command(&mut self, argument: u32) -> Result {
        self.send_command(&CmdInit {
            argument,
            cmd_index: 55,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// ACMD6 — SET_BUS_WIDTH  (must be preceded by CMD55)
    pub fn cmd_bus_width(&mut self, bus_width: u32) -> Result {
        self.send_command(&CmdInit {
            argument: bus_width,
            cmd_index: 6,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// ACMD13 — SD_STATUS  (must be preceded by CMD55)
    pub fn cmd_status_register(&mut self) -> Result {
        self.send_command(&CmdInit {
            argument: 0,
            cmd_index: 13,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::WithData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// ACMD23 — SET_WR_BLK_ERASE_COUNT  (must be preceded by CMD55)
    pub fn cmd_set_wr_blk_erase_cnt(&mut self, block_cnt: u32) -> Result {
        self.send_command(&CmdInit {
            argument: block_cnt,
            cmd_index: 23,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }

    /// ACMD41 — SD_APP_OP_COND  (must be preceded by CMD55)
    pub fn cmd_app_oper_command(&mut self, argument: u32) -> Result {
        self.send_command(&CmdInit {
            argument,
            cmd_index: 41,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R3,
            data_present: DataPresent::NoData,
        });
        self.wait_resp(RespType::R3, SDMMC_CMD_TIMEOUT)
    }

    /// ACMD51 — SEND_SCR  (must be preceded by CMD55)
    pub fn cmd_send_scr(&mut self) -> Result {
        self.send_command(&CmdInit {
            argument: 0,
            cmd_index: 51,
            cmd_type: CmdType::Normal,
            resp_type: RespType::R1,
            data_present: DataPresent::WithData,
        });
        self.wait_resp(RespType::R1, SDMMC_CMD_TIMEOUT)
    }
}

// ---------------------------------------------------------------------------
// Private free functions
// ---------------------------------------------------------------------------

/// Wrapping-safe timeout check (mirrors C `is_timeout`).
#[inline]
fn is_timeout(start_us: u32, timeout_us: u32) -> bool {
    let now = crate::debugtimer::timestamp();
    if now < start_us {
        (u32::MAX - start_us).wrapping_add(now) > timeout_us
    } else {
        now.wrapping_sub(start_us) > timeout_us
    }
}

/// Check R1 card-status response for errors (mirrors `SDMMC_CheckErrResp1`).
pub fn check_err_resp1(resp: u32) -> Result {
    const ERR_BITS: u32 = 0xFDFF_E008;
    if resp & ERR_BITS == 0 {
        return Ok(());
    }
    if resp & 0x8000_0000 != 0 {
        return Err(SdError::AddrOutOfRange);
    }
    if resp & 0x4000_0000 != 0 {
        return Err(SdError::AddrMisaligned);
    }
    if resp & 0x2000_0000 != 0 {
        return Err(SdError::BlockLenErr);
    }
    if resp & 0x1000_0000 != 0 {
        return Err(SdError::EraseSeqErr);
    }
    if resp & 0x0800_0000 != 0 {
        return Err(SdError::BadEraseParam);
    }
    if resp & 0x0400_0000 != 0 {
        return Err(SdError::WriteProt);
    }
    if resp & 0x0100_0000 != 0 {
        return Err(SdError::LockUnlockFailed);
    }
    if resp & 0x0080_0000 != 0 {
        return Err(SdError::ComCrcFailed);
    }
    if resp & 0x0040_0000 != 0 {
        return Err(SdError::IllegalCmd);
    }
    if resp & 0x0020_0000 != 0 {
        return Err(SdError::CardEccFailed);
    }
    if resp & 0x0010_0000 != 0 {
        return Err(SdError::CcErr);
    }
    if resp & 0x0004_0000 != 0 {
        return Err(SdError::StreamReadUnderrun);
    }
    if resp & 0x0002_0000 != 0 {
        return Err(SdError::StreamWriteOverrun);
    }
    if resp & 0x0001_0000 != 0 {
        return Err(SdError::CidCsdOverwrite);
    }
    if resp & 0x0000_8000 != 0 {
        return Err(SdError::WpEraseSkip);
    }
    if resp & 0x0000_4000 != 0 {
        return Err(SdError::CardEccDisabled);
    }
    if resp & 0x0000_2000 != 0 {
        return Err(SdError::EraseReset);
    }
    if resp & 0x0000_0008 != 0 {
        return Err(SdError::AkeSeqErr);
    }
    Err(SdError::GeneralUnknown)
}

/// Check R5 response for errors (mirrors `SDMMC_CheckErrResp5`).
pub fn check_err_resp5(resp: u32) -> Result {
    const ERR_BITS: u32 = 0x0000_CB00;
    if resp & ERR_BITS == 0 {
        return Ok(());
    }
    if resp & 0x0000_8000 != 0 {
        return Err(SdError::CmdCrcFail);
    }
    if resp & 0x0000_4000 != 0 {
        return Err(SdError::IllegalCmd);
    }
    if resp & 0x0000_0100 != 0 {
        return Err(SdError::AddrOutOfRange);
    }
    Err(SdError::GeneralUnknown)
}

/// Check R6 response for errors (mirrors `SDMMC_CheckErrResp6`).
pub fn check_err_resp6(resp: u32) -> Result {
    if resp & (0x0000_2000 | 0x0000_4000 | 0x0000_8000) == 0 {
        return Ok(());
    }
    if resp & 0x0000_4000 != 0 {
        return Err(SdError::IllegalCmd);
    }
    if resp & 0x0000_8000 != 0 {
        return Err(SdError::ComCrcFailed);
    }
    Err(SdError::GeneralUnknown)
}
