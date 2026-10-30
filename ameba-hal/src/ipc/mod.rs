use core::sync::atomic::{AtomicUsize, Ordering};

use rtl8721f_pac::{Ipc0Reg, Ipc1Reg, ipc0_reg};

// ────────────────────────────────────────────────────────────────────────────
// Constants
// ────────────────────────────────────────────────────────────────────────────

const IPC_CHANNEL_NUM: usize = 32;

// Shared SRAM: IpcMsg[0..15] = N2A slots, IpcMsg[16..31] = A2N slots.
// SDK: ameba_layout.ld : KM4NS_IPC_RAM
const SHARED_BASE: usize = 0x2000_4400;
const TX_SLOT_BASE: usize = 16;

// ── N2A channel IDs (NP sends → AP receives) ─────────────────────────────────
pub mod n2a {
    pub const TICKLESS: u8 = 0;
    pub const WAKE_AP: u8 = 1;
    pub const WIFI_FW_INFO: u8 = 2;
    pub const FLASHPG_REQ: u8 = 3;
    pub const LOGUART_RX: u8 = 4;
    pub const COEX_API: u8 = 5;
    pub const WIFI_TRX: u8 = 6;
    pub const WIFI_API: u8 = 7;
}

// ── A2N channel IDs (AP sends → NP receives) ─────────────────────────────────
pub mod a2n {
    pub const TICKLESS: u8 = 0;
    pub const WIFI_FW_INFO: u8 = 1;
    pub const WAKE_AP: u8 = 2;
    pub const LOGUART_RX: u8 = 3;
    pub const FLASHPG_REQ: u8 = 4;
    pub const COEX_API: u8 = 5;
    pub const WIFI_TRX: u8 = 6;
    pub const WIFI_API: u8 = 7;
    pub const DIAGNOSE: u8 = 8;
}

// ────────────────────────────────────────────────────────────────────────────
// IPC message (mirrors C IPC_MSG_STRUCT)
// ────────────────────────────────────────────────────────────────────────────

pub const MSG_TYPE_POINT: u32 = 0; // msg field is a pointer
pub const MSG_TYPE_DATA: u32 = 1; // msg field is inline data

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IpcMsg {
    pub msg_type: u32,
    pub msg: u32,
    pub msg_len: u32,
    pub rsvd: u32,
}

// ────────────────────────────────────────────────────────────────────────────
// IRQ handler table (32 slots per IPC instance)
// ────────────────────────────────────────────────────────────────────────────

struct IrqSlot {
    handler: AtomicUsize,
    data: AtomicUsize,
}

unsafe impl Sync for IrqSlot {}

impl IrqSlot {
    const fn empty() -> Self {
        IrqSlot {
            handler: AtomicUsize::new(0),
            data: AtomicUsize::new(0),
        }
    }
}

macro_rules! empty_slots {
    () => {
        [
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
            IrqSlot::empty(),
        ]
    };
}

static AP_IRQ_TABLE: [IrqSlot; IPC_CHANNEL_NUM] = empty_slots!();
static NP_IRQ_TABLE: [IrqSlot; IPC_CHANNEL_NUM] = empty_slots!();

// ────────────────────────────────────────────────────────────────────────────
// IPC direction
// ────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpcDir {
    NpToAp = 0,
    ApToNp = 1,
}

// ────────────────────────────────────────────────────────────────────────────
// Ipc driver struct
// ────────────────────────────────────────────────────────────────────────────

pub struct Ipc {
    base: usize,
    irq_table: &'static [IrqSlot; IPC_CHANNEL_NUM],
}

impl Ipc {
    /// # Safety
    /// Only one [`Ipc`] instance per base may exist at a time.
    pub unsafe fn ap() -> Self {
        Ipc {
            base: Ipc0Reg::PTR as usize,
            irq_table: &AP_IRQ_TABLE,
        }
    }

    /// # Safety
    /// Only one [`Ipc`] instance per base may exist at a time.
    pub unsafe fn np() -> Self {
        Ipc {
            base: Ipc1Reg::PTR as usize,
            irq_table: &NP_IRQ_TABLE,
        }
    }

    fn regs(&self) -> &'static ipc0_reg::RegisterBlock {
        unsafe { &*(self.base as *const ipc0_reg::RegisterBlock) }
    }

    // ── IMR ──────────────────────────────────────────────────────────────────

    pub fn set_channel_interrupt(&self, channel: u8, enabled: bool) {
        let cur = self.get_mask();
        self.set_mask(if enabled {
            cur | (1u32 << channel)
        } else {
            cur & !(1u32 << channel)
        });
    }

    pub fn set_mask(&self, mask: u32) {
        unsafe {
            self.regs().ipc_imr().as_ptr().write_volatile(mask);
        }
    }

    pub fn get_mask(&self) -> u32 {
        let r = self.regs().ipc_imr().read();
        ((r.imr_tx0_empty_mask().bits() as u32) << 16) | (r.imr_rx0_full_mask().bits() as u32)
    }

    // ── ISR ──────────────────────────────────────────────────────────────────

    pub fn get_status(&self) -> u32 {
        self.regs().ipc_isr().read().bits()
    }

    pub fn clear_interrupt(&self, channel: u8) {
        unsafe {
            self.regs()
                .ipc_isr()
                .as_ptr()
                .write_volatile(1u32 << channel);
        }
    }

    // ── TX trigger ───────────────────────────────────────────────────────────

    /// Post a TX interrupt on channel `ch` (0–15). Returns false if already pending.
    pub fn request(&self, ch: u8) -> bool {
        debug_assert!(ch < 16, "TX channel 0-15");
        let bit = 1u16 << ch;
        if self.regs().ipc_data().read().tx0_data().bits() & bit != 0 {
            return false;
        }
        self.regs()
            .ipc_data()
            .modify(|_, w| unsafe { w.tx0_data().bits(bit) });
        true
    }

    // ── Handler registration ─────────────────────────────────────────────────

    /// Register a per-channel handler.
    ///
    /// Signature: `unsafe fn(data: *mut (), ipc_status: u32, channel: u8)`
    ///
    /// For RX channels (0–15) the IMR bit is automatically enabled.
    ///
    /// # Safety
    /// `handler` and `data` must remain valid for the lifetime of this `Ipc` instance.
    pub unsafe fn register_handler(
        &self,
        channel: u8,
        handler: unsafe fn(*mut (), u32, u8),
        data: *mut (),
    ) {
        let slot = &self.irq_table[channel as usize];
        slot.data.store(data as usize, Ordering::Release);
        slot.handler.store(handler as usize, Ordering::Release);
        if channel < 16 {
            self.set_channel_interrupt(channel, true);
        }
    }

    // ── Dispatch ─────────────────────────────────────────────────────────────

    /// Call from the IPC IRQ handler to dispatch pending channels.
    pub fn dispatch(&self) {
        let pending = self.get_status() & self.get_mask();
        for i in 0u8..32 {
            if pending & (1 << i) != 0 {
                let slot = &self.irq_table[i as usize];
                let handler_ptr = slot.handler.load(Ordering::Acquire);
                let data_ptr = slot.data.load(Ordering::Acquire);
                if handler_ptr != 0 {
                    unsafe {
                        let f: unsafe fn(*mut (), u32, u8) = core::mem::transmute(handler_ptr);
                        f(data_ptr as *mut (), pending, i);
                    }
                }
                self.clear_interrupt(i);
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Shared SRAM message access
// ────────────────────────────────────────────────────────────────────────────

/// Read the IPC message for an N2A RX channel from shared SRAM.
/// Call from inside the registered RX handler.
pub fn get_message(channel: u8) -> IpcMsg {
    debug_assert!(channel < 16);
    let ptr = (SHARED_BASE + channel as usize * core::mem::size_of::<IpcMsg>()) as *const IpcMsg;
    unsafe {
        let mut cp = cortex_m::peripheral::Peripherals::steal();
        cp.SCB
            .invalidate_dcache_by_address(ptr as usize, core::mem::size_of::<IpcMsg>());
        core::ptr::read_volatile(ptr)
    }
}

/// Write an IPC message to an A2N TX channel slot in shared SRAM and trigger the interrupt.
/// Returns false if the TX channel is still pending.
pub fn send(channel: u8, msg: &IpcMsg) -> bool {
    debug_assert!(channel < 16);
    let ipc = unsafe { Ipc::ap() };
    // C SDK checks IPC_DATA.tx0_data (not ISR) to detect a pending TX.
    // tx0_data bit stays 1 until NP clears its RX ISR; ISR[16+ch] (TX empty)
    // is set only AFTER the pending TX clears — using ISR here was wrong.
    if ipc.regs().ipc_data().read().tx0_data().bits() & (1u16 << channel) != 0 {
        return false;
    }
    let tx_bit = 1u32 << (16 + channel as u32);
    let slot = TX_SLOT_BASE + channel as usize;
    let ptr = (SHARED_BASE + slot * core::mem::size_of::<IpcMsg>()) as *mut IpcMsg;
    unsafe {
        core::ptr::write_volatile(ptr, *msg);
        let mut cp = cortex_m::peripheral::Peripherals::steal();
        cp.SCB
            .clean_dcache_by_address(ptr as usize, core::mem::size_of::<IpcMsg>());
        ipc.regs().ipc_data().as_ptr().write_volatile(tx_bit);
    }
    true
}

// ────────────────────────────────────────────────────────────────────────────
// Init + IRQ handler
// ────────────────────────────────────────────────────────────────────────────

/// Enable IPC_CPU0_IRQ in NVIC so inter-core messages from km4ns are received.
/// Call once before registering any IPC handlers.
/// SDK: ameba_vector_table.h : IPC_CPU0_IRQ
pub fn init() {
    unsafe {
        cortex_m::peripheral::NVIC::unmask(rtl8721f_pac::Interrupt::IPC_CPU0_IRQ);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn IPC_CPU0_IRQ() {
    unsafe {
        Ipc::ap().dispatch();
    }
}
