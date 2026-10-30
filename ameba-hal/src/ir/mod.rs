//! RTL8721F IR (Infra-Red) peripheral driver.
//!
//! PAC module: `ir_reg` at base 0x4100_7000.
//! No ROM calls exist for this peripheral; all register access goes through
//! the PAC field methods directly.

use bitflags::bitflags;
use rtl8721f_pac::{IrReg, ir_reg};

// ─────────────────────────────────────────────────────────────────────────────
// No ROM FFI for IR (IR has no ROM calls)
// mod sys { extern "C" { ... } }  — intentionally absent
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// Bitflags
// ─────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// TX interrupt sources (enable / mask / status bits in IR_TX_CONFIG / IR_TX_SR).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct IrTxInt: u32 {
        /// TX FIFO empty interrupt
        const FIFO_EMPTY = 1 << 0;
        /// TX FIFO level (≤ threshold) interrupt
        const FIFO_LEVEL = 1 << 1;
        /// TX FIFO overflow interrupt
        const FIFO_OVER  = 1 << 4;
    }
}

bitflags! {
    /// RX interrupt sources (enable / mask / status bits in IR_RX_CONFIG / IR_RX_SR).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct IrRxInt: u32 {
        /// RX FIFO full interrupt
        const FIFO_FULL   = 1 << 0;
        /// RX FIFO level (≥ threshold) interrupt
        const FIFO_LEVEL  = 1 << 1;
        /// RX counter overflow interrupt
        const CNT_OF      = 1 << 2;
        /// RX FIFO overflow interrupt
        const FIFO_OF     = 1 << 3;
        /// RX counter threshold interrupt
        const CNT_THR     = 1 << 4;
        /// RX FIFO error (read-when-empty) interrupt
        const FIFO_ERROR  = 1 << 5;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Type-safe enums
// ─────────────────────────────────────────────────────────────────────────────

/// IR operating mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrMode {
    /// Transmit mode (IR_TX_CONFIG.IR_MODE_SEL = 0)
    Tx,
    /// Receive mode (IR_TX_CONFIG.IR_MODE_SEL = 1)
    Rx,
}

/// TX idle output level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxIdleLevel {
    /// Output low during idle
    Low = 0,
    /// Output high during idle
    High = 1,
}

/// TX data / carrier inversion control.
///
/// Encodes the two-bit field `IR_TX_DATA_TYPE_SET(x)` (bits 13–14 of IR_TX_CONFIG):
/// - bit 13 = `ir_tx_de_inverse` (FIFO data inverse)
/// - bit 14 = `ir_tx_output_inverse` (carrier output inverse)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxInverse {
    /// Data normal, carrier normal (0b00)
    DataNormalCarrierNormal = 0,
    /// Data normal, carrier inverse (0b01) — bit13 set
    DataNormalCarrierInverse = 1,
    /// Data inverse, carrier normal (0b10) — bit14 set
    DataInverseCarrierNormal = 2,
    /// Data inverse, carrier inverse (0b11) — both bits set
    DataInverseCarrierInverse = 3,
}

/// RX start mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxStartMode {
    /// Manual mode — IR_RX_MAN_START controls start
    Manual = 0,
    /// Auto mode — trigger-mode continuous
    Auto = 1,
}

/// RX FIFO full behaviour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFifoFullCtrl {
    /// Discard oldest entry when full (IR_RX_FIFO_DISCARD_SET = 0)
    DiscardOldest = 0,
    /// Discard newest (reject) when full (IR_RX_FIFO_DISCARD_SET = 1)
    DiscardNewest = 1,
}

/// RX trigger edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxTriggerMode {
    /// High → low (falling edge)
    SingleFallingEdge = 0,
    /// Low → high (rising edge)
    SingleRisingEdge = 1,
    /// Both edges
    DoubleEdge = 2,
}

/// RX counter threshold trigger level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxCounterTrigger {
    /// Trigger when low-level counter ≥ threshold
    Low = 0,
    /// Trigger when high-level counter ≥ threshold
    High = 1,
}

/// RX glitch filter time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFilterTime {
    /// Filter glitches ≤ 50 ns
    Ns50 = 0,
    /// Filter glitches ≤ 75 ns
    Ns75 = 1,
    /// Filter glitches ≤ 100 ns
    Ns100 = 2,
    /// Filter glitches ≤ 125 ns
    Ns125 = 3,
    /// Filter glitches ≤ 150 ns
    Ns150 = 4,
    /// Filter glitches ≤ 175 ns
    Ns175 = 5,
    /// Filter glitches ≤ 200 ns
    Ns200 = 6,
    /// Filter glitches ≤ 225 ns
    Ns225 = 7,
}

// ─────────────────────────────────────────────────────────────────────────────
// Init params (mirrors IR_InitTypeDef)
// ─────────────────────────────────────────────────────────────────────────────

/// Initialization parameters for the IR peripheral (mirrors `IR_InitTypeDef`).
///
/// The `Default` implementation replicates `IR_StructInit` defaults.
#[derive(Clone, Debug)]
pub struct IrInitParams {
    /// IR IP input clock in Hz (e.g. 40_000_000 for 40 MHz)
    pub clock: u32,
    /// Carrier frequency in Hz (e.g. 38_000 for 38 kHz)
    pub freq: u32,
    /// Duty cycle divisor (e.g. 3 for 1/3 duty)
    pub duty_cycle: u32,
    /// Operating mode
    pub mode: IrMode,

    // TX-specific
    /// TX idle output level
    pub tx_idle_level: TxIdleLevel,
    /// TX data / carrier inversion
    pub tx_inverse: TxInverse,
    /// TX FIFO interrupt threshold (interrupt when depth ≤ threshold)
    pub tx_fifo_thr_level: u8,
    /// TX compensation clock in Hz (e.g. 1_000_000 for 1 MHz)
    pub tx_comp_clk: u32,

    // RX-specific
    /// RX start mode
    pub rx_start_mode: RxStartMode,
    /// RX FIFO interrupt threshold (interrupt when depth > threshold)
    pub rx_fifo_thr_level: u8,
    /// RX FIFO full behaviour
    pub rx_fifo_full_ctrl: RxFifoFullCtrl,
    /// RX trigger edge
    pub rx_trigger_mode: RxTriggerMode,
    /// RX glitch filter time
    pub rx_filter_time: RxFilterTime,
    /// RX counter threshold trigger level
    pub rx_cnt_thr_type: RxCounterTrigger,
    /// RX counter threshold value
    pub rx_cnt_thr: u32,
}

impl Default for IrInitParams {
    /// Matches the C `IR_StructInit` defaults exactly.
    fn default() -> Self {
        IrInitParams {
            clock: 40_000_000,
            freq: 38_000,
            duty_cycle: 3,
            mode: IrMode::Tx,
            tx_idle_level: TxIdleLevel::Low,
            tx_inverse: TxInverse::DataNormalCarrierNormal,
            tx_fifo_thr_level: 0,
            tx_comp_clk: 1_000_000,
            rx_start_mode: RxStartMode::Auto,
            rx_fifo_thr_level: 0,
            rx_fifo_full_ctrl: RxFifoFullCtrl::DiscardOldest,
            rx_trigger_mode: RxTriggerMode::DoubleEdge,
            rx_filter_time: RxFilterTime::Ns50,
            rx_cnt_thr_type: RxCounterTrigger::Low,
            rx_cnt_thr: 0x23a,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TX FIFO size constant
// ─────────────────────────────────────────────────────────────────────────────

const IR_TX_FIFO_SIZE: u32 = 32; // SDK: ameba_ir.h : IR_TX_FIFO_SIZE

// ─────────────────────────────────────────────────────────────────────────────
// Ir struct
// ─────────────────────────────────────────────────────────────────────────────

/// Handle to the IR peripheral register block.
///
/// Construct via [`Ir::steal`] (non-secure instance at 0x4100_7000)
/// or [`Ir::new_at`] for a custom base address (e.g. secure alias).
pub struct Ir(*const ir_reg::RegisterBlock);

// SAFETY: embedded single-CPU target; `Ir` has exclusive ownership of its peripheral.
unsafe impl Send for Ir {}

impl Ir {
    /// Obtain a handle to the non-secure IR instance at 0x4100_7000.
    ///
    /// # Safety
    /// The caller must ensure exclusive access to the IR peripheral for the
    /// lifetime of the returned `Ir`. No two `Ir` instances may coexist if
    /// they alias the same hardware.
    pub unsafe fn steal() -> Self {
        Ir(IrReg::PTR)
    }

    /// Obtain a handle to an IR register block at an arbitrary base address.
    ///
    /// Use this to access the secure alias or a different memory-mapped copy.
    ///
    /// # Safety
    /// `base` must be a valid, aligned base address for an IR register block
    /// and the caller must guarantee exclusive access.
    pub unsafe fn new_at(base: usize) -> Self {
        Ir(base as *const ir_reg::RegisterBlock)
    }

    #[inline(always)]
    fn regs(&self) -> &'static ir_reg::RegisterBlock {
        unsafe { &*self.0 }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Initialisation / De-initialisation
    // ─────────────────────────────────────────────────────────────────────────

    /// De-initialize the IR peripheral (no-op; RCC gating is not managed here).
    pub fn deinit(&self) {
        // The C IR_DeInit body is empty (RCC is commented out).
    }

    /// Initialize the IR peripheral from `params`.
    ///
    /// Mirrors `IR_Init` faithfully but uses PAC field methods exclusively.
    pub fn init(&self, params: &IrInitParams) {
        let regs = self.regs();

        // CLK_DIV = clock / freq - 1
        let clk_div = params.clock / params.freq - 1;
        regs.ir_clk_div()
            .write(|w| unsafe { w.ir_clk_div().bits(clk_div as u16) });

        match params.mode {
            IrMode::Tx => {
                // duty_num = (clk_div + 1) / duty_cycle
                let duty_num = (clk_div + 1) / params.duty_cycle;

                // Decompose tx_inverse into the two inversion bits.
                let raw_inv = params.tx_inverse as u8;
                let de_inverse = (raw_inv & 0x1) != 0; // bit 0 → ir_tx_de_inverse (bit 13)
                let output_inverse = (raw_inv & 0x2) != 0; // bit 1 → ir_tx_output_inverse (bit 14)

                // RMW IR_TX_CONFIG: clear mode-sel (Tx), set duty, inversion, threshold.
                regs.ir_tx_config().modify(|_, w| unsafe {
                    w.ir_mode_sel()
                        .clear_bit()
                        .ir_tx_duty_num()
                        .bits(duty_num as u16)
                        .ir_tx_de_inverse()
                        .bit(de_inverse)
                        .ir_tx_output_inverse()
                        .bit(output_inverse)
                        .ir_tx_fifo_level_th()
                        .bits(params.tx_fifo_thr_level)
                        .ir_tx_idle_state()
                        .bit(params.tx_idle_level == TxIdleLevel::High)
                });

                // Clear all TX interrupts and FIFO.
                regs.ir_tx_int_clr().write(|w| {
                    w.ir_tx_fifo_empty_int_clr()
                        .set_bit()
                        .ir_tx_fifo_level_int_clr()
                        .set_bit()
                        .ir_tx_fifo_over_int_clr()
                        .set_bit()
                        .ir_tx_fifo_clr()
                        .set_bit()
                });

                // TX compensation divider: comp_clk_div = clock / tx_comp_clk - 1
                let comp_div = params.clock / params.tx_comp_clk - 1;
                regs.ir_tx_compe_div()
                    .write(|w| unsafe { w.tx_compe_div().bits(comp_div as u16) });
            }

            IrMode::Rx => {
                // Set mode-sel bit to select RX.
                regs.ir_tx_config().modify(|_, w| w.ir_mode_sel().set_bit());

                // Build IR_RX_CONFIG: start-mode, trigger-mode, filter, FIFO threshold.
                regs.ir_rx_config().modify(|_, w| unsafe {
                    w.ir_rx_start_mode()
                        .bit(params.rx_start_mode == RxStartMode::Auto)
                        .ir_rx_trigger_mode()
                        .bits(params.rx_trigger_mode as u8)
                        .ir_rx_filter_stagetx()
                        .bits(params.rx_filter_time as u8)
                        .ir_rx_fifo_level_th()
                        .bits(params.rx_fifo_thr_level)
                        .ir_rx_fifo_discard_set()
                        .bit(params.rx_fifo_full_ctrl == RxFifoFullCtrl::DiscardNewest)
                });

                // RX counter threshold.
                regs.ir_rx_cnt_int_sel().write(|w| unsafe {
                    w.ir_rx_cnt_thr_trigger_lv()
                        .bit(params.rx_cnt_thr_type == RxCounterTrigger::High)
                        .ir_rx_cnt_thr()
                        .bits(params.rx_cnt_thr & 0x7FFF_FFFF)
                });

                // Clear all RX interrupts and FIFO.
                regs.ir_rx_int_clr().write(|w| {
                    w.ir_rx_fifo_full_int_clr()
                        .set_bit()
                        .ir_rx_fifo_level_int_clr()
                        .set_bit()
                        .ir_rx_cnt_of_int_clr()
                        .set_bit()
                        .ir_rx_fifo_of_int_clr()
                        .set_bit()
                        .ir_rx_cnt_thr_int_clr()
                        .set_bit()
                        .ir_rx_fifo_error_int_clr()
                        .set_bit()
                        .ir_rx_fifo_clr()
                        .set_bit()
                });
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Enable / Disable
    // ─────────────────────────────────────────────────────────────────────────

    /// Enable or disable the IR FSM for the given mode.
    ///
    /// Mirrors `IR_Cmd`.
    pub fn cmd(&self, mode: IrMode, enable: bool) {
        let regs = self.regs();
        match (mode, enable) {
            (IrMode::Tx, true) => {
                regs.ir_tx_config().modify(|_, w| w.ir_tx_start().set_bit());
            }
            (IrMode::Tx, false) => {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_start().clear_bit());
            }
            (IrMode::Rx, true) => {
                regs.ir_rx_config().modify(|_, w| w.ir_rx_start().set_bit());
            }
            (IrMode::Rx, false) => {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_start().clear_bit());
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // TX threshold
    // ─────────────────────────────────────────────────────────────────────────

    /// Set the TX FIFO interrupt threshold.
    ///
    /// Mirrors `IR_SetTxThreshold`.
    pub fn set_tx_threshold(&self, thd: u8) {
        self.regs()
            .ir_tx_config()
            .modify(|_, w| unsafe { w.ir_tx_fifo_level_th().bits(thd) });
    }

    // ─────────────────────────────────────────────────────────────────────────
    // RX threshold
    // ─────────────────────────────────────────────────────────────────────────

    /// Set the RX FIFO interrupt threshold.
    ///
    /// Mirrors `IR_SetRxThreshold`.
    pub fn set_rx_threshold(&self, thd: u8) {
        self.regs()
            .ir_rx_config()
            .modify(|_, w| unsafe { w.ir_rx_fifo_level_th().bits(thd) });
    }

    // ─────────────────────────────────────────────────────────────────────────
    // RX counter threshold
    // ─────────────────────────────────────────────────────────────────────────

    /// Set the RX counter threshold and trigger level.
    ///
    /// Mirrors `IR_SetRxCounterThreshold`.
    pub fn set_rx_counter_threshold(&self, trigger: RxCounterTrigger, thr: u32) {
        self.regs().ir_rx_cnt_int_sel().write(|w| unsafe {
            w.ir_rx_cnt_thr_trigger_lv()
                .bit(trigger == RxCounterTrigger::High)
                .ir_rx_cnt_thr()
                .bits(thr & 0x7FFF_FFFF)
        });
    }

    // ─────────────────────────────────────────────────────────────────────────
    // TX FIFO helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Return the number of free slots in the TX FIFO (= 32 − current depth).
    ///
    /// Mirrors `IR_GetTxFIFOFreeLen`.
    pub fn tx_fifo_free_len(&self) -> u32 {
        let offset = self.regs().ir_tx_sr().read().ir_tx_fifo_offset().bits() as u32;
        IR_TX_FIFO_SIZE - offset
    }

    /// Write a single word to the TX FIFO.
    ///
    /// Mirrors `IR_SendData`. The caller is responsible for not writing
    /// more than [`Self::tx_fifo_free_len`] words without checking.
    ///
    /// # Safety
    /// Writing to the TX FIFO when the FIFO is full causes an overflow fault.
    pub fn send_data(&self, data: u32) {
        // The TX FIFO register is write-only; we write the raw 32-bit value.
        // We decompose the word into PAC fields to avoid raw masking.
        // Bits 30:0 are ir_tx_data_end_flag (bit 30) + ir_tx_compensation (29:28)
        // + ir_tx_data_time (27:0).  We pass the full word through the
        // write-only register using the PAC `bits()` backdoor.
        // Per c2rust.txt rules: FieldWriter is used exclusively; the caller
        // constructs `data` using the hardware data format directly.
        self.regs().ir_tx_fifo().write(|w| unsafe {
            // ir_tx_data_time occupies bits 27:0
            w.ir_tx_data_time()
                .bits(data & 0x0FFF_FFFF)
                // ir_tx_compensation bits 29:28
                .ir_tx_compensation()
                .bits(((data >> 28) & 0x3) as u8)
                // ir_tx_data_end_flag bit 30
                .ir_tx_data_end_flag()
                .bit((data >> 30) & 1 != 0)
                // ir_tx_data_type bit 31
                .ir_tx_data_type()
                .bit((data >> 31) & 1 != 0)
        });
    }

    /// Write a buffer of TX data words.
    ///
    /// When `is_last_packet` is `true`, the last word is written with the
    /// end-of-packet flag (bit 30) set, as required by the hardware protocol.
    ///
    /// Mirrors `IR_SendBuf`.
    ///
    /// # Safety
    /// Caller must ensure the TX FIFO has sufficient free space.
    pub fn send_buf(&self, buf: &[u32], is_last_packet: bool) {
        if buf.is_empty() {
            return;
        }
        let (last, head) = buf.split_last().expect("non-empty");
        for &word in head {
            self.send_data(word);
        }
        if is_last_packet {
            // Set bit 30 (ir_tx_data_end_flag) on the last word.
            self.send_data(*last | (1u32 << 30));
        } else {
            self.send_data(*last);
        }
    }

    /// Clear the TX FIFO.
    ///
    /// Mirrors `IR_ClearTxFIFO`.
    pub fn clear_tx_fifo(&self) {
        self.regs()
            .ir_tx_int_clr()
            .write(|w| w.ir_tx_fifo_clr().set_bit());
    }

    // ─────────────────────────────────────────────────────────────────────────
    // RX FIFO helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Return the number of words currently in the RX FIFO.
    ///
    /// Mirrors `IR_GetRxDataLen`.
    pub fn rx_data_len(&self) -> u32 {
        self.regs().ir_rx_sr().read().ir_rx_fifo_offset().bits() as u32
    }

    /// Read a single word from the RX FIFO.
    ///
    /// Mirrors `IR_ReceiveData`. Returns the raw 32-bit FIFO word (bit 31 =
    /// level, bits 30:0 = cycle duration count).
    pub fn receive_data(&self) -> u32 {
        self.regs().ir_rx_fifo().read().bits()
    }

    /// Read `buf.len()` words from the RX FIFO into `buf`.
    ///
    /// Mirrors `IR_ReceiveBuf`.
    pub fn receive_buf(&self, buf: &mut [u32]) {
        for slot in buf.iter_mut() {
            *slot = self.receive_data();
        }
    }

    /// Clear the RX FIFO.
    ///
    /// Mirrors `IR_ClearRxFIFO`.
    pub fn clear_rx_fifo(&self) {
        self.regs()
            .ir_rx_int_clr()
            .write(|w| w.ir_rx_fifo_clr().set_bit());
    }

    /// Start a manual RX trigger (only valid in manual start mode).
    ///
    /// Mirrors `IR_StartManualRxTrigger`.
    pub fn start_manual_rx_trigger(&self) {
        self.regs()
            .ir_rx_config()
            .modify(|_, w| w.ir_rx_man_start().set_bit());
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Interrupt enable / mask (mode-dispatched)
    // ─────────────────────────────────────────────────────────────────────────

    /// Enable or disable TX interrupt sources.
    ///
    /// Mirrors `IR_INTConfig` (TX path).
    pub fn set_tx_interrupt(&self, ints: IrTxInt, enable: bool) {
        let regs = self.regs();
        if enable {
            if ints.contains(IrTxInt::FIFO_EMPTY) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_empty_int_en().set_bit());
            }
            if ints.contains(IrTxInt::FIFO_LEVEL) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_level_int_en().set_bit());
            }
            if ints.contains(IrTxInt::FIFO_OVER) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_over_int_en().set_bit());
            }
        } else {
            if ints.contains(IrTxInt::FIFO_EMPTY) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_empty_int_en().clear_bit());
            }
            if ints.contains(IrTxInt::FIFO_LEVEL) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_level_int_en().clear_bit());
            }
            if ints.contains(IrTxInt::FIFO_OVER) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_over_int_en().clear_bit());
            }
        }
    }

    /// Enable or disable TX interrupt masks.
    ///
    /// Mirrors `IR_MaskINTConfig` (TX path). Masking suppresses delivery
    /// to the interrupt controller while keeping the status bits active.
    pub fn set_tx_interrupt_mask(&self, ints: IrTxInt, mask: bool) {
        let regs = self.regs();
        if mask {
            if ints.contains(IrTxInt::FIFO_EMPTY) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_empty_int_mask().set_bit());
            }
            if ints.contains(IrTxInt::FIFO_LEVEL) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_level_int_mask().set_bit());
            }
            if ints.contains(IrTxInt::FIFO_OVER) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_over_int_mask().set_bit());
            }
        } else {
            if ints.contains(IrTxInt::FIFO_EMPTY) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_empty_int_mask().clear_bit());
            }
            if ints.contains(IrTxInt::FIFO_LEVEL) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_level_int_mask().clear_bit());
            }
            if ints.contains(IrTxInt::FIFO_OVER) {
                regs.ir_tx_config()
                    .modify(|_, w| w.ir_tx_fifo_over_int_mask().clear_bit());
            }
        }
    }

    /// Enable or disable RX interrupt sources.
    ///
    /// Mirrors `IR_INTConfig` (RX path).
    pub fn set_rx_interrupt(&self, ints: IrRxInt, enable: bool) {
        let regs = self.regs();
        if enable {
            if ints.contains(IrRxInt::FIFO_FULL) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_full_int_en().set_bit());
            }
            if ints.contains(IrRxInt::FIFO_LEVEL) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_level_int_en().set_bit());
            }
            if ints.contains(IrRxInt::CNT_OF) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_cnt_of_int_en().set_bit());
            }
            if ints.contains(IrRxInt::FIFO_OF) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_of_int_en().set_bit());
            }
            if ints.contains(IrRxInt::CNT_THR) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_cnt_thr_int_en().set_bit());
            }
            if ints.contains(IrRxInt::FIFO_ERROR) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_error_int_en().set_bit());
            }
        } else {
            if ints.contains(IrRxInt::FIFO_FULL) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_full_int_en().clear_bit());
            }
            if ints.contains(IrRxInt::FIFO_LEVEL) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_level_int_en().clear_bit());
            }
            if ints.contains(IrRxInt::CNT_OF) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_cnt_of_int_en().clear_bit());
            }
            if ints.contains(IrRxInt::FIFO_OF) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_of_int_en().clear_bit());
            }
            if ints.contains(IrRxInt::CNT_THR) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_cnt_thr_int_en().clear_bit());
            }
            if ints.contains(IrRxInt::FIFO_ERROR) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_error_int_en().clear_bit());
            }
        }
    }

    /// Enable or disable RX interrupt masks.
    ///
    /// Mirrors `IR_MaskINTConfig` (RX path).
    pub fn set_rx_interrupt_mask(&self, ints: IrRxInt, mask: bool) {
        let regs = self.regs();
        if mask {
            if ints.contains(IrRxInt::FIFO_FULL) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_full_int_mask().set_bit());
            }
            if ints.contains(IrRxInt::FIFO_LEVEL) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_level_int_mask().set_bit());
            }
            if ints.contains(IrRxInt::CNT_OF) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_cnt_of_int_mask().set_bit());
            }
            if ints.contains(IrRxInt::FIFO_OF) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_of_int_mask().set_bit());
            }
            if ints.contains(IrRxInt::CNT_THR) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_cnt_thr_int_mask().set_bit());
            }
            if ints.contains(IrRxInt::FIFO_ERROR) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_error_int_mask().set_bit());
            }
        } else {
            if ints.contains(IrRxInt::FIFO_FULL) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_full_int_mask().clear_bit());
            }
            if ints.contains(IrRxInt::FIFO_LEVEL) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_level_int_mask().clear_bit());
            }
            if ints.contains(IrRxInt::CNT_OF) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_cnt_of_int_mask().clear_bit());
            }
            if ints.contains(IrRxInt::FIFO_OF) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_of_int_mask().clear_bit());
            }
            if ints.contains(IrRxInt::CNT_THR) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_cnt_thr_int_mask().clear_bit());
            }
            if ints.contains(IrRxInt::FIFO_ERROR) {
                regs.ir_rx_config()
                    .modify(|_, w| w.ir_rx_fifo_error_int_mask().clear_bit());
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Status / IMR readback (mode-dispatched)
    // ─────────────────────────────────────────────────────────────────────────

    /// Return current mode (TX or RX) as detected from `IR_TX_CONFIG.IR_MODE_SEL`.
    pub fn current_mode(&self) -> IrMode {
        if self.regs().ir_tx_config().read().ir_mode_sel().bit_is_set() {
            IrMode::Rx
        } else {
            IrMode::Tx
        }
    }

    /// Read the raw interrupt status register for the currently active mode.
    ///
    /// Returns TX SR bits when in TX mode, RX SR bits when in RX mode.
    ///
    /// Mirrors `IR_GetINTStatus`.
    pub fn get_int_status_raw(&self) -> u32 {
        match self.current_mode() {
            IrMode::Rx => self.regs().ir_rx_sr().read().bits(),
            IrMode::Tx => self.regs().ir_tx_sr().read().bits(),
        }
    }

    /// Read TX interrupt status.
    pub fn tx_int_status(&self) -> u32 {
        self.regs().ir_tx_sr().read().bits()
    }

    /// Read RX interrupt status.
    pub fn rx_int_status(&self) -> u32 {
        self.regs().ir_rx_sr().read().bits()
    }

    /// Read the interrupt mask register (IMR) for the currently active mode.
    ///
    /// Returns only the mask bits from the relevant config register.
    ///
    /// Mirrors `IR_GetIMR`.
    pub fn get_imr_raw(&self) -> u32 {
        const TX_MASK_BITS: u32 = (1 << 2) | (1 << 3) | (1 << 5); // bits 2,3,5
        const RX_MASK_BITS: u32 =
            (1 << 14) | (1 << 15) | (1 << 16) | (1 << 17) | (1 << 18) | (1 << 19);
        match self.current_mode() {
            IrMode::Rx => self.regs().ir_rx_config().read().bits() & RX_MASK_BITS,
            IrMode::Tx => self.regs().ir_tx_config().read().bits() & TX_MASK_BITS,
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // FSM running query (mode-dispatched)
    // ─────────────────────────────────────────────────────────────────────────

    /// Return `true` if the IR FSM is currently running (TX or RX).
    ///
    /// Mirrors `IR_FSMRunning`.
    pub fn fsm_running(&self) -> bool {
        match self.current_mode() {
            IrMode::Rx => self.regs().ir_rx_sr().read().ir_rx_state().bit_is_set(),
            IrMode::Tx => self.regs().ir_tx_sr().read().ir_tx_status().bit_is_set(),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Clear interrupt pending bits (mode-dispatched)
    // ─────────────────────────────────────────────────────────────────────────

    /// Clear TX interrupt pending bits.
    ///
    /// `bits` is a bitmask in the IR_TX_INT_CLR register layout:
    /// - bit 0: FIFO CLR (not an interrupt clear, clears FIFO)
    /// - bit 1: empty int CLR
    /// - bit 2: level int CLR
    /// - bit 3: overflow int CLR
    ///
    /// Prefer the typed helpers [`Self::clear_tx_int`] and [`Self::clear_tx_fifo`].
    pub fn clear_tx_int(&self, ints: IrTxInt) {
        self.regs().ir_tx_int_clr().write(|w| {
            let mut w = w;
            if ints.contains(IrTxInt::FIFO_EMPTY) {
                w = w.ir_tx_fifo_empty_int_clr().set_bit();
            }
            if ints.contains(IrTxInt::FIFO_LEVEL) {
                w = w.ir_tx_fifo_level_int_clr().set_bit();
            }
            if ints.contains(IrTxInt::FIFO_OVER) {
                w = w.ir_tx_fifo_over_int_clr().set_bit();
            }
            w
        });
    }

    /// Clear all TX interrupt pending bits (not the FIFO).
    pub fn clear_all_tx_int(&self) {
        self.regs().ir_tx_int_clr().write(|w| {
            w.ir_tx_fifo_empty_int_clr()
                .set_bit()
                .ir_tx_fifo_level_int_clr()
                .set_bit()
                .ir_tx_fifo_over_int_clr()
                .set_bit()
        });
    }

    /// Clear RX interrupt pending bits.
    pub fn clear_rx_int(&self, ints: IrRxInt) {
        self.regs().ir_rx_int_clr().write(|w| {
            let mut w = w;
            if ints.contains(IrRxInt::FIFO_FULL) {
                w = w.ir_rx_fifo_full_int_clr().set_bit();
            }
            if ints.contains(IrRxInt::FIFO_LEVEL) {
                w = w.ir_rx_fifo_level_int_clr().set_bit();
            }
            if ints.contains(IrRxInt::CNT_OF) {
                w = w.ir_rx_cnt_of_int_clr().set_bit();
            }
            if ints.contains(IrRxInt::FIFO_OF) {
                w = w.ir_rx_fifo_of_int_clr().set_bit();
            }
            if ints.contains(IrRxInt::CNT_THR) {
                w = w.ir_rx_cnt_thr_int_clr().set_bit();
            }
            if ints.contains(IrRxInt::FIFO_ERROR) {
                w = w.ir_rx_fifo_error_int_clr().set_bit();
            }
            w
        });
    }

    /// Clear all RX interrupt pending bits (not the FIFO).
    pub fn clear_all_rx_int(&self) {
        self.regs().ir_rx_int_clr().write(|w| {
            w.ir_rx_fifo_full_int_clr()
                .set_bit()
                .ir_rx_fifo_level_int_clr()
                .set_bit()
                .ir_rx_cnt_of_int_clr()
                .set_bit()
                .ir_rx_fifo_of_int_clr()
                .set_bit()
                .ir_rx_cnt_thr_int_clr()
                .set_bit()
                .ir_rx_fifo_error_int_clr()
                .set_bit()
        });
    }
}
