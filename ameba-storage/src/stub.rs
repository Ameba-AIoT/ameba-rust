//! Emulation backend: in-memory Flash simulation for host/unit tests.
//!
//! Enabled with `--features emulation`.  Faithfully models the key constraint
//! of NOR Flash: **write can only clear bits (0xFF → lower value), never set
//! them** (0 cannot become 1 without an erase).
//!
//! `ERASE_SECTOR` and `ERASE_BLOCK` fill the target range with 0xFF.
//! `spiflash_write` applies `dst &= src` (bitwise AND), matching real hardware.
//!
//! # Thread safety in tests
//!
//! In `#[cfg(test)]` builds the backing store is `thread_local!`, giving each
//! test thread its own independent Flash image.  The Rust test runner assigns
//! each test to exactly one thread (they are serialised within a thread), so
//! `fresh()` → write → read sequences in one test cannot be interrupted by
//! another test's `fresh()` reset — even when the test suite runs in parallel.

const FLASH_SIZE: usize = 4 * 1024 * 1024; // 4 MiB emulated Flash
const SECTOR: usize = 4096;
const BLOCK: usize = 65536;

// SAFETY: emulation backend runs on single-threaded bare-metal only.
#[cfg(not(test))]
static mut FLASH_DATA: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];

// ── chip_specific API ────────────────────────────────────────────────────────

pub(crate) fn spiflash_read(src_addr: u32, data: *mut u32, len: u32) -> i32 {
    let src = src_addr as usize;
    let n = len as usize;
    if src + n > FLASH_SIZE {
        return -1;
    }

    #[cfg(test)]
    state::with(|flash| unsafe {
        core::ptr::copy_nonoverlapping(flash[src..].as_ptr(), data as *mut u8, n);
    });

    #[cfg(not(test))]
    unsafe {
        let flash = &*core::ptr::addr_of!(FLASH_DATA);
        core::ptr::copy_nonoverlapping(flash[src..].as_ptr(), data as *mut u8, n);
    }
    0
}

pub(crate) fn spiflash_write(dest_addr: u32, data: *const u32, len: u32) -> i32 {
    let dst = dest_addr as usize;
    let n = len as usize;
    if dst + n > FLASH_SIZE {
        return -1;
    }
    let src_bytes = unsafe { core::slice::from_raw_parts(data as *const u8, n) };

    #[cfg(test)]
    state::with(|flash| {
        for (d, s) in flash[dst..dst + n].iter_mut().zip(src_bytes) {
            *d &= *s; // NOR Flash: write can only clear bits
        }
    });

    #[cfg(not(test))]
    unsafe {
        let flash = &mut *core::ptr::addr_of_mut!(FLASH_DATA);
        for (d, s) in flash[dst..dst + n].iter_mut().zip(src_bytes) {
            *d &= *s;
        }
    }
    0
}

pub(crate) fn spiflash_erase_sector(sector_index: u32) -> i32 {
    let start = sector_index as usize * SECTOR;
    if start + SECTOR > FLASH_SIZE {
        return -1;
    }

    #[cfg(test)]
    state::with(|flash| flash[start..start + SECTOR].fill(0xFF));

    #[cfg(not(test))]
    // SAFETY: single-threaded embedded context; no concurrent access.
    unsafe {
        (&mut *core::ptr::addr_of_mut!(FLASH_DATA))[start..start + SECTOR].fill(0xFF);
    }
    0
}

pub(crate) fn spiflash_erase_block(block_index: u32) -> i32 {
    let start = block_index as usize * BLOCK;
    if start + BLOCK > FLASH_SIZE {
        return -1;
    }

    #[cfg(test)]
    state::with(|flash| flash[start..start + BLOCK].fill(0xFF));

    #[cfg(not(test))]
    // SAFETY: single-threaded embedded context; no concurrent access.
    unsafe {
        (&mut *core::ptr::addr_of_mut!(FLASH_DATA))[start..start + BLOCK].fill(0xFF);
    }
    0
}

pub(crate) fn get_flash_size() -> usize {
    FLASH_SIZE
}

/// Reset the emulated Flash to the all-erased state (all 0xFF).
///
/// In test builds this resets only the calling thread's Flash image, so it is
/// safe to call from parallel test threads without affecting other tests.
#[cfg_attr(not(test), allow(dead_code))]
pub fn reset() {
    #[cfg(test)]
    state::reset_inner();

    #[cfg(not(test))]
    unsafe {
        (&mut *core::ptr::addr_of_mut!(FLASH_DATA)).fill(0xFF);
    }
}

// ── test backing store ───────────────────────────────────────────────────────

// One Vec<u8> per thread — parallel tests don't interfere with each other.
#[cfg(test)]
mod state {
    use std::cell::RefCell;

    thread_local! {
        // Heap-allocated so the 4 MiB array doesn't blow the thread stack.
        static FLASH: RefCell<Vec<u8>> =
            RefCell::new(vec![0xFF_u8; super::FLASH_SIZE]);
    }

    #[inline]
    pub fn with<R>(f: impl FnOnce(&mut Vec<u8>) -> R) -> R {
        FLASH.with(|flash| f(&mut flash.borrow_mut()))
    }

    #[inline]
    pub fn reset_inner() {
        FLASH.with(|flash| flash.borrow_mut().fill(0xFF));
    }
}
