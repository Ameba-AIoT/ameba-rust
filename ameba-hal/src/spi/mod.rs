//! RTL8721F SPI (SSI) driver — `#![no_std]`, PAC-based.
//!
//! Covers non-GDMA functions from `ameba_spi.c` and DMA-backed async via `SpiDma`.
//! Interrupt-driven async (`Spi<P, Async>`) is obtained via `Spi::into_async()`.
//! DMA-backed async (`SpiDma`) is obtained via `Spi::into_dma_async(tx_ch, rx_ch)`.

use bitflags::bitflags;
use rtl8721f_pac::{LpSys, Spi0Reg, Spi1Reg, spi0_reg};

use crate::mode::{Async, Blocking, DriverMode};
use core::future::Future;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::task::{Context, Poll};
use embassy_sync::waitqueue::AtomicWaker;

pub type RegisterBlock = spi0_reg::RegisterBlock;

const TX_FIFO_DEPTH: u32 = 64; // SDK: ameba_spi.h : SSI_TX_FIFO_DEPTH

/// Drain whatever is currently in the RX FIFO into `buf`, stopping as soon as
/// the FIFO reports empty — never busy-waits. Safe to call from a
/// [`SpiIrqHandler::on_irq`] callback.
///
/// Takes a raw register-block pointer (e.g. `Spi0Reg::PTR`) rather than a
/// `Spi<P, Dm>` handle, so a raw-ISR callback doesn't need its own `steal()`'d
/// instance racing the owner's.
///
/// # Safety
/// `ptr` must point to a valid, live SPI register block (e.g. one of the
/// `Spi0Reg::PTR`/`Spi1Reg::PTR` constants).
pub unsafe fn read_available(ptr: *const RegisterBlock, buf: &mut [u8]) -> usize {
    let r = unsafe { &*ptr };
    let mut n = 0;
    while n < buf.len() && r.sr().read().rfne().bit_is_set() {
        buf[n] = r.dr0().read().dr0().bits() as u8;
        n += 1;
    }
    n
}

// ---------------------------------------------------------------------------
// Sealed trait + typed peripheral instances
// ---------------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}
}

#[derive(Default, Debug)]
pub struct Spi0;
#[derive(Default, Debug)]
pub struct Spi1;

crate::any_peripheral! {
    pub peripheral AnySpi<'d> {
        Spi0(Spi0),
        Spi1(Spi1),
    }
}

pub trait SpiInstance: sealed::Sealed + Send + 'static + any::Degrade {
    type IrqType: crate::interrupt::Irq;
    const PTR: *const RegisterBlock;
    const HS_TX: u8;
    const HS_RX: u8;
    fn state_static() -> &'static SpiState;
    fn state(&self) -> &'static SpiState {
        Self::state_static()
    }
    fn set_role_bit(&self, is_master: bool);
    fn ptr(&self) -> *const RegisterBlock {
        Self::PTR
    }
}

macro_rules! impl_spi_instance {
    ($ty:ty, $irq_type:ty, $pac:ty,
     $hs_tx:expr_2021, $hs_rx:expr_2021, $state:ident, $plat_field:ident) => {
        impl sealed::Sealed for $ty {}
        impl $crate::private::Sealed for $ty {}
        impl SpiInstance for $ty {
            type IrqType = $irq_type;
            const PTR: *const RegisterBlock = <$pac>::PTR as *const RegisterBlock;
            const HS_TX: u8 = $hs_tx;
            const HS_RX: u8 = $hs_rx;
            fn ptr(&self) -> *const RegisterBlock {
                Self::PTR
            }
            fn state_static() -> &'static SpiState {
                &$state
            }
            fn state(&self) -> &'static SpiState {
                &$state
            }
            fn set_role_bit(&self, is_master: bool) {
                let lp_sys = unsafe { &*LpSys::PTR };
                lp_sys
                    .reg_plat_ctrl()
                    .modify(|_, w| w.$plat_field().bit(is_master));
            }
        }
    };
}

impl_spi_instance!(
    Spi0,
    crate::interrupt::typelevel::SPI0_IRQ,
    Spi0Reg,
    6,
    7,
    SPI0_STATE,
    spi0_mst
);
impl_spi_instance!(
    Spi1,
    crate::interrupt::typelevel::SPI1_IRQ,
    Spi1Reg,
    8,
    9,
    SPI1_STATE,
    spi1_mst
);

/// Call this from your `bind_interrupts!` binding:
///
///     bind_interrupts!(struct Irqs {
///         SPI0_IRQ => spi::InterruptHandler<spi::Spi0>;
///     });
pub struct InterruptHandler<P: SpiInstance>(PhantomData<P>);

impl<P: SpiInstance> crate::interrupt::Handler<P::IrqType> for InterruptHandler<P> {
    unsafe fn on_interrupt() {
        spi_int_dispatch(P::PTR, P::state_static());
    }
}

// ---------------------------------------------------------------------------
// Raw ISR callback (user-registered, independent of the async waker path)
// ---------------------------------------------------------------------------

/// Raw SPI interrupt event, decoded from the ISR snapshot taken by
/// `spi_int_dispatch`. Mirrors UART's `UartIrqEvent` — the callback gets "what
/// happened", not raw register access; RC error registers are already
/// consumed by the time this is delivered.
#[derive(Clone, Copy, Debug, Default)]
pub struct SpiIrqEvent {
    /// RX FIFO reached its trigger level (ISR.RXFIS). Drain via
    /// [`read_available`] — RXFIS is level-held until the FIFO count drops
    /// back below threshold.
    pub rx_ready: bool,
    /// Overrun/underrun/mode-fault/slave-select error (RXOIS/RXUIS/TXOIS/
    /// TXUIS/MSTIS_FAEIS/SSRIS). Already read-to-clear'd by the time this
    /// event is delivered.
    pub error: bool,
    /// Raw (masked) ISR register snapshot at dispatch time, for handlers
    /// needing detail beyond the decoded flags above.
    pub isr: u32,
}

/// User-registered raw ISR callback for a SPI instance. Runs directly in the
/// SPI's NVIC interrupt context, independent of the `Spi::read_async`/
/// `write_async`/`transfer_async` chunked-transfer path. Register via
/// [`Spi::register_irq`].
pub trait SpiIrqHandler: Send {
    fn on_irq(&mut self, event: SpiIrqEvent);
}

/// Type-erased trampoline stored in `SpiState::irq_handler`, resolved back to
/// `H::on_irq` via the matching `irq_data` pointer. Mirrors UART's
/// `uart_handler_trampoline`.
unsafe fn spi_handler_trampoline<H: SpiIrqHandler>(data: usize, event: SpiIrqEvent) {
    let handler = unsafe { &mut *(data as *mut H) };
    handler.on_irq(event);
}

// ---------------------------------------------------------------------------
// Public enumerations
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Master,
    Slave,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    MotorolaSpi = 0,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrameSize {
    Bits4 = 0x03,
    Bits5 = 0x04,
    Bits6 = 0x05,
    Bits7 = 0x06,
    Bits8 = 0x07,
    Bits9 = 0x08,
    Bits10 = 0x09,
    Bits11 = 0x0A,
    Bits12 = 0x0B,
    Bits13 = 0x0C,
    Bits14 = 0x0D,
    Bits15 = 0x0E,
    Bits16 = 0x0F,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClkPhase {
    TogglesInMiddle = 0,
    TogglesAtStart = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClkPolarity {
    InactiveLow = 0,
    InactiveHigh = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TransferMode {
    TransmitReceive = 0,
    TransmitOnly = 1,
    ReceiveOnly = 2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SsToggle {
    NoToggle = 0,
    Toggle = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DmaChannel {
    Tx,
    Rx,
    Both,
}

bitflags! {
    /// Interrupt mask/status bits (IMR / ISR / RISR).
    /// Bit positions: PAC spi0_reg::imr / isr named accessors.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct InterruptFlags: u32 {
        const TXEIM       = 1 << 0; // PAC: spi0_reg::imr : txeim() / isr : txeis()
        const TXOIM       = 1 << 1; // PAC: spi0_reg::imr : txoim() / isr : txois()
        const RXUIM       = 1 << 2; // PAC: spi0_reg::imr : rxuim() / isr : rxuis()
        const RXOIM       = 1 << 3; // PAC: spi0_reg::imr : rxoim() / isr : rxois()
        const RXFIM       = 1 << 4; // PAC: spi0_reg::imr : rxfim() / isr : rxfis()
        const MSTIM_FAEIM = 1 << 5; // PAC: spi0_reg::imr : mstim_faeim() / isr : mstis_faeis()
        const TXUIM       = 1 << 6; // PAC: spi0_reg::imr : txuim() / isr : txuis()
        const SSRIM       = 1 << 7; // PAC: spi0_reg::imr : ssrim() / isr : ssris()
    }
}

bitflags! {
    /// Data swap control bits in CTRLR0.
    /// Bit positions: PAC spi0_reg::ctrlr0 named accessors.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DataSwap: u32 {
        const TX_BYTE = 1 << 21; // PAC: spi0_reg::ctrlr0 : txbyteswap()
        const TX_BIT  = 1 << 22; // PAC: spi0_reg::ctrlr0 : txbitswap()
        const RX_BYTE = 1 << 23; // PAC: spi0_reg::ctrlr0 : rxbyteswap()
        const RX_BIT  = 1 << 24; // PAC: spi0_reg::ctrlr0 : rxbitswap()
    }
}

// ---------------------------------------------------------------------------
// Configuration builder
// ---------------------------------------------------------------------------

pub struct SpiConfig {
    pub rx_threshold: u8,
    pub tx_threshold: u8,
    pub dma_rx_level: u8,
    pub dma_tx_level: u8,
    pub slave_select: u32,
    pub clock_divider: u16,
    pub frame_number: u16,
    pub frame_format: FrameFormat,
    pub frame_size: FrameSize,
    pub int_mask: InterruptFlags,
    pub clk_phase: ClkPhase,
    pub clk_polarity: ClkPolarity,
    pub transfer_mode: TransferMode,
    pub role: Role,
}

impl Default for SpiConfig {
    fn default() -> Self {
        Self {
            rx_threshold: 0,
            tx_threshold: 32,
            dma_rx_level: 3,
            dma_tx_level: 56,
            slave_select: 0,
            clock_divider: 6,
            frame_number: 0,
            frame_format: FrameFormat::MotorolaSpi,
            frame_size: FrameSize::Bits8,
            int_mask: InterruptFlags::empty(),
            clk_phase: ClkPhase::TogglesAtStart,
            clk_polarity: ClkPolarity::InactiveHigh,
            transfer_mode: TransferMode::TransmitReceive,
            role: Role::Master,
        }
    }
}

// ---------------------------------------------------------------------------
// Spi driver struct
// ---------------------------------------------------------------------------

/// Handle to one RTL8721F SPI peripheral.
///
/// `P` is the peripheral type (`Spi0` / `Spi1`); `Dm` is the [`DriverMode`] —
/// [`Blocking`] (default) or [`Async`]. Master async transfers live on
/// `Spi<P, Async>`; convert with [`Spi::into_async`] / [`Spi::into_blocking`].
/// Async **slave** mode uses the separate [`SpiSlaveAsync`] handle.
pub struct Spi<P: SpiInstance, Dm: DriverMode = Blocking> {
    peri: P,
    _dm: PhantomData<Dm>,
}

unsafe impl<P: SpiInstance, Dm: DriverMode> Send for Spi<P, Dm> {}

impl<P: SpiInstance + Default, Dm: DriverMode> Spi<P, Dm> {
    /// # Safety
    ///
    /// The caller must ensure no other [`Spi<P, _>`] instance for the same peripheral
    /// is alive concurrently.
    pub unsafe fn steal() -> Self {
        Self {
            peri: P::default(),
            _dm: PhantomData,
        }
    }
}

impl<P: SpiInstance, Dm: DriverMode> Spi<P, Dm> {
    fn regs(&self) -> &RegisterBlock {
        unsafe { &*self.peri.ptr() }
    }

    /// Address of the SPI data register (`DR`), for use as a GDMA
    /// source/destination. The address comes from the PAC register block
    /// (svd2rust → SVD), not a hand-written offset.
    #[inline(always)]
    pub fn dr_ptr(&self) -> *mut u32 {
        self.regs().dr0().as_ptr()
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn set_enabled(&self, en: bool) {
        let r = self.regs();
        r.ssienr().write(|w| w.ssi_en().bit(en));
    }

    // -----------------------------------------------------------------------
    // Public API
    // -----------------------------------------------------------------------

    /// Initialise the peripheral from a configuration struct.
    ///
    /// Call `set_role()` before this to configure the LSYS master/slave bit
    /// (SDK: `ameba_spi.c : SSI_SetRole`); `init()` only uses `cfg.role` to
    /// conditionally program master-only registers (CTRLR1 / SER / BAUDR).
    pub fn init(&self, cfg: &SpiConfig) {
        self.set_enabled(false);

        let r = self.regs();

        // CTRLR0: write all fields at once to avoid partial-state issues.
        unsafe {
            r.ctrlr0().write(|w| {
                w.dfs()
                    .bits(cfg.frame_size as u8)
                    .frf()
                    .bits(cfg.frame_format as u8)
                    .scph()
                    .bit(cfg.clk_phase == ClkPhase::TogglesAtStart)
                    .scpol()
                    .bit(cfg.clk_polarity == ClkPolarity::InactiveHigh)
                    .tmod()
                    .bits(cfg.transfer_mode as u8)
                    // SLV_OE=0: slave txd enabled by default
                    .slv_oe()
                    .clear_bit()
            });
        }

        // TX / RX FIFO thresholds.
        unsafe {
            r.txftlr().write(|w| w.tft().bits(cfg.tx_threshold & 0x3F));
            r.rxftlr().write(|w| w.rft().bits(cfg.rx_threshold & 0x3F));
        }

        // Master-only registers.
        if cfg.role == Role::Master {
            unsafe {
                r.ctrlr1().write(|w| w.ndf().bits(cfg.frame_number));
                r.baudr().write(|w| w.sckdv().bits(cfg.clock_divider));
            }
            self.set_slave_enable(cfg.slave_select);
            // SDK: ssi_api.h : SSI_SetSampleDelay — 1-cycle MISO sample delay for master.
            self.set_sample_delay(1);
        }

        // Interrupt mask.
        let m = cfg.int_mask;
        r.imr().write(|w| {
            w.txeim()
                .bit(m.contains(InterruptFlags::TXEIM))
                .txoim()
                .bit(m.contains(InterruptFlags::TXOIM))
                .rxuim()
                .bit(m.contains(InterruptFlags::RXUIM))
                .rxoim()
                .bit(m.contains(InterruptFlags::RXOIM))
                .rxfim()
                .bit(m.contains(InterruptFlags::RXFIM))
                .mstim_faeim()
                .bit(m.contains(InterruptFlags::MSTIM_FAEIM))
                .txuim()
                .bit(m.contains(InterruptFlags::TXUIM))
                .ssrim()
                .bit(m.contains(InterruptFlags::SSRIM))
        });

        // DMA levels.
        self.set_dma_level(cfg.dma_tx_level, cfg.dma_rx_level);

        self.set_enabled(true);
    }

    /// Enable or disable the peripheral.
    pub fn enable(&self) {
        self.set_enabled(true);
    }

    pub fn disable(&self) {
        self.set_enabled(false);
    }

    /// Configure master/slave role via the LSYS platform control register.
    ///
    /// Must be called before `init()`. SDK: `ameba_spi.c : SSI_SetRole`.
    /// PAC: `LpSys::reg_plat_ctrl` → `spi0_mst` / `spi1_mst`.
    pub fn set_role(&self, role: Role) {
        self.peri.set_role_bit(role == Role::Master);
    }

    /// Enable or disable specific interrupt sources.
    pub fn set_interrupts(&self, flags: InterruptFlags, enable: bool) {
        let r = self.regs();
        r.imr().modify(|_, w| {
            if flags.contains(InterruptFlags::TXEIM) {
                w.txeim().bit(enable);
            }
            if flags.contains(InterruptFlags::TXOIM) {
                w.txoim().bit(enable);
            }
            if flags.contains(InterruptFlags::RXUIM) {
                w.rxuim().bit(enable);
            }
            if flags.contains(InterruptFlags::RXOIM) {
                w.rxoim().bit(enable);
            }
            if flags.contains(InterruptFlags::RXFIM) {
                w.rxfim().bit(enable);
            }
            if flags.contains(InterruptFlags::MSTIM_FAEIM) {
                w.mstim_faeim().bit(enable);
            }
            if flags.contains(InterruptFlags::TXUIM) {
                w.txuim().bit(enable);
            }
            if flags.contains(InterruptFlags::SSRIM) {
                w.ssrim().bit(enable);
            }
            w
        });
    }

    /// Set clock polarity. Disables and re-enables the peripheral.
    pub fn set_clk_polarity(&self, pol: ClkPolarity) {
        self.set_enabled(false);
        self.regs()
            .ctrlr0()
            .modify(|_, w| w.scpol().bit(pol == ClkPolarity::InactiveHigh));
        self.set_enabled(true);
    }

    /// Set clock phase. Disables and re-enables the peripheral.
    pub fn set_clk_phase(&self, phase: ClkPhase) {
        self.set_enabled(false);
        self.regs()
            .ctrlr0()
            .modify(|_, w| w.scph().bit(phase == ClkPhase::TogglesAtStart));
        self.set_enabled(true);
    }

    /// Enable or disable CS-force mode (master only).
    pub fn set_cs_force(&self, enable: bool) {
        self.set_enabled(false);
        self.regs().ctrlr0().modify(|_, w| w.csf_en().bit(enable));
        self.set_enabled(true);
    }

    /// Set data frame size. Disables and re-enables the peripheral.
    pub fn set_frame_size(&self, size: FrameSize) {
        self.set_enabled(false);
        unsafe {
            self.regs().ctrlr0().modify(|_, w| w.dfs().bits(size as u8));
        }
        self.set_enabled(true);
    }

    /// Set number of data frames (master only, 1..=65536).
    pub fn set_frame_number(&self, n: u16) {
        self.set_enabled(false);
        unsafe {
            self.regs().ctrlr1().modify(|_, w| {
                // Hardware field stores n-1.
                w.ndf().bits(n.saturating_sub(1))
            });
        }
        self.set_enabled(true);
    }

    /// Set SS toggle phase (master, relevant when SCPH=0).
    pub fn set_ss_toggle(&self, toggle: SsToggle) {
        self.set_enabled(false);
        self.regs()
            .ctrlr0()
            .modify(|_, w| w.ss_t().bit(toggle == SsToggle::Toggle));
        self.set_enabled(true);
    }

    /// Enable or disable data byte/bit swap on TX/RX paths.
    pub fn set_data_swap(&self, swap: DataSwap, enable: bool) {
        self.set_enabled(false);
        self.regs().ctrlr0().modify(|_, w| {
            if swap.contains(DataSwap::TX_BYTE) {
                w.txbyteswap().bit(enable);
            }
            if swap.contains(DataSwap::TX_BIT) {
                w.txbitswap().bit(enable);
            }
            if swap.contains(DataSwap::RX_BYTE) {
                w.rxbyteswap().bit(enable);
            }
            if swap.contains(DataSwap::RX_BIT) {
                w.rxbitswap().bit(enable);
            }
            w
        });
        self.set_enabled(true);
    }

    /// Set Rx sample delay in ssi_clk cycles (max 4 on this SoC).
    pub fn set_sample_delay(&self, delay: u8) {
        self.set_enabled(false);
        unsafe {
            self.regs().rx_sample_dly().write(|w| w.rsd().bits(delay));
        }
        self.set_enabled(true);
    }

    /// Set number of frames to receive (master, receive-only or CSF_EN mode).
    /// Valid range: 1..=65536.
    pub fn set_read_len(&self, n: u16) {
        self.set_enabled(false);
        unsafe {
            // Write n-1 directly to the NDF field (whole register = NDF).
            self.regs()
                .ctrlr1()
                .write(|w| w.ndf().bits(n.saturating_sub(1)));
        }
        self.set_enabled(true);
    }

    /// Set clock divider (master only). Must be even, 2..=65534.
    pub fn set_baud_div(&self, div: u16) {
        self.set_enabled(false);
        unsafe {
            self.regs().baudr().write(|w| w.sckdv().bits(div));
        }
        self.set_enabled(true);
    }

    /// Enable or disable the DMA transmit/receive channel(s).
    pub fn set_dma_enable(&self, ch: DmaChannel, enable: bool) {
        let r = self.regs();
        r.dmacr().modify(|_, w| match ch {
            DmaChannel::Tx => w.tdmae().bit(enable),
            DmaChannel::Rx => w.rdmae().bit(enable),
            DmaChannel::Both => w.tdmae().bit(enable).rdmae().bit(enable),
        });
    }

    /// Set DMA TX and RX FIFO watermark levels.
    pub fn set_dma_level(&self, tx_level: u8, rx_level: u8) {
        let r = self.regs();
        unsafe {
            r.dmatdlr().write(|w| w.dmatdl().bits(tx_level & 0x3F));
            r.dmardlr().write(|w| w.dmardl().bits(rx_level & 0x3F));
        }
    }

    /// Clear interrupt sources indicated by `status`.
    ///
    /// These are RC (Read-to-Clear) registers: a read triggers the hardware clear;
    /// writing has no effect. SDK: ameba_spi.h : SPI_BIT_TXOICR / SPI_BIT_RXOICR /
    /// SPI_BIT_RXUICR / SPI_BIT_MSTICR_FAEICR / SPI_BIT_TXUICR / SPI_BIT_SSRICR
    pub fn clear_interrupts(&self, status: InterruptFlags) {
        let r = self.regs();
        unsafe {
            if status.contains(InterruptFlags::TXOIM) {
                r.txoicr().as_ptr().read_volatile();
            }
            if status.contains(InterruptFlags::RXUIM) {
                r.rxuicr().as_ptr().read_volatile();
            }
            if status.contains(InterruptFlags::RXOIM) {
                r.rxoicr().as_ptr().read_volatile();
            }
            if status.contains(InterruptFlags::MSTIM_FAEIM) {
                r.msticr_faeicr().as_ptr().read_volatile();
            }
            if status.contains(InterruptFlags::TXUIM) {
                r.txuicr().as_ptr().read_volatile();
            }
            if status.contains(InterruptFlags::SSRIM) {
                r.ssricr().as_ptr().read_volatile();
            }
        }
    }

    /// Write one word to the TX FIFO.
    pub fn write_data(&self, value: u32) {
        unsafe {
            self.regs().dr0().write(|w| w.dr0().bits(value as u16));
        }
    }

    /// Set the RX FIFO threshold level (0..63).
    pub fn set_rx_fifo_level(&self, level: u8) {
        unsafe {
            self.regs().rxftlr().write(|w| w.rft().bits(level & 0x3F));
        }
    }

    /// Set the TX FIFO threshold level (0..63).
    pub fn set_tx_fifo_level(&self, level: u8) {
        unsafe {
            self.regs().txftlr().write(|w| w.tft().bits(level & 0x3F));
        }
    }

    /// Select a slave by index (master only).
    pub fn set_slave_enable(&self, slave_index: u32) {
        if self.is_busy() {
            return;
        }
        unsafe {
            // SER register has a single `ser` bit at position 0, but the
            // hardware supports one-hot encoding per slave. Write whole register.
            self.regs()
                .ser()
                .write_with_zero(|w| w.bits(1u32.wrapping_shl(slave_index)));
        }
    }

    /// Returns `true` when the TX FIFO has at least one empty slot.
    pub fn is_tx_not_full(&self) -> bool {
        self.regs().sr().read().tfnf().bit_is_set()
    }

    /// Returns `true` when the RX FIFO has at least one entry.
    pub fn is_rx_not_empty(&self) -> bool {
        self.regs().sr().read().rfne().bit_is_set()
    }

    /// Read one word from the RX FIFO.
    pub fn read_data(&self) -> u32 {
        self.regs().dr0().read().dr0().bits() as u32
    }

    /// Drain the RX FIFO into `buf` (up to `len` entries).
    /// Returns the number of entries actually read.
    ///
    /// `wide` — if `true`, each entry is 16 bits (9–16-bit frame size mode);
    ///          otherwise 8 bits (4–8-bit frame size mode).
    pub fn receive(&self, mut buf: Option<&mut [u8]>, len: u32, wide: bool) -> u32 {
        let mut remaining = len;

        while self.is_rx_not_empty() && remaining > 0 {
            let level = self.rx_count();
            let to_read = level.min(remaining);

            for _ in 0..to_read {
                let data = self.read_data();
                if let Some(ref mut buf) = buf {
                    // Safety: bounds check is handled by `remaining`.
                    let idx = (len - remaining) as usize;
                    if wide {
                        let idx16 = idx / 2;
                        unsafe {
                            let p = buf.as_mut_ptr().add(idx16 * 2) as *mut u16;
                            p.write_unaligned(data as u16);
                        }
                    } else {
                        // SAFETY: caller guarantees buf.len() >= len.
                        unsafe {
                            let p = buf.as_mut_ptr().add(idx);
                            p.write(data as u8);
                        }
                    }
                }
                // remaining is guaranteed > 0 by the while loop condition
                remaining -= 1;
            }
        }

        len - remaining
    }

    /// Push data from `buf` into the TX FIFO (up to `len` entries).
    /// Returns the number of entries actually written.
    ///
    /// When `buf` is `None` in master mode, dummy bytes are pushed to clock
    /// out a receive-only transfer.
    ///
    /// `wide` — 16-bit (9–16-bit) frame mode when `true`.
    pub fn send(&self, buf: Option<&[u8]>, len: u32, role: Role, wide: bool) -> u32 {
        if !self.is_tx_not_full() {
            return 0;
        }

        let space = TX_FIFO_DEPTH - self.tx_count();
        let write_max = space.min(len);

        // Mask TX-empty IRQ during fill to avoid re-entry.
        self.set_interrupts(InterruptFlags::TXEIM, false);

        let mut written = 0u32;
        while written < write_max {
            match buf {
                Some(data) => {
                    let val = if wide {
                        let idx = (written as usize) * 2;
                        let lo = *data.get(idx).unwrap_or(&0) as u16;
                        let hi = *data.get(idx + 1).unwrap_or(&0) as u16;
                        (hi << 8 | lo) as u32
                    } else {
                        *data.get(written as usize).unwrap_or(&0) as u32
                    };
                    self.write_data(val);
                }
                None => {
                    if role == Role::Master {
                        self.write_data(0);
                    } else {
                        break;
                    }
                }
            }
            written += 1;
        }

        self.set_interrupts(InterruptFlags::TXEIM, true);

        written
    }

    /// Returns the current interrupt mask register as a flags value.
    pub fn int_mask(&self) -> InterruptFlags {
        let bits = self.regs().imr().read().bits();
        InterruptFlags::from_bits_truncate(bits)
    }

    /// Returns the number of valid entries in the RX FIFO.
    pub fn rx_count(&self) -> u32 {
        self.regs().rxflr().read().rxtfl().bits() as u32
    }

    /// Returns the number of valid entries in the TX FIFO.
    pub fn tx_count(&self) -> u32 {
        self.regs().txflr().read().txtfl().bits() as u32
    }

    /// Returns the raw SR register value.
    pub fn status(&self) -> u32 {
        self.regs().sr().read().bits()
    }

    /// Returns the data frame size in bits (4..=16).
    pub fn frame_size_bits(&self) -> u32 {
        (self.regs().ctrlr0().read().dfs().bits() as u32) + 1
    }

    /// Returns the programmed number of data frames (NDF+1).
    pub fn frame_number(&self) -> u32 {
        (self.regs().ctrlr1().read().ndf().bits() as u32) + 1
    }

    /// Returns `true` when the peripheral is actively transferring data.
    pub fn is_busy(&self) -> bool {
        self.regs().sr().read().busy().bit_is_set()
    }

    /// Returns the masked interrupt status register.
    pub fn isr(&self) -> InterruptFlags {
        InterruptFlags::from_bits_truncate(self.regs().isr().read().bits())
    }

    /// Returns the raw (unmasked) interrupt status register.
    pub fn raw_isr(&self) -> InterruptFlags {
        InterruptFlags::from_bits_truncate(self.regs().risr().read().bits())
    }

    /// Returns the slave-enable register (bitmask of selected slaves).
    pub fn slave_enable(&self) -> u32 {
        self.regs().ser().read().bits()
    }

    /// Enable or disable slave output (slave mode only).
    ///
    /// SLV_OE=0 means slave txd is enabled; SLV_OE=1 disables it.
    /// This mirrors the (inverted) C driver convention: `Status=ENABLE` -> txd enabled.
    pub fn set_slave_output_enable(&self, enable: bool) {
        self.regs().ctrlr0().modify(|_, w| {
            // SLV_OE=0: txd enabled; SLV_OE=1: txd disabled.
            w.slv_oe().bit(!enable)
        });
    }

    /// Recover from a Frame Alignment Error (slave mode).
    ///
    /// Returns `true` if a FAE was detected and recovery was performed.
    pub fn slave_err_recovery(&self) -> bool {
        // MSTIR_FAEIR is bit 5 in RISR.
        if self.regs().risr().read().mstir_faeir().bit_is_set() {
            self.set_enabled(false);
            self.set_enabled(true);
            return true;
        }
        false
    }

    /// Disable interrupt-driven operation: mask all peripheral-level SPI
    /// interrupts, then mask the shared NVIC line (order must not reverse).
    /// Used by both `Drop` (async only) and [`into_blocking`](Spi::into_blocking).
    fn disable_interrupts(&self) {
        let r = unsafe { &*self.peri.ptr() };
        r.imr().write(|w| {
            w.txeim()
                .clear_bit()
                .txoim()
                .clear_bit()
                .rxuim()
                .clear_bit()
                .rxoim()
                .clear_bit()
                .rxfim()
                .clear_bit()
                .mstim_faeim()
                .clear_bit()
                .txuim()
                .clear_bit()
                .ssrim()
                .clear_bit()
        });
        crate::interrupt::disable::<P::IrqType>();
    }

    /// Register a raw ISR callback for RX-ready / error events on this instance.
    ///
    /// Requires a [`crate::interrupt::Binding`] token proving `InterruptHandler<P>`
    /// is wired to `P::IrqType` (same token `into_async` takes — obtain it via
    /// [`crate::bind_interrupts!`]); this call itself unmasks the NVIC line, so
    /// `into_async`/`into_dma_async` are not required to use this standalone.
    ///
    /// `handler.on_irq()` runs directly in interrupt context whenever the RX
    /// FIFO reaches its trigger level (drain via [`read_available`]) or an
    /// overrun/underrun/mode-fault/slave-select error occurs. TXEIS is not
    /// covered on this path — re-arming TXEIM unconditionally would storm
    /// (ISR.TXEIS stays set until the TX FIFO is written), unlike RXFIS which
    /// self-clears once the handler drains the FIFO, and the error bits which
    /// are RC and already consumed before dispatch.
    pub fn register_irq<H, B>(&self, handler: &'static mut H, _binding: B)
    where
        H: SpiIrqHandler + 'static,
        B: crate::interrupt::Binding<P::IrqType, InterruptHandler<P>>,
    {
        let state = self.peri.state();
        let trampoline: unsafe fn(usize, SpiIrqEvent) = spi_handler_trampoline::<H>;
        state
            .irq_handler
            .store(trampoline as usize, Ordering::Release);
        state
            .irq_data
            .store(handler as *mut H as usize, Ordering::Release);
        crate::interrupt::enable::<P::IrqType, InterruptHandler<P>, B>();
        self.set_interrupts(
            InterruptFlags::RXFIM
                | InterruptFlags::RXOIM
                | InterruptFlags::RXUIM
                | InterruptFlags::TXOIM
                | InterruptFlags::TXUIM
                | InterruptFlags::MSTIM_FAEIM
                | InterruptFlags::SSRIM,
            true,
        );
    }

    /// Deregister the raw ISR callback: mask the RX/error interrupt-enable
    /// bits and clear the state slot. Does not mask the shared NVIC line
    /// (mirrors UART's `unregister_irq`) — a concurrently active async handle
    /// on the same instance keeps working.
    pub fn unregister_irq(&self) {
        self.set_interrupts(
            InterruptFlags::RXFIM
                | InterruptFlags::RXOIM
                | InterruptFlags::RXUIM
                | InterruptFlags::TXOIM
                | InterruptFlags::TXUIM
                | InterruptFlags::MSTIM_FAEIM
                | InterruptFlags::SSRIM,
            false,
        );
        let state = self.peri.state();
        state.irq_handler.store(0, Ordering::Release);
        state.irq_data.store(0, Ordering::Release);
    }
}

impl<P: SpiInstance> Spi<P, Blocking> {
    /// Convert to async mode; requires an interrupt binding token.
    ///
    ///     bind_interrupts!(struct Irqs { SPI0_IRQ => spi::InterruptHandler<spi::Spi0>; });
    ///     let spi_async = spi.into_async(Irqs);
    pub fn into_async<B>(self, _: B) -> Spi<P, Async>
    where
        B: crate::interrupt::Binding<P::IrqType, InterruptHandler<P>>,
    {
        // Suppress the blocking handle's (no-op) Drop, then enable.
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        crate::interrupt::enable::<P::IrqType, InterruptHandler<P>, B>();
        Spi {
            peri,
            _dm: PhantomData,
        }
    }

    /// Convert to async **slave** mode; requires an interrupt binding token.
    ///
    /// The caller must have already selected the slave role and initialised the
    /// peripheral (`set_role(Role::Slave)` + `init(&cfg)` with `role: Role::Slave`)
    /// before calling this, mirroring the master [`Spi::into_async`] flow.
    pub fn into_async_slave<B>(self, _: B) -> SpiSlaveAsync<P>
    where
        B: crate::interrupt::Binding<P::IrqType, InterruptHandler<P>>,
    {
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        crate::interrupt::enable::<P::IrqType, InterruptHandler<P>, B>();
        SpiSlaveAsync { peri }
    }

    /// Convert to DMA-backed async mode. Enables NVIC IRQs for both GDMA channels.
    ///
    /// Handshake IDs are SPI-instance-specific.
    /// SDK: ameba_gdma.h : GDMA_HANDSHAKE_INTERFACE_SPI0_TX / SPI0_RX / SPI1_TX / SPI1_RX
    pub fn into_dma_async<const TX_CH: u8, const RX_CH: u8>(
        self,
        tx_ch: crate::dma::DmaChannel<TX_CH, Blocking>,
        rx_ch: crate::dma::DmaChannel<RX_CH, Blocking>,
    ) -> SpiDma<P, TX_CH, RX_CH> {
        core::mem::forget(self);
        SpiDma {
            _phantom: PhantomData,
            tx_ch: tx_ch.into_async(),
            rx_ch: rx_ch.into_async(),
        }
    }
}

pub mod eh;

// ---------------------------------------------------------------------------
// Async interrupt state
// ---------------------------------------------------------------------------

pub struct SpiState {
    pub(crate) waker: AtomicWaker,
    pub(crate) done: AtomicBool,
    tx_ptr: AtomicUsize, // *const u8 as usize; 0 = send dummy bytes (read_async)
    tx_rem: AtomicUsize,
    rx_ptr: AtomicUsize, // *mut u8 as usize; 0 = discard RX (write_async)
    rx_rem: AtomicUsize,
    irq_handler: AtomicUsize, // unsafe fn(usize, SpiIrqEvent) as usize; 0 = none registered
    irq_data: AtomicUsize,    // *mut H as usize, paired with irq_handler
}

impl Default for SpiState {
    fn default() -> Self {
        Self::new()
    }
}

impl SpiState {
    pub const fn new() -> Self {
        Self {
            waker: AtomicWaker::new(),
            done: AtomicBool::new(false),
            tx_ptr: AtomicUsize::new(0),
            tx_rem: AtomicUsize::new(0),
            rx_ptr: AtomicUsize::new(0),
            rx_rem: AtomicUsize::new(0),
            irq_handler: AtomicUsize::new(0),
            irq_data: AtomicUsize::new(0),
        }
    }
}

pub static SPI0_STATE: SpiState = SpiState::new();
pub static SPI1_STATE: SpiState = SpiState::new();

/// Call from the SPI ISR. Drives chunked TX/RX: each TXEIS fires fill the next FIFO
/// batch; each RXFIS drains received bytes. Signals done when all data is consumed.
/// SDK: example_raw_spi_trx_interrupt.c : ssi_interrupt
pub(crate) fn spi_int_dispatch(ptr: *const RegisterBlock, state: &'static SpiState) {
    let r = unsafe { &*ptr };
    let isr = r.isr().read(); // PAC typed reader. SDK: ameba_spi.c : SSI_GetIsr

    // Clear RC error registers. SDK: ameba_spi.c : SSI_SetIsrClean
    unsafe {
        if isr.txois().bit_is_set() {
            r.txoicr().as_ptr().read_volatile();
        }
        if isr.rxuis().bit_is_set() {
            r.rxuicr().as_ptr().read_volatile();
        }
        if isr.rxois().bit_is_set() {
            r.rxoicr().as_ptr().read_volatile();
        }
        if isr.mstis_faeis().bit_is_set() {
            r.msticr_faeicr().as_ptr().read_volatile();
        }
        if isr.txuis().bit_is_set() {
            r.txuicr().as_ptr().read_volatile();
        }
        if isr.ssris().bit_is_set() {
            r.ssricr().as_ptr().read_volatile();
        }
    }

    // Raw-callback error dispatch: all six error bits above are RC and were
    // just unconditionally consumed, so no re-arm is needed after delivery.
    if isr.txois().bit_is_set()
        || isr.rxuis().bit_is_set()
        || isr.rxois().bit_is_set()
        || isr.mstis_faeis().bit_is_set()
        || isr.txuis().bit_is_set()
        || isr.ssris().bit_is_set()
    {
        dispatch_user_irq(
            state,
            SpiIrqEvent {
                rx_ready: false,
                error: true,
                isr: isr.bits(),
            },
        );
    }

    // TX path: TXEIS fired — fill next chunk into TX FIFO.
    // SDK: ssi_interrupt → SSI_SendData (SSI_TX_FIFO_DEPTH - SSI_GetTxCount beats per call)
    if isr.txeis().bit_is_set() {
        let tx_rem = state.tx_rem.load(Ordering::Relaxed);
        if tx_rem > 0 {
            let tx_count = r.txflr().read().txtfl().bits() as usize;
            let space = TX_FIFO_DEPTH as usize - tx_count;
            let to_write = space.min(tx_rem);
            let tx_ptr = state.tx_ptr.load(Ordering::Relaxed) as *const u8;
            for i in 0..to_write {
                let val = if !tx_ptr.is_null() {
                    (unsafe { tx_ptr.add(i).read() }) as u32
                } else {
                    0u32 // dummy byte for master read. SDK: SSI_SendData TxData==NULL path
                };
                unsafe {
                    r.dr0().write(|w| w.dr0().bits(val as u16));
                }
            }
            if !tx_ptr.is_null() {
                state
                    .tx_ptr
                    .store(unsafe { tx_ptr.add(to_write) } as usize, Ordering::Relaxed);
            }
            let new_tx_rem = tx_rem - to_write;
            state.tx_rem.store(new_tx_rem, Ordering::Relaxed);
            if new_tx_rem == 0 {
                // All data loaded to FIFO. SDK: SSI_INTConfig(SPI_BIT_TXOIM|SPI_BIT_TXEIM, DISABLE)
                r.imr()
                    .modify(|_, w| w.txeim().clear_bit().txoim().clear_bit());
            }
        }
        // In write-only mode (rx_rem == 0): drain echoed RX to prevent FIFO overflow.
        // SDK: SSI_ReceiveData(spi_dev, NULL, len) discards data in master TX-only context.
        if state.rx_rem.load(Ordering::Relaxed) == 0 {
            while r.sr().read().rfne().bit_is_set() {
                let _ = r.dr0().read();
            }
            if state.tx_rem.load(Ordering::Relaxed) == 0 {
                state.done.store(true, Ordering::Release);
                state.waker.wake();
            }
        }
    }

    // RX path: RXFIS fired — drain RX FIFO into receive buffer.
    // SDK: ssi_interrupt → SSI_ReceiveData
    if isr.rxfis().bit_is_set() {
        let rx_rem = state.rx_rem.load(Ordering::Relaxed);
        if rx_rem > 0 {
            let rx_ptr = state.rx_ptr.load(Ordering::Relaxed) as *mut u8;
            let mut recvd = 0usize;
            while r.sr().read().rfne().bit_is_set() && recvd < rx_rem {
                let val = r.dr0().read().dr0().bits() as u8;
                if !rx_ptr.is_null() {
                    unsafe {
                        rx_ptr.add(recvd).write(val);
                    }
                }
                recvd += 1;
            }
            if !rx_ptr.is_null() {
                state
                    .rx_ptr
                    .store(unsafe { rx_ptr.add(recvd) } as usize, Ordering::Relaxed);
            }
            let new_rx_rem = rx_rem - recvd;
            state.rx_rem.store(new_rx_rem, Ordering::Relaxed);
            if new_rx_rem == 0 {
                // All RX data received. SDK: SSI_INTConfig(SPI_BIT_RXFIM|SPI_BIT_RXOIM|SPI_BIT_RXUIM, DISABLE)
                r.imr().modify(|_, w| {
                    w.rxfim()
                        .clear_bit()
                        .rxoim()
                        .clear_bit()
                        .rxuim()
                        .clear_bit()
                });
                state.done.store(true, Ordering::Release);
                state.waker.wake();
            }
        } else {
            // No chunked async RX in flight — hand RXFIS to the raw callback
            // instead. RXFIS is level-held; the handler must drain the FIFO
            // (e.g. via `read_available`) or the ISR will re-fire immediately.
            dispatch_user_irq(
                state,
                SpiIrqEvent {
                    rx_ready: true,
                    error: false,
                    isr: isr.bits(),
                },
            );
        }
    }
}

/// Invoke the user-registered raw ISR callback, if any. No-op (and no IMR
/// re-arm) when nothing is registered — mirrors UART's `dispatch_user_irq`.
fn dispatch_user_irq(state: &SpiState, event: SpiIrqEvent) {
    let handler_fn = state.irq_handler.load(Ordering::Acquire);
    if handler_fn == 0 {
        return;
    }
    let data = state.irq_data.load(Ordering::Acquire);
    let trampoline: unsafe fn(usize, SpiIrqEvent) = unsafe { core::mem::transmute(handler_fn) };
    unsafe { trampoline(data, event) };
}

// ---------------------------------------------------------------------------
// Async future
// ---------------------------------------------------------------------------

pub struct SpiFuture {
    ptr: *const RegisterBlock,
    state: &'static SpiState,
}

// SAFETY: embedded single-CPU target; pointer is valid for the peripheral's lifetime.
unsafe impl Send for SpiFuture {}

impl Future for SpiFuture {
    type Output = ();
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.state.waker.register(cx.waker());
        if self.state.done.load(Ordering::Acquire) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Drop for SpiFuture {
    fn drop(&mut self) {
        if !self.state.done.load(Ordering::Relaxed) {
            let r = unsafe { &*self.ptr };
            r.imr().write(|w| {
                w.txeim()
                    .clear_bit()
                    .txoim()
                    .clear_bit()
                    .rxuim()
                    .clear_bit()
                    .rxoim()
                    .clear_bit()
                    .rxfim()
                    .clear_bit()
                    .mstim_faeim()
                    .clear_bit()
                    .txuim()
                    .clear_bit()
                    .ssrim()
                    .clear_bit()
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Unified Drop — teardown only for the async handle
// ---------------------------------------------------------------------------

impl<P: SpiInstance, Dm: DriverMode> Drop for Spi<P, Dm> {
    fn drop(&mut self) {
        // Only the async handle owns the single `interrupt::enable` from
        // `into_async`; the transient blocking `steal()` handles used pervasively
        // inside async ops must NOT tear anything down. `IS_ASYNC` monomorphizes
        // the blocking branch to `if false` (dead-code eliminated → zero-cost).
        if Dm::IS_ASYNC {
            self.disable_interrupts();
        }
    }
}

// ---------------------------------------------------------------------------
// Spi<P, Async> — async handle obtained via Spi::into_async()
// ---------------------------------------------------------------------------

impl<P: SpiInstance> Spi<P, Async> {
    /// Convert back to a blocking handle, tearing down interrupts.
    pub fn into_blocking(self) -> Spi<P, Blocking> {
        self.disable_interrupts();
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        Spi {
            peri,
            _dm: PhantomData,
        }
    }

    fn start_transfer(
        &self,
        tx_ptr: *const u8,
        tx_rem: usize,
        rx_ptr: *mut u8,
        rx_rem: usize,
    ) -> SpiFuture {
        let state = self.peri.state();
        state.done.store(false, Ordering::Release);
        state.tx_ptr.store(tx_ptr as usize, Ordering::Relaxed);
        state.tx_rem.store(tx_rem, Ordering::Relaxed);
        state.rx_ptr.store(rx_ptr as usize, Ordering::Relaxed);
        state.rx_rem.store(rx_rem, Ordering::Relaxed);
        if rx_rem > 0 {
            self.regs().imr().modify(|_, w| {
                w.txeim()
                    .set_bit()
                    .txoim()
                    .set_bit()
                    .rxfim()
                    .set_bit()
                    .rxoim()
                    .set_bit()
                    .rxuim()
                    .set_bit()
            });
        } else {
            self.regs()
                .imr()
                .modify(|_, w| w.txeim().set_bit().txoim().set_bit());
        }
        SpiFuture {
            ptr: self.peri.ptr(),
            state,
        }
    }

    /// Async write: send `data`, discard echoed RX bytes.
    pub async fn write_async(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        self.start_transfer(data.as_ptr(), data.len(), core::ptr::null_mut(), 0)
            .await;
        // Drain residual RX bytes that arrived after the final ISR drain.
        while self.is_rx_not_empty() {
            let _ = self.read_data();
        }
    }

    /// Async transfer: send `tx`, receive into `rx`.
    pub async fn transfer_async(&mut self, rx: &mut [u8], tx: &[u8]) {
        let len = rx.len().min(tx.len());
        if len == 0 {
            return;
        }
        self.start_transfer(tx.as_ptr(), len, rx.as_mut_ptr(), len)
            .await;
    }

    /// Async in-place transfer: send and overwrite `words` with received bytes.
    ///
    /// Safe because TX bytes are read into the FIFO before the corresponding RX
    /// bytes arrive at the same buffer positions (SPI pipeline depth ≥ 1 frame).
    pub async fn transfer_in_place_async(&mut self, words: &mut [u8]) {
        if words.is_empty() {
            return;
        }
        let p = words.as_mut_ptr();
        self.start_transfer(p as *const u8, words.len(), p, words.len())
            .await;
    }

    /// Async read: clock out dummy bytes, receive into `buf`.
    pub async fn read_async(&mut self, buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }
        self.start_transfer(core::ptr::null(), buf.len(), buf.as_mut_ptr(), buf.len())
            .await;
    }

    /// Async flush: yield until the TX FIFO drains below threshold, then spin for
    /// shift-register completion.
    ///
    /// No completion interrupt exists for SR.BUSY→0 (SDK: `SSI_Busy()` spin is the
    /// only signal, consistent with `SSIMasterTxInterruptTest` in `ameba_ssi_test.c`).
    pub async fn flush_async(&self) {
        if !self.regs().sr().read().tfe().bit_is_set() {
            let state = self.peri.state();
            state.done.store(false, Ordering::Release);
            state.tx_ptr.store(0, Ordering::Relaxed);
            state.tx_rem.store(0, Ordering::Relaxed);
            state.rx_ptr.store(0, Ordering::Relaxed);
            state.rx_rem.store(0, Ordering::Relaxed);
            self.regs().imr().modify(|_, w| w.txeim().set_bit());
            SpiFuture {
                ptr: self.peri.ptr(),
                state,
            }
            .await;
        }
        // SDK: poll SR.BUSY after interrupt fires (no shift-register-empty IRQ).
        while self.is_busy() {}
        // Drain any RX bytes that arrived during the final shift.
        while self.is_rx_not_empty() {
            let _ = self.read_data();
        }
    }
}

// ---------------------------------------------------------------------------
// SpiSlaveAsync — async slave handle obtained via Spi::into_async_slave()
// ---------------------------------------------------------------------------

/// SPI handle in async **slave** mode. Obtained via [`Spi::into_async_slave`].
///
/// A slave never drives the clock: it pre-loads its outgoing data into the TX
/// FIFO and waits for the master to clock it out (which simultaneously shifts
/// the master's data into the slave RX FIFO). Completion is signalled by the
/// `RXFIM` interrupt once the programmed number of frames has been received.
pub struct SpiSlaveAsync<P: SpiInstance> {
    peri: P,
}

// SAFETY: embedded single-CPU target; `SpiSlaveAsync` has exclusive ownership of its peripheral.
unsafe impl<P: SpiInstance> Send for SpiSlaveAsync<P> {}

impl<P: SpiInstance> Drop for SpiSlaveAsync<P> {
    fn drop(&mut self) {
        // Symmetric teardown for the single `interrupt::enable` in `into_async_slave`.
        // Clear all peripheral-level interrupt masks first, THEN mask the shared
        // NVIC line (order must not reverse).
        let r = unsafe { &*self.peri.ptr() };
        r.imr().write(|w| {
            w.txeim()
                .clear_bit()
                .txoim()
                .clear_bit()
                .rxuim()
                .clear_bit()
                .rxoim()
                .clear_bit()
                .rxfim()
                .clear_bit()
                .mstim_faeim()
                .clear_bit()
                .txuim()
                .clear_bit()
                .ssrim()
                .clear_bit()
        });
        crate::interrupt::disable::<P::IrqType>();
    }
}

impl<P: SpiInstance + Default> SpiSlaveAsync<P> {
    fn spi(&self) -> Spi<P> {
        unsafe { Spi::steal() }
    }

    /// Async full-duplex slave transfer.
    ///
    /// Pre-loads `tx` into the TX FIFO so slave data is ready on MISO before the
    /// master starts clocking. Subsequent TXEIS interrupts refill the FIFO for
    /// transfers larger than 64 bytes. RXFIS interrupts drain received bytes into
    /// `rx`. Done when all `min(tx.len(), rx.len())` RX bytes are received.
    pub async fn transfer_async(&mut self, tx: &[u8], rx: &mut [u8]) {
        let len = tx.len().min(rx.len());
        if len == 0 {
            return;
        }
        let spi = self.spi();
        // Drain stale state. SDK: Spi_slave_read_stream waits SSI_Busy first.
        while spi.is_busy() {}
        while spi.is_rx_not_empty() {
            let _ = spi.read_data();
        }
        let state = self.peri.state();
        state.done.store(false, Ordering::Release);
        state.tx_ptr.store(tx.as_ptr() as usize, Ordering::Relaxed);
        state.tx_rem.store(len, Ordering::Relaxed);
        state
            .rx_ptr
            .store(rx.as_mut_ptr() as usize, Ordering::Relaxed);
        state.rx_rem.store(len, Ordering::Relaxed);
        spi.regs().imr().modify(|_, w| {
            w.txeim()
                .set_bit()
                .txoim()
                .set_bit()
                .rxfim()
                .set_bit()
                .rxoim()
                .set_bit()
                .rxuim()
                .set_bit()
        });
        SpiFuture {
            ptr: self.peri.ptr(),
            state,
        }
        .await;
    }
}

// ---------------------------------------------------------------------------
// SpiDma — DMA-backed async SPI.  Obtained via Spi::into_dma_async().
// ---------------------------------------------------------------------------

/// Alignment-aware TX DMA params for SPI (all frame sizes 4–16 bit).
/// SDK: ameba_spi.c : SSI_TXGDMA_Init
fn spi_tx_dma_params(
    buf: *const u8,
    len: usize,
    frame_bits: u32,
) -> (
    crate::dma::TrWidth,
    crate::dma::TrWidth,
    crate::dma::Msize,
    crate::dma::Msize,
) {
    use crate::dma::{Msize, TrWidth};
    if frame_bits > 8 {
        // 9–16 bit mode. Dst (SPI DR) = TwoBytes/Eight. SDK: DstDataWidth=TrWidthTwoBytes / DstMsize=MsizeEight
        // 4-byte aligned src: FourBytes/Four/BlockSize=len>>2. SDK: SrcDataWidth=TrWidthFourBytes / SrcMsize=MsizeFour
        // 2-byte aligned src: TwoBytes/Eight/BlockSize=len>>1. SDK: SrcDataWidth=TrWidthTwoBytes / SrcMsize=MsizeEight
        // else: SDK returns FALSE (alignment error). SDK: SSI_TXGDMA_Init log "Aligment Err"
        if (len & 3 == 0) && ((buf as usize) & 3 == 0) {
            (
                TrWidth::FourBytes,
                TrWidth::TwoBytes,
                Msize::Four,
                Msize::Eight,
            )
        } else if (len & 1 == 0) && ((buf as usize) & 1 == 0) {
            (
                TrWidth::TwoBytes,
                TrWidth::TwoBytes,
                Msize::Eight,
                Msize::Eight,
            )
        } else {
            panic!("SPI 16-bit DMA TX: buffer must be at least 2-byte aligned");
        }
    } else {
        // 4–8 bit mode. Dst (SPI DR) = OneByte/Four. SDK: DstDataWidth=TrWidthOneByte / DstMsize=MsizeFour
        // 4-byte aligned src: FourBytes/One/BlockSize=len>>2. SDK: SrcDataWidth=TrWidthFourBytes / SrcMsize=MsizeOne
        // else: OneByte/Four/BlockSize=len. SDK: SrcDataWidth=TrWidthOneByte / SrcMsize=MsizeFour
        if (len & 3 == 0) && ((buf as usize) & 3 == 0) {
            (
                TrWidth::FourBytes,
                TrWidth::OneByte,
                Msize::One,
                Msize::Four,
            )
        } else {
            (TrWidth::OneByte, TrWidth::OneByte, Msize::Four, Msize::Four)
        }
    }
}

/// Alignment-aware RX DMA params for SPI (all frame sizes 4–16 bit).
/// SDK: ameba_spi.c : SSI_RXGDMA_Init
fn spi_rx_dma_params(
    buf: *const u8,
    len: usize,
    frame_bits: u32,
) -> (
    crate::dma::TrWidth,
    crate::dma::TrWidth,
    crate::dma::Msize,
    crate::dma::Msize,
) {
    use crate::dma::{Msize, TrWidth};
    if frame_bits > 8 {
        // 9–16 bit mode. Src (SPI DR) = TwoBytes/Four/BlockSize=len>>1. SDK: SrcDataWidth=TrWidthTwoBytes / SrcMsize=MsizeFour
        // 4-byte aligned dst: FourBytes/Four. SDK: DstDataWidth=TrWidthFourBytes / DstMsize=MsizeFour
        // 2-byte aligned dst: TwoBytes/Eight. SDK: DstDataWidth=TrWidthTwoBytes / DstMsize=MsizeEight
        // else: SDK returns FALSE (alignment error). SDK: SSI_RXGDMA_Init log "Aligment Err"
        if (len & 3 == 0) && ((buf as usize) & 3 == 0) {
            (
                TrWidth::TwoBytes,
                TrWidth::FourBytes,
                Msize::Four,
                Msize::Four,
            )
        } else if (len & 1 == 0) && ((buf as usize) & 1 == 0) {
            (
                TrWidth::TwoBytes,
                TrWidth::TwoBytes,
                Msize::Four,
                Msize::Eight,
            )
        } else {
            panic!("SPI 16-bit DMA RX: buffer must be at least 2-byte aligned");
        }
    } else {
        // 4–8 bit mode. Src (SPI DR) = OneByte/Four/BlockSize=len. SDK: SrcDataWidth=TrWidthOneByte / SrcMsize=MsizeFour
        // 4-byte aligned dst: FourBytes/One. SDK: DstDataWidth=TrWidthFourBytes / DstMsize=MsizeOne
        // else: OneByte/Four. SDK: DstDataWidth=TrWidthOneByte / DstMsize=MsizeFour
        if (len & 3 == 0) && ((buf as usize) & 3 == 0) {
            (
                TrWidth::OneByte,
                TrWidth::FourBytes,
                Msize::Four,
                Msize::One,
            )
        } else {
            (TrWidth::OneByte, TrWidth::OneByte, Msize::Four, Msize::Four)
        }
    }
}

/// SPI handle in DMA-backed async mode. Obtained via [`Spi::into_dma_async`].
pub struct SpiDma<P: SpiInstance, const TX_CH: u8, const RX_CH: u8> {
    _phantom: PhantomData<P>,
    tx_ch: crate::dma::DmaChannel<TX_CH, crate::dma::Async>,
    rx_ch: crate::dma::DmaChannel<RX_CH, crate::dma::Async>,
}

// SAFETY: embedded single-CPU target; `SpiDma` has exclusive ownership of its peripheral and channels.
unsafe impl<P: SpiInstance, const TX_CH: u8, const RX_CH: u8> Send for SpiDma<P, TX_CH, RX_CH> {}

impl<P: SpiInstance + Default, const TX_CH: u8, const RX_CH: u8> SpiDma<P, TX_CH, RX_CH> {
    fn spi(&self) -> Spi<P> {
        unsafe { Spi::steal() }
    }
    fn frame_bits(&self) -> u32 {
        self.spi().frame_size_bits()
    } // SDK: SSI_GetDataFrameSize(SPIx)

    /// Async write: transmit `data`, discard echoed RX bytes.
    ///
    /// TX DMA fills the SPI FIFO; echoed RX bytes (hardware always clocks them in
    /// on TMOD_TR mode) may overflow the 64-entry FIFO for long transfers — this
    /// matches SDK behaviour (`spi_master_write_stream_dma` makes no TMOD change).
    /// SDK: spi_api.c : ssi_dma_send / ssi_dma_tx_irq
    pub async fn write_async(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let len = data.len();
        let (sw, dw, sm, dm) = spi_tx_dma_params(data.as_ptr(), len, self.frame_bits());
        unsafe { crate::cache::dcache_clean(data.as_ptr(), len) }; // SDK: SSI_TXGDMA_Init → DCache_CleanInvalidate
        let spi = self.spi();
        // SDK: example_raw_spi_trx_dma.c : SSI_TXGDMA_Init before SSI_SetDmaEnable(ENABLE, TDMAE)
        let tp = crate::dma::TrParams {
            src_width: sw,
            dst_width: dw,
            src_msize: sm,
            dst_msize: dm,
            hs_id: P::HS_TX,
        };
        let tx_fut = self.tx_ch.periph_tx(data.as_ptr(), spi.dr_ptr(), len, tp);
        spi.set_dma_enable(DmaChannel::Tx, true);
        tx_fut.await;
        spi.set_dma_enable(DmaChannel::Tx, false);
        // Wait for shift register to drain; drain echoed RX bytes to prevent FIFO overflow.
        // is_busy() required: CPU runs faster than SPI clock so FIFO can appear empty while
        // the last frame is still being shifted out. SDK: SSI_Busy() poll after DMA completes.
        while spi.is_busy() || spi.is_rx_not_empty() {
            while spi.is_rx_not_empty() {
                let _ = spi.read_data();
            }
        }
    }

    /// Async full-duplex transfer: transmit `tx`, receive into `rx`.
    ///
    /// RX DMA is started first (ready before TX triggers clocking), then TX DMA.
    /// `_tx_fut` is held alive to prevent its Drop from aborting the TX channel;
    /// TX always completes before RX so Drop is a no-op when it runs.
    /// SDK: spi_api.c : ssi_dma_recv then ssi_dma_send (spi_master_write_read_stream_dma)
    pub async fn transfer_async(&mut self, rx: &mut [u8], tx: &[u8]) {
        let len = rx.len().min(tx.len());
        if len == 0 {
            return;
        }
        let fb = self.frame_bits();
        let (tx_sw, tx_dw, tx_sm, tx_dm) = spi_tx_dma_params(tx.as_ptr(), len, fb);
        let (rx_sw, rx_dw, rx_sm, rx_dm) = spi_rx_dma_params(rx.as_ptr(), len, fb);
        unsafe {
            crate::cache::dcache_clean(tx.as_ptr(), len);
            crate::cache::dcache_clean_invalidate(rx.as_ptr(), len); // SDK: SSI_RXGDMA_Init → DCache_CleanInvalidate
        }
        let spi = self.spi();
        let dr = spi.dr_ptr();
        // SDK: example_raw_spi_trx_dma.c : RXGDMA_Init → SetDmaEnable(RX) → TXGDMA_Init → SetDmaEnable(TX)
        let tp = crate::dma::TrParams {
            src_width: rx_sw,
            dst_width: rx_dw,
            src_msize: rx_sm,
            dst_msize: rx_dm,
            hs_id: P::HS_RX,
        };
        let rx_fut = self
            .rx_ch
            .periph_rx(dr as *const u32, rx.as_mut_ptr(), len, tp);
        spi.set_dma_enable(DmaChannel::Rx, true);
        let tp_tx = crate::dma::TrParams {
            src_width: tx_sw,
            dst_width: tx_dw,
            src_msize: tx_sm,
            dst_msize: tx_dm,
            hs_id: P::HS_TX,
        };
        let _tx_fut = self.tx_ch.periph_tx(tx.as_ptr(), dr, len, tp_tx);
        spi.set_dma_enable(DmaChannel::Tx, true);
        rx_fut.await;
        spi.set_dma_enable(DmaChannel::Both, false);
        unsafe { crate::cache::dcache_invalidate(rx.as_ptr(), len) }; // SDK: ssi_dma_rx_irq → DCache_Invalidate
    }

    /// Async in-place transfer: transmit and overwrite `words` with received bytes.
    ///
    /// TX and RX DMA share the same buffer. Safe because TX reads `words[k]` before
    /// RX overwrites `words[k]` (SPI pipeline depth ≥ 1 frame guarantees ordering).
    pub async fn transfer_in_place_async(&mut self, words: &mut [u8]) {
        if words.is_empty() {
            return;
        }
        let len = words.len();
        let p = words.as_mut_ptr();
        let fb = self.frame_bits();
        let (tx_sw, tx_dw, tx_sm, tx_dm) = spi_tx_dma_params(p as *const u8, len, fb);
        let (rx_sw, rx_dw, rx_sm, rx_dm) = spi_rx_dma_params(p as *const u8, len, fb);
        // TX flush + RX prepare in one op — TX and RX share the same buffer.
        // SDK: SSI_TXGDMA_Init + SSI_RXGDMA_Init both call DCache_CleanInvalidate.
        unsafe { crate::cache::dcache_clean_invalidate(p as *const u8, len) };
        let spi = self.spi();
        let dr = spi.dr_ptr();
        // SDK: example_raw_spi_trx_dma.c : RXGDMA_Init → SetDmaEnable(RX) → TXGDMA_Init → SetDmaEnable(TX)
        let tp_rx = crate::dma::TrParams {
            src_width: rx_sw,
            dst_width: rx_dw,
            src_msize: rx_sm,
            dst_msize: rx_dm,
            hs_id: P::HS_RX,
        };
        let rx_fut = self.rx_ch.periph_rx(dr as *const u32, p, len, tp_rx);
        spi.set_dma_enable(DmaChannel::Rx, true);
        let tp_tx = crate::dma::TrParams {
            src_width: tx_sw,
            dst_width: tx_dw,
            src_msize: tx_sm,
            dst_msize: tx_dm,
            hs_id: P::HS_TX,
        };
        let _tx_fut = self.tx_ch.periph_tx(p as *const u8, dr, len, tp_tx);
        spi.set_dma_enable(DmaChannel::Tx, true);
        rx_fut.await;
        spi.set_dma_enable(DmaChannel::Both, false);
        unsafe { crate::cache::dcache_invalidate(p as *const u8, len) };
    }

    /// Async read: clock out dummy bytes (from `buf` itself), receive into `buf`.
    ///
    /// TX DMA generates the clock using the RX buffer as its source (content irrelevant).
    /// Safe for the same reason as `transfer_in_place_async`: TX reads `buf[k]` before
    /// RX writes `buf[k]`.
    /// SDK: spi_api.c : spi_master_read_stream_dma — uses rx_buffer as TX source too
    pub async fn read_async(&mut self, buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }
        self.transfer_in_place_async(buf).await;
    }
}

// ---------------------------------------------------------------------------
// Peripheral tokens
// ---------------------------------------------------------------------------

impl sealed::Sealed for AnySpi<'_> {}
impl SpiInstance for AnySpi<'static> {
    type IrqType = crate::interrupt::typelevel::IrqAny;
    const PTR: *const RegisterBlock = core::ptr::null();
    const HS_TX: u8 = 0;
    const HS_RX: u8 = 0;

    fn ptr(&self) -> *const RegisterBlock {
        any::delegate!(self, s => { s.ptr() })
    }
    fn state(&self) -> &'static SpiState {
        any::delegate!(self, s => { s.state() })
    }
    fn state_static() -> &'static SpiState {
        unreachable!("AnySpi::state_static called -- use state(&self) instead")
    }
    fn set_role_bit(&self, is_master: bool) {
        any::delegate!(self, s => { s.set_role_bit(is_master) })
    }
}
