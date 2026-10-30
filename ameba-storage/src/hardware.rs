//! Real-hardware backend: calls RTL8721F ROM Flash functions via FFI.
//!
//! # Cross-core write protocol
//!
//! RTL8721F has two Cortex-M33 cores sharing one SPIC.  KM4NS (NP) runs
//! FreeRTOS and continuously XIP-fetches from 0x02000000.  Before AP can
//! enter SPIC user-mode (required by FLASH_TxData / FLASH_Erase), it must
//! pause NP's Flash access via IPC:
//!
//! 1. `IPC_SEMTake(IPC_SEM_FLASH)` — acquire hardware semaphore
//! 2. `ipc_flash_sync(LOCK)` — IPC msg to NP: pause Flash; spin until ACK
//! 3. `CPSID I` — disable AP interrupts (SPIC in user-mode, XIP invalid)
//! 4. FLASH_TxData / FLASH_Erase
//! 5. `CPSIE I`
//! 6. `ipc_flash_sync(UNLOCK)` — IPC msg to NP: resume Flash
//! 7. `IPC_SEMFree(IPC_SEM_FLASH)`
//!
//! Ported from `Flash_Write_Lock_IPC` / `FLASH_Write_Lock` in
//! `ameba_flash_ram.c` (CONFIG_ARM_CORE_CM4_KM4TZ branch), without the
//! FreeRTOS scheduler calls.

// ── Constants ─────────────────────────────────────────────────────────────────

// XIP memory-mapped base for SPIC auto-mode reads (KM4TZ window).
const SPI_FLASH_BASE: u32 = 0x0800_0000;

// Erase type codes (ameba_spic.h)
const ERASE_SECTOR: u32 = 2;
const ERASE_BLOCK: u32 = 1;

// JEDEC RDID command
const FLASH_CMD_RDID: u8 = 0x9F;

// IPC0 (IPCAP_DEV, 0x40815000): AP writes BIT(ch+16) to IPC_DATA to trigger NP RX.
const IPC0_DATA: u32 = 0x4081_5000;
// IPC_A2N_FLASHPG_REQ = ch 4: BIT(4+16) = 0x00100000
const IPC_A2N_FLASHPG_TRIG: u32 = 1 << (4 + 16);
// __ipc_memory_start__ = 0x20004400; slot 20 = base + 20*16
const IPC_MEMORY_BASE: u32 = 0x2000_4400;
const IPC_MSG_SIZE: usize = 16;
const IPC_FLASH_MSG_IDX: usize = 20;
const IPC_SEM_FLASH: u32 = 1;
const IPC_USER_POINT: u32 = 0; // msg field is a pointer

const WRITE_SYNC_CLEAR: u8 = 0; // NP writes 0 to ACK
const WRITE_SYNC_LOCK: u8 = 1; // AP sets: request NP pause Flash
const WRITE_SYNC_UNLOCK: u8 = 2; // AP sets: request NP resume Flash

// Re-trigger IPC at most this many times (~2M iters each) before panic.
const IPC_MAX_RETRIGGER: u32 = 3;

/// 32-byte-aligned sync flag shared with NP via IPC message.
/// Equivalent to SDK's `ALIGNMTO(CACHE_LINE_SIZE) static u8 Flash_Sync_Flag[CACHE_LINE_SIZE]`.
#[repr(C, align(32))]
struct FlashSyncFlag([u8; 32]);

static mut FLASH_SYNC_FLAG: FlashSyncFlag = FlashSyncFlag([0u8; 32]);

// ── ROM FFI ───────────────────────────────────────────────────────────────────

unsafe extern "C" {
    fn FLASH_TxData(start_addr: u32, data_phase_len: u32, p_data: *const u8);
    fn FLASH_Erase(erase_type: u32, address: u32);
    fn FLASH_RxCmd(cmd: u8, read_len: u32, read_data: *mut u8);
    fn RSIP_MMU_Cache_Clean();
    fn IPC_SEMTake(sem_idx: u32, timeout: u32) -> u32;
    fn IPC_SEMFree(sem_idx: u32);
}

// ── IPC flash sync ────────────────────────────────────────────────────────────

/// Send an IPC message to NP to pause (`WRITE_SYNC_LOCK`) or resume
/// (`WRITE_SYNC_UNLOCK`) Flash access, then spin until NP acknowledges.
///
/// NP's `FLASH_Write_IPC_Int` ISR reads the flag pointer from IPC slot 20,
/// writes `WRITE_SYNC_CLEAR` back, and DCache-cleans so AP sees the update.
#[unsafe(link_section = ".sram.only.text")]
unsafe fn ipc_flash_sync(sync_type: u8) {
    unsafe {
        let flag_ptr = core::ptr::addr_of_mut!(FLASH_SYNC_FLAG) as *mut u8;
        let flag_addr = flag_ptr as u32;
        let slot_addr = IPC_MEMORY_BASE + (IPC_FLASH_MSG_IDX * IPC_MSG_SIZE) as u32;
        let slot_ptr = slot_addr as *mut u32;

        let mut cp = cortex_m::Peripherals::steal();

        // Write flag and clean DCache so NP reads fresh SRAM.
        core::ptr::write_volatile(flag_ptr, sync_type);
        cortex_m::asm::dsb();
        cp.SCB.clean_dcache_by_address(flag_addr as usize, 32);
        cortex_m::asm::dsb();

        // Fill IPC slot 20: { msg_type=0, msg=&flag, msg_len=1, rsvd=0 }
        core::ptr::write_volatile(slot_ptr, IPC_USER_POINT);
        core::ptr::write_volatile(slot_ptr.add(1), flag_addr);
        core::ptr::write_volatile(slot_ptr.add(2), 1_u32);
        core::ptr::write_volatile(slot_ptr.add(3), 0_u32);
        cortex_m::asm::dsb();
        cp.SCB.clean_dcache_by_address(slot_addr as usize, 16);
        cortex_m::asm::dsb();

        // Trigger IPC_A2N_FLASHPG_REQ (ch 4) → NP ISR fires.
        core::ptr::write_volatile(IPC0_DATA as *mut u32, IPC_A2N_FLASHPG_TRIG);

        // Spin until NP ACKs; re-trigger if IRQ was lost.
        let mut iters: u32 = 0;
        let mut retriggers: u32 = 0;
        loop {
            cp.SCB.invalidate_dcache_by_address(flag_addr as usize, 32);
            cortex_m::asm::dsb();
            if core::ptr::read_volatile(flag_ptr) == WRITE_SYNC_CLEAR {
                break;
            }
            iters = iters.wrapping_add(1);
            if iters.is_multiple_of(2_000_000) {
                retriggers += 1;
                if retriggers >= IPC_MAX_RETRIGGER {
                    panic!(
                        "ipc_flash_sync: NP did not respond (sync_type={})",
                        sync_type
                    );
                }
                core::ptr::write_volatile(flag_ptr, sync_type);
                cortex_m::asm::dsb();
                cp.SCB.clean_dcache_by_address(flag_addr as usize, 32);
                cortex_m::asm::dsb();
                cp.SCB.clean_dcache_by_address(slot_addr as usize, 16);
                cortex_m::asm::dsb();
                core::ptr::write_volatile(IPC0_DATA as *mut u32, IPC_A2N_FLASHPG_TRIG);
            }
        }
    }
}

// ── Cross-core flash lock ──────────────────────────────────────────────────────

/// Run `f` while holding the Flash hardware semaphore, the IPC cross-core
/// lock, and with AP interrupts disabled.
///
/// `ipc_flash_sync(LOCK)` must precede `CPSID I`: the IPC poll loop requires
/// cache-maintenance interrupts to be live.  PRIMASK is saved/restored so
/// nested callers that already disabled interrupts remain disabled on return.
#[unsafe(link_section = ".sram.only.text")]
fn with_flash_lock<R>(f: impl FnOnce() -> R) -> R {
    unsafe {
        while IPC_SEMTake(IPC_SEM_FLASH, 0xFFFF_FFFF) == 0 {
            cortex_m::asm::nop();
        }
        ipc_flash_sync(WRITE_SYNC_LOCK);
    }

    let primask = cortex_m::register::primask::read();
    cortex_m::interrupt::disable();

    let result = f();

    if primask.is_active() {
        unsafe { cortex_m::interrupt::enable() };
    }

    unsafe {
        ipc_flash_sync(WRITE_SYNC_UNLOCK);
        IPC_SEMFree(IPC_SEM_FLASH);
    }
    result
}

// ── Cache flush after write ────────────────────────────────────────────────────

/// Invalidate SPIC auto-mode cache and AP DCache for the written/erased region.
///
/// Must be called while still inside `with_flash_lock` (SPIC in user-mode).
#[inline]
#[unsafe(link_section = ".sram.only.text")]
fn cache_flush_after_write_at(flash_off: u32, byte_len: u32) {
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    unsafe { RSIP_MMU_Cache_Clean() };
    let xip_start = (SPI_FLASH_BASE + flash_off) as usize;
    let rounded = (byte_len as usize + 31) & !31;
    let mut cp = unsafe { cortex_m::Peripherals::steal() };
    unsafe {
        cp.SCB
            .invalidate_dcache_by_address(xip_start, rounded.max(32))
    };
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}

// ── Public chip-specific API ───────────────────────────────────────────────────

/// Read `len` bytes from Flash offset `src_addr` via the XIP auto-mode window.
///
/// Uses volatile byte copies to avoid compiler optimizations that could race
/// with DCache invalidation in `cache_flush_after_write_at`.
#[unsafe(link_section = ".sram.only.text")]
pub(crate) fn spiflash_read(src_addr: u32, data: *mut u32, len: u32) -> i32 {
    let flash_ptr = (SPI_FLASH_BASE + src_addr) as *const u8;
    let dst_ptr = data as *mut u8;
    unsafe {
        for i in 0..len as usize {
            core::ptr::write(dst_ptr.add(i), core::ptr::read_volatile(flash_ptr.add(i)));
        }
    }
    0
}

/// Write `len` bytes from `data` to Flash offset `dest_addr`.
///
/// Splits at 256-byte page boundaries: ROM's `FLASH_TxData` requires
/// `(start_addr & 0xFF) + len <= 256`.
#[unsafe(link_section = ".sram.only.text")]
pub(crate) fn spiflash_write(dest_addr: u32, data: *const u32, len: u32) -> i32 {
    with_flash_lock(|| unsafe {
        let mut offset: u32 = 0;
        while offset < len {
            let page_remaining = 256 - ((dest_addr + offset) & 0xFF);
            let chunk = (len - offset).min(page_remaining);
            FLASH_TxData(
                dest_addr + offset,
                chunk,
                (data as *const u8).add(offset as usize),
            );
            offset += chunk;
        }
        cache_flush_after_write_at(dest_addr, len);
    });
    0
}

/// Erase the 4 KiB sector at `sector_index * 4096`.
#[unsafe(link_section = ".sram.only.text")]
pub(crate) fn spiflash_erase_sector(sector_index: u32) -> i32 {
    let addr = sector_index * 4096;
    with_flash_lock(|| unsafe {
        FLASH_Erase(ERASE_SECTOR, addr);
        cache_flush_after_write_at(addr, 4096);
    });
    0
}

/// Erase the 64 KiB block at `block_index * 65536`.
#[unsafe(link_section = ".sram.only.text")]
pub(crate) fn spiflash_erase_block(block_index: u32) -> i32 {
    let addr = block_index * 65536;
    with_flash_lock(|| unsafe {
        FLASH_Erase(ERASE_BLOCK, addr);
        cache_flush_after_write_at(addr, 65536);
    });
    0
}

/// Detect Flash capacity via JEDEC RDID (0x9F), capacity byte = id_buf[2].
#[unsafe(link_section = ".sram.only.text")]
pub(crate) fn get_flash_size() -> usize {
    let mut id_buf = [0u8; 3];
    with_flash_lock(|| unsafe {
        FLASH_RxCmd(FLASH_CMD_RDID, 3, id_buf.as_mut_ptr());
    });
    // Standard JEDEC capacity encoding.
    match id_buf[2] {
        0x12 => 256 << 10,
        0x13 => 512 << 10,
        0x14 => 1 << 20,
        0x15 => 2 << 20,
        0x16 => 4 << 20,
        0x17 => 8 << 20,
        0x18 => 16 << 20,
        0x19 => 32 << 20,
        0x1A => 64 << 20,
        0x1B => 128 << 20,
        _ => 4 << 20, // conservative default
    }
}
