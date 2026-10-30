//! RTL8721F I2C driver.
//!
//! All register access goes through the svd2rust PAC (`rtl8721f-pac`).
//! No raw bit-masking or address arithmetic is used anywhere in this file.

use core::future::Future;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU32, Ordering};
use core::task::{Context, Poll};
use embassy_sync::waitqueue::AtomicWaker;

use bitflags::bitflags;
use embedded_hal::delay::DelayNs;
use rtl8721f_pac::{I2c0Reg, I2c1Reg, i2c0_reg};

// ─────────────────────────────────────────────────────────────────────────────
// Typed register-block pointer constants
// ─────────────────────────────────────────────────────────────────────────────

pub type RegisterBlock = i2c0_reg::RegisterBlock;

pub const I2C0_PTR: *const RegisterBlock = I2c0Reg::PTR;
pub const I2C1_PTR: *const RegisterBlock = I2c1Reg::PTR as *const RegisterBlock;

// ─────────────────────────────────────────────────────────────────────────────
// Sealed trait (prevents external I2cInstance impls)
// ─────────────────────────────────────────────────────────────────────────────

mod sealed {
    pub trait Sealed {}
}

#[derive(Default, Debug)]
pub struct I2c0;
#[derive(Default, Debug)]
pub struct I2c1;

crate::any_peripheral! {
    pub peripheral AnyI2c<'d> {
        I2c0(I2c0),
        I2c1(I2c1),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// I2cInstance: compile-time per-peripheral constants
// ─────────────────────────────────────────────────────────────────────────────

/// Implemented by [`I2c0`] and [`I2c1`].
///
/// Carries all compile-time hardware constants for one I2C peripheral.
/// Use the typelevel tag (`type IrqType`) together with [`crate::bind_interrupts!`]
/// to register an interrupt handler and receive a [`crate::interrupt::Binding`] token.
pub trait I2cInstance: sealed::Sealed + Send + 'static + any::Degrade {
    /// Typelevel IRQ tag, used as `I` in [`crate::interrupt::Binding<I, H>`].
    type IrqType: crate::interrupt::Irq;
    /// Non-Secure register-block base address (derived from PAC `::PTR`).
    const PTR: *const RegisterBlock;
    /// Shared async waker/flag state for this instance (static, for interrupt handler).
    fn state_static() -> &'static I2cState;
    /// Shared async waker/flag state (instance, delegable by AnyI2c).
    fn state(&self) -> &'static I2cState {
        Self::state_static()
    }
    /// Register-block pointer (defaults to [`Self::PTR`]; overridden by Any types).
    fn ptr(&self) -> *const RegisterBlock {
        Self::PTR
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Speed-mode timing constants (nanoseconds, matching C source)
// ─────────────────────────────────────────────────────────────────────────────

const SS_MIN_SCL_HTIME: u32 = 4600; // SDK: ameba_i2c.h : I2C_SS_MIN_SCL_HTIME
const SS_MIN_SCL_LTIME: u32 = 5400; // SDK: ameba_i2c.h : I2C_SS_MIN_SCL_LTIME
const FS_MIN_SCL_HTIME: u32 = 800; // SDK: ameba_i2c.h : I2C_FS_MIN_SCL_HTIME
const FS_MIN_SCL_LTIME: u32 = 1700; // SDK: ameba_i2c.h : I2C_FS_MIN_SCL_LTIME
const HS_MIN_SCL_HTIME_100: u32 = 100; // SDK: ameba_i2c.h : I2C_HS_MIN_SCL_HTIME_100
const HS_MIN_SCL_LTIME_100: u32 = 194; // SDK: ameba_i2c.h : I2C_HS_MIN_SCL_LTIME_100

const TRX_BUFFER_DEPTH: u32 = 16; // SDK: ameba_i2c.h : I2C_TRX_BUFFER_DEPTH
const POLL_DELAY_US: u32 = 2; // SDK: ameba_i2c.h : I2C_POLL_DELAY_US
const POLL_TIMEOUT_MS: u32 = 1000; // SDK: ameba_i2c.h : I2C_POLL_TIMEOUT_MS

// ─────────────────────────────────────────────────────────────────────────────
// Public enumerations
// ─────────────────────────────────────────────────────────────────────────────

/// Master / Slave mode selection.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Master,
    Slave,
}

/// I2C address width.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AddrFormat {
    Bit7,
    Bit10,
}

/// I2C bus speed mode.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpeedMode {
    /// Standard speed (≤100 kHz)
    Ss,
    /// Fast speed (≤400 kHz)
    Fs,
    /// High speed (≤3.4 MHz) — not supported on I2C0 of all variants
    Hs,
}

impl SpeedMode {
    fn bits(self) -> u8 {
        match self {
            SpeedMode::Ss => 1,
            SpeedMode::Fs => 2,
            SpeedMode::Hs => 3,
        }
    }
}

/// DMA channel direction.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DmaChannel {
    Rx,
    Tx,
}

// ─────────────────────────────────────────────────────────────────────────────
// bitflags types
// ─────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// Bits in IC_STATUS used for polling.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct StatusFlags: u32 {
        const ACTIVITY      = 1 << 0;
        const TFNF          = 1 << 1;
        const TFE           = 1 << 2;
        const RFNE          = 1 << 3;
        const RFF           = 1 << 4;
        const MST_ACTIVITY  = 1 << 5;
        const SLV_ACTIVITY  = 1 << 6;
    }
}

bitflags! {
    /// Bits in IC_RAW_INTR_STAT.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct RawIntFlags: u32 {
        const RX_UNDER  = 1 << 0;
        const RX_OVER   = 1 << 1;
        const RX_FULL   = 1 << 2;
        const TX_OVER   = 1 << 3;
        const TX_EMPTY  = 1 << 4;
        const RD_REQ    = 1 << 5;
        const TX_ABRT   = 1 << 6;
        const RX_DONE   = 1 << 7;
        const ACTIVITY  = 1 << 8;
        const STOP_DET  = 1 << 9;
        const START_DET = 1 << 10;
        const GEN_CALL  = 1 << 11;
        const LP_WAKE_1 = 1 << 12;
        const LP_WAKE_2 = 1 << 13;
    }
}

bitflags! {
    /// Bits for IC_INTR_MASK / IC_INTR_STAT (masked view).
    /// The M_ prefix is dropped for ergonomics.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct IntMask: u32 {
        const RX_UNDER  = 1 << 0;
        const RX_OVER   = 1 << 1;
        const RX_FULL   = 1 << 2;
        const TX_OVER   = 1 << 3;
        const TX_EMPTY  = 1 << 4;
        const RD_REQ    = 1 << 5;
        const TX_ABRT   = 1 << 6;
        const RX_DONE   = 1 << 7;
        const ACTIVITY  = 1 << 8;
        const STOP_DET  = 1 << 9;
        const START_DET = 1 << 10;
        const GEN_CALL  = 1 << 11;
        const LP_WAKE_1 = 1 << 12;
        const LP_WAKE_2 = 1 << 13;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Saved-register snapshot for suspend / resume
// ─────────────────────────────────────────────────────────────────────────────

/// Caller-allocated save area for `I2c::suspend` / `I2c::resume`.
/// The driver owns no global state; the caller owns this struct.
#[derive(Clone, Copy, Default)]
pub struct I2cBackup {
    pub ic_con: u32,
    pub ic_tar: u32,
    pub ic_sar: u32,
    pub ic_hs_mar: u32,
    pub ic_ss_scl_hcnt: u32,
    pub ic_ss_scl_lcnt: u32,
    pub ic_fs_scl_hcnt: u32,
    pub ic_fs_scl_lcnt: u32,
    pub ic_hs_scl_hcnt: u32,
    pub ic_hs_scl_lcnt: u32,
    pub ic_intr_mask: u32,
    pub ic_rx_tl: u32,
    pub ic_tx_tl: u32,
    pub ic_sda_hold: u32,
    pub ic_slv_data_nack_only: u32,
    pub ic_dma_tdlr: u32,
    pub ic_dma_rdlr: u32,
    pub ic_sda_setup: u32,
    pub ic_ack_general_call: u32,
    pub ic_out_smp_dly: u32,
    pub ic_filter: u32,
    pub ic_sar2: u32,
}

// ─────────────────────────────────────────────────────────────────────────────
// Interrupt-mode semaphore abstraction
// ─────────────────────────────────────────────────────────────────────────────

/// Abstraction over the wait/signal semaphore pair used by interrupt-mode
/// transfers.  Implement this on your RTOS wrapper type; the concrete
/// `unsafe extern "C"` function pointers from the C API are never exposed.
///
/// `is_tx`: `true` when the caller is waiting on a TX event, `false` for RX.
pub trait I2cSemaphore {
    fn wait(&self, is_tx: bool);
    fn signal(&self, is_tx: bool);
}

// ─────────────────────────────────────────────────────────────────────────────
// Configuration / builder
// ─────────────────────────────────────────────────────────────────────────────

/// All parameters needed to initialise the I2C peripheral.
pub struct Config {
    pub role: Role,
    pub addr_format: AddrFormat,
    pub speed_mode: SpeedMode,
    /// Bus clock in kHz (e.g. 100, 400, 1000, 3400)
    pub bus_clk_khz: u32,
    /// IP peripheral clock in Hz (e.g. 40_000_000)
    pub ip_clk_hz: u32,
    /// Primary acknowledge address (7- or 10-bit value)
    pub ack_addr: u16,
    /// Secondary slave address (slave mode only)
    pub ack_addr2: u16,
    pub sda_hold_cycles: u32,
    pub sda_setup_cycles: u32,
    pub rx_fifo_threshold: u8,
    pub tx_fifo_threshold: u8,
    pub master_restart_en: bool,
    pub master_general_call: bool,
    pub master_start_byte: bool,
    pub slave_no_ack: bool,
    pub slave_ack_gc: bool,
    /// Combined filter register value: bits `[5:0]` = deg, bit `[8]` = sel
    pub filter: u16,
    pub dma_tx_req_level: u8,
    pub dma_rx_req_level: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            role: Role::Master,
            addr_format: AddrFormat::Bit7,
            speed_mode: SpeedMode::Ss,
            bus_clk_khz: 100,
            ip_clk_hz: 40_000_000, // SDK: ameba_i2c.h : I2C0_1_IPCLK (XTAL = 40 MHz)
            ack_addr: 0x11,        // SDK: ameba_i2c.c : I2C_StructInit : I2CAckAddr
            ack_addr2: 0x12,       // SDK: ameba_i2c.c : I2C_StructInit : I2CAckAddr1
            sda_hold_cycles: 2,    // SDK: ameba_i2c.c : I2C_StructInit : I2CSdaHd
            sda_setup_cycles: 3,   // SDK: ameba_i2c.c : I2C_StructInit : I2CSlvSetup
            rx_fifo_threshold: 0x00, // SDK: ameba_i2c.c : I2C_StructInit : I2CRXTL
            tx_fifo_threshold: 0x10, // SDK: ameba_i2c.c : I2C_StructInit : I2CTXTL
            // true: embedded-hal I2c::transaction contract requires SR (repeated start) between
            // Write→Read operations, not SP+ST. SDK default is DISABLE but modern I2C sensors
            // all support RESTART; only legacy slaves that explicitly reject it need false.
            // SDK: ameba_i2c.h : I2C_InitTypeDef.I2CMstReSTR
            master_restart_en: true,
            master_general_call: false,
            master_start_byte: false,
            slave_no_ack: false,
            slave_ack_gc: false,
            filter: 0x0101,         // SDK: ameba_i2c.c : I2C_StructInit : I2CFilter
            dma_tx_req_level: 0x09, // SDK: ameba_i2c.c : I2C_StructInit : I2CTxDMARqLv
            dma_rx_req_level: 0x03, // SDK: ameba_i2c.c : I2C_StructInit : I2CRxDMARqLv
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    Timeout,
    TxAbort,
}

// ─────────────────────────────────────────────────────────────────────────────
// Type-state markers
// ─────────────────────────────────────────────────────────────────────────────

pub struct Disabled;
pub struct Enabled;

// ─────────────────────────────────────────────────────────────────────────────
// I2c driver struct
// ─────────────────────────────────────────────────────────────────────────────

pub struct I2c<P: I2cInstance, State, Dm: crate::mode::DriverMode = crate::mode::Blocking> {
    peri: P,
    _state: core::marker::PhantomData<State>,
    _dm: core::marker::PhantomData<Dm>,
}

// SAFETY: embedded single-CPU target; `I2c<P, State, Dm>` has exclusive ownership of its peripheral.
unsafe impl<P: I2cInstance, S, Dm: crate::mode::DriverMode> Send for I2c<P, S, Dm> {}

// ─────────────────────────────────────────────────────────────────────────────
// Private helpers
// ─────────────────────────────────────────────────────────────────────────────

impl<P: I2cInstance, State, Dm: crate::mode::DriverMode> I2c<P, State, Dm> {
    #[inline(always)]
    fn regs(&self) -> &RegisterBlock {
        unsafe { &*self.peri.ptr() }
    }

    /// Clear all pending interrupts (reads IC_CLR_INTR — RW1C, PAC models as read-only).
    fn clear_all_int_inner(&self) {
        let _ = self.regs().ic_clr_intr().read();
    }

    /// Returns the raw interrupt status word.
    fn raw_int_inner(&self) -> RawIntFlags {
        let bits = self.regs().ic_raw_intr_stat().read().bits();
        RawIntFlags::from_bits_truncate(bits)
    }

    /// Poll until either any status flag in `want_status` is set,
    /// or any raw interrupt flag in `want_raw` is set.
    /// Returns `Ok(txflr)` where `txflr` is the TX FIFO level captured on abort/timeout.
    fn poll_flag(
        &self,
        want_status: StatusFlags,
        want_raw: RawIntFlags,
        timeout_ms: u32,
    ) -> Result<u32, Error> {
        let max_iters = timeout_ms * 1000 / POLL_DELAY_US;
        let mut iters = max_iters;

        loop {
            let status = self.regs().ic_status().read();
            let status_bits = StatusFlags::from_bits_truncate(status.bits());
            let raw = self.raw_int_inner();

            // TX_ABRT takes priority: NACK causes FIFO flush (TFE=1), so check
            // abort before any status/raw flag to avoid masking the error.
            if raw.contains(RawIntFlags::TX_ABRT) {
                self.clear_all_int_inner();
                return Err(Error::TxAbort);
            }

            if (!want_status.is_empty() && status_bits.intersects(want_status))
                || (!want_raw.is_empty() && raw.intersects(want_raw))
            {
                return Ok(0);
            }

            crate::delay::Delay.delay_us(POLL_DELAY_US); // SDK: ameba_i2c.h : I2C_POLL_DELAY_US

            if iters == 0 {
                return Err(Error::Timeout);
            }
            iters -= 1;
        }
    }

    /// Compute and write SS/FS/HS SCL counts.
    fn set_speed(&self, speed: SpeedMode, bus_khz: u32, ip_hz: u32) {
        let ip_mhz = ip_hz / 1_000_000;
        let regs = self.regs();

        match speed {
            SpeedMode::Ss => {
                let period_ns = 1_000_000 / bus_khz;
                let total = SS_MIN_SCL_HTIME + SS_MIN_SCL_LTIME;
                let htime = period_ns * SS_MIN_SCL_HTIME / total;
                let ltime = period_ns * SS_MIN_SCL_LTIME / total;
                let hcnt = (htime * ip_mhz + 500) / 1000;
                let lcnt = (ltime * ip_mhz + 500) / 1000;
                regs.ic_ss_scl_hcnt()
                    .write(|w| unsafe { w.ic_ss_scl_hcnt().bits(hcnt as u16) });
                regs.ic_ss_scl_lcnt()
                    .write(|w| unsafe { w.ic_ss_scl_lcnt().bits(lcnt as u16) });
                self.clamp_filter(htime, ip_mhz);
            }
            SpeedMode::Fs => {
                let period_ns = 1_000_000 / bus_khz;
                let total = FS_MIN_SCL_HTIME + FS_MIN_SCL_LTIME;
                let htime = period_ns * FS_MIN_SCL_HTIME / total;
                let ltime = period_ns * FS_MIN_SCL_LTIME / total;
                let hcnt = (htime * ip_mhz + 500) / 1000;
                let lcnt = (ltime * ip_mhz + 500) / 1000;
                regs.ic_fs_scl_hcnt()
                    .write(|w| unsafe { w.ic_fs_scl_hcnt().bits(hcnt as u16) });
                regs.ic_fs_scl_lcnt()
                    .write(|w| unsafe { w.ic_fs_scl_lcnt().bits(lcnt as u16) });
                self.clamp_filter(htime, ip_mhz);
            }
            SpeedMode::Hs => {
                // Fast-mode master code preamble (at 400 kHz)
                let period_fs = 1_000_000 / 400u32;
                let total_fs = FS_MIN_SCL_HTIME + FS_MIN_SCL_LTIME;
                let htime_fs = period_fs * FS_MIN_SCL_HTIME / total_fs;
                let ltime_fs = period_fs * FS_MIN_SCL_LTIME / total_fs;
                let hcnt_fs = (htime_fs * ip_mhz + 500) / 1000;
                let lcnt_fs = (ltime_fs * ip_mhz + 500) / 1000;
                regs.ic_fs_scl_hcnt()
                    .write(|w| unsafe { w.ic_fs_scl_hcnt().bits(hcnt_fs as u16) });
                regs.ic_fs_scl_lcnt()
                    .write(|w| unsafe { w.ic_fs_scl_lcnt().bits(lcnt_fs as u16) });

                // High-speed payload
                let period_hs = 1_000_000 / bus_khz;
                let total_hs = HS_MIN_SCL_HTIME_100 + HS_MIN_SCL_LTIME_100;
                let htime = period_hs * HS_MIN_SCL_HTIME_100 / total_hs;
                let ltime = period_hs * HS_MIN_SCL_LTIME_100 / total_hs;
                let hcnt = (htime * ip_mhz + 500) / 1000;
                let lcnt = (ltime * ip_mhz + 500) / 1000;
                regs.ic_hs_scl_hcnt()
                    .write(|w| unsafe { w.ic_hs_scl_hcnt().bits(hcnt as u16) });
                regs.ic_hs_scl_lcnt()
                    .write(|w| unsafe { w.ic_hs_scl_lcnt().bits(lcnt as u16) });
                self.clamp_filter(htime, ip_mhz);
            }
        }
    }

    /// If the filter degree exceeds tHigh/4*2/clk_ns, clamp it. If tHigh ≤ 200 ns,
    /// disable the filter entirely (IC_FILTER = 0).
    fn clamp_filter(&self, htime_ns: u32, ip_mhz: u32) {
        let regs = self.regs();
        if htime_ns <= 200 {
            // Filter cannot be used at this speed.
            unsafe { regs.ic_filter().write_with_zero(|w| w) };
            return;
        }
        let limit = htime_ns * ip_mhz / 2000;
        let current_deg = regs.ic_filter().read().ic_dig_fltr_deg().bits() as u32;
        if current_deg > limit {
            regs.ic_filter()
                .modify(|_, w| unsafe { w.ic_dig_fltr_deg().bits(limit as u8) });
        }
    }

    /// Disable interrupt-driven operation: mask all peripheral-level I2C
    /// interrupts, then mask the shared NVIC line (order must not reverse).
    /// Called by both `Drop` (async only) and `into_blocking`.
    fn disable_interrupts(&self) {
        let regs = unsafe { &*self.peri.ptr() };
        unsafe { regs.ic_intr_mask().write_with_zero(|w| w) };
        crate::interrupt::disable::<P::IrqType>();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Construction (state-independent)
// ─────────────────────────────────────────────────────────────────────────────

impl<P: I2cInstance + Default> I2c<P, Disabled, crate::mode::Blocking> {
    /// Construct an `I2c` handle for peripheral `P`.
    ///
    /// # Safety
    /// The caller must ensure no other `I2c<P, _>` instance for the same peripheral
    /// is alive concurrently.  Hardware must not be accessed concurrently.
    pub unsafe fn steal() -> Self
    where
        P: Default,
    {
        I2c {
            peri: P::default(),
            _state: core::marker::PhantomData::<Disabled>,
            _dm: core::marker::PhantomData,
        }
    }

    /// Apply `config`. The peripheral remains disabled; call `.enable()` when ready.
    pub fn init(self, mut config: Config) -> I2c<P, Disabled> {
        let regs = self.regs();

        // Disable peripheral first (must be disabled to write most config regs)
        regs.ic_enable().modify(|_, w| w.enable().clear_bit());

        // Validate and write filter
        {
            let max_filter: u16 = (1 << 8) | 0x3f; // sel bit | max deg
            if config.filter > max_filter {
                config.filter = max_filter;
            }
            let deg = (config.filter & 0x3f) as u8;
            let sel = (config.filter >> 8) != 0;
            regs.ic_filter().write(|w| unsafe {
                w.ic_dig_fltr_deg().bits(deg);
                w.ic_dig_fltr_sel().bit(sel)
            });
        }

        if config.role == Role::Master {
            // RESTART is mandatory for 10-bit addressing, HS mode, or START_BYTE
            if config.addr_format == AddrFormat::Bit10
                || config.speed_mode == SpeedMode::Hs
                || config.master_start_byte
            {
                config.master_restart_en = true;
            }

            regs.ic_con().write(|w| unsafe {
                w.master_mode().set_bit();
                w.speed().bits(config.speed_mode.bits());
                w.ic_restatrt_en().bit(config.master_restart_en);
                w.ic_slave_disable_0().set_bit();
                w.ic_slave_disable_1().set_bit()
            });

            let special = config.master_general_call || config.master_start_byte;
            let addr_10bit = config.addr_format == AddrFormat::Bit10;
            regs.ic_tar().write(|w| unsafe {
                w.ic_tar().bits(config.ack_addr & 0x3ff);
                w.special().bit(special);
                w.gc_or_start().bit(config.master_start_byte);
                w.ic_10bitaddr_master().bit(addr_10bit)
            });

            // Speed counts
            self.set_speed(config.speed_mode, config.bus_clk_khz, config.ip_clk_hz);
        } else {
            // Slave mode
            let addr_10bit = config.addr_format == AddrFormat::Bit10;
            regs.ic_con().write(|w| unsafe {
                // master_mode = 0 → bits 0, 6, 7 clear
                w.master_mode().clear_bit();
                w.speed().bits(config.speed_mode.bits());
                w.ic_10bitaddr_slave().bit(addr_10bit);
                w.ic_slave_disable_0().clear_bit();
                w.ic_slave_disable_1().clear_bit()
            });
            regs.ic_sar()
                .write(|w| unsafe { w.ic_sar().bits(config.ack_addr & 0x3ff) });
            // IC_SAR2 is slave2 address — 7-bit only
            regs.ic_sar2()
                .write(|w| unsafe { w.ic_sar2().bits((config.ack_addr2 & 0x7f) as u8) });
            regs.ic_slv_data_nack_only()
                .write(|w| w.nack().bit(config.slave_no_ack));
            regs.ic_ack_general_call()
                .write(|w| w.ack_gen_call().bit(config.slave_ack_gc));
            regs.ic_sda_setup()
                .write(|w| unsafe { w.ic_sda_setup().bits(config.sda_setup_cycles as u8) });
        }

        // SDA hold time
        regs.ic_sda_hold()
            .write(|w| unsafe { w.ic_sda_hold().bits(config.sda_hold_cycles as u16) });

        // FIFO thresholds
        regs.ic_tx_tl()
            .write(|w| unsafe { w.ic_tx_0_tl().bits(config.tx_fifo_threshold) });
        regs.ic_rx_tl()
            .write(|w| unsafe { w.ic_rx_0_tl().bits(config.rx_fifo_threshold) });

        // DMA watermarks (dmatdl/dmardl are 4-bit fields)
        regs.ic_dma_tdlr()
            .write(|w| unsafe { w.dmatdl().bits(config.dma_tx_req_level & 0xf) });
        regs.ic_dma_rdlr()
            .write(|w| unsafe { w.dmardl().bits(config.dma_rx_req_level & 0xf) });

        // Clear + mask all interrupts
        let _ = regs.ic_clr_intr().read();
        unsafe { regs.ic_intr_mask().write_with_zero(|w| w) };

        I2c {
            peri: P::default(),
            _state: core::marker::PhantomData::<Disabled>,
            _dm: core::marker::PhantomData,
        }
    }

    /// Enable the peripheral; consumes `Disabled`, produces `Enabled`.
    pub fn enable(self) -> I2c<P, Enabled> {
        self.regs().ic_enable().modify(|_, w| w.enable().set_bit());
        I2c {
            peri: P::default(),
            _state: core::marker::PhantomData::<Enabled>,
            _dm: core::marker::PhantomData,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Enabled-state API
// ─────────────────────────────────────────────────────────────────────────────

impl<P: I2cInstance> I2c<P, Enabled, crate::mode::Blocking> {
    /// Disable the peripheral.  Consumes `Enabled`, produces `Disabled`.
    pub fn disable(self) -> I2c<P, Disabled> {
        self.regs()
            .ic_enable()
            .modify(|_, w| w.enable().clear_bit());
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        I2c {
            peri,
            _state: core::marker::PhantomData,
            _dm: core::marker::PhantomData,
        }
    }

    // ── Address helpers ────────────────────────────────────────────────────

    /// Set target / slave address (updates both IC_TAR and IC_SAR).
    pub fn set_slave_address(&self, addr: u16) {
        self.regs()
            .ic_tar()
            .modify(|_, w| unsafe { w.ic_tar().bits(addr & 0x3ff) });
        self.regs()
            .ic_sar()
            .modify(|_, w| unsafe { w.ic_sar().bits(addr & 0x3ff) });
    }

    /// Change the address format (7-bit / 10-bit) on the fly.
    pub fn set_addr_format(&self, fmt: AddrFormat) {
        let is_10bit = fmt == AddrFormat::Bit10;
        self.regs()
            .ic_tar()
            .modify(|_, w| w.ic_10bitaddr_master().bit(is_10bit));
        if is_10bit {
            self.regs()
                .ic_con()
                .modify(|_, w| w.ic_restatrt_en().set_bit());
        }
    }

    // ── Status / interrupt helpers ─────────────────────────────────────────

    /// Returns `true` if any bit in `flag` is set in IC_STATUS.
    pub fn check_status_flag(&self, flag: StatusFlags) -> bool {
        let v = StatusFlags::from_bits_truncate(self.regs().ic_status().read().bits());
        v.intersects(flag)
    }

    /// Enable or disable the given interrupt mask bits.
    pub fn int_config(&self, mask: IntMask, enable: bool) {
        if enable {
            // Must read-modify-write: we are OR-ing bits in
            let cur = self.regs().ic_intr_mask().read().bits();
            let new = cur | mask.bits();
            self.regs().ic_intr_mask().write(|w| unsafe { w.bits(new) });
        } else {
            let cur = self.regs().ic_intr_mask().read().bits();
            let new = cur & !mask.bits();
            self.regs().ic_intr_mask().write(|w| unsafe { w.bits(new) });
        }
    }

    /// Clear a specific raw interrupt bit by reading its dedicated clear register.
    pub fn clear_int(&self, bit: RawIntFlags) {
        let regs = self.regs();
        // Each interrupt has its own read-to-clear register (PAC models as read-only).
        if bit.contains(RawIntFlags::LP_WAKE_2) || bit.contains(RawIntFlags::LP_WAKE_1) {
            let _ = regs.ic_clr_addr_match().read();
        }
        if bit.contains(RawIntFlags::TX_ABRT) {
            let _ = regs.ic_clr_tx_abrt().read();
        }
        if bit.contains(RawIntFlags::RD_REQ) {
            let _ = regs.ic_clr_rd_req().read();
        }
        if bit.contains(RawIntFlags::RX_DONE) {
            let _ = regs.ic_clr_rx_done().read();
        }
        if bit.contains(RawIntFlags::ACTIVITY) {
            let _ = regs.ic_clr_activity().read();
        }
        if bit.contains(RawIntFlags::STOP_DET) {
            let _ = regs.ic_clr_stop_det().read();
        }
        if bit.contains(RawIntFlags::START_DET) {
            let _ = regs.ic_clr_start_det().read();
        }
        if bit.contains(RawIntFlags::GEN_CALL) {
            let _ = regs.ic_clr_gen_call().read();
        }
        if bit.contains(RawIntFlags::TX_OVER) {
            let _ = regs.ic_clr_tx_over().read();
        }
        if bit.contains(RawIntFlags::RX_OVER) {
            let _ = regs.ic_clr_rx_over().read();
        }
        if bit.contains(RawIntFlags::RX_UNDER) {
            let _ = regs.ic_clr_rx_under().read();
        }
        // TX_EMPTY (bit 4) and RX_FULL (bit 2) are auto-cleared by hardware; no action needed.
    }

    /// Clear all interrupts via IC_CLR_INTR.
    pub fn clear_all_int(&self) {
        self.clear_all_int_inner();
    }

    /// Read IC_RAW_INTR_STAT.
    pub fn raw_int(&self) -> RawIntFlags {
        self.raw_int_inner()
    }

    /// Read IC_INTR_STAT (masked view).
    pub fn masked_int(&self) -> IntMask {
        IntMask::from_bits_truncate(self.regs().ic_intr_stat().read().bits())
    }

    // ── DMA helpers ────────────────────────────────────────────────────────

    /// Enable or disable the TX or RX DMA channel.
    pub fn dma_control(&self, channel: DmaChannel, enable: bool) {
        match channel {
            DmaChannel::Rx => {
                self.regs().ic_dma_cr().modify(|_, w| w.rdmae().bit(enable));
            }
            DmaChannel::Tx => {
                self.regs().ic_dma_cr().modify(|_, w| w.tdmae().bit(enable));
            }
        }
    }

    /// Configure DMA register mode (mode 1: register-based, IC_DMA_MODE = 1).
    /// `cmd` is written to IC_DMA_CMD, `len` to IC_DMA_DATA_LEN.
    pub fn dma_mode1_config(&self, cmd: u32, len: u32) {
        debug_assert!(len <= 0xFFFF, "DMA data length out of range");
        // IC_DMA_MODE value 1 = DMA with control register
        self.regs()
            .ic_dma_mode()
            .write(|w| unsafe { w.dma_mode().bits(1) });
        self.regs()
            .ic_dma_data_len()
            .write(|w| unsafe { w.dma_dat_len().bits(len as u16) });
        self.regs().ic_dma_cmd().write(|w| unsafe { w.bits(cmd) });
    }

    /// Configure DMA descriptor mode (mode 2, IC_DMA_MODE = 2).
    /// `cmd` `BIT[0]` enables DMA mode.
    pub fn dma_mode2_config(&self, cmd: u32) {
        // IC_DMA_MODE value 2 = DMA with transfer descriptor
        self.regs()
            .ic_dma_mode()
            .write(|w| unsafe { w.dma_mode().bits(2) });
        self.regs().ic_dma_cmd().write(|w| unsafe { w.bits(cmd) });
    }

    // ── Single-byte master sends ───────────────────────────────────────────

    /// Queue a single write byte to the TX FIFO. Sets stop/restart as requested.
    pub fn master_send_byte(&self, data: u8, stop: bool, restart: bool) {
        self.regs().ic_data_cmd().write(|w| unsafe {
            w.ic_data().bits(data);
            w.cmd_stop().bit(stop);
            w.cmd_restart().bit(restart)
        });
    }

    /// Queue a single null-data byte (bit 11 set) for slave-address probing.
    pub fn master_send_null(&self, data: u8, stop: bool, restart: bool) {
        self.regs().ic_data_cmd().write(|w| unsafe {
            w.ic_data().bits(data);
            w.null_data().set_bit();
            w.cmd_stop().bit(stop);
            w.cmd_restart().bit(restart)
        });
    }

    // ── Slave single-byte ─────────────────────────────────────────────────

    /// Write a single byte to the TX FIFO in slave mode.
    pub fn slave_send(&self, data: u8) {
        self.regs()
            .ic_data_cmd()
            .write(|w| unsafe { w.ic_data().bits(data) });
    }

    /// Read a single byte from IC_DATA_CMD (RX FIFO).
    pub fn receive_byte(&self) -> u8 {
        self.regs().ic_data_cmd().read().ic_data().bits()
    }

    // ── Polled bulk transfers ──────────────────────────────────────────────

    /// Master transmit: send `buf`, generate STOP after last byte.
    /// Returns the number of bytes successfully sent.
    pub fn master_write(&self, buf: &[u8]) -> usize {
        let len = buf.len();
        let mut cnt = 0usize;

        for (i, &byte) in buf.iter().enumerate() {
            match self.poll_flag(StatusFlags::TFNF, RawIntFlags::empty(), POLL_TIMEOUT_MS) {
                Err(Error::TxAbort) => {
                    let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                    return cnt.saturating_sub(txflr);
                }
                Err(Error::Timeout) => {
                    let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                    return cnt.saturating_sub(txflr);
                }
                Ok(_) => {}
            }
            let stop = i == len - 1;
            self.regs().ic_data_cmd().write(|w| unsafe {
                w.ic_data().bits(byte);
                w.cmd_stop().bit(stop)
            });
            cnt += 1;
        }

        // Wait for TX FIFO empty (matches SDK ameba_i2c.c : I2C_MasterWrite, which
        // completes on I2C_BIT_TFE). TX_ABRT is checked every iteration inside
        // poll_flag with top priority, so a NACK still aborts before TFE masks it.
        // On TX_ABRT the hardware flushes the FIFO (txflr→0), so cnt-txflr would
        // wrongly equal cnt; return 0 explicitly to signal total failure.
        match self.poll_flag(StatusFlags::TFE, RawIntFlags::empty(), POLL_TIMEOUT_MS) {
            Err(Error::TxAbort) => 0,
            Err(_) => {
                let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                cnt.saturating_sub(txflr)
            }
            Ok(_) => cnt,
        }
    }

    /// Master transmit with per-byte custom timeout.
    pub fn master_write_timeout(&self, buf: &[u8], timeout_ms: u32) -> usize {
        let len = buf.len();
        let mut cnt = 0usize;

        for (i, &byte) in buf.iter().enumerate() {
            if self
                .poll_flag(StatusFlags::TFNF, RawIntFlags::empty(), timeout_ms)
                .is_err()
            {
                let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                return cnt.saturating_sub(txflr);
            }
            let stop = i == len - 1;
            self.regs().ic_data_cmd().write(|w| unsafe {
                w.ic_data().bits(byte);
                w.cmd_stop().bit(stop)
            });
            cnt += 1;
        }

        // Same completion criterion as master_write (SDK I2C_MasterWrite: TFE).
        // TX_ABRT flushes the FIFO (txflr→0), so return 0 explicitly rather than
        // letting cnt-txflr wrongly report full success.
        match self.poll_flag(StatusFlags::TFE, RawIntFlags::empty(), timeout_ms) {
            Err(Error::TxAbort) => 0,
            Err(_) => {
                let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                cnt.saturating_sub(txflr)
            }
            Ok(_) => cnt,
        }
    }

    /// Master receive (in-house IP): send read commands then drain RX FIFO.
    pub fn master_read(&self, buf: &mut [u8]) -> usize {
        let len = buf.len();
        let mut rcvd = 0usize;

        for i in 0..len {
            let stop = i == len - 1;
            self.regs().ic_data_cmd().write(|w| {
                w.cmd_rw().set_bit();
                w.cmd_stop().bit(stop)
            });

            match self.poll_flag(StatusFlags::RFNE, RawIntFlags::empty(), POLL_TIMEOUT_MS) {
                Err(Error::Timeout) => {
                    // Skip — skip_cnt logic from C source
                    continue;
                }
                Err(Error::TxAbort) => return rcvd,
                Ok(_) => {
                    buf[rcvd] = self.regs().ic_data_cmd().read().ic_data().bits();
                    rcvd += 1;
                }
            }
        }
        rcvd
    }

    /// Master receive (DW IP): interleaved command/data as described in C source.
    pub fn master_read_dw(&self, buf: &mut [u8]) -> usize {
        let len = buf.len();

        for i in 0..len {
            let stop = i == len - 1;
            self.regs().ic_data_cmd().write(|w| {
                w.cmd_rw().set_bit();
                w.cmd_stop().bit(stop)
            });
            if i > 0 {
                if self
                    .poll_flag(StatusFlags::RFNE, RawIntFlags::empty(), POLL_TIMEOUT_MS)
                    .is_err()
                {
                    return i - 1;
                }
                buf[i - 1] = self.regs().ic_data_cmd().read().ic_data().bits();
            }
        }

        // Receive last byte
        if self
            .poll_flag(StatusFlags::RFNE, RawIntFlags::empty(), POLL_TIMEOUT_MS)
            .is_err()
        {
            return len - 1;
        }
        buf[len - 1] = self.regs().ic_data_cmd().read().ic_data().bits();
        len
    }

    /// Master receive with per-byte custom timeout.
    pub fn master_read_timeout(&self, buf: &mut [u8], timeout_ms: u32) -> usize {
        let len = buf.len();

        for (i, byte) in buf.iter_mut().enumerate() {
            let stop = i == len - 1;
            self.regs().ic_data_cmd().write(|w| {
                w.cmd_rw().set_bit();
                w.cmd_stop().bit(stop)
            });
            if self
                .poll_flag(StatusFlags::RFNE, RawIntFlags::empty(), timeout_ms)
                .is_err()
            {
                return i;
            }
            *byte = self.regs().ic_data_cmd().read().ic_data().bits();
        }
        len
    }

    /// Write then read (repeated-start): writes `wr_buf`, issues restart, reads `rd_buf`.
    pub fn master_repeat_read(&self, wr_buf: &[u8], rd_buf: &mut [u8]) -> usize {
        let wlen = wr_buf.len();

        // Write phase: last byte gets RESTART instead of STOP
        for (i, &byte) in wr_buf.iter().enumerate() {
            if self
                .poll_flag(StatusFlags::TFNF, RawIntFlags::empty(), POLL_TIMEOUT_MS)
                .is_err()
            {
                return 0;
            }
            let restart = i == wlen - 1;
            self.regs().ic_data_cmd().write(|w| unsafe {
                w.ic_data().bits(byte);
                w.cmd_restart().bit(restart)
            });
        }

        // Wait TX empty
        if self
            .poll_flag(StatusFlags::TFE, RawIntFlags::empty(), POLL_TIMEOUT_MS)
            .is_err()
        {
            return 0;
        }

        self.master_read(rd_buf)
    }

    /// Probe for a slave device by sending a null-data byte and checking for NACK.
    /// Returns `Ok(())` if slave acknowledged, `Err(Error::TxAbort)` if NACK.
    pub fn master_probe(&self, address: u8, timeout_ms: u32) -> Result<(), Error> {
        let shifted = address << 1;
        self.master_send_null(shifted, true, false);

        // Poll until STOP_DET (ACK) or TX_ABRT (NACK); poll_flag clears TX_ABRT on error.
        // SDK: ameba_i2c.c : I2C_MasterSendNullData_TimeOut
        match self.poll_flag(StatusFlags::empty(), RawIntFlags::STOP_DET, timeout_ms) {
            Ok(_) => Ok(()),
            Err(Error::TxAbort) => {
                let regs = self.regs();
                regs.ic_enable().modify(|_, w| w.enable().clear_bit());
                // Wait for controller to fully disable before re-enabling.
                // SDK: ameba_i2c.c : I2C_MasterSendNullData_TimeOut
                while regs.ic_enable_status().read().ic_en().bit_is_set() {}
                regs.ic_enable().modify(|_, w| w.enable().set_bit());
                Err(Error::TxAbort)
            }
            Err(e) => Err(e),
        }
    }

    // ── Slave polled transfers ─────────────────────────────────────────────

    /// Slave transmit: wait for RD_REQ then fill TX FIFO.
    pub fn slave_write(&self, buf: &[u8]) -> usize {
        let mut cnt = 0usize;

        if self.raw_int().contains(RawIntFlags::RX_DONE) {
            self.clear_int(RawIntFlags::RX_DONE);
        }

        for &byte in buf.iter() {
            // Wait for read-request or early RX_DONE
            if self
                .poll_flag(
                    StatusFlags::empty(),
                    RawIntFlags::RD_REQ | RawIntFlags::RX_DONE,
                    POLL_TIMEOUT_MS,
                )
                .is_err()
            {
                let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                return cnt.saturating_sub(txflr);
            }
            self.clear_int(RawIntFlags::RD_REQ);

            // Wait for TXFIFO space, bail on early RX_DONE
            if self
                .poll_flag(StatusFlags::TFNF, RawIntFlags::RX_DONE, POLL_TIMEOUT_MS)
                .is_err()
            {
                let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                return cnt.saturating_sub(txflr);
            }
            if self.raw_int().contains(RawIntFlags::RX_DONE) {
                let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                return cnt.saturating_sub(txflr);
            }

            self.regs()
                .ic_data_cmd()
                .write(|w| unsafe { w.ic_data().bits(byte) });
            cnt += 1;
        }

        // Drain TX FIFO
        match self.poll_flag(StatusFlags::TFE, RawIntFlags::empty(), POLL_TIMEOUT_MS) {
            Err(_) => {
                let txflr = self.regs().ic_txflr().read().txflr().bits() as usize;
                cnt.saturating_sub(txflr)
            }
            Ok(_) => {
                self.clear_all_int_inner();
                cnt
            }
        }
    }

    /// Slave receive: read `buf.len()` bytes from the RX FIFO.
    pub fn slave_read(&self, buf: &mut [u8]) -> usize {
        for (i, slot) in buf.iter_mut().enumerate() {
            if self
                .poll_flag(
                    StatusFlags::RFNE | StatusFlags::RFF,
                    RawIntFlags::empty(),
                    POLL_TIMEOUT_MS,
                )
                .is_err()
            {
                return i;
            }
            *slot = self.regs().ic_data_cmd().read().ic_data().bits();
        }
        buf.len()
    }

    /// Slave receive with custom per-byte timeout.
    pub fn slave_read_timeout(&self, buf: &mut [u8], timeout_ms: u32) -> usize {
        for (i, slot) in buf.iter_mut().enumerate() {
            if self
                .poll_flag(
                    StatusFlags::RFNE | StatusFlags::RFF,
                    RawIntFlags::empty(),
                    timeout_ms,
                )
                .is_err()
            {
                return i;
            }
            *slot = self.regs().ic_data_cmd().read().ic_data().bits();
        }
        buf.len()
    }

    // ── Interrupt-mode transfers ───────────────────────────────────────────

    /// Master transmit in interrupt mode. Uses `sem` to gate on TX-empty/abort.
    /// Returns the count of bytes actually transmitted.
    pub fn master_write_int(&self, sem: &dyn I2cSemaphore, buf: &[u8]) -> usize {
        let mut remaining = buf.len();
        let mut ptr = 0usize;

        while remaining > 0 {
            let txflr = self.regs().ic_txflr().read().txflr().bits() as u32;
            if txflr < TRX_BUFFER_DEPTH {
                let stop = remaining == 1;
                self.regs().ic_data_cmd().write(|w| unsafe {
                    w.ic_data().bits(buf[ptr]);
                    w.cmd_stop().bit(stop)
                });
                ptr += 1;
                remaining -= 1;
            } else {
                self.int_config(IntMask::TX_ABRT | IntMask::TX_EMPTY, true);
                sem.wait(true);
            }

            if self.raw_int().contains(RawIntFlags::TX_ABRT) {
                self.clear_all_int_inner();
                return ptr;
            }
        }
        ptr
    }

    /// Master receive in interrupt mode. Uses `sem` to gate on RX-full/over/abort.
    /// Returns the count of bytes received.
    pub fn master_read_int(&self, sem: &dyn I2cSemaphore, buf: &mut [u8]) -> usize {
        let mut left = buf.len();
        let mut rcvd = 0usize;

        while left > 0 {
            // Issue up to TRX_BUFFER_DEPTH read commands
            let mut trigger = 0u32;
            loop {
                if left == 0 || trigger >= TRX_BUFFER_DEPTH {
                    break;
                }
                let stop = left == 1;
                self.regs().ic_data_cmd().write(|w| {
                    w.cmd_rw().set_bit();
                    w.cmd_stop().bit(stop)
                });
                left -= 1;
                trigger += 1;
            }

            // Adjust RX threshold to match the number of commands issued
            let thresh = if trigger < TRX_BUFFER_DEPTH {
                trigger.saturating_sub(1)
            } else {
                TRX_BUFFER_DEPTH - 1
            };
            self.regs()
                .ic_rx_tl()
                .write(|w| unsafe { w.ic_rx_0_tl().bits(thresh as u8) });

            self.int_config(IntMask::TX_ABRT | IntMask::RX_FULL | IntMask::RX_OVER, true);
            sem.wait(false);

            // Drain RX FIFO
            while self.regs().ic_rxflr().read().rxflr().bits() > 0 {
                buf[rcvd] = self.regs().ic_data_cmd().read().ic_data().bits();
                rcvd += 1;
            }

            if self.raw_int().contains(RawIntFlags::TX_ABRT) {
                self.clear_all_int_inner();
                return rcvd;
            }
        }
        rcvd
    }

    /// Default ISR handler: dispatches TX-empty/abort and RX-full/over events
    /// to the `sem` callbacks.  Matches the C `I2C_ISRHandle` weak function.
    pub fn isr_handle(&self, sem: &dyn I2cSemaphore) {
        let intr = self.masked_int();

        if intr.intersects(IntMask::TX_ABRT | IntMask::TX_EMPTY) {
            let to_clear = intr & (IntMask::TX_ABRT | IntMask::TX_EMPTY);
            self.clear_int(RawIntFlags::from_bits_truncate(to_clear.bits()));
            self.int_config(IntMask::TX_ABRT | IntMask::TX_EMPTY, false);
            sem.signal(true);
        }

        if intr.intersects(IntMask::TX_ABRT | IntMask::RX_OVER | IntMask::RX_FULL) {
            let to_clear = intr & (IntMask::TX_ABRT | IntMask::RX_OVER | IntMask::RX_FULL);
            self.clear_int(RawIntFlags::from_bits_truncate(to_clear.bits()));
            self.int_config(
                IntMask::TX_ABRT | IntMask::RX_OVER | IntMask::RX_FULL,
                false,
            );
            sem.signal(false);
        }
    }

    // ── Suspend / Resume ──────────────────────────────────────────────────

    /// Save all configuration registers into `backup` and disable the peripheral.
    pub fn suspend(&self, backup: &mut I2cBackup) {
        let r = self.regs();
        backup.ic_con = r.ic_con().read().bits();
        backup.ic_tar = r.ic_tar().read().bits();
        backup.ic_sar = r.ic_sar().read().bits();
        backup.ic_hs_mar = r.ic_hs_mar().read().bits();
        backup.ic_ss_scl_hcnt = r.ic_ss_scl_hcnt().read().bits();
        backup.ic_ss_scl_lcnt = r.ic_ss_scl_lcnt().read().bits();
        backup.ic_fs_scl_hcnt = r.ic_fs_scl_hcnt().read().bits();
        backup.ic_fs_scl_lcnt = r.ic_fs_scl_lcnt().read().bits();
        backup.ic_hs_scl_hcnt = r.ic_hs_scl_hcnt().read().bits();
        backup.ic_hs_scl_lcnt = r.ic_hs_scl_lcnt().read().bits();
        backup.ic_intr_mask = r.ic_intr_mask().read().bits();
        backup.ic_rx_tl = r.ic_rx_tl().read().bits();
        backup.ic_tx_tl = r.ic_tx_tl().read().bits();
        backup.ic_sda_hold = r.ic_sda_hold().read().bits();
        backup.ic_slv_data_nack_only = r.ic_slv_data_nack_only().read().bits();
        backup.ic_dma_tdlr = r.ic_dma_tdlr().read().bits();
        backup.ic_dma_rdlr = r.ic_dma_rdlr().read().bits();
        backup.ic_sda_setup = r.ic_sda_setup().read().bits();
        backup.ic_ack_general_call = r.ic_ack_general_call().read().bits();
        backup.ic_out_smp_dly = r.ic_out_smp_dly().read().bits();
        backup.ic_filter = r.ic_filter().read().bits();
        backup.ic_sar2 = r.ic_sar2().read().bits();

        r.ic_enable().modify(|_, w| w.enable().clear_bit());
    }

    /// Restore all configuration registers from `backup` and re-enable the peripheral.
    pub fn resume(&self, backup: &I2cBackup) {
        let r = self.regs();

        // Keep peripheral disabled while restoring
        r.ic_enable().modify(|_, w| w.enable().clear_bit());

        r.ic_con().write(|w| unsafe { w.bits(backup.ic_con) });
        r.ic_tar().write(|w| unsafe { w.bits(backup.ic_tar) });
        r.ic_sar().write(|w| unsafe { w.bits(backup.ic_sar) });
        r.ic_hs_mar()
            .write(|w| unsafe { w.ic_hs_mar().bits((backup.ic_hs_mar & 0x7) as u8) });
        r.ic_ss_scl_hcnt().write(|w| unsafe {
            w.ic_ss_scl_hcnt()
                .bits((backup.ic_ss_scl_hcnt & 0xffff) as u16)
        });
        r.ic_ss_scl_lcnt().write(|w| unsafe {
            w.ic_ss_scl_lcnt()
                .bits((backup.ic_ss_scl_lcnt & 0xffff) as u16)
        });
        r.ic_fs_scl_hcnt().write(|w| unsafe {
            w.ic_fs_scl_hcnt()
                .bits((backup.ic_fs_scl_hcnt & 0xffff) as u16)
        });
        r.ic_fs_scl_lcnt().write(|w| unsafe {
            w.ic_fs_scl_lcnt()
                .bits((backup.ic_fs_scl_lcnt & 0xffff) as u16)
        });
        r.ic_hs_scl_hcnt().write(|w| unsafe {
            w.ic_hs_scl_hcnt()
                .bits((backup.ic_hs_scl_hcnt & 0xffff) as u16)
        });
        r.ic_hs_scl_lcnt().write(|w| unsafe {
            w.ic_hs_scl_lcnt()
                .bits((backup.ic_hs_scl_lcnt & 0xffff) as u16)
        });
        r.ic_intr_mask()
            .write(|w| unsafe { w.bits(backup.ic_intr_mask) });
        r.ic_rx_tl()
            .write(|w| unsafe { w.ic_rx_0_tl().bits((backup.ic_rx_tl & 0xff) as u8) });
        r.ic_tx_tl()
            .write(|w| unsafe { w.ic_tx_0_tl().bits((backup.ic_tx_tl & 0xff) as u8) });
        r.ic_sda_hold()
            .write(|w| unsafe { w.ic_sda_hold().bits((backup.ic_sda_hold & 0xffff) as u16) });
        r.ic_slv_data_nack_only()
            .write(|w| w.nack().bit((backup.ic_slv_data_nack_only & 1) != 0));
        r.ic_dma_tdlr()
            .write(|w| unsafe { w.dmatdl().bits((backup.ic_dma_tdlr & 0xf) as u8) });
        r.ic_dma_rdlr()
            .write(|w| unsafe { w.dmardl().bits((backup.ic_dma_rdlr & 0xf) as u8) });
        r.ic_sda_setup()
            .write(|w| unsafe { w.ic_sda_setup().bits((backup.ic_sda_setup & 0xff) as u8) });
        r.ic_ack_general_call()
            .write(|w| w.ack_gen_call().bit((backup.ic_ack_general_call & 1) != 0));
        r.ic_out_smp_dly()
            .write(|w| unsafe { w.ic_out_smp_dly().bits((backup.ic_out_smp_dly & 0x7) as u8) });
        r.ic_filter().write(|w| unsafe {
            w.ic_dig_fltr_deg().bits((backup.ic_filter & 0x3f) as u8);
            w.ic_dig_fltr_sel().bit((backup.ic_filter >> 8 & 1) != 0)
        });
        // IC_SAR2 is 7-bit
        r.ic_sar2()
            .write(|w| unsafe { w.ic_sar2().bits((backup.ic_sar2 & 0x7f) as u8) });

        r.ic_enable().modify(|_, w| w.enable().set_bit());
    }

    /// Convert to async mode.
    ///
    /// Requires a [`crate::interrupt::Binding`] token proving that
    /// `InterruptHandler<P>` is registered as the vector-table entry for
    /// `P::IrqType`. Obtain the token via [`crate::bind_interrupts!`].
    pub fn into_async<B>(self, _: B) -> I2c<P, Enabled, crate::mode::Async>
    where
        B: crate::interrupt::Binding<P::IrqType, InterruptHandler<P>>,
    {
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        crate::interrupt::enable::<P::IrqType, InterruptHandler<P>, B>();
        I2c {
            peri,
            _state: core::marker::PhantomData::<Enabled>,
            _dm: core::marker::PhantomData,
        }
    }

    /// Convert to async slave mode with acknowledge address `addr`.
    ///
    /// Rewrites IC_SAR (which requires the peripheral to be briefly disabled)
    /// and enables the NVIC IRQ for this I2C instance.
    ///
    /// Requires a [`crate::interrupt::Binding`] token proving that
    /// `InterruptHandler<P>` is registered as the vector-table entry for
    /// `P::IrqType`. Obtain the token via [`crate::bind_interrupts!`].
    pub fn into_async_slave<B>(self, addr: u8, _: B) -> I2cSlaveAsync<P>
    where
        B: crate::interrupt::Binding<P::IrqType, InterruptHandler<P>>,
    {
        let regs = self.regs();
        regs.ic_enable().modify(|_, w| w.enable().clear_bit());
        regs.ic_sar()
            .write(|w| unsafe { w.ic_sar().bits((addr as u16) & 0x3ff) });
        regs.ic_enable().modify(|_, w| w.enable().set_bit());
        crate::interrupt::enable::<P::IrqType, InterruptHandler<P>, B>();
        // Blocking Drop is a no-op (IS_ASYNC=false); drop happens naturally after this scope.
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        I2cSlaveAsync { peri }
    }
}

pub mod eh;

// ─────────────────────────────────────────────────────────────────────────────
// Async interrupt infrastructure
// ─────────────────────────────────────────────────────────────────────────────

/// Per-peripheral async state (waker + triggered interrupt status).
pub struct I2cState {
    waker: AtomicWaker,
    /// Stores IC_INTR_STAT value captured in ISR; 0 = not yet triggered.
    triggered: AtomicU32,
}

impl Default for I2cState {
    fn default() -> Self {
        Self::new()
    }
}

impl I2cState {
    pub const fn new() -> Self {
        Self {
            waker: AtomicWaker::new(),
            triggered: AtomicU32::new(0),
        }
    }
}

pub static I2C0_STATE: I2cState = I2cState::new();
pub static I2C1_STATE: I2cState = I2cState::new();

/// Call from I2C0_IRQ / I2C1_IRQ.  Reads masked interrupt status, disables
/// all I2C interrupts, stores the status, and wakes any waiting future.
pub(crate) fn i2c_int_dispatch(ptr: *const RegisterBlock, state: &'static I2cState) {
    let regs = unsafe { &*ptr };
    // .bits() is intentional: full status word is stored for I2cFuture::poll to inspect
    // via IntMask — this is state transfer, not bit interpretation (KP-2 exemption).
    let intr = regs.ic_intr_stat().read().bits();
    unsafe { regs.ic_intr_mask().write_with_zero(|w| w) };
    state
        .triggered
        .store(if intr != 0 { intr } else { 1 }, Ordering::Release);
    state.waker.wake();
}

// ---------------------------------------------------------------------------
// Unified Drop — teardown only for the async handle
// ---------------------------------------------------------------------------

impl<P: I2cInstance, S, Dm: crate::mode::DriverMode> Drop for I2c<P, S, Dm> {
    fn drop(&mut self) {
        if Dm::IS_ASYNC {
            self.disable_interrupts();
        }
    }
}

// ---------------------------------------------------------------------------
// I2c<P, Enabled, Async> — async master handle obtained via into_async()
// ---------------------------------------------------------------------------

impl<P: I2cInstance> I2c<P, Enabled, crate::mode::Async> {
    /// Convert back to a blocking handle, tearing down interrupts.
    pub fn into_blocking(self) -> I2c<P, Enabled, crate::mode::Blocking> {
        self.disable_interrupts();
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        I2c {
            peri,
            _state: core::marker::PhantomData::<Enabled>,
            _dm: core::marker::PhantomData,
        }
    }

    /// Set the target slave address (IC_TAR).
    pub fn set_target(&self, addr: u16) {
        self.regs()
            .ic_tar()
            .modify(|_, w| unsafe { w.ic_tar().bits(addr & 0x3ff) });
    }

    /// Reset the I2C FSM after a TX_ABRT (NACK/arbitration loss).
    fn reset_after_abort(&self) {
        let regs = self.regs();
        let _ = regs.ic_clr_tx_abrt().read();
        let _ = regs.ic_clr_intr().read();
        regs.ic_enable().modify(|_, w| w.enable().clear_bit());
        // Wait for controller to fully disable before re-enabling.
        // SDK: ameba_i2c.c : I2C_MasterSendNullData_TimeOut
        while regs.ic_enable_status().read().ic_en().bit_is_set() {}
        regs.ic_enable().modify(|_, w| w.enable().set_bit());
    }

    /// Async master write. Sends `buf` in batches of up to FIFO depth,
    /// awaiting a TX_EMPTY interrupt between batches and STOP_DET/TX_ABRT
    /// on the last batch. Returns IC_INTR_STAT of the final interrupt.
    pub async fn write_async(&self, buf: &[u8]) -> u32 {
        let state = self.peri.state();
        let regs = self.regs();
        let _ = regs.ic_clr_intr().read();

        let len = buf.len();
        if len == 0 {
            return 0;
        }

        let mut pos = 0;
        loop {
            state.triggered.store(0, Ordering::Release);

            let batch_end = (pos + TRX_BUFFER_DEPTH as usize).min(len);
            let is_last = batch_end == len;

            for (idx, &byte) in buf[pos..batch_end].iter().enumerate() {
                let stop = is_last && (idx + 1 == batch_end - pos);
                regs.ic_data_cmd().write(|w| unsafe {
                    w.ic_data().bits(byte);
                    w.cmd_stop().bit(stop)
                });
            }

            let mask = if is_last {
                (IntMask::TX_ABRT | IntMask::STOP_DET).bits()
            } else {
                (IntMask::TX_ABRT | IntMask::TX_EMPTY).bits()
            };
            regs.ic_intr_mask().write(|w| unsafe { w.bits(mask) });

            let intr = I2cFuture { ptr: P::PTR, state }.await;
            pos = batch_end;

            if IntMask::from_bits_truncate(intr).contains(IntMask::TX_ABRT) {
                self.reset_after_abort();
                return intr;
            }
            if is_last {
                return intr;
            }
        }
    }

    /// Async master write with Repeated START after the last byte.
    /// Used internally by `transaction_async` when a Write is followed by a Read.
    /// The last byte has `cmd_restart=1` (not `cmd_stop=1`); waits for TX_EMPTY.
    async fn write_async_with_restart(&self, buf: &[u8]) -> u32 {
        let state = self.peri.state();
        let regs = self.regs();
        let _ = regs.ic_clr_intr().read();

        let len = buf.len();
        if len == 0 {
            return 0;
        }

        let mut pos = 0;
        loop {
            state.triggered.store(0, Ordering::Release);

            let batch_end = (pos + TRX_BUFFER_DEPTH as usize).min(len);
            let is_last = batch_end == len;

            for (idx, &byte) in buf[pos..batch_end].iter().enumerate() {
                let restart = is_last && (idx + 1 == batch_end - pos);
                regs.ic_data_cmd().write(|w| unsafe {
                    w.ic_data().bits(byte);
                    w.cmd_restart().bit(restart)
                    // cmd_stop intentionally omitted: Repeated START, not STOP
                });
            }

            // Wait TX_EMPTY to know FIFO has drained (no STOP_DET since no STOP)
            let mask = (IntMask::TX_ABRT | IntMask::TX_EMPTY).bits();
            regs.ic_intr_mask().write(|w| unsafe { w.bits(mask) });

            let intr = I2cFuture { ptr: P::PTR, state }.await;
            pos = batch_end;

            if IntMask::from_bits_truncate(intr).contains(IntMask::TX_ABRT) {
                self.reset_after_abort();
                return intr;
            }
            if is_last {
                return intr;
            }
        }
    }

    /// Async master read. Issues `buf.len()` read commands in batches,
    /// awaiting RX_FULL between batches. Returns IC_INTR_STAT of the final
    /// interrupt (TX_ABRT when no slave present is a valid result).
    pub async fn read_async(&self, buf: &mut [u8]) -> u32 {
        let state = self.peri.state();
        let regs = self.regs();
        let _ = regs.ic_clr_intr().read();

        let len = buf.len();
        if len == 0 {
            return 0;
        }

        let mut rcvd = 0usize;
        let mut cmds_issued = 0usize;

        loop {
            state.triggered.store(0, Ordering::Release);

            let batch = (len - cmds_issued).min(TRX_BUFFER_DEPTH as usize);
            let batch_end = cmds_issued + batch;

            for i in cmds_issued..batch_end {
                let stop = i == len - 1;
                regs.ic_data_cmd().write(|w| {
                    w.cmd_rw().set_bit();
                    w.cmd_stop().bit(stop)
                });
            }
            cmds_issued = batch_end;

            // RX_FULL fires when FIFO level > threshold; set threshold = batch - 1
            regs.ic_rx_tl()
                .write(|w| unsafe { w.ic_rx_0_tl().bits((batch - 1) as u8) });

            regs.ic_intr_mask().write(|w| unsafe {
                w.bits((IntMask::TX_ABRT | IntMask::RX_FULL | IntMask::RX_OVER).bits())
            });

            let intr = I2cFuture { ptr: P::PTR, state }.await;

            while regs.ic_rxflr().read().rxflr().bits() > 0 && rcvd < len {
                buf[rcvd] = regs.ic_data_cmd().read().ic_data().bits();
                rcvd += 1;
            }

            if IntMask::from_bits_truncate(intr).contains(IntMask::TX_ABRT) {
                self.reset_after_abort();
                return intr;
            }
            if rcvd >= len {
                return intr;
            }
        }
    }

    /// Async master transaction: sequence of write/read operations.
    /// A Write immediately followed by a Read issues a Repeated START (no STOP).
    /// Aborts early on TX_ABRT. Returns IC_INTR_STAT of the final interrupt.
    pub async fn transaction_async(&self, ops: &mut [embedded_hal::i2c::Operation<'_>]) -> u32 {
        let mut last_intr = 0u32;
        let mut i = 0;
        while i < ops.len() {
            let use_restart = matches!(
                (&ops[i], ops.get(i + 1)),
                (
                    embedded_hal::i2c::Operation::Write(_),
                    Some(embedded_hal::i2c::Operation::Read(_))
                )
            );
            match &mut ops[i] {
                embedded_hal::i2c::Operation::Write(buf) => {
                    if use_restart {
                        last_intr = self.write_async_with_restart(buf).await;
                    } else {
                        last_intr = self.write_async(buf).await;
                    }
                }
                embedded_hal::i2c::Operation::Read(buf) => {
                    last_intr = self.read_async(buf).await;
                }
            }
            if IntMask::from_bits_truncate(last_intr).contains(IntMask::TX_ABRT) {
                return last_intr;
            }
            i += 1;
        }
        last_intr
    }
}

/// One-shot future: resolves when any I2C interrupt fires.
/// Cancellation-safe: Drop disables all I2C interrupts and resets the peripheral
/// if the future was never resolved.
pub struct I2cFuture {
    ptr: *const RegisterBlock,
    state: &'static I2cState,
}

// SAFETY: embedded single-CPU target; pointer is valid for the peripheral's lifetime.
unsafe impl Send for I2cFuture {}

impl Future for I2cFuture {
    type Output = u32; // IC_INTR_STAT value captured in ISR
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        self.state.waker.register(cx.waker());
        let v = self.state.triggered.load(Ordering::Acquire);
        if v != 0 {
            Poll::Ready(v)
        } else {
            Poll::Pending
        }
    }
}

impl Drop for I2cFuture {
    fn drop(&mut self) {
        if self.state.triggered.load(Ordering::Relaxed) == 0 {
            let regs = unsafe { &*self.ptr };
            unsafe { regs.ic_intr_mask().write_with_zero(|w| w) };
            // Clear pending raw interrupt flags before reset.
            // SDK: ameba_i2c.c : I2C_MasterSendNullData_TimeOut — I2C_ClearAllINT before disable
            let _ = regs.ic_clr_intr().read();
            regs.ic_enable().modify(|_, w| w.enable().clear_bit());
            // Wait for controller to fully disable before re-enabling.
            // SDK: ameba_i2c.c : I2C_MasterSendNullData_TimeOut
            while regs.ic_enable_status().read().ic_en().bit_is_set() {}
            regs.ic_enable().modify(|_, w| w.enable().set_bit());
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Async slave mode
// ─────────────────────────────────────────────────────────────────────────────

/// I2C slave handle in async mode. Obtained via [`I2c::into_async_slave`].
///
/// Reuses the same per-peripheral [`I2cState`] / `i2c_int_dispatch` / [`I2cFuture`]
/// infrastructure as the master path. The captured `IC_INTR_STAT` distinguishes
/// slave events (STOP_DET / RX_FULL / RD_REQ / RX_DONE) by bit position; because
/// `into_async_slave` consumes the peripheral, master and slave never share an
/// instance so the shared status word is unambiguous.
pub struct I2cSlaveAsync<P: I2cInstance> {
    peri: P,
}

// SAFETY: embedded single-CPU target; `I2cSlaveAsync<P>` has exclusive ownership of its peripheral.
unsafe impl<P: I2cInstance> Send for I2cSlaveAsync<P> {}

impl<P: I2cInstance> Drop for I2cSlaveAsync<P> {
    fn drop(&mut self) {
        // Symmetric teardown for the single `interrupt::enable` in `into_async_slave`.
        // Mask all peripheral-level interrupts first, THEN mask the shared NVIC
        // line (order must not reverse).
        let regs = unsafe { &*self.peri.ptr() };
        unsafe { regs.ic_intr_mask().write_with_zero(|w| w) };
        crate::interrupt::disable::<P::IrqType>();
    }
}

impl<P: I2cInstance> I2cSlaveAsync<P> {
    fn regs(&self) -> &RegisterBlock {
        unsafe { &*self.peri.ptr() }
    }

    /// Async slave receive (master writes → slave reads).
    ///
    /// Enables RX_FULL / RX_OVER / STOP_DET, draining the RX FIFO into `buf`
    /// on each interrupt until the master issues STOP. Bytes beyond `buf.len()`
    /// are discarded (RX_OVER cleared). Returns the number of bytes stored.
    pub async fn recv_async(&self, buf: &mut [u8]) -> usize {
        let state = P::state_static();
        let regs = self.regs();
        let len = buf.len();
        let mut rcvd = 0usize;

        // Discard any stale latched status (e.g. a STOP_DET left over from a
        // preceding master-read transaction) so it cannot fire spuriously the
        // instant the mask is enabled below.
        let _ = regs.ic_clr_intr().read();

        loop {
            state.triggered.store(0, Ordering::Release);
            regs.ic_intr_mask().write(|w| unsafe {
                w.bits((IntMask::RX_FULL | IntMask::RX_OVER | IntMask::STOP_DET).bits())
            });

            let intr = I2cFuture { ptr: P::PTR, state }.await;

            // Drain whatever is currently in the RX FIFO.
            while regs.ic_rxflr().read().rxflr().bits() > 0 {
                let byte = regs.ic_data_cmd().read().ic_data().bits();
                if rcvd < len {
                    buf[rcvd] = byte;
                    rcvd += 1;
                }
            }

            let flags = IntMask::from_bits_truncate(intr);
            if flags.contains(IntMask::RX_OVER) {
                let _ = regs.ic_clr_rx_over().read();
            }
            if flags.contains(IntMask::STOP_DET) {
                // Latched — must be cleared via its dedicated register.
                let _ = regs.ic_clr_stop_det().read();
                return rcvd;
            }
        }
    }

    /// Async slave transmit (master reads → slave sends).
    ///
    /// Enables RD_REQ / RX_DONE / TX_ABRT, writing one byte to the TX FIFO per
    /// read request until the master terminates the read (RX_DONE) or the buffer
    /// is exhausted. Returns the number of bytes handed to the FIFO.
    pub async fn send_async(&self, buf: &[u8]) -> usize {
        let state = P::state_static();
        let regs = self.regs();
        let len = buf.len();
        let mut pos = 0usize;

        // Discard any stale latched status from a previous transaction.
        let _ = regs.ic_clr_intr().read();

        loop {
            state.triggered.store(0, Ordering::Release);
            regs.ic_intr_mask().write(|w| unsafe {
                w.bits(
                    (IntMask::RD_REQ | IntMask::RX_DONE | IntMask::TX_ABRT | IntMask::TX_OVER)
                        .bits(),
                )
            });

            let intr = I2cFuture { ptr: P::PTR, state }.await;
            let flags = IntMask::from_bits_truncate(intr);

            if flags.contains(IntMask::TX_ABRT) {
                let _ = regs.ic_clr_tx_abrt().read();
                return pos;
            }
            if flags.contains(IntMask::TX_OVER) {
                let _ = regs.ic_clr_tx_over().read();
            }
            if flags.contains(IntMask::RX_DONE) {
                // Master NACK'd the last byte → read transaction complete.
                let _ = regs.ic_clr_rx_done().read();
                return pos;
            }
            if flags.contains(IntMask::RD_REQ) {
                // Latched — clear before refilling the FIFO.
                let _ = regs.ic_clr_rd_req().read();
                let tfnf = StatusFlags::from_bits_truncate(regs.ic_status().read().bits())
                    .contains(StatusFlags::TFNF);
                if pos < len && tfnf {
                    let byte = buf[pos];
                    regs.ic_data_cmd()
                        .write(|w| unsafe { w.ic_data().bits(byte) });
                    pos += 1;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
impl sealed::Sealed for AnyI2c<'_> {}
impl I2cInstance for AnyI2c<'static> {
    type IrqType = crate::interrupt::typelevel::IrqAny;
    const PTR: *const RegisterBlock = core::ptr::null();

    fn ptr(&self) -> *const RegisterBlock {
        any::delegate!(self, i => { i.ptr() })
    }
    fn state(&self) -> &'static I2cState {
        any::delegate!(self, i => { i.state() })
    }
    fn state_static() -> &'static I2cState {
        unreachable!("AnyI2c::state_static called -- use state(&self) instead")
    }
}

// ---------------------------------------------------------------------------
// I2cInstance impls for concrete tokens
// ---------------------------------------------------------------------------

macro_rules! impl_i2c_instance {
    ($ty:ty, $irq_type:ty, $pac:ty, $state:ident) => {
        impl sealed::Sealed for $ty {}
        impl $crate::private::Sealed for $ty {}
        impl I2cInstance for $ty {
            type IrqType = $irq_type;
            const PTR: *const RegisterBlock = <$pac>::PTR as *const RegisterBlock;
            fn ptr(&self) -> *const RegisterBlock {
                Self::PTR
            }
            fn state_static() -> &'static I2cState {
                &$state
            }
            fn state(&self) -> &'static I2cState {
                &$state
            }
        }
    };
}

impl_i2c_instance!(
    I2c0,
    crate::interrupt::typelevel::I2C0_IRQ,
    I2c0Reg,
    I2C0_STATE
);
impl_i2c_instance!(
    I2c1,
    crate::interrupt::typelevel::I2C1_IRQ,
    I2c1Reg,
    I2C1_STATE
);

// ---------------------------------------------------------------------------
// Embassy-style interrupt handler
// ---------------------------------------------------------------------------

/// Embassy-style interrupt handler for a typed I2C instance.
///
/// Register it via [`crate::bind_interrupts!`], then pass the resulting token
/// to [`I2c::into_async`] or [`I2c::into_async_slave`]:
///
/// ```rust,ignore
/// bind_interrupts!(struct Irqs {
///     I2C0_IRQ => i2c::InterruptHandler<i2c::I2c0>;
/// });
///
/// let i2c = unsafe { I2c::<I2c0, _>::steal() };
/// let i2c_async = i2c.init(Config::default()).enable().into_async(Irqs);
/// ```
pub struct InterruptHandler<P: I2cInstance>(PhantomData<P>);

impl<P: I2cInstance> crate::interrupt::Handler<P::IrqType> for InterruptHandler<P> {
    unsafe fn on_interrupt() {
        i2c_int_dispatch(P::PTR, P::state_static());
    }
}
