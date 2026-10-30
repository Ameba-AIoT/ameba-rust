use bitflags::bitflags;
use rtl8721f_pac::{Can0Reg, Can1Reg, can0_reg};

// CAN controller typed register-block pointer constants.
pub const CAN0_PTR: *const can0_reg::RegisterBlock = Can0Reg::PTR;
pub const CAN1_PTR: *const can0_reg::RegisterBlock = Can1Reg::PTR as *const can0_reg::RegisterBlock;

// Fixed core clock: XTAL 40 MHz. CAN_CoreClockSet selects this source.
const XTAL_40M: u32 = 40_000_000; // SDK: ameba_clk.h : XTAL_40M

// Number of message buffers.
const MB_COUNT: usize = 16; // SDK: ameba_can.h : CAN_MESSAGE_BUFFER_SIZE

// CAN bit timing: sync segment is always 1 TQ.
const SYNC_SEG: u32 = 1;

// Hardware bit timing limits (from SDK CAN_TIMING_MIN / CAN_TIMING_MAX).
struct TimingLimits {
    prescaler_min: u16,
    prescaler_max: u16,
    prop_seg_min: u8,
    prop_seg_max: u8,
    phase_seg1_min: u8,
    phase_seg1_max: u8,
    phase_seg2_min: u8,
    phase_seg2_max: u8,
    sjw_min: u8,
    sjw_max: u8,
}

const LIMITS: TimingLimits = TimingLimits {
    prescaler_min: 1,
    prescaler_max: 256,
    prop_seg_min: 1,
    prop_seg_max: 8,
    phase_seg1_min: 1,
    phase_seg1_max: 8,
    phase_seg2_min: 1,
    phase_seg2_max: 8,
    sjw_min: 1,
    sjw_max: 4,
};

// FIFO virtual buffer index used when reading from the Rx FIFO.
const RX_FIFO_READ_IDX: u8 = 12; // SDK: ameba_can.h : CAN_RX_FIFO_READ_MSG_IDX

// ─────────────────────────────────────────────────────────────────────────────
// Public types
// ─────────────────────────────────────────────────────────────────────────────

/// CAN work mode.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WorkMode {
    Normal,
    /// Silence (no TX to bus).
    Silence,
    /// External loopback (TX feeds back through physical pin).
    ExternalLoopback,
    /// Internal loopback (TX tied internally, no bus traffic).
    InternalLoopback,
}

/// Frame type (data vs. remote).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FrameType {
    Data,
    Remote,
}

/// Frame ID format.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IdType {
    Standard,
    Extended,
}

/// CAN bit timing parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct BitTiming {
    pub prescaler: u16,
    pub sjw: u8,
    pub prop_seg: u8,
    pub phase_seg1: u8,
    pub phase_seg2: u8,
}

/// CAN initialisation configuration.
#[derive(Clone, Copy, Debug)]
pub struct CanConfig {
    pub auto_retx: bool,
    pub rx_fifo_en: bool,
    pub tri_sample: bool,
    pub timestamp_en: bool,
    pub timestamp_div: u8,
    pub err_warn_threshold: u16,
    pub work_mode: WorkMode,
    pub timing: BitTiming,
}

impl Default for CanConfig {
    fn default() -> Self {
        Self {
            auto_retx: false,
            rx_fifo_en: true,
            tri_sample: false,
            timestamp_en: true,
            timestamp_div: 128,
            err_warn_threshold: 64,
            work_mode: WorkMode::Normal,
            timing: BitTiming {
                prescaler: 2,
                sjw: 1,
                prop_seg: 2,
                phase_seg1: 13,
                phase_seg2: 4,
            },
        }
    }
}

/// Tx message descriptor.
#[derive(Clone, Debug)]
pub struct TxMsg {
    pub buf_idx: u8,
    pub frame_type: FrameType,
    pub id_type: IdType,
    pub std_id: u16,
    pub ext_id: u32,
    pub dlc: u8,
    /// 16 × u32 words (64 bytes max for FD; CAN 2.0 uses first 2 words).
    pub data: [u32; 16],
}

/// Rx message descriptor.
#[derive(Clone, Debug)]
pub struct RxMsg {
    pub buf_idx: u8,
    pub frame_type: FrameType,
    pub id_type: IdType,
    pub std_id: u16,
    pub ext_id: u32,
    pub dlc: u8,
    pub rx_lost: bool,
    pub timestamp: u16,
    pub id_mask: u32,
    pub ide_mask: bool,
    pub rtr_mask: bool,
    pub data: [u32; 16],
}

bitflags! {
    /// Interrupt enable/flag bits for `int_config` / `int_status` / `clear_int`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct IntFlags: u32 {
        const TX       = 1 << 0;
        const RX       = 1 << 1;
        const ERROR    = 1 << 2;
        const WAKEUP   = 1 << 3;
        const BUSOFF   = 1 << 4;
        const RAM_DONE = 1 << 5;
    }
}

bitflags! {
    /// Rx FIFO status flags.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct FifoStatus: u32 {
        const FULL     = 1 << 0;
        const EMPTY    = 1 << 1;
        const OVERFLOW = 1 << 2;
    }
}

bitflags! {
    /// Error counter status flags.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ErrCntStatus: u32 {
        const WARNING = 1 << 30;
        const PASSIVE = 1 << 28;
        const BUSOFF  = 1 << 29;
    }
}

/// Message buffer status nibble.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgBufStatus {
    PendingTx,
    FinishedTx,
    OverwriteSendTx,
    PendingRx,
    FinishedRx,
    Unknown(u32),
}

impl From<u32> for MsgBufStatus {
    fn from(v: u32) -> Self {
        match v & 0xf {
            0x1 => MsgBufStatus::PendingTx,
            0x2 => MsgBufStatus::FinishedTx,
            0x3 => MsgBufStatus::OverwriteSendTx,
            0x4 => MsgBufStatus::PendingRx,
            0x8 => MsgBufStatus::FinishedRx,
            other => MsgBufStatus::Unknown(other),
        }
    }
}

/// Bit timing calculation error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitTimingError {
    InvalidBitRate,
    NoSolutionFound,
    SegmentsOutOfRange,
}

// ─────────────────────────────────────────────────────────────────────────────
// CAN peripheral handle
// ─────────────────────────────────────────────────────────────────────────────

/// CAN peripheral driver.
///
/// Obtain an instance with [`Can::steal`], passing [`CAN0_PTR`] or [`CAN1_PTR`].
pub struct Can {
    ptr: *const can0_reg::RegisterBlock,
}

// SAFETY: embedded single-CPU target; `Can` has exclusive ownership of its peripheral.
unsafe impl Send for Can {}

impl Can {
    /// Construct a `Can` handle from a typed register-block pointer.
    ///
    /// # Safety
    /// Caller must ensure exclusive access to the peripheral at `ptr` and that
    /// `ptr` is one of `CAN0_PTR` or `CAN1_PTR`.
    pub unsafe fn steal(ptr: *const can0_reg::RegisterBlock) -> Self {
        Self { ptr }
    }

    #[inline(always)]
    fn regs(&self) -> &can0_reg::RegisterBlock {
        unsafe { &*self.ptr }
    }

    // ─── Sleep / Wake ────────────────────────────────────────────────────────

    /// Request the CAN controller to enter sleep mode.
    pub fn request_sleep(&self) {
        self.regs()
            .can_sleep_mode()
            .modify(|_, w| w.can_sleep_en().set_bit());
    }

    /// Request the CAN controller to wake from sleep mode.
    pub fn request_wake(&self) {
        self.regs()
            .can_sleep_mode()
            .modify(|_, w| w.can_sleep_en().clear_bit());
    }

    /// Returns `true` if the controller is currently in sleep state.
    pub fn sleep_state(&self) -> bool {
        self.regs()
            .can_sleep_mode()
            .read()
            .can_sleep_state()
            .bit_is_set()
    }

    // ─── Init ────────────────────────────────────────────────────────────────

    /// Initialise the peripheral from a [`CanConfig`].
    pub fn init(&self, cfg: &CanConfig) {
        let r = self.regs();

        // Bit timing: tseg1 = (prop_seg + phase_seg1 - 1), tseg2 = phase_seg2 - 1.
        r.can_bit_timing().modify(|_, w| unsafe {
            w.can_brp()
                .bits(cfg.timing.prescaler.saturating_sub(1) as u8)
                .can_sjw()
                .bits(cfg.timing.sjw)
                .can_tseg1()
                .bits(
                    cfg.timing
                        .prop_seg
                        .wrapping_add(cfg.timing.phase_seg1)
                        .wrapping_sub(1),
                )
                .can_tseg2()
                .bits(cfg.timing.phase_seg2.saturating_sub(1))
        });

        // Control register.
        r.can_ctl().modify(|_, w| {
            w.can_tri_sample()
                .bit(cfg.tri_sample)
                .auto_re_tx_en()
                .bit(cfg.auto_retx)
                .rx_fifo_en()
                .bit(cfg.rx_fifo_en)
                .test_mode_en()
                .bit(cfg.work_mode != WorkMode::Normal)
        });

        // Test mode configuration.
        if cfg.work_mode != WorkMode::Normal {
            let test_cfg: u8 = match cfg.work_mode {
                WorkMode::Silence => 0,
                WorkMode::ExternalLoopback => 1,
                WorkMode::InternalLoopback => 2,
                WorkMode::Normal => 0,
            };
            r.can_test()
                .modify(|_, w| unsafe { w.can_test_cfg().bits(test_cfg) });
        }

        // Error warning threshold.
        r.can_err_cnt_ctl()
            .modify(|_, w| unsafe { w.can_error_warn_th().bits(cfg.err_warn_threshold) });

        // Timestamp.
        if cfg.timestamp_en {
            r.can_time_stamp().modify(|_, w| unsafe {
                w.can_time_stamp_div()
                    .bits(cfg.timestamp_div)
                    .can_time_stamp_en()
                    .set_bit()
            });
        } else {
            r.can_time_stamp()
                .modify(|_, w| w.can_time_stamp_en().clear_bit());
        }
    }

    // ─── Enable / Disable ────────────────────────────────────────────────────

    /// Enable the CAN controller.
    pub fn enable(&self) {
        self.regs().can_ctl().modify(|_, w| w.can_en().set_bit());
    }

    /// Disable the CAN controller.
    pub fn disable(&self) {
        self.regs().can_ctl().modify(|_, w| w.can_en().clear_bit());
    }

    /// Request bus-on.
    pub fn bus_on(&self) {
        self.regs()
            .can_ctl()
            .modify(|_, w| w.bus_on_req().set_bit());
    }

    /// Release bus-on request.
    pub fn bus_off(&self) {
        self.regs()
            .can_ctl()
            .modify(|_, w| w.bus_on_req().clear_bit());
    }

    // ─── Low-power filter ────────────────────────────────────────────────────

    /// Configure wakeup-pin digital filter length.
    pub fn lp_filter_config(&self, flt_len: u8) {
        self.regs()
            .can_sleep_mode()
            .modify(|_, w| unsafe { w.can_wakepin_flt_length().bits(flt_len) });
    }

    /// Enable or disable the wakeup-pin digital filter.
    pub fn lp_filter_enable(&self, enable: bool) {
        self.regs()
            .can_sleep_mode()
            .modify(|_, w| w.can_wakepin_flt_en().bit(enable));
    }

    // ─── Bus status ──────────────────────────────────────────────────────────

    /// Returns `true` if the bus is in bus-on state.
    pub fn bus_status(&self) -> bool {
        self.regs().can_sts().read().bus_on_state().bit_is_set()
    }

    // ─── FIFO status ─────────────────────────────────────────────────────────

    /// Returns the FIFO status flags.
    pub fn fifo_status(&self) -> FifoStatus {
        let raw = self.regs().can_fifo_sts().read().bits();
        FifoStatus::from_bits_truncate(raw & 0x7)
    }

    /// Returns the FIFO message level (number of pending frames).
    pub fn fifo_level(&self) -> u8 {
        self.regs().can_fifo_sts().read().fifo_msg_lvl().bits()
    }

    // ─── Error counters ──────────────────────────────────────────────────────

    /// Returns the transmit error counter value.
    pub fn tx_err_count(&self) -> u16 {
        self.regs().can_err_cnt_sts().read().can_tec().bits()
    }

    /// Returns the receive error counter value.
    pub fn rx_err_count(&self) -> u16 {
        self.regs().can_err_cnt_sts().read().can_rec().bits()
    }

    /// Clears the transmit error counter.
    pub fn clear_tx_err_count(&self) {
        self.regs()
            .can_err_cnt_ctl()
            .modify(|_, w| w.tx_err_cnt_clr().set_bit());
    }

    /// Clears the receive error counter.
    pub fn clear_rx_err_count(&self) {
        self.regs()
            .can_err_cnt_ctl()
            .modify(|_, w| w.rx_err_cnt_clr().set_bit());
    }

    /// Returns the error counter status flags.
    pub fn err_cnt_status(&self) -> ErrCntStatus {
        let raw = self.regs().can_err_cnt_sts().read().bits();
        ErrCntStatus::from_bits_truncate(raw)
    }

    // ─── Interrupt control ───────────────────────────────────────────────────

    /// Enable or disable the global CAN interrupt sources in `flags`.
    pub fn int_config(&self, flags: IntFlags, enable: bool) {
        let bits = flags.bits();
        if enable {
            self.regs()
                .can_int_en()
                .modify(|r, w| unsafe { w.bits(r.bits() | bits) });
        } else {
            self.regs()
                .can_int_en()
                .modify(|r, w| unsafe { w.bits(r.bits() & !bits) });
        }
    }

    /// Returns the raw interrupt flag register value.
    pub fn int_status(&self) -> IntFlags {
        let raw = self.regs().can_int_flag().read().bits();
        IntFlags::from_bits_truncate(raw)
    }

    /// Clear specified interrupt flags (RW1C).
    pub fn clear_int(&self, flags: IntFlags) {
        // CAN_INT_FLAG is RW1C; PAC marks it read-only, so use write_volatile.
        unsafe {
            self.regs()
                .can_int_flag()
                .as_ptr()
                .write_volatile(flags.bits());
        }
    }

    /// Clear all interrupt flags.
    pub fn clear_all_int(&self) {
        unsafe {
            self.regs()
                .can_int_flag()
                .as_ptr()
                .write_volatile(0xFFFF_FFFF);
        }
    }

    // ─── Error status ────────────────────────────────────────────────────────

    /// Returns the raw error status register.
    pub fn err_status(&self) -> u32 {
        self.regs().can_err_status().read().bits()
    }

    /// Clear specified error status bits (RW1C).
    pub fn clear_err_status(&self, mask: u32) {
        unsafe {
            self.regs().can_err_status().as_ptr().write_volatile(mask);
        }
    }

    // ─── Message buffer interrupt control ────────────────────────────────────

    /// Enable or disable Rx message buffer interrupts.
    /// `buf_mask` is a 16-bit bitmask, one bit per buffer.
    pub fn rx_msgbuf_int_config(&self, buf_mask: u16, enable: bool) {
        if enable {
            self.regs().can_mb_rxint_en().modify(|r, w| unsafe {
                w.can_mb_rxint_en()
                    .bits(r.can_mb_rxint_en().bits() | buf_mask)
            });
        } else {
            self.regs().can_mb_rxint_en().modify(|r, w| unsafe {
                w.can_mb_rxint_en()
                    .bits(r.can_mb_rxint_en().bits() & !buf_mask)
            });
        }
    }

    /// Enable or disable Tx message buffer interrupts.
    pub fn tx_msgbuf_int_config(&self, buf_mask: u16, enable: bool) {
        if enable {
            self.regs().can_mb_txint_en().modify(|r, w| unsafe {
                w.can_mb_txint_en()
                    .bits(r.can_mb_txint_en().bits() | buf_mask)
            });
        } else {
            self.regs().can_mb_txint_en().modify(|r, w| unsafe {
                w.can_mb_txint_en()
                    .bits(r.can_mb_txint_en().bits() & !buf_mask)
            });
        }
    }

    // ─── Tx error flags ──────────────────────────────────────────────────────

    /// Returns the Tx message buffer error flag register.
    pub fn tx_msgbuf_err(&self) -> u32 {
        self.regs().can_tx_error_flag().read().bits()
    }

    /// Clear Tx message buffer error flags (RW1C).
    pub fn clear_tx_msgbuf_err(&self, flags: u32) {
        unsafe {
            self.regs()
                .can_tx_error_flag()
                .as_ptr()
                .write_volatile(flags);
        }
    }

    // ─── Message buffer status ───────────────────────────────────────────────

    /// Returns the raw status register for message buffer `idx` (0..15).
    pub fn msgbuf_status_reg(&self, idx: usize) -> u32 {
        msgbuf_sts_read(self.regs(), idx)
    }

    /// Returns the parsed status for message buffer `idx` (0..15).
    pub fn msgbuf_status(&self, idx: usize) -> MsgBufStatus {
        MsgBufStatus::from(msgbuf_sts_read(self.regs(), idx))
    }

    /// Returns `true` if message buffer `idx` is available (not pending TX or RX).
    pub fn msgbuf_available(&self, idx: usize) -> bool {
        let s = self.msgbuf_status(idx);
        s != MsgBufStatus::PendingTx && s != MsgBufStatus::PendingRx
    }

    // ─── TX / RX done status ─────────────────────────────────────────────────

    /// Returns the Tx done bitmask (one bit per message buffer).
    pub fn tx_done_status(&self) -> u32 {
        self.regs().can_tx_done().read().bits()
    }

    /// Returns `true` if message buffer `idx` completed transmission.
    pub fn msgbuf_tx_done(&self, idx: usize) -> bool {
        (self.regs().can_tx_done().read().bits() >> idx) & 1 != 0
    }

    /// Returns the Rx done bitmask.
    pub fn rx_done_status(&self) -> u32 {
        self.regs().can_rx_done().read().bits()
    }

    /// Returns `true` if message buffer `idx` completed reception.
    pub fn msgbuf_rx_done(&self, idx: usize) -> bool {
        (self.regs().can_rx_done().read().bits() >> idx) & 1 != 0
    }

    /// Clear specified Tx done bits (RW1C).
    pub fn clear_tx_done(&self, mask: u32) {
        unsafe {
            self.regs().can_tx_done().as_ptr().write_volatile(mask);
        }
    }

    /// Clear Tx done for a single message buffer (RW1C).
    pub fn clear_msgbuf_tx_done(&self, idx: usize) {
        self.clear_tx_done(1 << idx);
    }

    /// Clear specified Rx done bits (RW1C).
    pub fn clear_rx_done(&self, mask: u32) {
        unsafe {
            self.regs().can_rx_done().as_ptr().write_volatile(mask);
        }
    }

    /// Clear Rx done for a single message buffer (RW1C).
    pub fn clear_msgbuf_rx_done(&self, idx: usize) {
        self.clear_rx_done(1 << idx);
    }

    // ─── TX trigger ──────────────────────────────────────────────────────────

    /// Enable or disable the TX timer trigger.
    pub fn tx_trigger_enable(&self, enable: bool) {
        self.regs()
            .can_mb_trigger()
            .modify(|_, w| w.tx_trigger_en().bit(enable));
    }

    /// Configure TX timer trigger timing.
    pub fn tx_trigger_config(&self, begin: u16, close_offset: u8) {
        self.regs().can_mb_trigger().modify(|_, w| unsafe {
            w.tx_trigger_begin()
                .bits(begin)
                .tx_trigger_close_offset()
                .bits(close_offset)
        });
    }

    // ─── Message buffer DMA / address configuration ──────────────────────────

    /// Configure message buffer control: RX DMA enable and DMA offset.
    pub fn msgbuf_ctrl_config(&self, idx: usize, rxdma_en: bool, dma_offset: u32) {
        msgbuf_ctrl_modify(self.regs(), idx, rxdma_en, dma_offset);
    }

    /// Set the base address for message buffer `idx` in RAM.
    pub fn msgbuf_base_addr(&self, idx: usize, addr: u32) {
        msgbuf_base_set(self.regs(), idx, addr);
    }

    /// Set the end-of-message-buffer address register.
    pub fn msgbuf_end_addr(&self, end_addr: u32) {
        self.regs()
            .can_mb_ba_end()
            .modify(|_, w| unsafe { w.can_msg_ba_end().bits(end_addr as u16) });
    }

    /// Configure all 16 message buffer base addresses plus the end address.
    /// `addr_list` must have at least 17 elements.
    pub fn ram_buffer_map(&self, addr_list: &[u32]) {
        for (i, &addr) in addr_list[..MB_COUNT].iter().enumerate() {
            self.msgbuf_base_addr(i, addr);
        }
        self.msgbuf_end_addr(addr_list[MB_COUNT]);
    }

    // ─── DMA ─────────────────────────────────────────────────────────────────

    /// Set the RX DMA destination base address.
    ///
    /// C: `CANx->CAN_RXDMA_CFG = Addr`. The PAC has no writable address field
    /// for this register in the standard fields, so the raw address is written
    /// directly via `write_volatile`.
    pub fn rx_dma_dest_base_addr(&self, addr: u32) {
        unsafe {
            self.regs().can_rxdma_cfg().as_ptr().write_volatile(addr);
        }
    }

    /// Read the RX DMA data register.
    pub fn rx_dma_data(&self) -> u32 {
        self.regs().can_rx_dma_data().read().bits()
    }

    // ─── Clock ───────────────────────────────────────────────────────────────

    /// Returns the CAN core clock frequency in Hz (currently fixed to 40 MHz XTAL).
    pub fn core_clock_get() -> u32 {
        XTAL_40M
    }

    // ─── Bit timing calculation ───────────────────────────────────────────────

    /// Calculate optimal bit timing parameters for `bit_rate` bps.
    ///
    /// On success, fills `result` and returns `Ok(sample_point_error_permille)`.
    pub fn calc_bit_timing(bit_rate: u32, result: &mut BitTiming) -> Result<i32, BitTimingError> {
        if bit_rate == 0 {
            return Err(BitTimingError::InvalidBitRate);
        }

        let core_clock = Self::core_clock_get();
        let sample_point = default_sample_point(bit_rate);

        let max_tq = SYNC_SEG
            + LIMITS.prop_seg_max as u32
            + LIMITS.phase_seg1_max as u32
            + LIMITS.phase_seg2_max as u32;

        let start_prescaler = {
            let min_from_clock = core_clock / (max_tq * bit_rate);
            min_from_clock.max(LIMITS.prescaler_min as u32) as i32
        };

        let mut err_min = i32::MAX;
        let mut best = BitTiming::default();

        let mut prescaler = start_prescaler;
        while prescaler <= LIMITS.prescaler_max as i32 {
            if !core_clock.is_multiple_of(prescaler as u32 * bit_rate) {
                prescaler += 1;
                continue;
            }

            let total_tq = core_clock / (prescaler as u32 * bit_rate);
            let mut tmp = BitTiming::default();

            if let Ok(err) = calc_time_segments(total_tq, sample_point, &mut tmp)
                && err < err_min
            {
                err_min = err;
                best = tmp;
                best.prescaler = prescaler as u16;
                if err == 0 {
                    break;
                }
            }
            prescaler += 1;
        }

        if err_min == i32::MAX {
            return Err(BitTimingError::NoSolutionFound);
        }

        // SJW = min(phase_seg1, phase_seg2 / 2), clamped to limits.
        let sjw = best.phase_seg1.min(best.phase_seg2 / 2);
        best.sjw = sjw.clamp(LIMITS.sjw_min, LIMITS.sjw_max);

        *result = best;
        Ok(err_min)
    }

    // ─── RAM message buffer I/O ───────────────────────────────────────────────

    /// Write a Tx message into a RAM message buffer.
    pub fn write_msg(&self, msg: &TxMsg) {
        self.ram_write_setup(msg.buf_idx, true);
        self.ram_write_arb(msg);
        self.ram_write_cs_tx(msg, false);
        if msg.frame_type == FrameType::Data {
            self.ram_write_data(&msg.data);
        }
        self.ram_commit(msg.buf_idx, true);
    }

    /// Write a Tx message into the DMA RAM message buffer.
    ///
    /// Identical to `write_msg` but sets `CAN_BIT_RAM_RXTX` without
    /// unconditionally setting DLC (DLC only set for data frames).
    pub fn fill_tx_dma_buffer(&self, msg: &TxMsg) {
        self.ram_write_setup(msg.buf_idx, true);
        self.ram_write_arb(msg);

        // CS: rxtx=1, autoreply=0, dlc only for data frames.
        let r = self.regs();
        r.can_ram_cs().modify(|_, w| unsafe {
            let dlc = if msg.frame_type == FrameType::Data {
                msg.dlc & 0xf
            } else {
                0
            };
            w.can_ram_dlc()
                .bits(dlc)
                .can_ram_autoreply()
                .clear_bit()
                .can_ram_rxtx()
                .set_bit()
        });

        if msg.frame_type == FrameType::Data {
            self.ram_write_data(&msg.data);
        }

        self.ram_commit(msg.buf_idx, true);
    }

    /// Configure a message buffer for Rx (set filter / mask).
    pub fn set_rx_msg_buf(&self, msg: &RxMsg) {
        let r = self.regs();

        // CMD: setup access (no dir bit → read direction for RX setup).
        r.can_ram_cmd().modify(|_, w| unsafe {
            w.can_ram_acc_num()
                .bits(msg.buf_idx)
                .can_ram_acc_mask()
                .set_bit()
                .can_ram_acc_cs()
                .set_bit()
                .can_ram_acc_arb()
                .set_bit()
                .can_ram_acc_data0()
                .set_bit()
                // data0..15 — set all data access bits.
                .can_ram_acc_data1()
                .set_bit()
                .can_ram_acc_data2()
                .set_bit()
                .can_ram_acc_data3()
                .set_bit()
                .can_ram_acc_data4()
                .set_bit()
                .can_ram_acc_data5()
                .set_bit()
                .can_ram_acc_data6()
                .set_bit()
                .can_ram_acc_data7()
                .set_bit()
                .can_ram_acc_data8()
                .set_bit()
                .can_ram_acc_data9()
                .set_bit()
                .can_ram_acc_data10()
                .set_bit()
                .can_ram_acc_data11()
                .set_bit()
                .can_ram_acc_data12()
                .set_bit()
                .can_ram_acc_data13()
                .set_bit()
                .can_ram_acc_data14()
                .set_bit()
                .can_ram_acc_data15()
                .set_bit()
                .can_ram_buffer_en()
                .set_bit()
                .can_ram_dir()
                .set_bit()
        });

        // ARB: ID + IDE + RTR.
        let id_bits: u32 = match msg.id_type {
            IdType::Standard => (msg.std_id as u32 & 0x7ff) << 18,
            IdType::Extended => msg.ext_id & 0x1FFF_FFFF,
        };
        r.can_ram_arb().modify(|_, w| unsafe {
            w.can_ram_id()
                .bits(id_bits)
                .can_ram_ide()
                .bit(msg.id_type == IdType::Extended)
                .can_ram_rtr()
                .bit(msg.frame_type == FrameType::Remote)
        });

        // MASK: ID mask + IDE mask + RTR mask.
        let mask_bits: u32 = match msg.id_type {
            IdType::Standard => (msg.id_mask & 0x7ff) << 18,
            IdType::Extended => msg.id_mask & 0x1FFF_FFFF,
        };
        r.can_ram_mask().modify(|_, w| unsafe {
            w.can_ram_id_mask()
                .bits(mask_bits)
                .can_ram_ide_mask()
                .bit(msg.ide_mask)
                .can_ram_rtr_mask()
                .bit(msg.rtr_mask)
        });

        // Commit.
        r.can_ram_cmd().modify(|_, w| w.can_ram_start().set_bit());
        while r.can_ram_cmd().read().can_ram_start().bit_is_set() {}
    }

    /// Read a message from a RAM message buffer.
    pub fn read_msg(&self, rx: &mut RxMsg) {
        let r = self.regs();

        // CMD: read direction (dir=0), trigger start.
        r.can_ram_cmd().modify(|_, w| unsafe {
            w.can_ram_acc_num()
                .bits(rx.buf_idx)
                .can_ram_acc_mask()
                .set_bit()
                .can_ram_acc_cs()
                .set_bit()
                .can_ram_acc_arb()
                .set_bit()
                .can_ram_acc_data0()
                .set_bit()
                .can_ram_acc_data1()
                .set_bit()
                .can_ram_acc_data2()
                .set_bit()
                .can_ram_acc_data3()
                .set_bit()
                .can_ram_acc_data4()
                .set_bit()
                .can_ram_acc_data5()
                .set_bit()
                .can_ram_acc_data6()
                .set_bit()
                .can_ram_acc_data7()
                .set_bit()
                .can_ram_acc_data8()
                .set_bit()
                .can_ram_acc_data9()
                .set_bit()
                .can_ram_acc_data10()
                .set_bit()
                .can_ram_acc_data11()
                .set_bit()
                .can_ram_acc_data12()
                .set_bit()
                .can_ram_acc_data13()
                .set_bit()
                .can_ram_acc_data14()
                .set_bit()
                .can_ram_acc_data15()
                .set_bit()
                // buffer_en=1, dir=0 (read direction: RAM → staging registers).
                .can_ram_buffer_en()
                .set_bit()
                .can_ram_dir()
                .clear_bit()
                .can_ram_start()
                .set_bit()
        });
        while r.can_ram_cmd().read().can_ram_start().bit_is_set() {}

        // ARB.
        let arb = r.can_ram_arb().read();
        rx.frame_type = if arb.can_ram_rtr().bit_is_set() {
            FrameType::Remote
        } else {
            FrameType::Data
        };
        rx.id_type = if arb.can_ram_ide().bit_is_set() {
            IdType::Extended
        } else {
            IdType::Standard
        };
        let raw_id = arb.can_ram_id().bits();
        match rx.id_type {
            IdType::Standard => rx.std_id = (raw_id >> 18) as u16,
            IdType::Extended => rx.ext_id = raw_id,
        }

        // CS.
        let cs = r.can_ram_cs().read();
        rx.timestamp = cs.can_ram_timestamp().bits();
        rx.rx_lost = cs.can_ram_lost().bit_is_set();
        rx.dlc = cs.can_ram_dlc().bits();

        // MASK.
        let mask = r.can_ram_mask().read();
        rx.id_mask = mask.can_ram_id_mask().bits();
        rx.ide_mask = mask.can_ram_ide_mask().bit_is_set();
        rx.rtr_mask = mask.can_ram_rtr_mask().bit_is_set();

        // Data (data frame only).
        if rx.frame_type == FrameType::Data {
            self.ram_read_data(&mut rx.data);
        }
    }

    /// Read the next Rx message from the FIFO.
    pub fn read_rx_from_fifo(&self, rx: &mut RxMsg) {
        rx.buf_idx = RX_FIFO_READ_IDX;
        self.read_msg(rx);
    }

    /// Set up an auto-reply Tx (send data when a remote frame is received).
    pub fn tx_auto_reply(&self, msg: &TxMsg) {
        let r = self.regs();

        self.ram_write_setup(msg.buf_idx, true);
        self.ram_write_arb(msg);

        // CS: rxtx=1, autoreply=1.
        r.can_ram_cs().modify(|_, w| unsafe {
            w.can_ram_dlc()
                .bits(msg.dlc & 0xf)
                .can_ram_autoreply()
                .set_bit()
                .can_ram_rxtx()
                .set_bit()
        });

        // Data stored in normal (non-reversed) order for auto-reply.
        if msg.frame_type == FrameType::Data {
            self.ram_write_data_fwd(&msg.data);
        }

        self.ram_commit(msg.buf_idx, true);
    }

    /// Configure auto-reply Rx: set up the Rx filter and enable auto-reply flag.
    pub fn rx_auto_reply(&self, msg: &RxMsg) {
        let r = self.regs();

        r.can_ram_cmd().modify(|_, w| unsafe {
            w.can_ram_acc_num()
                .bits(msg.buf_idx)
                .can_ram_acc_mask()
                .set_bit()
                .can_ram_acc_cs()
                .set_bit()
                .can_ram_acc_arb()
                .set_bit()
                .can_ram_acc_data0()
                .set_bit()
                .can_ram_acc_data1()
                .set_bit()
                .can_ram_acc_data2()
                .set_bit()
                .can_ram_acc_data3()
                .set_bit()
                .can_ram_acc_data4()
                .set_bit()
                .can_ram_acc_data5()
                .set_bit()
                .can_ram_acc_data6()
                .set_bit()
                .can_ram_acc_data7()
                .set_bit()
                .can_ram_acc_data8()
                .set_bit()
                .can_ram_acc_data9()
                .set_bit()
                .can_ram_acc_data10()
                .set_bit()
                .can_ram_acc_data11()
                .set_bit()
                .can_ram_acc_data12()
                .set_bit()
                .can_ram_acc_data13()
                .set_bit()
                .can_ram_acc_data14()
                .set_bit()
                .can_ram_acc_data15()
                .set_bit()
                .can_ram_buffer_en()
                .set_bit()
                .can_ram_dir()
                .set_bit()
        });

        // ARB: read current, update RTR/IDE/ID.
        let id_bits: u32 = match msg.id_type {
            IdType::Standard => (msg.std_id as u32 & 0x7ff) << 18,
            IdType::Extended => msg.ext_id & 0x1FFF_FFFF,
        };
        r.can_ram_arb().modify(|_, w| unsafe {
            w.can_ram_id()
                .bits(id_bits)
                .can_ram_ide()
                .bit(msg.id_type == IdType::Extended)
                .can_ram_rtr()
                .bit(msg.frame_type == FrameType::Remote)
        });

        // CS: rxtx=0, autoreply=1.
        r.can_ram_cs()
            .modify(|_, w| w.can_ram_rxtx().clear_bit().can_ram_autoreply().set_bit());

        // MASK.
        let mask_bits: u32 = match msg.id_type {
            IdType::Standard => (msg.id_mask & 0x7ff) << 18,
            IdType::Extended => msg.id_mask & 0x1FFF_FFFF,
        };
        r.can_ram_mask().modify(|_, w| unsafe {
            w.can_ram_id_mask()
                .bits(mask_bits)
                .can_ram_ide_mask()
                .bit(msg.ide_mask)
                .can_ram_rtr_mask()
                .bit(msg.rtr_mask)
        });

        r.can_ram_cmd().modify(|_, w| w.can_ram_start().set_bit());
        while r.can_ram_cmd().read().can_ram_start().bit_is_set() {}
    }

    // ─── Private RAM access helpers ───────────────────────────────────────────

    /// Set up the RAM CMD register for a write (dir=1) or read (dir=0) transaction.
    fn ram_write_setup(&self, buf_idx: u8, write_dir: bool) {
        let r = self.regs();
        r.can_ram_cmd().modify(|_, w| unsafe {
            w.can_ram_acc_num()
                .bits(buf_idx)
                .can_ram_acc_mask()
                .set_bit()
                .can_ram_acc_cs()
                .set_bit()
                .can_ram_acc_arb()
                .set_bit()
                .can_ram_acc_data0()
                .set_bit()
                .can_ram_acc_data1()
                .set_bit()
                .can_ram_acc_data2()
                .set_bit()
                .can_ram_acc_data3()
                .set_bit()
                .can_ram_acc_data4()
                .set_bit()
                .can_ram_acc_data5()
                .set_bit()
                .can_ram_acc_data6()
                .set_bit()
                .can_ram_acc_data7()
                .set_bit()
                .can_ram_acc_data8()
                .set_bit()
                .can_ram_acc_data9()
                .set_bit()
                .can_ram_acc_data10()
                .set_bit()
                .can_ram_acc_data11()
                .set_bit()
                .can_ram_acc_data12()
                .set_bit()
                .can_ram_acc_data13()
                .set_bit()
                .can_ram_acc_data14()
                .set_bit()
                .can_ram_acc_data15()
                .set_bit()
                .can_ram_buffer_en()
                .set_bit()
                .can_ram_dir()
                .bit(write_dir)
        });
    }

    /// Write ARB (ID / IDE / RTR) from a `TxMsg`.
    fn ram_write_arb(&self, msg: &TxMsg) {
        let id_bits: u32 = match msg.id_type {
            IdType::Standard => (msg.std_id as u32 & 0x7ff) << 18,
            IdType::Extended => msg.ext_id & 0x1FFF_FFFF,
        };
        self.regs().can_ram_arb().modify(|_, w| unsafe {
            w.can_ram_id()
                .bits(id_bits)
                .can_ram_ide()
                .bit(msg.id_type == IdType::Extended)
                .can_ram_rtr()
                .bit(msg.frame_type == FrameType::Remote)
        });
    }

    /// Write the CS register for a Tx message (DLC + RXTX, no autoreply).
    fn ram_write_cs_tx(&self, msg: &TxMsg, autoreply: bool) {
        self.regs().can_ram_cs().modify(|_, w| unsafe {
            w.can_ram_dlc()
                .bits(msg.dlc & 0xf)
                .can_ram_autoreply()
                .bit(autoreply)
                .can_ram_rxtx()
                .set_bit()
        });
    }

    /// Trigger the RAM transaction start and poll until done.
    fn ram_commit(&self, _buf_idx: u8, _write_dir: bool) {
        let r = self.regs();
        r.can_ram_cmd().modify(|_, w| w.can_ram_start().set_bit());
        while r.can_ram_cmd().read().can_ram_start().bit_is_set() {}
    }

    /// Write data words in reversed order (index 15 first, as C driver does).
    fn ram_write_data(&self, data: &[u32; 16]) {
        let r = self.regs();
        // C: CANx->CAN_RAM_DATA_x[16 - i - 1] = TxMsg->Data_32[i]
        // PAC names: can_ram_fddata_0 .. can_ram_fddata_15
        // data[0] → fddata_15, data[15] → fddata_0
        r.can_ram_fddata_15().write(|w| unsafe { w.bits(data[0]) });
        r.can_ram_fddata_14().write(|w| unsafe { w.bits(data[1]) });
        r.can_ram_fddata_13().write(|w| unsafe { w.bits(data[2]) });
        r.can_ram_fddata_12().write(|w| unsafe { w.bits(data[3]) });
        r.can_ram_fddata_11().write(|w| unsafe { w.bits(data[4]) });
        r.can_ram_fddata_10().write(|w| unsafe { w.bits(data[5]) });
        r.can_ram_fddata_9().write(|w| unsafe { w.bits(data[6]) });
        r.can_ram_fddata_8().write(|w| unsafe { w.bits(data[7]) });
        r.can_ram_fddata_7().write(|w| unsafe { w.bits(data[8]) });
        r.can_ram_fddata_6().write(|w| unsafe { w.bits(data[9]) });
        r.can_ram_fddata_5().write(|w| unsafe { w.bits(data[10]) });
        r.can_ram_fddata_4().write(|w| unsafe { w.bits(data[11]) });
        r.can_ram_fddata_3().write(|w| unsafe { w.bits(data[12]) });
        r.can_ram_fddata_2().write(|w| unsafe { w.bits(data[13]) });
        r.can_ram_fddata_1().write(|w| unsafe { w.bits(data[14]) });
        r.can_ram_fddata_0().write(|w| unsafe { w.bits(data[15]) });
    }

    /// Write data words in forward order (for auto-reply: C uses [i] = Data_32[15-i]).
    fn ram_write_data_fwd(&self, data: &[u32; 16]) {
        let r = self.regs();
        // C: CANx->CAN_RAM_DATA_x[i] = TxMsg->Data_32[(16 - i - 1)]
        r.can_ram_fddata_0().write(|w| unsafe { w.bits(data[15]) });
        r.can_ram_fddata_1().write(|w| unsafe { w.bits(data[14]) });
        r.can_ram_fddata_2().write(|w| unsafe { w.bits(data[13]) });
        r.can_ram_fddata_3().write(|w| unsafe { w.bits(data[12]) });
        r.can_ram_fddata_4().write(|w| unsafe { w.bits(data[11]) });
        r.can_ram_fddata_5().write(|w| unsafe { w.bits(data[10]) });
        r.can_ram_fddata_6().write(|w| unsafe { w.bits(data[9]) });
        r.can_ram_fddata_7().write(|w| unsafe { w.bits(data[8]) });
        r.can_ram_fddata_8().write(|w| unsafe { w.bits(data[7]) });
        r.can_ram_fddata_9().write(|w| unsafe { w.bits(data[6]) });
        r.can_ram_fddata_10().write(|w| unsafe { w.bits(data[5]) });
        r.can_ram_fddata_11().write(|w| unsafe { w.bits(data[4]) });
        r.can_ram_fddata_12().write(|w| unsafe { w.bits(data[3]) });
        r.can_ram_fddata_13().write(|w| unsafe { w.bits(data[2]) });
        r.can_ram_fddata_14().write(|w| unsafe { w.bits(data[1]) });
        r.can_ram_fddata_15().write(|w| unsafe { w.bits(data[0]) });
    }

    /// Read data words in reversed order.
    fn ram_read_data(&self, data: &mut [u32; 16]) {
        let r = self.regs();
        data[0] = r.can_ram_fddata_15().read().bits();
        data[1] = r.can_ram_fddata_14().read().bits();
        data[2] = r.can_ram_fddata_13().read().bits();
        data[3] = r.can_ram_fddata_12().read().bits();
        data[4] = r.can_ram_fddata_11().read().bits();
        data[5] = r.can_ram_fddata_10().read().bits();
        data[6] = r.can_ram_fddata_9().read().bits();
        data[7] = r.can_ram_fddata_8().read().bits();
        data[8] = r.can_ram_fddata_7().read().bits();
        data[9] = r.can_ram_fddata_6().read().bits();
        data[10] = r.can_ram_fddata_5().read().bits();
        data[11] = r.can_ram_fddata_4().read().bits();
        data[12] = r.can_ram_fddata_3().read().bits();
        data[13] = r.can_ram_fddata_2().read().bits();
        data[14] = r.can_ram_fddata_1().read().bits();
        data[15] = r.can_ram_fddata_0().read().bits();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free functions for indexed register access
// (PAC exposes can_mb0..can_mb15 as separate registers, not an array.)
// ─────────────────────────────────────────────────────────────────────────────

fn msgbuf_sts_read(r: &can0_reg::RegisterBlock, idx: usize) -> u32 {
    match idx {
        0 => r.can_mb0_sts().read().bits(),
        1 => r.can_mb1_sts().read().bits(),
        2 => r.can_mb2_sts().read().bits(),
        3 => r.can_mb3_sts().read().bits(),
        4 => r.can_mb4_sts().read().bits(),
        5 => r.can_mb5_sts().read().bits(),
        6 => r.can_mb6_sts().read().bits(),
        7 => r.can_mb7_sts().read().bits(),
        8 => r.can_mb8_sts().read().bits(),
        9 => r.can_mb9_sts().read().bits(),
        10 => r.can_mb10_sts().read().bits(),
        11 => r.can_mb11_sts().read().bits(),
        12 => r.can_mb12_sts().read().bits(),
        13 => r.can_mb13_sts().read().bits(),
        14 => r.can_mb14_sts().read().bits(),
        15 => r.can_mb15_sts().read().bits(),
        _ => 0,
    }
}

fn msgbuf_ctrl_modify(r: &can0_reg::RegisterBlock, idx: usize, rxdma_en: bool, dma_offset: u32) {
    // The 16 ctrl registers share the same field layout (can_msgN_ba / can_msgN_rxdma_en)
    // but each Spec type is distinct. Use a macro to avoid repetition.
    macro_rules! do_ctrl {
        ($reg:ident) => {
            r.$reg().modify(|cur_r, w| unsafe {
                let mut v = cur_r.bits();
                // Clear rxdma_en bit (bit 24) and ba field (bits 0:8).
                v &= !(1 << 24 | 0x1FF);
                if rxdma_en {
                    v |= 1 << 24;
                }
                v |= (dma_offset & 0x1FF) as u32;
                w.bits(v)
            })
        };
    }
    match idx {
        0 => do_ctrl!(can_mb0_ctrl),
        1 => do_ctrl!(can_mb1_ctrl),
        2 => do_ctrl!(can_mb2_ctrl),
        3 => do_ctrl!(can_mb3_ctrl),
        4 => do_ctrl!(can_mb4_ctrl),
        5 => do_ctrl!(can_mb5_ctrl),
        6 => do_ctrl!(can_mb6_ctrl),
        7 => do_ctrl!(can_mb7_ctrl),
        8 => do_ctrl!(can_mb8_ctrl),
        9 => do_ctrl!(can_mb9_ctrl),
        10 => do_ctrl!(can_mb10_ctrl),
        11 => do_ctrl!(can_mb11_ctrl),
        12 => do_ctrl!(can_mb12_ctrl),
        13 => do_ctrl!(can_mb13_ctrl),
        14 => do_ctrl!(can_mb14_ctrl),
        15 => do_ctrl!(can_mb15_ctrl),
        _ => 0,
    };
}

fn msgbuf_base_set(r: &can0_reg::RegisterBlock, idx: usize, addr: u32) {
    // Only the ba field (bits 0:8) is updated; rxdma_en preserved.
    macro_rules! do_base {
        ($reg:ident) => {
            r.$reg()
                .modify(|cur_r, w| unsafe { w.bits((cur_r.bits() & !0x1FF) | (addr & 0x1FF)) })
        };
    }
    match idx {
        0 => do_base!(can_mb0_ctrl),
        1 => do_base!(can_mb1_ctrl),
        2 => do_base!(can_mb2_ctrl),
        3 => do_base!(can_mb3_ctrl),
        4 => do_base!(can_mb4_ctrl),
        5 => do_base!(can_mb5_ctrl),
        6 => do_base!(can_mb6_ctrl),
        7 => do_base!(can_mb7_ctrl),
        8 => do_base!(can_mb8_ctrl),
        9 => do_base!(can_mb9_ctrl),
        10 => do_base!(can_mb10_ctrl),
        11 => do_base!(can_mb11_ctrl),
        12 => do_base!(can_mb12_ctrl),
        13 => do_base!(can_mb13_ctrl),
        14 => do_base!(can_mb14_ctrl),
        15 => do_base!(can_mb15_ctrl),
        _ => 0,
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Bit timing calculation helpers
// ─────────────────────────────────────────────────────────────────────────────

/// CiA default sample point in permille based on bitrate.
fn default_sample_point(bit_rate: u32) -> u32 {
    if bit_rate > 800_000 {
        750
    } else if bit_rate > 500_000 {
        800
    } else {
        875
    }
}

fn clamp_u32(v: u32, lo: u32, hi: u32) -> u32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

/// Calculate PropSeg / PhaseSeg1 / PhaseSeg2 from TotalTq and target SamplePoint.
/// Returns `Ok(abs_error_permille)` or `Err(())` if hardware limits cannot be met.
fn calc_time_segments(total_tq: u32, sample_point: u32, result: &mut BitTiming) -> Result<i32, ()> {
    let tseg1_max = LIMITS.prop_seg_max as u32 + LIMITS.phase_seg1_max as u32;
    let tseg1_min = LIMITS.prop_seg_min as u32 + LIMITS.phase_seg1_min as u32;

    // tseg2 = total_tq * (1 - sample_point/1000)
    let mut tseg2 = total_tq - total_tq * sample_point / 1000;
    tseg2 = clamp_u32(
        tseg2,
        LIMITS.phase_seg2_min as u32,
        LIMITS.phase_seg2_max as u32,
    );

    let mut tseg1 = total_tq.saturating_sub(SYNC_SEG).saturating_sub(tseg2);

    if tseg1 > tseg1_max {
        tseg1 = tseg1_max;
        tseg2 = total_tq.saturating_sub(SYNC_SEG).saturating_sub(tseg1);
        if tseg2 > LIMITS.phase_seg2_max as u32 {
            return Err(());
        }
    } else if tseg1 < tseg1_min {
        tseg1 = tseg1_min;
        tseg2 = total_tq.saturating_sub(SYNC_SEG).saturating_sub(tseg1);
        if tseg2 < LIMITS.phase_seg2_min as u32 {
            return Err(());
        }
    }

    result.phase_seg2 = tseg2 as u8;

    // Split tseg1 into prop_seg + phase_seg1 evenly.
    let prop = clamp_u32(
        tseg1 / 2,
        LIMITS.prop_seg_min as u32,
        LIMITS.prop_seg_max as u32,
    );
    let mut ph1 = tseg1 - prop;

    if ph1 > LIMITS.phase_seg1_max as u32 {
        ph1 = LIMITS.phase_seg1_max as u32;
    } else if ph1 < LIMITS.phase_seg1_min as u32 {
        ph1 = LIMITS.phase_seg1_min as u32;
    }
    let prop = tseg1 - ph1;

    result.prop_seg = prop as u8;
    result.phase_seg1 = ph1 as u8;

    // Actual sample point.
    let actual_sp = (SYNC_SEG + tseg1) * 1000 / total_tq;
    let err = if actual_sp > sample_point {
        (actual_sp - sample_point) as i32
    } else {
        (sample_point - actual_sp) as i32
    };
    Ok(err)
}
