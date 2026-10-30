//! GDMA0 driver for RTL8721F — `#![no_std]`
//!
//! Provides single-block mem-to-mem DMA with Blocking and Async paths, plus
//! an auto-reload multi-block mode (`mem_to_mem_reload_start`) and a raw
//! per-block/per-transfer ISR callback (`register_irq`/`GdmaIrqHandler`) for
//! streaming/ping-pong use cases where executor-scheduling latency between
//! blocks is unacceptable.
//! GDMA0 has 8 channels, each with its own IRQ (GDMA0_CHANNELn_IRQ = 30+n,
//! SDK: ameba_vector_table.h : GDMA0_CHANNEL0_IRQ).

use core::future::Future;
use core::marker::PhantomData;
use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::task::{Context, Poll};
use embassy_sync::waitqueue::AtomicWaker;

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

pub const NUM_CHANNELS: usize = 8;

// CTL_L bit fields — SDK: ameba_gdma.h : GDMA_BIT_CTLx_L_* / GDMA_MASK_CTLx_L_* / GDMA_CTLx_L_*
const CTL_INT_EN: u32 = 1 << 0; // SDK: ameba_gdma.h : GDMA_BIT_CTLx_L_INT_EN
const CTL_DST_TR_WIDTH_32: u32 = 2 << 1; // SDK: ameba_gdma.h : TrWidthFourBytes, bits 3:1 (GDMA_MASK_CTLx_L_DST_TR_WIDTH missing; field at bits 3:1 per PAC ctl0_l_dst_tr_width)
const CTL_SRC_TR_WIDTH_32: u32 = 2 << 4; // SDK: ameba_gdma.h : TrWidthFourBytes, bits 6:4 (per PAC ctl0_l_src_tr_width)
const CTL_DINC_INC: u32 = 0 << 7; // SDK: ameba_gdma.h : GDMA_CTLx_L_DINC(0), value 0 = increment (NoChange=2 implies 0=increment)
const CTL_SINC_INC: u32 = 0 << 9; // SDK: ameba_gdma.h : GDMA_CTLx_L_SINC(0), value 0 = increment
const CTL_DINC_FIXED: u32 = 2 << 7; // SDK: ameba_gdma.h : GDMA_CTLx_L_DINC(NoChange), NoChange=2
const CTL_SINC_FIXED: u32 = 2 << 9; // SDK: ameba_gdma.h : GDMA_CTLx_L_SINC(NoChange), NoChange=2
const CTL_DEST_MSIZE_8: u32 = 2 << 11; // SDK: ameba_gdma.h : MsizeEight (GDMA_MASK_CTLx_L_DEST_MSIZE bits 13:11)
const CTL_SRC_MSIZE_8: u32 = 2 << 14; // SDK: ameba_gdma.h : MsizeEight (GDMA_MASK_CTLx_L_SRC_MSIZE bits 16:14)
const CTL_TT_FC_MEM_MEM: u32 = 0 << 20; // SDK: ameba_gdma.h : TTFCMemToMem (GDMA_MASK_CTLx_L_TT_FC bits 22:20)
const CTL_TT_FC_MEM_PER: u32 = 1 << 20; // SDK: ameba_gdma.h : TTFCMemToPeri
const CTL_TT_FC_PER_MEM: u32 = 2 << 20; // SDK: ameba_gdma.h : TTFCPeriToMem

/// Transfer width per DMA beat. SDK: ameba_gdma.h : TrWidthOneByte / TrWidthTwoBytes / TrWidthFourBytes
#[derive(Clone, Copy)]
pub enum TrWidth {
    OneByte = 0,   // SDK: ameba_gdma.h : TrWidthOneByte
    TwoBytes = 1,  // SDK: ameba_gdma.h : TrWidthTwoBytes
    FourBytes = 2, // SDK: ameba_gdma.h : TrWidthFourBytes
}

impl TrWidth {
    fn bytes(self) -> usize {
        match self {
            TrWidth::OneByte => 1,
            TrWidth::TwoBytes => 2,
            TrWidth::FourBytes => 4,
        }
    }
}

/// Burst transaction length. SDK: ameba_gdma.h : MsizeOne / MsizeFour / MsizeEight
#[derive(Clone, Copy)]
pub enum Msize {
    One = 0,   // SDK: ameba_gdma.h : MsizeOne
    Four = 1,  // SDK: ameba_gdma.h : MsizeFour
    Eight = 2, // SDK: ameba_gdma.h : MsizeEight
}

/// Bundled transfer-width / burst-size / hardware-handshake parameters for one DMA channel.
#[derive(Clone, Copy)]
pub(crate) struct TrParams {
    pub src_width: TrWidth,
    pub dst_width: TrWidth,
    pub src_msize: Msize,
    pub dst_msize: Msize,
    pub hs_id: u8,
}

// CFG_L bit fields — SDK: ameba_gdma.h : GDMA_BIT_CFGx_L_*
const CFG_HS_SEL_DST: u32 = 1 << 10; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_L_HS_SEL_DST
const CFG_HS_SEL_SRC: u32 = 1 << 11; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_L_HS_SEL_SRC

// CFG_L bit fields (channel) — SDK: ameba_gdma.h : GDMA_BIT_CFGx_L_*
const CFG_L_INACTIVE: u32 = 1 << 0; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_L_INACTIVE
const CFG_L_CH_SUSP: u32 = 1 << 8; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_L_CH_SUSP
const CFG_L_RELOAD_SRC: u32 = 1 << 30; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_L_RELOAD_SRC
const CFG_L_RELOAD_DST: u32 = 1 << 31; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_L_RELOAD_DST

/// Auto-reload selection for [`DmaChannel::mem_to_mem_reload_start`] /
/// [`DmaChannel::clean_auto_reload`]. SDK: ameba_gdma.h : GDMA_ReloadSrc / GDMA_ReloadDst
#[derive(Clone, Copy, Default)]
pub struct ReloadConfig {
    pub reload_src: bool,
    pub reload_dst: bool,
}

// CFG_H bit fields — SDK: ameba_gdma.h : GDMA_BIT_CFGx_H_* / GDMA_MASK_CFGx_H_*
const CFG_PROTCTL_NS: u32 = 1 << 3; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_H_PROTCTL (1=NS DMA)
const CFG_H_DEST_PER_SHF: u32 = 11; // SDK: ameba_gdma.h : GDMA_MASK_CFGx_H_DEST_PER (0xF << 11)
const CFG_H_SRC_PER_SHF: u32 = 7; // SDK: ameba_gdma.h : GDMA_MASK_CFGx_H_SRC_PER (0xF << 7)
const CFG_H_EXTENDED_DEST_PER1: u32 = 1 << 16; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_H_EXTENDED_DEST_PER1
const CFG_H_EXTENDED_SRC_PER1: u32 = 1 << 15; // SDK: ameba_gdma.h : GDMA_BIT_CFGx_H_EXTENDED_SRC_PER1

// ─────────────────────────────────────────────────────────────────────────────
// ChannelRegs — HAL-side struct mirroring one GDMA channel's register layout
// ─────────────────────────────────────────────────────────────────────────────

/// One GDMA channel's registers (0x58 bytes).
/// Layout derived from PAC gdma0_reg::RegisterBlock: sar0 @ +0x00, sar1 @ +0x58.
/// SDK reference: ameba_gdma.h : GDMA_ChannelTypeDef
///
/// Note: the PAC exposes per-channel registers as individually-named accessors (ctl0_l()..ctl7_l())
/// with no array-indexed API. This struct mirrors the PAC layout to enable runtime channel indexing
/// via ch_ptr(n). The size_of assert below guarantees the layout stays in sync with the PAC.
#[repr(C)]
struct ChannelRegs {
    sar: u32,           // +0x00 GDMA_SARx
    curr_sar: u32,      // +0x04 (read-only, layout placeholder)
    dar: u32,           // +0x08 GDMA_DARx
    curr_dar: u32,      // +0x0C (read-only, layout placeholder)
    llp: u32,           // +0x10 GDMA_LLPx_L
    _llp_h: u32,        // +0x14
    ctl_l: u32,         // +0x18 GDMA_CTLx_L
    ctl_h: u32,         // +0x1C GDMA_CTLx_H (block size)
    _pad: [u32; 8],     // +0x20–0x3F
    cfg_l: u32,         // +0x40 GDMA_CFGx_L
    cfg_h: u32,         // +0x44 GDMA_CFGx_H
    _sgr_dsr: [u32; 4], // +0x48–0x54 (SGR_L, SGR_H, DSR_L, DSR_H)
} // +0x58: next channel

const _: () = assert!(
    core::mem::size_of::<ChannelRegs>() == 0x58,
    "ChannelRegs size mismatch — verify PAC sar0→sar1 stride",
);

fn ch_ptr(ch: u8) -> *mut ChannelRegs {
    (rtl8721f_pac::Gdma0Reg::PTR as *mut ChannelRegs).wrapping_add(ch as usize)
}

fn gdma() -> &'static rtl8721f_pac::gdma0_reg::RegisterBlock {
    unsafe { &*rtl8721f_pac::Gdma0Reg::PTR }
}

// ─────────────────────────────────────────────────────────────────────────────
// IRQ mask / clear helpers
// ─────────────────────────────────────────────────────────────────────────────

fn int_enable(ch: u8) {
    // PAC: MASKTFR_L/MASKBLOCK_L/MASKERR_L int_mask_we bits[15:8] = WE, int_mask bits[7:0] = value
    let val = 1u8 << ch;
    gdma().masktfr_l().write(|w| unsafe {
        w.masktfr_l_int_mask_we()
            .bits(val)
            .masktfr_l_int_mask()
            .bits(val)
    });
    gdma().maskblock_l().write(|w| unsafe {
        w.maskblock_l_int_mask_we()
            .bits(val)
            .maskblock_l_int_mask()
            .bits(val)
    });
    gdma().maskerr_l().write(|w| unsafe {
        w.maskerr_l_int_mask_we()
            .bits(val)
            .maskerr_l_int_mask()
            .bits(val)
    });
}

fn int_disable(ch: u8) {
    // Clear channel-level INT_EN. SDK: GDMA_INTConfig(DISABLE) → CTLx_L &= ~GDMA_BIT_CTLx_L_INT_EN
    let r = ch_ptr(ch);
    unsafe {
        let ctl_l = read_volatile(addr_of!((*r).ctl_l));
        write_volatile(addr_of_mut!((*r).ctl_l), ctl_l & !CTL_INT_EN);
    }
    // Mask global interrupt (WE=1, value=0). SDK: GDMA_INT_MASK(MASKTFR_L / MASKBLOCK_L / MASKERR_L, ch)
    let we = 1u8 << ch;
    gdma()
        .masktfr_l()
        .write(|w| unsafe { w.masktfr_l_int_mask_we().bits(we) });
    gdma()
        .maskblock_l()
        .write(|w| unsafe { w.maskblock_l_int_mask_we().bits(we) });
    gdma()
        .maskerr_l()
        .write(|w| unsafe { w.maskerr_l_int_mask_we().bits(we) });
}

fn int_clear(ch: u8) {
    let bit = 1u16 << ch;
    gdma()
        .cleartfr_l()
        .write(|w| unsafe { w.cleartfr_l_clear_tfr().bits(bit) });
    gdma()
        .clearblock_l()
        .write(|w| unsafe { w.clearblock_l_clear_block().bits(bit) }); // SDK: GDMA_ClearINT clears BlockType too
    gdma()
        .clearerr_l()
        .write(|w| unsafe { w.clearerr_l_clear_protocol_error().bits(bit) });
    gdma()
        .clearerr_h()
        .write(|w| unsafe { w.clearerr_h_clear_sec_error().bits(bit) });
}

/// Read which interrupt type(s) are pending (masked status — respects the
/// per-channel masks already set by `int_enable`) for `ch`, WITHOUT clearing.
/// SDK: GDMA_ClearINT reads STATUSTFR_L/STATUSBLOCK_L/STATUSERR_L/STATUSERR_H
/// before clearing; the reference `raw_gdma_multiblock` example deliberately
/// calls the clear only *after* it has consumed the just-completed block's
/// DST data ("Clear Pending ISR, next block will start transfer") — on this
/// IP, an auto-reload channel holds off starting its next block until the
/// pending interrupt is cleared. Clearing up front would release the hardware
/// to start overwriting the reused DST buffer before the user callback has a
/// chance to read it.
fn peek_isr(ch: u8) -> u32 {
    let bit = 1u16 << ch;
    let mut isr = 0u32;
    if gdma().statustfr_l().read().statustfr_l_status_tfr().bits() & bit != 0 {
        isr |= ISR_TRANSFER;
    }
    if gdma()
        .statusblock_l()
        .read()
        .statusblock_l_status_block()
        .bits()
        & bit
        != 0
    {
        isr |= ISR_BLOCK;
    }
    if gdma().statuserr_l().read().statuserr_l_status_err().bits() & bit != 0 {
        isr |= ISR_ERROR;
    }
    if gdma().statuserr_h().read().statuserr_h_status_err().bits() & bit != 0 {
        isr |= ISR_ERROR;
    }
    isr
}

// ─────────────────────────────────────────────────────────────────────────────
// Mode markers
// ─────────────────────────────────────────────────────────────────────────────

// One definition for the whole HAL; see `crate::mode`.
pub use crate::mode::{Async, Blocking};

// ─────────────────────────────────────────────────────────────────────────────
// Raw ISR callback (user-registered, independent of the async waker path)
// ─────────────────────────────────────────────────────────────────────────────

/// GDMA_Interrupt_Type bitmap. SDK: ameba_gdma.h : TransferType / BlockType / ErrType
pub const ISR_TRANSFER: u32 = 0x01;
pub const ISR_BLOCK: u32 = 0x02;
pub const ISR_ERROR: u32 = 0x10;

/// Raw GDMA channel interrupt event, decoded from the masked status
/// registers by `clear_isr` (mirrors SDK `GDMA_ClearINT`'s return bitmap).
/// For a plain single-block transfer this fires once with `transfer_done`
/// set. For an auto-reload/multi-block transfer started via
/// [`DmaChannel::mem_to_mem_reload_start`], it fires once per block with only
/// `block_done` set — the channel keeps running automatically, no CPU
/// intervention needed to start the next block — until
/// [`DmaChannel::clean_auto_reload`] has been called, at which point the
/// final block's completion sets `transfer_done` instead.
#[derive(Clone, Copy, Debug, Default)]
pub struct GdmaIrqEvent {
    /// A block finished (IntBlock). Set on every block of an auto-reload
    /// transfer, including — possibly alongside `transfer_done` — the final one.
    pub block_done: bool,
    /// The whole transfer finished (IntTfr); the channel has stopped and the
    /// async `Future` (if one is in flight on this channel) has been woken.
    pub transfer_done: bool,
    /// Protocol or secure-access error (IntErr, either side). Terminal like
    /// `transfer_done` — the channel is disabled after this fires.
    pub error: bool,
    /// Raw bitmap: `ISR_TRANSFER | ISR_BLOCK | ISR_ERROR`.
    pub isr: u32,
}

/// User-registered raw ISR callback for a DMA channel. Runs directly in the
/// channel's NVIC interrupt context, independent of the async
/// `mem_to_mem`/`periph_tx`/`periph_rx` `Future` path. Register via
/// [`DmaChannel::register_irq`].
pub trait GdmaIrqHandler: Send {
    fn on_irq(&mut self, event: GdmaIrqEvent);
}

/// Type-erased trampoline stored in `ChannelState::irq_handler`, resolved
/// back to `H::on_irq` via the matching `irq_data` pointer. Mirrors SPI's
/// `spi_handler_trampoline`.
unsafe fn gdma_handler_trampoline<H: GdmaIrqHandler>(data: usize, event: GdmaIrqEvent) {
    let handler = unsafe { &mut *(data as *mut H) };
    handler.on_irq(event);
}

fn dispatch_user_irq(ch: u8, event: GdmaIrqEvent) {
    let state = &CHAN_STATE[ch as usize];
    let handler_fn = state.irq_handler.load(Ordering::Acquire);
    if handler_fn == 0 {
        return;
    }
    let data = state.irq_data.load(Ordering::Acquire);
    let trampoline: unsafe fn(usize, GdmaIrqEvent) = unsafe { core::mem::transmute(handler_fn) };
    unsafe { trampoline(data, event) };
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-channel async state
// ─────────────────────────────────────────────────────────────────────────────

struct ChannelState {
    waker: AtomicWaker,
    done: AtomicBool,
    irq_handler: AtomicUsize, // unsafe fn(usize, GdmaIrqEvent) as usize; 0 = none registered
    irq_data: AtomicUsize,    // *mut H as usize, paired with irq_handler
}

impl ChannelState {
    const fn new() -> Self {
        Self {
            waker: AtomicWaker::new(),
            done: AtomicBool::new(false),
            irq_handler: AtomicUsize::new(0),
            irq_data: AtomicUsize::new(0),
        }
    }
}

static CHAN_STATE: [ChannelState; NUM_CHANNELS] = [
    ChannelState::new(),
    ChannelState::new(),
    ChannelState::new(),
    ChannelState::new(),
    ChannelState::new(),
    ChannelState::new(),
    ChannelState::new(),
    ChannelState::new(),
];

// ─────────────────────────────────────────────────────────────────────────────
// DmaChannel struct
// ─────────────────────────────────────────────────────────────────────────────

pub struct DmaChannel<const CH: u8, MODE> {
    _mode: PhantomData<MODE>,
}

impl<const CH: u8, MODE> DmaChannel<CH, MODE> {
    /// Obtain a channel handle.
    ///
    /// # Safety
    /// Caller must guarantee no other `DmaChannel<CH, _>` exists simultaneously.
    pub unsafe fn steal() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<const CH: u8> DmaChannel<CH, Blocking> {
    /// Convert to async mode; enables NVIC IRQ for this channel.
    pub fn into_async(self) -> DmaChannel<CH, Async> {
        const GDMA_IRQS: [rtl8721f_pac::Interrupt; 8] = [
            rtl8721f_pac::Interrupt::GDMA0_CHANNEL0_IRQ, // SDK: ameba_vector_table.h : IRQ 30
            rtl8721f_pac::Interrupt::GDMA0_CHANNEL1_IRQ,
            rtl8721f_pac::Interrupt::GDMA0_CHANNEL2_IRQ,
            rtl8721f_pac::Interrupt::GDMA0_CHANNEL3_IRQ,
            rtl8721f_pac::Interrupt::GDMA0_CHANNEL4_IRQ,
            rtl8721f_pac::Interrupt::GDMA0_CHANNEL5_IRQ,
            rtl8721f_pac::Interrupt::GDMA0_CHANNEL6_IRQ,
            rtl8721f_pac::Interrupt::GDMA0_CHANNEL7_IRQ, // SDK: ameba_vector_table.h : IRQ 37
        ];
        unsafe { cortex_m::peripheral::NVIC::unmask(GDMA_IRQS[CH as usize]) };
        DmaChannel { _mode: PhantomData }
    }

    /// Transfer `len` bytes from `src` to `dst`. Busy-waits for completion.
    ///
    /// `src` and `dst` must be 4-byte aligned; `len` must be a multiple of 4.
    pub fn mem_to_mem_blocking(&mut self, src: *const u8, dst: *mut u8, len: usize) {
        setup_mem_to_mem(CH, src, dst, len, false);
        // CHEN bit auto-clears when the transfer completes.
        while gdma().chenreg_l_1().read().chenreg_l_1_ch_en().bits() >> CH & 1 != 0 {
            core::hint::spin_loop();
        }
        int_clear(CH);
    }
}

impl<const CH: u8> DmaChannel<CH, Async> {
    /// Start an async mem-to-mem transfer. Await the returned future.
    ///
    /// `src` and `dst` must be 4-byte aligned; `len` must be a multiple of 4.
    pub fn mem_to_mem<'a>(
        &'a mut self,
        src: *const u8,
        dst: *mut u8,
        len: usize,
    ) -> DmaFuture<'a, CH> {
        CHAN_STATE[CH as usize].done.store(false, Ordering::Release);
        setup_mem_to_mem(CH, src, dst, len, true);
        DmaFuture { _ch: PhantomData }
    }

    /// Start an async peripheral TX transfer (memory → peripheral FIFO).
    ///
    /// `src_width`/`dst_width` and `src_msize`/`dst_msize` must be chosen by the
    /// caller based on the peripheral frame size and buffer alignment.
    /// SDK: SSI_TXGDMA_Init selects these dynamically per DataFrameSize and alignment.
    pub(crate) fn periph_tx<'a>(
        &'a mut self,
        src: *const u8,
        periph_dr: *mut u32,
        len: usize,
        p: TrParams,
    ) -> DmaFuture<'a, CH> {
        CHAN_STATE[CH as usize].done.store(false, Ordering::Release);
        setup_periph_tx(CH, src, periph_dr, len, p, true);
        DmaFuture { _ch: PhantomData }
    }

    /// Start an async peripheral RX transfer (peripheral FIFO → memory).
    ///
    /// `src_width`/`dst_width` and `src_msize`/`dst_msize` must be chosen by the
    /// caller based on the peripheral frame size and buffer alignment.
    /// SDK: SSI_RXGDMA_Init selects these dynamically per DataFrameSize and alignment.
    pub(crate) fn periph_rx<'a>(
        &'a mut self,
        periph_dr: *const u32,
        dst: *mut u8,
        len: usize,
        p: TrParams,
    ) -> DmaFuture<'a, CH> {
        CHAN_STATE[CH as usize].done.store(false, Ordering::Release);
        setup_periph_rx(CH, periph_dr, dst, len, p, true);
        DmaFuture { _ch: PhantomData }
    }

    /// Start a continuous auto-reload mem-to-mem transfer: SAR/DAR
    /// automatically reload to their initial value at the end of every
    /// `block_len`-byte block (SDK: ameba_gdma.h : GDMA_ReloadSrc /
    /// GDMA_ReloadDst), so the channel keeps running without CPU
    /// intervention between blocks.
    ///
    /// There is no `Future` to await here — a continuously-reloading channel
    /// never reaches a self-evident "done" — so register a
    /// [`GdmaIrqHandler`] via [`register_irq`](Self::register_irq) *before*
    /// calling this. The handler is the only way to observe block
    /// completions and to eventually call
    /// [`clean_auto_reload`](Self::clean_auto_reload) so the final block
    /// terminates the transfer instead of reloading again.
    ///
    /// `src`/`dst` and `block_len` follow the same alignment rules as
    /// [`mem_to_mem`](Self::mem_to_mem).
    pub fn mem_to_mem_reload_start(
        &mut self,
        src: *const u8,
        dst: *mut u8,
        block_len: usize,
        reload: ReloadConfig,
    ) {
        CHAN_STATE[CH as usize].done.store(false, Ordering::Release);
        setup_mem_to_mem_reload(CH, src, dst, block_len, reload);
    }

    /// Disable auto-reload (SDK: GDMA_ChCleanAutoReload) so the block
    /// currently in flight is the last one — its completion fires a
    /// `transfer_done` event instead of `block_done`. Call this from inside
    /// the registered handler, timed so it lands before the block you want
    /// to be final actually completes (mirrors the SDK multi-block example's
    /// `MaxMuliBlock == MuliBlockCunt + 2` check).
    pub fn clean_auto_reload(&mut self, reload: ReloadConfig) {
        let r = ch_ptr(CH);
        unsafe {
            let mut cfg_l = read_volatile(addr_of!((*r).cfg_l));
            if reload.reload_src {
                cfg_l &= !CFG_L_RELOAD_SRC;
            }
            if reload.reload_dst {
                cfg_l &= !CFG_L_RELOAD_DST;
            }
            write_volatile(addr_of_mut!((*r).cfg_l), cfg_l);
        }
    }

    /// Register a raw ISR callback for this channel's Block/Transfer/Error
    /// events. Runs directly in the channel's NVIC interrupt context — fires
    /// on *every* interrupt entry, not just final transfer completion, so it
    /// also covers per-block events during an auto-reload/multi-block
    /// transfer started via
    /// [`mem_to_mem_reload_start`](Self::mem_to_mem_reload_start). Coexists
    /// with the async `mem_to_mem`/`periph_tx`/`periph_rx` `Future` path —
    /// both are driven from the same ISR entry point.
    pub fn register_irq<H: GdmaIrqHandler + 'static>(&mut self, handler: &'static mut H) {
        let state = &CHAN_STATE[CH as usize];
        let trampoline: unsafe fn(usize, GdmaIrqEvent) = gdma_handler_trampoline::<H>;
        state
            .irq_handler
            .store(trampoline as usize, Ordering::Release);
        state
            .irq_data
            .store(handler as *mut H as usize, Ordering::Release);
    }

    /// Deregister the raw ISR callback. Does not touch NVIC/channel state —
    /// a concurrently in-flight async `Future` on this channel keeps working.
    pub fn unregister_irq(&mut self) {
        let state = &CHAN_STATE[CH as usize];
        state.irq_handler.store(0, Ordering::Release);
        state.irq_data.store(0, Ordering::Release);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DmaFuture
// ─────────────────────────────────────────────────────────────────────────────

pub struct DmaFuture<'a, const CH: u8> {
    _ch: PhantomData<&'a mut ()>,
}

impl<const CH: u8> Future for DmaFuture<'_, CH> {
    type Output = ();

    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let state = &CHAN_STATE[CH as usize];
        state.waker.register(cx.waker());
        if state.done.load(Ordering::Acquire) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl<const CH: u8> Drop for DmaFuture<'_, CH> {
    fn drop(&mut self) {
        if !CHAN_STATE[CH as usize].done.load(Ordering::Relaxed) {
            int_disable(CH);
            let r = ch_ptr(CH);
            // Suspend channel to stop new burst requests. SDK: GDMA_Abort → GDMA_Suspend
            unsafe {
                let cfg_l = read_volatile(addr_of!((*r).cfg_l));
                write_volatile(addr_of_mut!((*r).cfg_l), cfg_l | CFG_L_CH_SUSP);
            }
            // Wait for channel to become inactive or timeout. SDK: GDMA_Abort spin loop
            let mut timeout = 500u32;
            loop {
                let ch_en = gdma().chenreg_l_1().read().bits() & (1u32 << CH);
                let inactive = unsafe { read_volatile(addr_of!((*r).cfg_l)) } & CFG_L_INACTIVE;
                if ch_en == 0 || inactive != 0 || timeout == 0 {
                    break;
                }
                timeout -= 1;
                core::hint::spin_loop();
            }
            // Disable channel (WE=1, EN=0). SDK: GDMA_Cmd(DISABLE)
            gdma()
                .chenreg_l_1()
                .write(|w| unsafe { w.chenreg_l_1_ch_en_we().bits(1u8 << CH) });
            int_clear(CH);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Low-level transfer setup
// ─────────────────────────────────────────────────────────────────────────────

fn setup_mem_to_mem(ch: u8, src: *const u8, dst: *mut u8, len: usize, irq: bool) {
    gdma()
        .dmacfgreg_l()
        .write(|w| w.dmacfgreg_l_dma_en().set_bit()); // global DMA enable (idempotent)
    int_clear(ch);

    let r = ch_ptr(ch);
    let ctl_l = CTL_TT_FC_MEM_MEM
        | CTL_DST_TR_WIDTH_32
        | CTL_SRC_TR_WIDTH_32
        | CTL_DINC_INC
        | CTL_SINC_INC
        | CTL_DEST_MSIZE_8
        | CTL_SRC_MSIZE_8
        | if irq { CTL_INT_EN } else { 0 };
    unsafe {
        write_volatile(addr_of_mut!((*r).sar), src as u32);
        write_volatile(addr_of_mut!((*r).dar), dst as u32);
        write_volatile(addr_of_mut!((*r).llp), 0);
        write_volatile(addr_of_mut!((*r).ctl_l), ctl_l);
        write_volatile(addr_of_mut!((*r).ctl_h), (len / 4) as u32); // block size in 32-bit words
        write_volatile(addr_of_mut!((*r).cfg_l), CFG_HS_SEL_DST | CFG_HS_SEL_SRC);
        // CFG_H: set PROTCTL_NS (access NS SRAM from secure DMA), preserve other bits
        let cfg_h = read_volatile(addr_of!((*r).cfg_h));
        write_volatile(addr_of_mut!((*r).cfg_h), cfg_h | CFG_PROTCTL_NS);
    }

    if irq {
        int_enable(ch);
    }
    gdma().chenreg_l_1().write(|w| unsafe {
        w.chenreg_l_1_ch_en_we()
            .bits(1u8 << ch)
            .chenreg_l_1_ch_en()
            .bits(1u8 << ch)
    });
}

/// Set up an auto-reload mem-to-mem transfer: SAR/DAR reload at the end of
/// every block. Always IRQ-driven — there is no blocking variant, since
/// "done" is not well-defined for a continuously-reloading channel; the
/// caller observes progress entirely through a registered `GdmaIrqHandler`.
/// SDK: raw_gdma_multiblock example — GDMA_Init with GDMA_ReloadSrc/GDMA_ReloadDst set.
fn setup_mem_to_mem_reload(
    ch: u8,
    src: *const u8,
    dst: *mut u8,
    block_len: usize,
    reload: ReloadConfig,
) {
    gdma()
        .dmacfgreg_l()
        .write(|w| w.dmacfgreg_l_dma_en().set_bit()); // global DMA enable (idempotent)
    int_clear(ch);

    let r = ch_ptr(ch);
    let ctl_l = CTL_TT_FC_MEM_MEM
        | CTL_DST_TR_WIDTH_32
        | CTL_SRC_TR_WIDTH_32
        | CTL_DINC_INC
        | CTL_SINC_INC
        | CTL_DEST_MSIZE_8
        | CTL_SRC_MSIZE_8
        | CTL_INT_EN;
    let mut cfg_l = CFG_HS_SEL_DST | CFG_HS_SEL_SRC;
    if reload.reload_src {
        cfg_l |= CFG_L_RELOAD_SRC;
    }
    if reload.reload_dst {
        cfg_l |= CFG_L_RELOAD_DST;
    }
    unsafe {
        write_volatile(addr_of_mut!((*r).sar), src as u32);
        write_volatile(addr_of_mut!((*r).dar), dst as u32);
        write_volatile(addr_of_mut!((*r).llp), 0);
        write_volatile(addr_of_mut!((*r).ctl_l), ctl_l);
        write_volatile(addr_of_mut!((*r).ctl_h), (block_len / 4) as u32); // block size in 32-bit words
        write_volatile(addr_of_mut!((*r).cfg_l), cfg_l);
        // CFG_H: set PROTCTL_NS (access NS SRAM from secure DMA), preserve other bits
        let cfg_h = read_volatile(addr_of!((*r).cfg_h));
        write_volatile(addr_of_mut!((*r).cfg_h), cfg_h | CFG_PROTCTL_NS);
    }

    int_enable(ch);
    gdma().chenreg_l_1().write(|w| unsafe {
        w.chenreg_l_1_ch_en_we()
            .bits(1u8 << ch)
            .chenreg_l_1_ch_en()
            .bits(1u8 << ch)
    });
}

fn setup_periph_tx(
    ch: u8,
    src: *const u8,
    periph_dr: *mut u32,
    len: usize,
    p: TrParams,
    irq: bool,
) {
    gdma()
        .dmacfgreg_l()
        .write(|w| w.dmacfgreg_l_dma_en().set_bit());
    int_clear(ch);

    let r = ch_ptr(ch);
    // Source (memory) increments, destination (peripheral FIFO) fixed.
    // Width and msize set by caller per frame size and alignment. SDK: SSI_TXGDMA_Init
    let ctl_l = CTL_TT_FC_MEM_PER
        | ((p.dst_width as u32) << 1)
        | ((p.src_width as u32) << 4)
        | CTL_DINC_FIXED
        | CTL_SINC_INC
        | ((p.dst_msize as u32) << 11)
        | ((p.src_msize as u32) << 14)
        | if irq { CTL_INT_EN } else { 0 };
    let block_size = len / p.src_width.bytes(); // transfers = bytes / bytes_per_beat
    unsafe {
        write_volatile(addr_of_mut!((*r).sar), src as u32);
        write_volatile(addr_of_mut!((*r).dar), periph_dr as u32);
        write_volatile(addr_of_mut!((*r).llp), 0);
        write_volatile(addr_of_mut!((*r).ctl_l), ctl_l);
        write_volatile(addr_of_mut!((*r).ctl_h), block_size as u32);
        // HS ID > 15 requires extended bit. SDK: GDMA_Init → GDMA_BIT_CFGx_H_EXTENDED_DEST_PER1
        let cfg_h = ((p.hs_id as u32 & 0xF) << CFG_H_DEST_PER_SHF)
            | CFG_PROTCTL_NS
            | if p.hs_id >= 16 {
                CFG_H_EXTENDED_DEST_PER1
            } else {
                0
            };
        write_volatile(addr_of_mut!((*r).cfg_l), CFG_HS_SEL_SRC); // SW HS for source (memory); dest HW HS = 0 (implicit)
        write_volatile(addr_of_mut!((*r).cfg_h), cfg_h);
    }

    if irq {
        int_enable(ch);
    }
    gdma().chenreg_l_1().write(|w| unsafe {
        w.chenreg_l_1_ch_en_we()
            .bits(1u8 << ch)
            .chenreg_l_1_ch_en()
            .bits(1u8 << ch)
    });
}

fn setup_periph_rx(
    ch: u8,
    periph_dr: *const u32,
    dst: *mut u8,
    len: usize,
    p: TrParams,
    irq: bool,
) {
    gdma()
        .dmacfgreg_l()
        .write(|w| w.dmacfgreg_l_dma_en().set_bit());
    int_clear(ch);

    let r = ch_ptr(ch);
    // Source (peripheral FIFO) fixed, destination (memory) increments.
    // Width and msize set by caller per frame size and alignment. SDK: SSI_RXGDMA_Init
    let ctl_l = CTL_TT_FC_PER_MEM
        | ((p.dst_width as u32) << 1)
        | ((p.src_width as u32) << 4)
        | CTL_DINC_INC
        | CTL_SINC_FIXED
        | ((p.dst_msize as u32) << 11)
        | ((p.src_msize as u32) << 14)
        | if irq { CTL_INT_EN } else { 0 };
    let block_size = len / p.src_width.bytes(); // DW DMA BLOCK_TS counts src beats. SDK: SSI_RXGDMA_Init BlockSize=Length (SrcWidth=OneByte)
    unsafe {
        write_volatile(addr_of_mut!((*r).sar), periph_dr as u32);
        write_volatile(addr_of_mut!((*r).dar), dst as u32);
        write_volatile(addr_of_mut!((*r).llp), 0);
        write_volatile(addr_of_mut!((*r).ctl_l), ctl_l);
        write_volatile(addr_of_mut!((*r).ctl_h), block_size as u32);
        // HS ID > 15 requires extended bit. SDK: GDMA_Init → GDMA_BIT_CFGx_H_EXTENDED_SRC_PER1
        let cfg_h = ((p.hs_id as u32 & 0xF) << CFG_H_SRC_PER_SHF)
            | CFG_PROTCTL_NS
            | if p.hs_id >= 16 {
                CFG_H_EXTENDED_SRC_PER1
            } else {
                0
            };
        write_volatile(addr_of_mut!((*r).cfg_l), CFG_HS_SEL_DST); // SW HS for dest (memory); src HW HS = 0 (implicit)
        write_volatile(addr_of_mut!((*r).cfg_h), cfg_h);
    }

    if irq {
        int_enable(ch);
    }
    gdma().chenreg_l_1().write(|w| unsafe {
        w.chenreg_l_1_ch_en_we()
            .bits(1u8 << ch)
            .chenreg_l_1_ch_en()
            .bits(1u8 << ch)
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// ISR helpers
// ─────────────────────────────────────────────────────────────────────────────

fn handle_chan_irq(ch: u8) {
    let isr = peek_isr(ch);
    let event = GdmaIrqEvent {
        block_done: isr & ISR_BLOCK != 0,
        transfer_done: isr & ISR_TRANSFER != 0,
        error: isr & ISR_ERROR != 0,
        isr,
    };
    // Dispatch before clearing: on an auto-reload channel, hardware holds
    // the next block off until the pending interrupt is cleared, so this
    // ordering guarantees the callback's read of the DST buffer happens
    // before it can be overwritten again.
    dispatch_user_irq(ch, event);
    int_clear(ch);

    // Only a terminal event (whole transfer done, or an error) stops the
    // channel and wakes the async Future. A bare block_done mid auto-reload
    // transfer leaves the channel and its IRQ enabled — hardware resumes the
    // next block now that the interrupt has just been cleared above.
    if event.transfer_done || event.error {
        int_disable(ch);
        CHAN_STATE[ch as usize].done.store(true, Ordering::Release);
        CHAN_STATE[ch as usize].waker.wake();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Interrupt handlers (matched by name in ameba.x PROVIDE entries)
// ─────────────────────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
unsafe extern "C" fn GDMA0_CHANNEL0_IRQ() {
    handle_chan_irq(0);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GDMA0_CHANNEL1_IRQ() {
    handle_chan_irq(1);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GDMA0_CHANNEL2_IRQ() {
    handle_chan_irq(2);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GDMA0_CHANNEL3_IRQ() {
    handle_chan_irq(3);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GDMA0_CHANNEL4_IRQ() {
    handle_chan_irq(4);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GDMA0_CHANNEL5_IRQ() {
    handle_chan_irq(5);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GDMA0_CHANNEL6_IRQ() {
    handle_chan_irq(6);
}
#[unsafe(no_mangle)]
unsafe extern "C" fn GDMA0_CHANNEL7_IRQ() {
    handle_chan_irq(7);
}
