//! RTL8721F UART auxiliary driver — `#![no_std]`
//!
//! Covers: DMA burst-size & enable, LP-Rx baud monitor, IrDA SIR,
//! Rx debounce, DMA dummy data, RTS force, Rx-timeout thresholds.
//!
//! Functions that require GDMA (`UART_TXGDMA_Init`, `UART_RXGDMA_Init`)
//! are intentionally omitted — they depend on `GDMA_ChnlAlloc`/`GDMA_Init`.
//!
//! None of these register operations have ROM equivalents in RTL8721F;
//! all are implemented as direct PAC register access.

use crate::mode::{Async, Blocking, DriverMode};
use core::future::Future;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use core::task::{Context, Poll};
use embassy_sync::waitqueue::AtomicWaker;
use rtl8721f_pac::{LpSys, Uart0Reg, Uart1Reg, Uart2Reg, Uart3Reg, uart0_reg};

/// UART IP clock when the clock source is XTAL.
/// SDK: ameba_clk_rom.c : XTAL_ClkGet() returns XTAL_40M (40 MHz, fixed on RTL8721F).
const XTAL_CLK_HZ: u32 = 40_000_000;

// ---------------------------------------------------------------------------
// Configuration types
// ---------------------------------------------------------------------------

/// UART word length.
/// SDK: ameba_uart.h : RUART_WLS_7BITS / RUART_WLS_8BITS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DataBits {
    _7 = 0,
    #[default]
    _8 = 1,
}

/// UART parity.
/// SDK: ameba_uart.h : RUART_PARITY_DISABLE / RUART_PARITY_ENABLE / RUART_ODD_PARITY / RUART_EVEN_PARITY
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Parity {
    #[default]
    None,
    Odd,
    Even,
}

/// UART stop bits.
/// SDK: ameba_uart.h : RUART_STOP_BIT_1 / RUART_STOP_BIT_2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StopBits {
    #[default]
    One = 0,
    Two = 1,
}

/// UART hardware flow control.
/// SDK: ameba_uart.h : RUART_MCL_FLOW_ENABLE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlowControl {
    #[default]
    None,
    RtsCts,
}

/// UART configuration. Default = 8N1 @ 115200, no flow control.
#[derive(Debug, Clone, Copy)]
pub struct UartConfig {
    pub baudrate: u32,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            data_bits: DataBits::_8,
            parity: Parity::None,
            stop_bits: StopBits::One,
            flow_control: FlowControl::None,
        }
    }
}

impl UartConfig {
    pub fn with_baudrate(mut self, baudrate: u32) -> Self {
        self.baudrate = baudrate;
        self
    }
    pub fn with_data_bits(mut self, data_bits: DataBits) -> Self {
        self.data_bits = data_bits;
        self
    }
    pub fn with_parity(mut self, parity: Parity) -> Self {
        self.parity = parity;
        self
    }
    pub fn with_stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = stop_bits;
        self
    }
    pub fn with_flow_control(mut self, flow_control: FlowControl) -> Self {
        self.flow_control = flow_control;
        self
    }
}

// ---------------------------------------------------------------------------
// Register block alias + legacy pointer constants
// ---------------------------------------------------------------------------

pub type RegisterBlock = uart0_reg::RegisterBlock;

// Kept for callers that still reference base addresses (e.g. DMA addr computation).
pub const UART0_PTR: *const RegisterBlock = Uart0Reg::PTR;
pub const UART1_PTR: *const RegisterBlock = Uart1Reg::PTR as *const RegisterBlock;
pub const UART2_PTR: *const RegisterBlock = Uart2Reg::PTR as *const RegisterBlock;
pub const UART3_PTR: *const RegisterBlock = Uart3Reg::PTR as *const RegisterBlock;

// ---------------------------------------------------------------------------
// Low-level register helpers — ROM-free ports of ameba_uart_rom.c
// ---------------------------------------------------------------------------

/// Compute the baud-rate divisor parameters `(ovsr, ovsr_adj)` for `baudrate`.
///
/// Pure integer port of the ROM algorithm. On RTL8721F the UART IP clock is
/// the fixed 40 MHz XTAL ([`XTAL_CLK_HZ`]) — `UART_SetBaud` passes `XTAL_ClkGet()`,
/// which returns `XTAL_40M` unconditionally.
/// - `ovsr`     → integer divisor (STS.XFACTOR / MON_BAUD cycnum_perbit)
/// - `ovsr_adj` → 11-bit fractional adjust (SCR.XFACTOR_ADJ / RX_PATH_CTRL.RXBAUD_ADJ)
///
/// SDK: ameba_uart_rom.c : UART_BaudParaGetFull (lines 147-169); IPclk source
///      is `XTAL_ClkGet()` (UART_SetBaud line 184).
fn baud_para_get(baudrate: u32) -> (u32, u32) {
    // SDK: ameba_uart_rom.c:155-158
    let ovsr = XTAL_CLK_HZ / baudrate;
    let remainder = XTAL_CLK_HZ % baudrate;

    // SDK: ameba_uart_rom.c:161-165 — 11-iteration fractional accumulation.
    let mut ovsr_adj: u32 = 0;
    for i in 0..11u32 {
        ovsr_adj <<= 1;
        let temp_multly = remainder * (12 - i);
        if (temp_multly / baudrate) != ((temp_multly - remainder) / baudrate) {
            ovsr_adj |= 1;
        }
    }
    (ovsr, ovsr_adj)
}

/// Rx FIFO has valid data? (LSR.DRDY, bit 0)
/// SDK: ameba_uart_rom.c : UART_Readable (lines 338-348)
#[inline(always)]
fn uart_readable(rb: &RegisterBlock) -> bool {
    rb.uart_lsr().read().drdy().bit_is_set()
}

/// Tx FIFO writable (not full)? (LSR.TX_NOT_FULL, bit 6)
/// SDK: ameba_uart_rom.c : UART_Writable (lines 320-329)
#[inline(always)]
fn uart_writable(rb: &RegisterBlock) -> bool {
    rb.uart_lsr().read().tx_not_full().bit_is_set()
}

/// Write one byte to the Tx FIFO (THR).
/// SDK: ameba_uart_rom.c : UART_CharPut (lines 355-360)
#[inline(always)]
fn uart_char_put(rb: &RegisterBlock, byte: u8) {
    rb.uart_rbr_or_uart_thr()
        .write(|w| unsafe { w.databit().bits(byte) });
}

/// Read one byte from the Rx FIFO (RBR).
/// SDK: ameba_uart_rom.c : UART_CharGet (lines 367-372)
#[inline(always)]
fn uart_char_get(rb: &RegisterBlock) -> u8 {
    rb.uart_rbr_or_uart_thr().read().databit().bits()
}

/// Drain whatever is currently in the Rx FIFO into `buf`, stopping as soon as
/// the FIFO reports empty — never busy-waits. Safe to call from a
/// [`UartIrqHandler::on_irq`] callback (unlike `embedded_io::Read`, which
/// blocks on the first byte if none is ready yet).
///
/// Takes a raw register-block pointer (e.g. [`UART0_PTR`]) rather than a
/// `Uart<P, Dm>` handle, so a raw-ISR callback doesn't need its own `steal()`'d
/// instance racing the owner's.
///
/// # Safety
/// `ptr` must point to a valid, live UART register block (e.g. one of the
/// `UART0_PTR`/`UART1_PTR`/`UART2_PTR` constants).
pub unsafe fn read_available(ptr: *const RegisterBlock, buf: &mut [u8]) -> usize {
    let rb = unsafe { &*ptr };
    let mut n = 0;
    while n < buf.len() && uart_readable(rb) {
        buf[n] = uart_char_get(rb);
        n += 1;
    }
    n
}

// ---------------------------------------------------------------------------
// Sealed trait (prevents external UartInstance impls)
// ---------------------------------------------------------------------------

mod sealed {
    pub trait Sealed {}
}

// ---------------------------------------------------------------------------
// UartInstance: compile-time per-peripheral constants
// ---------------------------------------------------------------------------

/// Ownership token for UART0 peripheral.
#[derive(Default, Debug)]
pub struct Uart0;
/// Ownership token for UART1 peripheral.
#[derive(Default, Debug)]
pub struct Uart1;
/// Ownership token for UART2 peripheral.
#[derive(Default, Debug)]
pub struct Uart2;
/// Ownership token for UART3 peripheral.
#[derive(Default, Debug)]
pub struct Uart3;

crate::any_peripheral! {
    pub peripheral AnyUart<'d> {
        Uart0(Uart0),
        Uart1(Uart1),
        Uart2(Uart2),
        Uart3(Uart3),
    }
}

/// Implemented by [`Uart0`], [`Uart1`], [`Uart2`], [`Uart3`].
///
/// Carries all compile-time hardware constants for one UART peripheral.
/// Use the typelevel tag (`type IrqType`) together with [`crate::bind_interrupts!`]
/// to register an interrupt handler and receive a [`crate::interrupt::Binding`] token.
pub trait UartInstance: sealed::Sealed + Send + any::Degrade + 'static {
    /// Typelevel IRQ tag, used as `I` in [`crate::interrupt::Binding<I, H>`].
    type IrqType: crate::interrupt::Irq;
    /// Non-Secure register-block base address (derived from PAC `::PTR`).
    const PTR: *const RegisterBlock;
    /// GDMA hardware handshake interface for TX.
    /// SDK: ameba_gdma.h : GDMA_HANDSHAKE_INTERFACE_UARTx_TX
    const HS_TX: u8;
    /// GDMA hardware handshake interface for RX.
    /// SDK: ameba_gdma.h : GDMA_HANDSHAKE_INTERFACE_UARTx_RX
    const HS_RX: u8;
    /// Shared async waker/flag state for this instance (static, for interrupt handler).
    fn state_static() -> &'static UartState;
    /// Shared async waker/flag state (instance, delegable by AnyUart).
    fn state(&self) -> &'static UartState {
        Self::state_static()
    }
    /// Switch the UART RX clock source to XTAL.
    /// SDK: sysreg_lsys.h : LSYS_MASK_CKSL_UARTx (bit 1 = XTAL)
    fn set_clock_source_xtal(&self);
    /// Register-block pointer (defaults to [`Self::PTR`]; overridden by Any types).
    fn ptr(&self) -> *const RegisterBlock {
        Self::PTR
    }
}

macro_rules! impl_uart_instance {
    (
        $ty:ty, $irq_type:ty, $pac:ty,
        $hs_tx:expr_2021, $hs_rx:expr_2021, $state:ident, $cksl_bit:ident
    ) => {
        impl sealed::Sealed for $ty {}
        impl $crate::private::Sealed for $ty {}
        impl UartInstance for $ty {
            type IrqType = $irq_type;
            const PTR: *const RegisterBlock = <$pac>::PTR as *const RegisterBlock;
            const HS_TX: u8 = $hs_tx;
            const HS_RX: u8 = $hs_rx;
            #[inline(always)]
            fn ptr(&self) -> *const RegisterBlock {
                Self::PTR
            }
            fn state_static() -> &'static UartState {
                &$state
            }
            #[inline(always)]
            fn set_clock_source_xtal(&self) {
                unsafe {
                    (*LpSys::PTR)
                        .reg_cksl_grp1()
                        .modify(|_, w| w.$cksl_bit().set_bit());
                }
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Interrupt handler type
// ---------------------------------------------------------------------------

/// Embassy-style interrupt handler for a typed UART instance.
///
/// Register it via [`crate::bind_interrupts!`], then pass the resulting token
/// to [`Uart::into_async`]:
///
/// ```rust,ignore
/// bind_interrupts!(struct Irqs {
///     UART0_IRQ => uart::InterruptHandler<uart::Uart0>;
/// });
///
/// let uart = unsafe { Uart::<Uart0>::steal() };
/// uart.init(&UartConfig::default());
/// let uart_async = uart.into_async(Irqs);
/// ```
pub struct InterruptHandler<P: UartInstance>(PhantomData<P>);

impl<P: UartInstance> crate::interrupt::Handler<P::IrqType> for InterruptHandler<P> {
    unsafe fn on_interrupt() {
        uart_int_dispatch(P::PTR, P::state_static());
    }
}

// ---------------------------------------------------------------------------
// Raw ISR callback (user-registered, independent of the async waker path)
// ---------------------------------------------------------------------------

/// Raw UART interrupt event, decoded from the LSR/IER snapshot taken by
/// `uart_int_dispatch` before it clears/acks the hardware. Mirrors GPIO's
/// `GpioEvent` — the callback gets "what happened", not raw register access;
/// hardware-level ack (IER clear, TOICF latch) is already done by the time
/// this is delivered.
#[derive(Clone, Copy, Debug, Default)]
pub struct UartIrqEvent {
    /// Rx FIFO reached its trigger level (LSR.RXFIFO_INT).
    pub rx_ready: bool,
    /// Rx timeout — FIFO non-empty but below trigger level (LSR.TIMEOUT_INT).
    pub rx_timeout: bool,
    /// Overrun / parity / framing / break / Rx-FIFO error (LSR error bits).
    pub rx_error: bool,
    /// Raw LSR register snapshot at dispatch time, for handlers needing detail
    /// beyond the decoded flags above.
    pub lsr: u32,
}

/// User-registered raw ISR callback for a UART instance. Runs directly in the
/// UART's NVIC interrupt context, independent of the `Uart::read_async`/
/// `write_async` waker path. Register via [`Uart::register_irq`].
pub trait UartIrqHandler: Send {
    fn on_irq(&mut self, event: UartIrqEvent);
}

/// Type-erased trampoline stored in `UartState::irq_handler`, resolved back to
/// `H::on_irq` via the matching `irq_data` pointer. Mirrors GPIO's
/// `handler_trampoline`.
unsafe fn uart_handler_trampoline<H: UartIrqHandler>(data: usize, event: UartIrqEvent) {
    let handler = unsafe { &mut *(data as *mut H) };
    handler.on_irq(event);
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// IrDA SIR pulse-edge shift direction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PulseShift {
    Left,
    Right,
}

/// Configuration for the IrDA SIR interface.
#[derive(Clone, Copy, Debug)]
pub struct IrDaConfig {
    pub rx_invert: bool,
    pub tx_invert: bool,
    pub low_shift: PulseShift,
    /// 15-bit value (0..=0x7FFF)
    pub low_shift_val: u16,
    pub upper_shift: PulseShift,
    /// 15-bit value (0..=0x7FFF)
    pub upper_shift_val: u16,
    pub rx_filter_enable: bool,
    /// 15-bit value
    pub rx_filter_thres: u16,
}

impl Default for IrDaConfig {
    fn default() -> Self {
        Self {
            rx_invert: false,
            tx_invert: false,
            low_shift: PulseShift::Left,
            low_shift_val: 0,
            upper_shift: PulseShift::Left,
            upper_shift_val: 0,
            rx_filter_enable: true,
            rx_filter_thres: 7,
        }
    }
}

// ---------------------------------------------------------------------------
// Uart driver struct
// ---------------------------------------------------------------------------

/// Handle to one RTL8721F UART peripheral.
///
/// `P` is the peripheral type (`Uart0`, `Uart1`, `Uart2`, or `Uart3`); `Dm` is
/// the [`DriverMode`] — [`Blocking`] (default) or [`Async`]. Convert between
/// modes with [`Uart::into_async`] / [`Uart::into_blocking`].
///
/// Construct via [`Uart::steal`].
pub struct Uart<P: UartInstance, Dm: DriverMode = Blocking> {
    peri: P,
    _dm: PhantomData<Dm>,
}

// SAFETY: embedded single-CPU target; `Uart<P, Dm>` has exclusive ownership of its peripheral.
unsafe impl<P: UartInstance, Dm: DriverMode> Send for Uart<P, Dm> {}

impl<P: UartInstance + Default, Dm: DriverMode> Uart<P, Dm> {
    /// Construct a `Uart<P, Dm>` handle.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other `Uart<P, _>` instance for the same peripheral is alive
    /// concurrently.
    pub unsafe fn steal() -> Self {
        Self {
            peri: P::default(),
            _dm: PhantomData,
        }
    }
}

impl<P: UartInstance, Dm: DriverMode> Uart<P, Dm> {
    #[inline(always)]
    fn regs(&self) -> &RegisterBlock {
        unsafe { &*self.peri.ptr() }
    }

    /// Address of the RX/TX FIFO data register (`RBR`/`THR`), for use as a
    /// GDMA source/destination. The address comes from the PAC register block
    /// (svd2rust → SVD), not a hand-written offset.
    #[inline(always)]
    pub fn dr_ptr(&self) -> *mut u32 {
        self.regs().uart_rbr_or_uart_thr().as_ptr()
    }

    // -----------------------------------------------------------------------
    // DMA burst size & enable
    // -----------------------------------------------------------------------

    /// Set the TX DMA burst size (max 16; should match GDMA_DstMsize).
    pub fn tx_dma_burst_size(&self, size: u8) {
        self.regs()
            .uart_miscr()
            .modify(|_, w| unsafe { w.txdma_burstsize().bits(size) });
    }

    /// Set the RX DMA burst size (max 32 due to DMA FIFO; should match GDMA_SrcMsize).
    pub fn rx_dma_burst_size(&self, size: u8) {
        self.regs()
            .uart_miscr()
            .modify(|_, w| unsafe { w.rxdma_burstsize().bits(size) });
    }

    /// Enable or disable TX DMA.
    pub fn tx_dma_cmd(&self, enable: bool) {
        self.regs()
            .uart_miscr()
            .modify(|_, w| w.txdma_en().bit(enable));
    }

    /// Enable or disable RX DMA.
    pub fn rx_dma_cmd(&self, enable: bool) {
        self.regs()
            .uart_miscr()
            .modify(|_, w| w.rxdma_en().bit(enable));
    }

    // -----------------------------------------------------------------------
    // LP Rx baud monitor
    // -----------------------------------------------------------------------

    /// Configure the LP Rx baud monitor parameters.
    ///
    /// `bit_num_thres` must be in 1..=127.
    /// `osc_perbit_upd` enables auto-update of the OSC cycnum_perbit value.
    pub fn monitor_para_config(&self, bit_num_thres: u8, osc_perbit_upd: bool) {
        // Step 1: assert reset of low-power Rx path (active-low).
        self.regs()
            .uart_rx_path_ctrl()
            .modify(|_, w| w.r_rst_newrx_n().clear_bit());

        self.regs().uart_mon_baud_ctrl().modify(|_, w| unsafe {
            w.r_bit_num_thres()
                .bits(bit_num_thres)
                .r_upd_osc_in_xtal()
                .bit(osc_perbit_upd)
        });
    }

    /// Set the LP Rx baud rate.
    ///
    /// `rx_ip_clock_hz` is the UART Rx clock frequency.
    pub fn lp_rx_baud_set(&self, baud_rate: u32, rx_ip_clock_hz: u32) {
        let cyc_perbit = if (rx_ip_clock_hz % baud_rate) >= baud_rate.div_ceil(2) {
            rx_ip_clock_hz / baud_rate + 1
        } else {
            rx_ip_clock_hz / baud_rate
        };

        self.regs()
            .uart_mon_baud_sts()
            .modify(|_, w| unsafe { w.r_cycnum_perbit_xtal().bits(cyc_perbit) });

        self.regs()
            .uart_mon_baud_ctrl()
            .modify(|_, w| unsafe { w.r_cycnum_perbit_osc().bits(cyc_perbit) });

        // Clear the Rx baud adjustment factor.
        self.regs()
            .uart_rx_path_ctrl()
            .modify(|_, w| unsafe { w.rxbaud_adj_10_0().bits(0) });
    }

    /// Enable or disable LP Rx baud monitoring.
    pub fn rx_monitor_cmd(&self, enable: bool) {
        self.regs()
            .uart_mon_baud_ctrl()
            .modify(|_, w| w.r_mon_baud_en().bit(enable));
    }

    /// Read the raw `MON_BAUD_CTRL` register value.
    ///
    /// Bits `[28:9]` contain the OSC cycnum_perbit monitor result.
    pub fn rx_mon_baud_ctrl_get(&self) -> u32 {
        self.regs().uart_mon_baud_ctrl().read().bits()
    }

    /// Read the raw `MON_BAUD_STS` register value.
    ///
    /// Reading this register clears the monitor interrupt status.
    /// Bits `[19:0]` contain the XTAL cycnum_perbit monitor result.
    pub fn rx_monitor_status_get(&self) -> u32 {
        self.regs().uart_mon_baud_sts().read().bits()
    }

    // -----------------------------------------------------------------------
    // IrDA SIR
    // -----------------------------------------------------------------------

    /// Configure the IrDA SIR interface from an `IrDaConfig`.
    pub fn irda_init(&self, cfg: &IrDaConfig) {
        self.regs().uart_miscr().modify(|_, w| {
            w.irda_rx_inv()
                .bit(cfg.rx_invert)
                .irda_tx_inv()
                .bit(cfg.tx_invert)
        });

        self.regs().uart_sir_tx_pwc0().modify(|_, w| unsafe {
            w.upperbound_shiftright()
                .bit(cfg.upper_shift == PulseShift::Right)
                .upperbound_shiftval()
                .bits(cfg.upper_shift_val)
                .lowbound_shiftright()
                .bit(cfg.low_shift == PulseShift::Right)
                .lowbound_shiftval()
                .bits(cfg.low_shift_val)
        });

        self.regs().uart_sir_rx_pfc().modify(|_, w| unsafe {
            w.r_sir_rx_filter_thrs()
                .bits(cfg.rx_filter_thres)
                .r_sir_rx_filter_en()
                .bit(cfg.rx_filter_enable)
        });
    }

    /// Enable or disable the IrDA SIR mode.
    pub fn irda_cmd(&self, enable: bool) {
        self.regs()
            .uart_miscr()
            .modify(|_, w| w.irda_enable().bit(enable));
    }

    // -----------------------------------------------------------------------
    // Rx debounce
    // -----------------------------------------------------------------------

    /// Set the Rx debounce cycle count (unit: sclk cycles; 15-bit value).
    pub fn rx_debounce_config(&self, cycles: u16) {
        self.regs()
            .uart_rxdbcr()
            .modify(|_, w| unsafe { w.dbnc_cyc().bits(cycles) });
    }

    /// Enable or disable Rx debounce.
    pub fn rx_debounce_cmd(&self, enable: bool) {
        self.regs()
            .uart_rxdbcr()
            .modify(|_, w| w.dbnc_fen().bit(enable));
    }

    // -----------------------------------------------------------------------
    // DMA dummy data
    // -----------------------------------------------------------------------

    /// Set the RX DMA dummy byte (padded when UART is DMA flow controller).
    pub fn rx_dma_dummy_data_config(&self, byte: u8) {
        self.regs()
            .uart_miscr()
            .modify(|_, w| unsafe { w.dummy_data().bits(byte) });
    }

    /// Return `true` if the master has read dummy data from the Rx FIFO.
    ///
    /// `CLR_DUMMY_FLAG` (bit 30) is write-only in the PAC; we read the raw
    /// register word and inspect that bit directly.
    pub fn get_rx_dma_dummy_flag(&self) -> bool {
        let bits = self.regs().uart_miscr().read().bits();
        (bits >> 30) & 1 != 0
    }

    /// Clear the RX DMA dummy flag.
    pub fn rx_clear_dma_dummy_flag(&self) {
        self.regs()
            .uart_miscr()
            .modify(|_, w| w.clr_dummy_flag().set_bit());
    }

    // -----------------------------------------------------------------------
    // RTS force
    // -----------------------------------------------------------------------

    /// Force RTS output.
    ///
    /// `force_inactive = true`  → RTS output is logic 1 (inactive), telling
    ///   the peer to stop sending.
    /// `force_inactive = false` → RTS output tracks Rx FIFO level (auto-flow),
    ///   telling the peer to continue sending.
    ///
    /// `MCR.AFE` must be set before calling this.
    pub fn rts_force_cmd(&self, force_inactive: bool) {
        // Hardware polarity: MCR.RTS = 0 → rts output logic 1 (inactive).
        self.regs()
            .uart_mcr()
            .modify(|_, w| w.rts().bit(!force_inactive));
    }

    // -----------------------------------------------------------------------
    // Rx timeout thresholds
    // -----------------------------------------------------------------------

    /// Set the Rx timeout threshold (unit: one bit-time; 16-bit value).
    pub fn rx_to_thres_config(&self, timeout: u16) {
        self.regs()
            .uart_rx_path_ctrl()
            .modify(|_, w| unsafe { w.r_rxto_thrs().bits(timeout) });
    }

    /// Set the Rx FIFO empty timeout threshold (unit: uart_rclk cycles; 28-bit value).
    pub fn rx_reti_thres_config(&self, timeout: u32) {
        self.regs()
            .uart_th_reti()
            .modify(|_, w| unsafe { w.th_reti().bits(timeout) });
    }

    // -----------------------------------------------------------------------
    // Init / async API
    // -----------------------------------------------------------------------

    /// Enable or disable internal hardware loopback (MCR.LOOP_EN).
    pub fn loopback_cmd(&self, enable: bool) {
        self.regs()
            .uart_mcr()
            .modify(|_, w| w.loop_en().bit(enable));
    }

    /// Enable or disable FIFO DMA mode (FCR.DMA_MODE).
    ///
    /// Must be called after `init()`. Rewrites FCR with FIFO enabled and the
    /// DMA_MODE bit set/cleared. Required before starting UART GDMA transfers.
    pub fn dma_mode_cmd(&self, enable: bool) {
        self.regs().uart_fcr().write(|w| unsafe {
            w.fifo_en()
                .set_bit()
                .dma_mode()
                .bit(enable)
                .recvtrg()
                .bits(0)
        });
    }

    /// Enable (`true`) or disable (`false`) the UART receiver.
    /// SDK: ameba_uart_rom.c : UART_RxCmd (lines 301-311) — RX_PATH_CTRL.R_RST_NEWRX_N.
    fn rx_cmd(&self, enable: bool) {
        self.regs()
            .uart_rx_path_ctrl()
            .modify(|_, w| w.r_rst_newrx_n().bit(enable));
    }

    /// Clear the Rx FIFO, polling DRDY up to 5 times for it to drain.
    /// SDK: ameba_uart_rom.c : UART_ClearRxFifo (lines 538-563).
    fn clear_rx_fifo(&self) {
        // Write 1 to FCR[1] (recv_clr) to flush the Rx FIFO.
        self.regs().uart_fcr().modify(|_, w| w.recv_clr().set_bit());
        let mut wait = 5;
        while wait > 0 {
            if !self.regs().uart_lsr().read().drdy().bit_is_set() {
                break;
            }
            wait -= 1;
        }
    }

    /// Clear the Tx FIFO.
    /// SDK: ameba_uart_rom.c : UART_ClearTxFifo (lines 569-574) — FCR[2] (xmit_clr).
    fn clear_tx_fifo(&self) {
        self.regs().uart_fcr().modify(|_, w| w.xmit_clr().set_bit());
    }

    /// Program the baud-rate divisor registers for `baudrate`.
    /// SDK: ameba_uart_rom.c : UART_SetBaud (lines 176-222).
    fn set_baud(&self, baudrate: u32) {
        let (ovsr, ovsr_adj) = baud_para_get(baudrate);
        let rb = self.regs();

        // Set DLAB to access divisor path; clear the Rx break interrupt status.
        rb.uart_lcr().modify(|_, w| w.dlab().set_bit());
        rb.uart_scr().modify(|_, w| w.scratch_7().set_bit());

        // OVSR → STS.XFACTOR (Tx path integer divisor).
        rb.uart_sts()
            .modify(|_, w| unsafe { w.xfactor().bits(ovsr) });
        // OVSR_ADJ → SCR.XFACTOR_ADJ (Tx path fractional adjust, 11-bit).
        rb.uart_scr()
            .modify(|_, w| unsafe { w.xfactor_adj().bits(ovsr_adj as u16) });

        // Clear DLAB.
        rb.uart_lcr().modify(|_, w| w.dlab().clear_bit());

        // Rx path baud configuration.
        rb.uart_mon_baud_sts()
            .modify(|_, w| unsafe { w.r_cycnum_perbit_xtal().bits(ovsr) });
        rb.uart_mon_baud_ctrl()
            .modify(|_, w| unsafe { w.r_cycnum_perbit_osc().bits(ovsr) });
        rb.uart_rx_path_ctrl()
            .modify(|_, w| unsafe { w.rxbaud_adj_10_0().bits(ovsr_adj as u16) });
    }

    /// Initialize UART with the given configuration.
    ///
    /// Call `ameba_hal::rcc::enable(rcc::Peripheral::UARTx)` before this.
    /// SDK: ameba_uart_rom.c : UART_Init (lines 60-124) + UART_StructInit defaults
    ///      (lines 38-52); serial_api.c : serial_init / serial_baud.
    pub fn init(&self, config: &UartConfig) {
        let rb = self.regs();

        // Disable Rx path, then flush both FIFOs. SDK: UART_Init:69-75.
        rb.uart_rx_path_ctrl()
            .modify(|_, w| w.r_rst_newrx_n().clear_bit());
        self.clear_rx_fifo();
        self.clear_tx_fifo();

        // Disable all interrupts (IER reset value = 0). SDK: UART_Init:78.
        rb.uart_ier().reset();

        // Clear Rx break signal interrupt status. SDK: UART_Init:81.
        rb.uart_scr().modify(|_, w| w.scratch_7().set_bit());

        // FCR: Rx trigger level = 1 byte (0), enable FIFO (RxErReport = ENABLE default).
        // SDK: UART_Init:83-96 + UART_StructInit RxFifoTrigLevel=1BYTE / RxErReportCtrl=ENABLE.
        rb.uart_fcr()
            .modify(|_, w| unsafe { w.recvtrg().bits(0).fifo_en().set_bit() });

        // Hardware flow control (AFE + RTS). SDK: UART_Init:98-103.
        let flow = matches!(config.flow_control, FlowControl::RtsCts);
        rb.uart_mcr()
            .modify(|_, w| w.afe().bit(flow).rts().bit(flow));

        // Word length / stop bits / parity — full LCR write (clears DLAB & break).
        // SDK: UART_Init:105-110. StickParity is always DISABLE (UART_StructInit:44).
        let word_8bit = matches!(config.data_bits, DataBits::_8);
        let stop_2 = matches!(config.stop_bits, StopBits::Two);
        let (parity_en, even) = match config.parity {
            Parity::None => (false, false),
            Parity::Odd => (true, false),
            Parity::Even => (true, true),
        };
        rb.uart_lcr().write(|w| {
            w.wls0()
                .bit(word_8bit)
                .stb()
                .bit(stop_2)
                .pen()
                .bit(parity_en)
                .eps()
                .bit(even)
                .stkp()
                .clear_bit()
        });

        // Enable FIFO DMA mode (DmaModeCtrl = ENABLE default). SDK: UART_Init:112-117.
        rb.uart_fcr().modify(|_, w| w.dma_mode().set_bit());

        // Rx timeout counter = 64. SDK: UART_Init:119-123 + UART_StructInit RxTimeOutCnt=64.
        rb.uart_rx_path_ctrl()
            .modify(|_, w| unsafe { w.r_rxto_thrs().bits(64) });

        // Select XTAL clock, program baud, enable receiver. SDK: serial_api.c serial_baud.
        self.peri.set_clock_source_xtal();
        self.set_baud(config.baudrate);
        self.rx_cmd(true);
    }

    /// Change baud rate without reinitialising the peripheral.
    /// SDK: serial_api.c : serial_baud
    pub fn set_baudrate(&self, baudrate: u32) {
        self.peri.set_clock_source_xtal();
        self.set_baud(baudrate);
        self.rx_cmd(true);
    }

    /// Disable interrupt-driven operation: clear every peripheral-level
    /// interrupt-enable bit, then mask the shared NVIC line.
    ///
    /// Order must not reverse — a still-enabled peripheral interrupt could latch
    /// pending after the NVIC is masked, then fire on the next `enable`. Used by
    /// both `Drop` and [`into_blocking`](Uart::into_blocking). Idempotent, so it
    /// is harmless on a handle that was never in async mode.
    fn disable_interrupts(&self) {
        let rb = unsafe { &*self.peri.ptr() };
        rb.uart_ier().reset();
        crate::interrupt::disable::<P::IrqType>();
    }

    // -----------------------------------------------------------------------
    // Raw ISR callback (register_irq) — mirrors ameba_hal::gpio::GpioPin::register_irq
    // -----------------------------------------------------------------------

    /// Register a raw ISR callback for Rx events on this instance.
    ///
    /// Requires a [`crate::interrupt::Binding`] token proving `InterruptHandler<P>`
    /// is wired to `P::IrqType` (same token `into_async` takes — obtain it via
    /// [`crate::bind_interrupts!`]); this call itself unmasks the NVIC line, so
    /// `into_async`/`into_dma_async` are not required to use this standalone.
    ///
    /// `handler.on_irq()` runs directly in interrupt context on every Rx data /
    /// timeout / error event, re-armed after each call for continuous delivery.
    /// TX events are not covered on this path — re-arming ETBEI unconditionally
    /// would storm (LSR.TX_EMPTY stays set until new data is written), unlike
    /// the Rx bits which are self-clearing once the handler drains the FIFO.
    pub fn register_irq<H, B>(&self, handler: &'static mut H, _binding: B)
    where
        H: UartIrqHandler + 'static,
        B: crate::interrupt::Binding<P::IrqType, InterruptHandler<P>>,
    {
        let state = self.peri.state();
        let trampoline: unsafe fn(usize, UartIrqEvent) = uart_handler_trampoline::<H>;
        state
            .irq_handler
            .store(trampoline as usize, Ordering::Release);
        state
            .irq_data
            .store(handler as *mut H as usize, Ordering::Release);
        crate::interrupt::enable::<P::IrqType, InterruptHandler<P>, B>();
        self.regs()
            .uart_ier()
            .modify(|_, w| w.erbi().set_bit().etoi().set_bit().elsi().set_bit());
    }

    /// Deregister the raw ISR callback: clear the Rx interrupt-enable bits and
    /// the state slot. Does not mask the shared NVIC line (mirrors GPIO's
    /// `unregister_irq`) — a concurrently active async handle on the same
    /// instance keeps working.
    pub fn unregister_irq(&self) {
        self.regs()
            .uart_ier()
            .modify(|_, w| w.erbi().clear_bit().etoi().clear_bit().elsi().clear_bit());
        let state = self.peri.state();
        state.irq_handler.store(0, Ordering::Release);
        state.irq_data.store(0, Ordering::Release);
    }
}

impl<P: UartInstance> Uart<P, Blocking> {
    /// Convert to DMA-backed async mode.
    ///
    /// Sets TX/RX DMA burst sizes then enables NVIC IRQs for both channels.
    /// SDK: example_raw_uart_dma_stream.c : DMA_TX_BURST_SIZE=8 / DMA_RX_BURST_SIZE=16
    pub fn into_dma_async<const TX_CH: u8, const RX_CH: u8>(
        self,
        tx_ch: crate::dma::DmaChannel<TX_CH, Blocking>,
        rx_ch: crate::dma::DmaChannel<RX_CH, Blocking>,
    ) -> UartDma<P, TX_CH, RX_CH> {
        // SDK: example_raw_uart_dma_stream.c : uart_dma_send UART_TXDMAConfig(DMA_TX_BURST_SIZE=8)
        self.tx_dma_burst_size(8);
        // SDK: example_raw_uart_dma_stream.c : uart_dma_recv UART_RXDMAConfig(DMA_RX_BURST_SIZE=16)
        self.rx_dma_burst_size(16);
        // Suppress the blocking handle's teardown Drop: the UART IRQ line is not
        // used in DMA mode, and DMA relies on the MISCR bits we just set.
        core::mem::forget(self);
        UartDma {
            _phantom: PhantomData,
            tx_ch: tx_ch.into_async(),
            rx_ch: rx_ch.into_async(),
        }
    }

    /// Convert to interrupt-driven async mode.
    ///
    /// Requires a [`crate::interrupt::Binding`] token proving that
    /// `InterruptHandler<P>` is registered as the vector-table entry for
    /// `P::IrqType`. Obtain the token via [`crate::bind_interrupts!`].
    pub fn into_async<B>(self, _: B) -> Uart<P, Async>
    where
        B: crate::interrupt::Binding<P::IrqType, InterruptHandler<P>>,
    {
        // Suppress the blocking handle's teardown Drop (it would mask the NVIC
        // line we are about to unmask); then enable.
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        crate::interrupt::enable::<P::IrqType, InterruptHandler<P>, B>();
        Uart {
            peri,
            _dm: PhantomData,
        }
    }
}

pub mod eh;

// ---------------------------------------------------------------------------
// Mode teardown + async API (Uart<P, Async>)
// ---------------------------------------------------------------------------

impl<P: UartInstance, Dm: DriverMode> Drop for Uart<P, Dm> {
    fn drop(&mut self) {
        // Symmetric teardown for the single `interrupt::enable` in `into_async`.
        // Only an `Async` handle owns that enable; guard on the const so a
        // `Blocking` drop (incl. the transient `steal()` handles the DMA path
        // creates) folds to a no-op and never touches interrupt state.
        if Dm::IS_ASYNC {
            self.disable_interrupts();
        }
    }
}

impl<P: UartInstance> Uart<P, Async> {
    /// Convert back to blocking mode: tear down the interrupt-driven setup
    /// (clear peripheral int-enable bits, mask the NVIC line) and hand back a
    /// [`Blocking`] handle. Symmetric counterpart to [`into_async`](Uart::into_async).
    pub fn into_blocking(self) -> Uart<P, Blocking> {
        self.disable_interrupts();
        // Use ManuallyDrop to prevent Drop (would repeat disable_interrupts).
        let this = core::mem::ManuallyDrop::new(self);
        let peri = unsafe { core::ptr::read(&this.peri) };
        Uart {
            peri,
            _dm: PhantomData,
        }
    }

    /// Async TX: fill TX FIFO via UART_Writable until full, await TX-empty
    /// interrupt, refill, repeat until all bytes are sent.
    pub async fn write_async(&self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        let mut pos = 0;
        while pos < data.len() {
            let state = self.peri.state();
            state.tx_triggered.store(0, Ordering::Release);

            // Fill TX FIFO until full or data exhausted — no hardcoded depth constant.
            // SDK: serial_api.c : uart_irqhandler → UART_SendDataTO (UART_Writable per byte)
            let rb = self.regs();
            while pos < data.len() {
                if !uart_writable(rb) {
                    break;
                }
                uart_char_put(rb, data[pos]);
                pos += 1;
            }

            if pos < data.len() {
                // FIFO full, more data remains — wait for TX-empty interrupt, then refill.
                self.regs().uart_ier().modify(|_, w| w.etbei().set_bit());
                UartTxFuture {
                    ptr: self.peri.ptr(),
                    state,
                }
                .await;
            }
        }
    }

    /// Async flush: wait until TX FIFO is empty (RUART_BIT_TX_EMPTY / LSR bit 5).
    ///
    /// RTL8721F has no TEMT bit; the shift register may still hold the last byte
    /// after this returns (~1 byte-time at the configured baud rate).
    /// SDK: ameba_uart.h : RUART_BIT_TX_EMPTY
    pub async fn flush_async(&self) {
        if !self.regs().uart_lsr().read().tx_empty().bit_is_set() {
            let state = self.peri.state();
            state.tx_triggered.store(0, Ordering::Release);
            self.regs().uart_ier().modify(|_, w| w.etbei().set_bit());
            UartTxFuture {
                ptr: self.peri.ptr(),
                state,
            }
            .await;
        }
    }

    /// Async RX: enable RX data/timeout/line-status interrupts and wait for data arrival.
    ///
    /// ELSI is enabled alongside ERBI+ETOI so that framing/parity/overrun errors
    /// trigger the future instead of causing an interrupt storm.
    /// SDK: serial_api.c : uart_irqhandler — always enables RUART_BIT_ELSI with ERBI+ETOI
    pub fn read_async(&self) -> UartRxFuture {
        let state = self.peri.state();
        state.rx_triggered.store(0, Ordering::Release);
        self.regs()
            .uart_ier()
            .modify(|_, w| w.erbi().set_bit().etoi().set_bit().elsi().set_bit());
        UartRxFuture {
            ptr: self.peri.ptr(),
            state,
        }
    }
}

// ---------------------------------------------------------------------------
// Async interrupt state
// ---------------------------------------------------------------------------

pub struct UartState {
    pub(crate) rx_waker: AtomicWaker,
    pub(crate) tx_waker: AtomicWaker,
    pub(crate) rx_triggered: AtomicU32,
    pub(crate) tx_triggered: AtomicU32,
    /// Trampoline fn pointer for a user-registered raw ISR callback (0 = none).
    /// See `Uart::register_irq`.
    pub(crate) irq_handler: AtomicUsize,
    /// `*mut H` for the registered callback, type-erased; paired with `irq_handler`.
    pub(crate) irq_data: AtomicUsize,
}

impl Default for UartState {
    fn default() -> Self {
        Self::new()
    }
}

impl UartState {
    pub const fn new() -> Self {
        Self {
            rx_waker: AtomicWaker::new(),
            tx_waker: AtomicWaker::new(),
            rx_triggered: AtomicU32::new(0),
            tx_triggered: AtomicU32::new(0),
            irq_handler: AtomicUsize::new(0),
            irq_data: AtomicUsize::new(0),
        }
    }
}

pub static UART0_STATE: UartState = UartState::new();
pub static UART1_STATE: UartState = UartState::new();
pub static UART2_STATE: UartState = UartState::new();
pub static UART3_STATE: UartState = UartState::new();

/// Call from the UART ISR: service TX-empty and RX-data interrupts.
pub(crate) fn uart_int_dispatch(ptr: *const RegisterBlock, state: &'static UartState) {
    let rb = unsafe { &*ptr };
    let lsr = rb.uart_lsr().read();
    let ier = rb.uart_ier().read();

    // TX FIFO empty (ETBEI = IER bit 1, tx_empty = LSR bit 5)
    if ier.etbei().bit_is_set() && lsr.tx_empty().bit_is_set() {
        rb.uart_ier().modify(|_, w| w.etbei().clear_bit());
        state.tx_triggered.store(1, Ordering::Release);
        state.tx_waker.wake();
    }

    // RX data available (erbi = IER bit 0, etoi = IER bit 5)
    // rxfifo_int = LSR bit 8, timeout_int = LSR bit 9
    if (ier.erbi().bit_is_set() || ier.etoi().bit_is_set())
        && (lsr.rxfifo_int().bit_is_set() || lsr.timeout_int().bit_is_set())
    {
        rb.uart_ier()
            .modify(|_, w| w.erbi().clear_bit().etoi().clear_bit().elsi().clear_bit());
        // Clear the timeout interrupt latch
        unsafe {
            rb.uart_icr().write_with_zero(|w| w.toicf().set_bit());
        }
        state.rx_triggered.store(lsr.bits(), Ordering::Release);
        state.rx_waker.wake();
        dispatch_user_irq(
            rb,
            state,
            UartIrqEvent {
                rx_ready: lsr.rxfifo_int().bit_is_set(),
                rx_timeout: lsr.timeout_int().bit_is_set(),
                rx_error: false,
                lsr: lsr.bits(),
            },
        );
    }

    // Line Status Error (elsi = IER bit 2): overrun / parity / framing / break / rxfifo error.
    // Reading LSR already clears the error flags (RC). Wake the RX future so the
    // caller sees the error bits in rx_triggered.
    // SDK: ameba_uart.h : RUART_BIT_ELSI, UART_ALL_RX_ERR (bits 1-4, 7)
    let rx_err = lsr.ovr_err().bit_is_set()
        || lsr.par_err().bit_is_set()
        || lsr.frm_err().bit_is_set()
        || lsr.break_int().bit_is_set()
        || lsr.rxfifo_err().bit_is_set();
    if ier.elsi().bit_is_set() && rx_err {
        rb.uart_ier()
            .modify(|_, w| w.erbi().clear_bit().etoi().clear_bit().elsi().clear_bit());
        state.rx_triggered.store(lsr.bits(), Ordering::Release);
        state.rx_waker.wake();
        dispatch_user_irq(
            rb,
            state,
            UartIrqEvent {
                rx_ready: false,
                rx_timeout: false,
                rx_error: true,
                lsr: lsr.bits(),
            },
        );
    }
}

/// Forward to a user-registered raw ISR callback, if any (`Uart::register_irq`),
/// then re-arm the Rx interrupt-enable bits for continuous delivery. Safe here
/// because Rx FIFO threshold / timeout self-clear once the handler drains the
/// FIFO — unlike Tx-empty, which stays level-held and is deliberately not
/// covered by this path (see `register_irq` doc comment).
fn dispatch_user_irq(rb: &RegisterBlock, state: &UartState, event: UartIrqEvent) {
    let handler_fn = state.irq_handler.load(Ordering::Acquire);
    if handler_fn == 0 {
        return;
    }
    let data = state.irq_data.load(Ordering::Acquire);
    // SAFETY: handler_fn/data were written together by `register_irq` from a
    // valid &'static mut H; the trampoline signature matches uart_handler_trampoline::<H>.
    unsafe {
        let trampoline: unsafe fn(usize, UartIrqEvent) = core::mem::transmute(handler_fn);
        trampoline(data, event);
    }
    rb.uart_ier()
        .modify(|_, w| w.erbi().set_bit().etoi().set_bit().elsi().set_bit());
}

// ---------------------------------------------------------------------------
// Async futures
// ---------------------------------------------------------------------------

pub struct UartTxFuture {
    ptr: *const RegisterBlock,
    state: &'static UartState,
}

// SAFETY: embedded single-CPU target; pointer is valid for the peripheral's lifetime.
unsafe impl Send for UartTxFuture {}

impl Future for UartTxFuture {
    type Output = ();
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.state.tx_waker.register(cx.waker());
        if self.state.tx_triggered.load(Ordering::Acquire) != 0 {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Drop for UartTxFuture {
    fn drop(&mut self) {
        if self.state.tx_triggered.load(Ordering::Relaxed) == 0 {
            let rb = unsafe { &*self.ptr };
            rb.uart_ier().modify(|_, w| w.etbei().clear_bit());
        }
    }
}

pub struct UartRxFuture {
    ptr: *const RegisterBlock,
    state: &'static UartState,
}

// SAFETY: embedded single-CPU target; pointer is valid for the peripheral's lifetime.
unsafe impl Send for UartRxFuture {}

impl Future for UartRxFuture {
    type Output = ();
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.state.rx_waker.register(cx.waker());
        if self.state.rx_triggered.load(Ordering::Acquire) != 0 {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Drop for UartRxFuture {
    fn drop(&mut self) {
        if self.state.rx_triggered.load(Ordering::Relaxed) == 0 {
            let rb = unsafe { &*self.ptr };
            // SDK: serial_api.c : uart_intrecv_complete — always disables ERBI+ELSI+ETOI together
            rb.uart_ier()
                .modify(|_, w| w.erbi().clear_bit().elsi().clear_bit().etoi().clear_bit());
        }
    }
}

// ---------------------------------------------------------------------------
// Peripheral tokens
// ---------------------------------------------------------------------------

impl_uart_instance!(
    Uart0,
    crate::interrupt::typelevel::UART0_IRQ,
    Uart0Reg,
    0,
    1,
    UART0_STATE,
    cksl_uart0
);
impl_uart_instance!(
    Uart1,
    crate::interrupt::typelevel::UART1_IRQ,
    Uart1Reg,
    2,
    3,
    UART1_STATE,
    cksl_uart1
);
impl_uart_instance!(
    Uart2,
    crate::interrupt::typelevel::UART2_IRQ,
    Uart2Reg,
    4,
    5,
    UART2_STATE,
    cksl_uart2
);
impl_uart_instance!(
    Uart3,
    crate::interrupt::typelevel::UART3_IRQ,
    Uart3Reg,
    24,
    25,
    UART3_STATE,
    cksl_uart3
);

impl sealed::Sealed for AnyUart<'_> {}
impl UartInstance for AnyUart<'static> {
    type IrqType = crate::interrupt::typelevel::IrqAny;
    const PTR: *const RegisterBlock = core::ptr::null();
    const HS_TX: u8 = 0;
    const HS_RX: u8 = 0;

    fn ptr(&self) -> *const RegisterBlock {
        any::delegate!(self, u => { u.ptr() })
    }
    fn state(&self) -> &'static UartState {
        any::delegate!(self, u => { u.state() })
    }
    fn state_static() -> &'static UartState {
        unreachable!("AnyUart::state_static called -- use state(&self) instead")
    }
    fn set_clock_source_xtal(&self) {
        any::delegate!(self, u => { u.set_clock_source_xtal() })
    }
}

// ---------------------------------------------------------------------------
// UartDma — DMA-backed async UART. Obtained via Uart::into_dma_async().
// ---------------------------------------------------------------------------

/// Alignment-aware TX DMA params for UART.
/// SDK: ameba_uart.c : UART_TXGDMA_Init
fn uart_tx_dma_params(buf: *const u8, len: usize, hs_id: u8) -> crate::dma::TrParams {
    use crate::dma::{Msize, TrParams, TrWidth};
    // Dst (UART THR) always OneByte/Four. SDK: GDMA_DstDataWidth=TrWidthOneByte / GDMA_DstMsize=MsizeFour
    // 4-byte aligned src: pack 4 bytes per beat. SDK: SrcWidth=FourBytes / SrcMsize=One / BlockSize=len>>2
    // Unaligned: 1 byte per beat. SDK: SrcWidth=OneByte / SrcMsize=Four / BlockSize=len
    let (sw, sm) = if (len & 3 == 0) && ((buf as usize) & 3 == 0) {
        (TrWidth::FourBytes, Msize::One)
    } else {
        (TrWidth::OneByte, Msize::Four)
    };
    TrParams {
        src_width: sw,
        dst_width: TrWidth::OneByte,
        src_msize: sm,
        dst_msize: Msize::Four,
        hs_id,
    }
}

/// RX DMA params for UART — no alignment optimization.
/// SDK: ameba_uart.c : UART_RXGDMA_Init
/// Both sides OneByte/One: SrcMsize=One / DstMsize=One / BlockSize=RxCount.
fn uart_rx_dma_params(hs_id: u8) -> crate::dma::TrParams {
    use crate::dma::{Msize, TrParams, TrWidth};
    TrParams {
        src_width: TrWidth::OneByte,
        dst_width: TrWidth::OneByte,
        src_msize: Msize::One,
        dst_msize: Msize::One,
        hs_id,
    }
}

/// UART handle in DMA-backed async mode. Obtained via [`Uart::into_dma_async`].
pub struct UartDma<P: UartInstance, const TX_CH: u8, const RX_CH: u8> {
    _phantom: PhantomData<P>,
    tx_ch: crate::dma::DmaChannel<TX_CH, crate::dma::Async>,
    rx_ch: crate::dma::DmaChannel<RX_CH, crate::dma::Async>,
}

// SAFETY: embedded single-CPU target; `UartDma` has exclusive ownership of its peripheral and channels.
unsafe impl<P: UartInstance, const TX_CH: u8, const RX_CH: u8> Send for UartDma<P, TX_CH, RX_CH> {}

impl<P: UartInstance + Default, const TX_CH: u8, const RX_CH: u8> UartDma<P, TX_CH, RX_CH> {
    fn uart(&self) -> Uart<P> {
        unsafe { Uart::steal() }
    }

    /// TX-only async DMA write. Flushes D-cache before transfer.
    /// SDK: ameba_uart.c : UART_TXGDMA_Init + serial_api.c : serial_send_stream_dma
    pub async fn dma_write(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let len = data.len();
        let tp = uart_tx_dma_params(data.as_ptr(), len, P::HS_TX);
        unsafe { crate::cache::dcache_clean(data.as_ptr(), len) };
        let uart = self.uart();
        uart.tx_dma_cmd(true);
        self.tx_ch
            .periph_tx(data.as_ptr(), uart.dr_ptr(), len, tp)
            .await;
        uart.tx_dma_cmd(false);
    }

    /// RX-only async DMA read. Invalidates D-cache before and after.
    /// SDK: ameba_uart.c : UART_RXGDMA_Init + serial_api.c : serial_recv_stream_dma
    pub async fn dma_read(&mut self, buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }
        let len = buf.len();
        let tp = uart_rx_dma_params(P::HS_RX);
        unsafe { crate::cache::dcache_clean_invalidate(buf.as_ptr(), len) }; // SDK: UART_RXGDMA_Init → DCache_CleanInvalidate
        let uart = self.uart();
        uart.rx_dma_cmd(true);
        self.rx_ch
            .periph_rx(uart.dr_ptr() as *const u32, buf.as_mut_ptr(), len, tp)
            .await;
        uart.rx_dma_cmd(false);
        unsafe { crate::cache::dcache_invalidate(buf.as_ptr(), len) };
    }

    /// Concurrent TX + RX async DMA transfer (e.g. loopback test).
    ///
    /// RX DMA is armed before TX starts so no bytes are missed.
    /// TX future is held alive until RX completes (TX always finishes first at FIFO level).
    /// SDK: ameba_uart.c : UART_TXGDMA_Init / UART_RXGDMA_Init
    pub async fn dma_transfer(&mut self, tx: &[u8], rx: &mut [u8]) {
        let len = tx.len().min(rx.len());
        if len == 0 {
            return;
        }
        let tx_tp = uart_tx_dma_params(tx.as_ptr(), len, P::HS_TX);
        let rx_tp = uart_rx_dma_params(P::HS_RX);
        unsafe {
            crate::cache::dcache_clean(tx.as_ptr(), len);
            crate::cache::dcache_clean_invalidate(rx.as_ptr(), len); // SDK: UART_RXGDMA_Init → DCache_CleanInvalidate
        }
        let uart = self.uart();
        let dr = uart.dr_ptr();
        uart.rx_dma_cmd(true);
        uart.tx_dma_cmd(true);
        // RX future created first: ready before TX starts clocking. SDK: serial_recv before serial_send
        let rx_fut = self
            .rx_ch
            .periph_rx(dr as *const u32, rx.as_mut_ptr(), len, rx_tp);
        // _tx_fut held alive until after rx_fut completes; TX finishes first so Drop is a no-op.
        let _tx_fut = self.tx_ch.periph_tx(tx.as_ptr(), dr, len, tx_tp);
        rx_fut.await;
        uart.rx_dma_cmd(false);
        uart.tx_dma_cmd(false);
        unsafe { crate::cache::dcache_invalidate(rx.as_ptr(), len) };
    }
}
