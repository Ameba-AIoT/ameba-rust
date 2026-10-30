//! WHC data plane — IPC channel 6 (TRX).
//!
//! AP→NP (ch6 A2N): TX packets.  NP→AP (ch6 N2A): RX packets.
//!
//! ISR pattern (in-place ACK):
//!   1. Read IpcMsg.msg → pointer to NP's `g_whc_ipc_ex_msg`.
//!   2. DCache_Invalidate that pointer.
//!   3. Enqueue (event_num, msg_addr, wlan_idx) to RX_CHAN.
//!   4. Write MSG_READ_DONE into `p_ipc_msg->event_num`.
//!   5. DCache_Clean.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU8, AtomicU32, AtomicUsize, Ordering};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::waitqueue::AtomicWaker;

use ameba_hal::ipc::{self, IpcMsg, MSG_TYPE_POINT, a2n};
use embedded_hal::delay::DelayNs;

use super::msg::*;
use crate::cache;

// Bounded: this ISR runs synchronously inside `IPC_CPU0_IRQ` (see
// `ameba_hal::ipc::Ipc::dispatch`) with no mutual exclusion against
// task-context LOGUART writers (e.g. `[ota-dbg]` in `ameba-ota`). If a
// task holds the TX FIFO in a state this ISR's poll never resolves, an
// unbounded spin here would stall every interrupt at this priority
// (Timer4 included) with no output at all.
macro_rules! dbg {
    ($s:literal) => {
        ameba_hal::print_bounded!($s)
    };
    ($fmt:literal, $($arg:expr_2021),+ $(,)?) => {
        ameba_hal::print_bounded!($fmt, $($arg),+)
    };
}

// ── Sync wrapper for UnsafeCell (single-core, no threads) ────────────────────

struct SyncUnsafeCell<T>(UnsafeCell<T>);
unsafe impl<T> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    const fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }
    fn get(&self) -> *mut T {
        self.0.get()
    }
}

// ── Static AP TRX ex-message (AP's own buffer used for A2N sends) ────────────

static AP_TRX_EX: SyncUnsafeCell<WhcIpcExMsg> = SyncUnsafeCell::new(WhcIpcExMsg::zeroed());
// Initialized to event_num=MSG_READ_DONE(0) → first send is immediately ready.

// ── Static SKB pool ───────────────────────────────────────────────────────────

static mut SKB_POOL: [SkBuff; N_SKB_AP] = [
    SkBuff {
        list: [0; 2],
        buf: 0,
        head: 0,
        data: 0,
        tail: 0,
        end: 0,
        dev: 0,
        len: 0,
        dyalloc_flag: 0,
        busy: 0,
        no_free: 0,
        tx_raw: [0; 13],
        _pad1: 0,
        ref_count: 0,
    },
    SkBuff {
        list: [0; 2],
        buf: 0,
        head: 0,
        data: 0,
        tail: 0,
        end: 0,
        dev: 0,
        len: 0,
        dyalloc_flag: 0,
        busy: 0,
        no_free: 0,
        tx_raw: [0; 13],
        _pad1: 0,
        ref_count: 0,
    },
    SkBuff {
        list: [0; 2],
        buf: 0,
        head: 0,
        data: 0,
        tail: 0,
        end: 0,
        dev: 0,
        len: 0,
        dyalloc_flag: 0,
        busy: 0,
        no_free: 0,
        tx_raw: [0; 13],
        _pad1: 0,
        ref_count: 0,
    },
    SkBuff {
        list: [0; 2],
        buf: 0,
        head: 0,
        data: 0,
        tail: 0,
        end: 0,
        dev: 0,
        len: 0,
        dyalloc_flag: 0,
        busy: 0,
        no_free: 0,
        tx_raw: [0; 13],
        _pad1: 0,
        ref_count: 0,
    },
];
static mut SKB_DATA: [[u8; SKB_BUF_SIZE]; N_SKB_AP] = [[0u8; SKB_BUF_SIZE]; N_SKB_AP];

/// Round-robin hint for SKB allocation.
static SKB_NEXT: AtomicUsize = AtomicUsize::new(0);

/// Track which AP-side TX SKB is pending NP ACK via CMD_XIMT_PKTS.
/// N_SKB_AP as u8 acts as sentinel value "none".
static PENDING_TX_IDX: AtomicU8 = AtomicU8::new(N_SKB_AP as u8);

/// NP SKB address awaiting MSG_RECV_DONE when the A2N TRX channel was busy.
/// 0 = none pending.
static PENDING_RECV_DONE: AtomicU32 = AtomicU32::new(0);

/// Return the `buf` field (base address of data buffer) for SKB `idx`.
/// Used by the net module to obtain the write pointer for TX.
#[inline]
pub fn skb_buf_ptr(idx: usize) -> u32 {
    unsafe { SKB_POOL[idx].buf }
}

// ── RX event queue (ISR → rx task) ───────────────────────────────────────────

/// An event received from NP on ch6.
#[derive(Clone, Copy)]
pub struct RxEvent {
    pub event_num: u32,
    pub msg_addr: u32, // pointer into NP SRAM (sk_buff address when EVT_RECV_PKTS)
    pub wlan_idx: u8,
}

/// Depth-8 ring buffer; ISR enqueues, WhcDevice::receive() dequeues.
pub static RX_CHAN: Channel<CriticalSectionRawMutex, RxEvent, 8> = Channel::new();

/// Woken whenever a TX SKB slot is freed (see `send_trx_msg_sync`'s ACK path),
/// so `embassy_net_driver::Driver::transmit()` can retry `alloc_skb()`.
pub static TX_WAKER: AtomicWaker = AtomicWaker::new();

// ── IPC ch6 N2A ISR handler ───────────────────────────────────────────────────

/// Registered with `ameba_hal::ipc::Ipc::register_handler(n2a::WIFI_TRX, ...)`.
///
/// # Safety
/// Called from IRQ context via Rust ABI dispatch; must complete quickly.
pub unsafe fn trx_isr(_data: *mut (), _ipc_status: u32, _chan: u8) {
    unsafe {
        let ipc_msg = ipc::get_message(a2n::WIFI_TRX); // reads ch6 N2A slot
        let p_ex = ipc_msg.msg as *mut WhcIpcExMsg;

        cache::invalidate(p_ex as usize, core::mem::size_of::<WhcIpcExMsg>());

        let event_num = core::ptr::read_volatile(&(*p_ex).event_num);
        let msg_addr = core::ptr::read_volatile(&(*p_ex).msg_addr);
        let wlan_idx = core::ptr::read_volatile(&(*p_ex).wlan_idx) as u8;

        let evt = RxEvent {
            event_num,
            msg_addr,
            wlan_idx,
        };

        // Non-blocking enqueue — if queue is full, drop the event (flow control).
        let status = if RX_CHAN.try_send(evt).is_ok() {
            0u32
        } else {
            MSG_MEMORY_NOT_ENOUGH
        };

        // In-place ACK: write MSG_READ_DONE into NP's ex_msg so NP can send again.
        core::ptr::write_volatile(&mut (*p_ex).msg_queue_status, status);
        core::ptr::write_volatile(&mut (*p_ex).event_num, MSG_READ_DONE);
        cache::clean(p_ex as usize, core::mem::size_of::<WhcIpcExMsg>());
    }
}

// ── Register IPC handler ──────────────────────────────────────────────────────

/// Register the ch6 N2A ISR. Call once during init.
pub fn register_trx_handler() {
    // Flush BSS-zeroed AP_TRX_EX to SRAM for the same reason as G_HOST_API_REQ:
    // cache::invalidate in send_trx_msg_sync would otherwise read SRAM garbage.
    unsafe {
        cache::clean(AP_TRX_EX.get() as usize, 64);
        ipc::Ipc::ap().register_handler(a2n::WIFI_TRX, trx_isr, core::ptr::null_mut());
    }
    // Bind the SKB pool data buffers.
    init_skb_pool();
    dbg!("[trx] handler registered ch6\n");
}

fn init_skb_pool() {
    unsafe {
        for i in 0..N_SKB_AP {
            let data_ptr = core::ptr::addr_of!(SKB_DATA[i]) as u32;
            SKB_POOL[i].buf = data_ptr;
        }
        // Clean so NP sees the initialised values.
        cache::clean(
            core::ptr::addr_of!(SKB_POOL) as usize,
            N_SKB_AP * core::mem::size_of::<SkBuff>(),
        );
        cache::clean(
            core::ptr::addr_of!(SKB_DATA) as usize,
            N_SKB_AP * SKB_BUF_SIZE,
        );
    }
}

// ── AP-side TX ───────────────────────────────────────────────────────────────

/// Find a free SKB slot (round-robin). Returns index or None.
pub fn alloc_skb() -> Option<usize> {
    let start = SKB_NEXT.load(Ordering::Relaxed);
    for offset in 0..N_SKB_AP {
        let i = (start + offset) % N_SKB_AP;
        // Invalidate the skb cache line before reading `busy`.
        unsafe {
            cache::invalidate(core::ptr::addr_of!(SKB_POOL[i]) as usize, 64);
        }
        if unsafe { core::ptr::read_volatile(&SKB_POOL[i].busy) } == 0 {
            SKB_NEXT.store((i + 1) % N_SKB_AP, Ordering::Relaxed);
            return Some(i);
        }
    }
    None
}

/// Commit a pre-filled SKB slot and send it to NP.
///
/// Called by the smoltcp `TxToken::consume` after writing the frame data.
/// `skb_idx` must be a slot returned by `alloc_skb()`.
pub fn send_pkt_from_skb(skb_idx: usize, frame_len: usize, wlan_idx: u8) {
    if frame_len > SKB_DATA_MAX {
        return;
    }
    unsafe {
        let skb = &mut SKB_POOL[skb_idx];
        let data_start = skb.buf + SKB_TX_HEADROOM as u32;

        skb.head = skb.buf;
        skb.data = data_start;
        skb.tail = data_start + frame_len as u32;
        skb.end = skb.buf + SKB_BUF_SIZE as u32;
        skb.len = frame_len as u32;
        skb.busy = 1;
        skb.no_free = 1;
        skb.ref_count = 1;

        cache::clean(skb.head as usize, SKB_BUF_SIZE);
        cache::clean(skb as *const _ as usize, core::mem::size_of::<SkBuff>());

        let skb_addr = skb as *const SkBuff as u32;
        send_trx_msg_sync(CMD_XIMT_PKTS, skb_addr, wlan_idx);
    }
}

/// Send an Ethernet frame to NP on wlan interface `wlan_idx`.
///
/// Finds a free SKB, fills it with `frame`, then triggers the A2N IPC.
/// Returns false if no SKB is available or the IPC TX channel is busy.
pub fn send_pkt(frame: &[u8], wlan_idx: u8) -> bool {
    if frame.len() > SKB_DATA_MAX {
        return false;
    }

    let i = match alloc_skb() {
        Some(i) => i,
        None => return false,
    };

    unsafe {
        let skb = &mut SKB_POOL[i];
        let buf_base = skb.buf;
        let data_start = buf_base + SKB_TX_HEADROOM as u32;
        let data_end = data_start + frame.len() as u32;
        let buf_end = buf_base + SKB_BUF_SIZE as u32;

        // Copy frame into SKB data region.
        core::ptr::copy_nonoverlapping(frame.as_ptr(), data_start as *mut u8, frame.len());

        // Initialise sk_buff fields.
        skb.head = buf_base;
        skb.data = data_start;
        skb.tail = data_end;
        skb.end = buf_end;
        skb.len = frame.len() as u32;
        skb.busy = 1;
        skb.no_free = 1;
        skb.ref_count = 1;

        // Flush skb and its data to SRAM.
        cache::clean(skb.head as usize, SKB_BUF_SIZE);
        cache::clean(skb as *const _ as usize, core::mem::size_of::<SkBuff>());

        let skb_addr = skb as *const SkBuff as u32;
        send_trx_msg_sync(CMD_XIMT_PKTS, skb_addr, wlan_idx)
    }
}

/// Synchronous TRX send (mirrors `whc_ipc_send_msg`).
///
/// Polls until NP has ACK'd the previous A2N send, then fills `AP_TRX_EX`
/// and triggers the IPC interrupt.  Returns false if IPC TX channel is busy.
///
/// Also frees the AP-side TX SKB that was sent in the previous A2N message:
/// once NP writes MSG_READ_DONE, it has finished reading the previous
/// AP_TRX_EX content, so the SKB pointer it received (if any) is no longer
/// needed and the SKB can be recycled.
pub fn send_trx_msg_sync(event_num: u32, msg_addr: u32, wlan_idx: u8) -> bool {
    unsafe {
        // Wait for NP to ACK the previous send (writes MSG_READ_DONE back to us).
        // Limit to 1000 x 2 us = 2 ms in case something is stuck.
        let mut acked = false;
        for _ in 0..1000u32 {
            cache::invalidate(AP_TRX_EX.get() as usize, 64);
            if core::ptr::read_volatile(&(*AP_TRX_EX.get()).event_num) == MSG_READ_DONE {
                acked = true;
                break;
            }
            ameba_hal::delay::Delay::new().delay_us(2);
        }
        if !acked {
            dbg!("[trx] send_trx_msg_sync TIMEOUT evt={}\n", event_num);
        }

        // NP has ACK'd the previous A2N message: free the previously tracked TX
        // SKB (if any).  Only free on acked=true so NP cannot read freed memory.
        if acked {
            let prev = PENDING_TX_IDX.swap(N_SKB_AP as u8, Ordering::Relaxed);
            if prev < N_SKB_AP as u8 {
                cache::invalidate(core::ptr::addr_of!(SKB_POOL[prev as usize]) as usize, 64);
                core::ptr::write_volatile(&mut SKB_POOL[prev as usize].busy, 0u8);
                TX_WAKER.wake();
            }
        }

        // Flush any pending MSG_RECV_DONE before handling the caller's message.
        flush_pending_recv_done();

        // Fill the AP-side ex_msg and clean to SRAM.
        let msg = &mut *AP_TRX_EX.get();
        msg.event_num = event_num;
        msg.msg_addr = msg_addr;
        msg.wlan_idx = wlan_idx as u32;
        cache::clean(AP_TRX_EX.get() as usize, 64);

        // Track this SKB for future freeing if we just sent a TX command.
        if event_num == CMD_XIMT_PKTS {
            let base = core::ptr::addr_of!(SKB_POOL[0]) as u32;
            let sz = core::mem::size_of::<SkBuff>() as u32;
            if msg_addr >= base {
                let off = msg_addr - base;
                if off.is_multiple_of(sz) {
                    let idx = (off / sz) as u8;
                    if idx < N_SKB_AP as u8 {
                        PENDING_TX_IDX.store(idx, Ordering::Relaxed);
                    }
                }
            }
        }

        // Trigger A2N ch6 interrupt.  ipc::send() returns false when the A2N
        // hardware bit is still set — NP may write MSG_READ_DONE to shared SRAM
        // before clearing the bit, creating a narrow race.  Retry until the bit
        // clears (~2 us is normally enough).
        let ipc_msg = IpcMsg {
            msg_type: MSG_TYPE_POINT,
            msg: AP_TRX_EX.get() as u32,
            msg_len: 0,
            rsvd: 0,
        };
        for _ in 0..100u32 {
            if ipc::send(a2n::WIFI_TRX, &ipc_msg) {
                return true;
            }
            ameba_hal::delay::Delay::new().delay_us(2);
        }
        dbg!("[trx] ipc::send stuck evt={}\n", event_num);
        false
    }
}

/// Flush a pending MSG_RECV_DONE if the A2N channel allows it.
/// Returns true if the pending message was sent (or there was none).
/// Called by send_trx_msg_sync after MSG_READ_DONE is received.
unsafe fn flush_pending_recv_done() -> bool {
    unsafe {
        let addr = PENDING_RECV_DONE.load(Ordering::Relaxed);
        if addr == 0 {
            return true;
        }
        // Check whether NP has ACK'd the previous A2N message.
        cache::invalidate(AP_TRX_EX.get() as usize, 64);
        if core::ptr::read_volatile(&(*AP_TRX_EX.get()).event_num) != MSG_READ_DONE {
            return false; // channel still busy, retry later
        }
        // Channel is free — send the pending MSG_RECV_DONE now.
        PENDING_RECV_DONE.store(0, Ordering::Relaxed);
        let msg = &mut *AP_TRX_EX.get();
        msg.event_num = MSG_RECV_DONE;
        msg.msg_addr = addr;
        msg.wlan_idx = 0;
        cache::clean(AP_TRX_EX.get() as usize, 64);

        let ipc_msg = IpcMsg {
            msg_type: MSG_TYPE_POINT,
            msg: AP_TRX_EX.get() as u32,
            msg_len: 0,
            rsvd: 0,
        };
        ipc::send(a2n::WIFI_TRX, &ipc_msg);
        true
    }
}

/// Try to send MSG_RECV_DONE non-blockingly, or queue it for later.
/// Called after copying packet data from `skb_addr`.
pub fn recv_done(skb_addr: u32) {
    unsafe {
        cache::invalidate(AP_TRX_EX.get() as usize, 64);
        if core::ptr::read_volatile(&(*AP_TRX_EX.get()).event_num) == MSG_READ_DONE {
            // Channel is free — send directly.
            let msg = &mut *AP_TRX_EX.get();
            msg.event_num = MSG_RECV_DONE;
            msg.msg_addr = skb_addr;
            msg.wlan_idx = 0;
            cache::clean(AP_TRX_EX.get() as usize, 64);

            let ipc_msg = IpcMsg {
                msg_type: MSG_TYPE_POINT,
                msg: AP_TRX_EX.get() as u32,
                msg_len: 0,
                rsvd: 0,
            };
            let _ = ipc::send(a2n::WIFI_TRX, &ipc_msg);
        } else {
            // Channel busy — queue for later; overwrite previous if any.
            PENDING_RECV_DONE.store(skb_addr, Ordering::Relaxed);
        }
    }
}
