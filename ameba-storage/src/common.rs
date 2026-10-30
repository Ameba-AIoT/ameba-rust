//! Core `FlashStorage` type and internal operations.
//!
//! All alignment checking, bounds checking and chip-specific dispatch live
//! here.  The higher-level modules (`storage.rs`, `nor_flash.rs`) call the
//! `internal_*` methods exposed in this module.

use crate::chip_specific;

// ── Error type ───────────────────────────────────────────────────────────────

/// Errors that can be returned by Flash storage operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FlashStorageError {
    /// Address + length exceeds the Flash capacity.
    OutOfBounds,
    /// Address or length is not aligned to the required boundary.
    NotAligned,
    /// The chip-specific read operation failed.
    ReadFailed,
    /// The chip-specific write operation failed.
    WriteFailed,
    /// The chip-specific erase operation failed.
    EraseFailed,
    /// The requested operation is not supported on this platform.
    NotSupported,
}

impl core::fmt::Display for FlashStorageError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfBounds => write!(f, "Flash: out of bounds"),
            Self::NotAligned => write!(f, "Flash: not aligned"),
            Self::ReadFailed => write!(f, "Flash: read failed"),
            Self::WriteFailed => write!(f, "Flash: write failed"),
            Self::EraseFailed => write!(f, "Flash: erase failed"),
            Self::NotSupported => write!(f, "Flash: not supported"),
        }
    }
}

// ── Flash ownership token ─────────────────────────────────────────────────────

/// SPIC (main Flash) ownership token.
///
/// Exactly one instance must exist while [`FlashStorage`] is in use.
/// On hardware, obtain via [`Flash::steal`]; in emulation builds, use `Flash::new` (gated behind the `emulation` feature).
pub struct Flash(());

impl Flash {
    /// Claim the Flash token without `unsafe`.
    ///
    /// Only available with `feature = "emulation"`: the stub backend is
    /// `thread_local`, so each test thread has independent state and multiple
    /// calls are safe.
    #[cfg(feature = "emulation")]
    pub fn new() -> Self {
        Flash(())
    }
}

#[cfg(feature = "emulation")]
impl Default for Flash {
    fn default() -> Self {
        Self::new()
    }
}

impl Flash {
    /// Unsafely claim the Flash token.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other `Flash` instance exists for the
    /// lifetime of the returned token.
    pub unsafe fn steal() -> Self {
        Flash(())
    }
}

// ── FlashStorage ─────────────────────────────────────────────────────────────

/// Main Flash storage handle.
///
/// Wraps the chip-specific backend and provides alignment/bounds checking.
///
/// # Usage
///
/// ```no_run
/// use ameba_storage::{Flash, FlashStorage};
///
/// // SAFETY: only one FlashStorage exists in this program.
/// let mut flash = FlashStorage::new(unsafe { Flash::steal() });
/// flash.write(0x100000, b"hello").unwrap();
/// let mut buf = [0u8; 5];
/// flash.read(0x100000, &mut buf).unwrap();
/// ```
pub struct FlashStorage {
    capacity: usize,
    _flash: Flash,
}

impl FlashStorage {
    /// Minimum read unit (bytes) — NOR Flash requires 4-byte word access.
    pub const WORD_SIZE: u32 = 4;
    /// Erase granularity — one sector = 4096 bytes.
    pub const SECTOR_SIZE: u32 = 4096;
    /// Block erase granularity — one block = 64 KiB.
    pub const BLOCK_SIZE: u32 = 65536;

    /// Create a new `FlashStorage`, consuming the [`Flash`] ownership token.
    ///
    /// Flash capacity is detected at construction time via JEDEC RDID.
    pub fn new(flash: Flash) -> Self {
        let capacity = chip_specific::get_flash_size();
        Self {
            capacity,
            _flash: flash,
        }
    }

    /// Flash capacity in bytes (detected at construction time via JEDEC RDID).
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // ── internal operations ───────────────────────────────────────────────
    //
    // No `unlock_once` needed: the Ameba ROM functions (`FLASH_TxData`,
    // `FLASH_Erase`) issue WREN internally before each operation.
    // Block-protect (BP) bits are cleared by the bootloader.
    // TODO: confirm BP=0 on hardware; if not, add a WRSR call in `new()`.

    /// Read `data` bytes from Flash at byte offset `offset`.
    ///
    /// `data` must be a multiple of 4 bytes AND its pointer must be 4-byte
    /// aligned (ensured by the callers through `FlashSectorBuffer`).
    pub(crate) fn internal_read(
        &mut self,
        offset: u32,
        data: &mut [core::mem::MaybeUninit<u8>],
    ) -> Result<(), FlashStorageError> {
        let rc =
            chip_specific::spiflash_read(offset, data.as_mut_ptr() as *mut u32, data.len() as u32);
        if rc != 0 {
            Err(FlashStorageError::ReadFailed)
        } else {
            Ok(())
        }
    }

    /// Write `data` to Flash at byte offset `offset`.
    ///
    /// The target region must already be erased (all 0xFF).  `offset` and
    /// `data.len()` must be 4-byte aligned.
    pub(crate) fn internal_write(
        &mut self,
        offset: u32,
        data: &[u8],
    ) -> Result<(), FlashStorageError> {
        let rc =
            chip_specific::spiflash_write(offset, data.as_ptr() as *const u32, data.len() as u32);
        if rc != 0 {
            Err(FlashStorageError::WriteFailed)
        } else {
            Ok(())
        }
    }

    /// Erase the 4 KiB sector that contains byte offset `sector_index * 4096`.
    pub(crate) fn internal_erase_sector(
        &mut self,
        sector_index: u32,
    ) -> Result<(), FlashStorageError> {
        let rc = chip_specific::spiflash_erase_sector(sector_index);
        if rc != 0 {
            Err(FlashStorageError::EraseFailed)
        } else {
            Ok(())
        }
    }

    /// Erase the 64 KiB block that contains byte offset `block_index * 65536`.
    pub(crate) fn internal_erase_block(
        &mut self,
        block_index: u32,
    ) -> Result<(), FlashStorageError> {
        let rc = chip_specific::spiflash_erase_block(block_index);
        if rc != 0 {
            Err(FlashStorageError::EraseFailed)
        } else {
            Ok(())
        }
    }

    // ── helpers ───────────────────────────────────────────────────────────

    /// Check that `[offset, offset+len)` lies within Flash.
    #[inline(always)]
    pub(crate) fn check_bounds(&self, offset: u32, len: usize) -> Result<(), FlashStorageError> {
        // Use checked_add to prevent wraparound on 32-bit targets.
        let end = (offset as usize)
            .checked_add(len)
            .ok_or(FlashStorageError::OutOfBounds)?;
        if end > self.capacity {
            Err(FlashStorageError::OutOfBounds)
        } else {
            Ok(())
        }
    }

    /// Check that both `offset` and `len` are multiples of `ALIGN`.
    #[inline(always)]
    pub(crate) fn check_alignment<const ALIGN: usize>(
        &self,
        offset: u32,
        len: usize,
    ) -> Result<(), FlashStorageError> {
        if !(offset as usize).is_multiple_of(ALIGN) || !len.is_multiple_of(ALIGN) {
            Err(FlashStorageError::NotAligned)
        } else {
            Ok(())
        }
    }
}

#[cfg(all(test, feature = "emulation"))]
pub(crate) mod tests {
    use super::*;
    use crate::chip_specific::reset as flash_reset;

    pub(crate) fn fresh() -> FlashStorage {
        flash_reset();
        FlashStorage::new(Flash::new())
    }

    pub(crate) fn make_pattern(len: usize, seed: u8) -> Vec<u8> {
        (0..len).map(|i| (i as u8).wrapping_add(seed)).collect()
    }

    #[test]
    fn capacity_is_4mb() {
        let f = fresh();
        assert_eq!(f.capacity(), 4 * 1024 * 1024);
    }
}
