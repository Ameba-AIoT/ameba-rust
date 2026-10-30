//! Low-level unsafe Flash API.
//!
//! Directly maps to the chip-specific backend functions with **no alignment
//! checks, no bounds checks, no locking**.  For advanced use cases where the
//! caller manages all preconditions (e.g. a bootloader or OTA writer).
//!
//! ⚠️ Use with care.  Incorrect calls can corrupt firmware or brick the device.

use crate::chip_specific;

/// Read `len` bytes from Flash offset `src_addr` into the 4-byte aligned
/// buffer pointer `data`.
///
/// # Safety
///
/// - `data` must be 4-byte aligned and point to `len` bytes of valid memory.
/// - `src_addr + len` must not exceed Flash capacity.
pub unsafe fn spiflash_read(src_addr: u32, data: *mut u32, len: u32) -> Result<(), i32> {
    match chip_specific::spiflash_read(src_addr, data, len) {
        0 => Ok(()),
        e => Err(e),
    }
}

/// Write `len` bytes from the 4-byte aligned pointer `data` to Flash offset
/// `dest_addr`.
///
/// # Safety
///
/// - `data` must be 4-byte aligned and point to `len` bytes of valid memory.
/// - `dest_addr` and `len` must be 4-byte aligned.
/// - The target region must be erased (all 0xFF).
pub unsafe fn spiflash_write(dest_addr: u32, data: *const u32, len: u32) -> Result<(), i32> {
    match chip_specific::spiflash_write(dest_addr, data, len) {
        0 => Ok(()),
        e => Err(e),
    }
}

/// Erase the 4 KiB sector at `sector_index * 4096`.
///
/// # Safety
///
/// The sector index must be within Flash bounds.
pub unsafe fn spiflash_erase_sector(sector_index: u32) -> Result<(), i32> {
    match chip_specific::spiflash_erase_sector(sector_index) {
        0 => Ok(()),
        e => Err(e),
    }
}

/// Erase the 64 KiB block at `block_index * 65536`.
///
/// # Safety
///
/// The block index must be within Flash bounds.
pub unsafe fn spiflash_erase_block(block_index: u32) -> Result<(), i32> {
    match chip_specific::spiflash_erase_block(block_index) {
        0 => Ok(()),
        e => Err(e),
    }
}
