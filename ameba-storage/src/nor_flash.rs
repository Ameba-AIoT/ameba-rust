//! `NorFlash` trait implementation — strict alignment, no automatic RMW.
//!
//! Unlike `storage.rs`, callers are responsible for meeting alignment and
//! pre-erase requirements:
//!
//! | Operation  | Offset alignment | Length alignment | Precondition |
//! |------------|-----------------|------------------|--------------|
//! | `read_nor` | 4 bytes         | 4 bytes          | —            |
//! | `write_nor`| 4 bytes         | 4 bytes          | Target must be erased (0xFF) |
//! | `erase`    | 4096 bytes      | 4096 bytes       | —            |

use crate::{
    buffer::FlashSectorBuffer,
    common::{FlashStorage, FlashStorageError},
};
use core::mem::MaybeUninit;

impl FlashStorage {
    /// Minimum read alignment — equals [`WORD_SIZE`](Self::WORD_SIZE).
    pub const READ_SIZE: usize = Self::WORD_SIZE as usize;
    /// Minimum write alignment — equals [`WORD_SIZE`](Self::WORD_SIZE).
    pub const WRITE_SIZE: usize = Self::WORD_SIZE as usize;
    /// Minimum erase size — equals [`SECTOR_SIZE`](Self::SECTOR_SIZE).
    pub const ERASE_SIZE: usize = Self::SECTOR_SIZE as usize;

    fn is_word_aligned(bytes: &[u8]) -> bool {
        (bytes.as_ptr() as usize).is_multiple_of(4)
    }

    /// Read with strict alignment requirements.
    ///
    /// Both `offset` and `bytes.len()` must be multiples of 4.
    /// If `bytes` is already 4-byte aligned the read goes directly into the
    /// user buffer (zero-copy fast path); otherwise a sector-size intermediate
    /// buffer is used.
    pub fn read_nor(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), FlashStorageError> {
        self.check_alignment::<{ FlashStorage::READ_SIZE }>(offset, bytes.len())?;
        self.check_bounds(offset, bytes.len())?;

        if Self::is_word_aligned(bytes) {
            // Fast path: the user buffer itself is word-aligned → read directly
            let mut off = offset;
            for chunk in bytes.chunks_mut(Self::SECTOR_SIZE as usize) {
                // SAFETY: chunk is word-aligned (we checked above) and
                //         len is a multiple of 4 (checked above).
                let uninit_chunk = unsafe {
                    core::slice::from_raw_parts_mut(
                        chunk.as_mut_ptr() as *mut MaybeUninit<u8>,
                        chunk.len(),
                    )
                };
                self.internal_read(off, uninit_chunk)?;
                off += chunk.len() as u32;
            }
        } else {
            // Slow path: buffer is not word-aligned → use intermediate buffer
            let mut sector_data = FlashSectorBuffer::uninit();
            let mut off = offset;
            for chunk in bytes.chunks_mut(Self::SECTOR_SIZE as usize) {
                let buf = &mut sector_data.as_bytes_mut()[..chunk.len()];
                self.internal_read(off, buf)?;
                let src = unsafe { &sector_data.assume_init_bytes()[..chunk.len()] };
                chunk.copy_from_slice(src);
                off += chunk.len() as u32;
            }
        }
        Ok(())
    }

    /// Write with strict alignment requirements.
    ///
    /// Both `offset` and `bytes.len()` must be multiples of 4.
    /// The target region must already be erased (all 0xFF).
    ///
    /// Uses a 4-byte intermediate buffer when `bytes` is not word-aligned.
    pub fn write_nor(&mut self, offset: u32, bytes: &[u8]) -> Result<(), FlashStorageError> {
        self.check_alignment::<{ FlashStorage::WRITE_SIZE }>(offset, bytes.len())?;
        self.check_bounds(offset, bytes.len())?;

        if Self::is_word_aligned(bytes) {
            // Fast path: write directly from user buffer
            let mut off = offset;
            for chunk in bytes.chunks(Self::SECTOR_SIZE as usize) {
                self.internal_write(off, chunk)?;
                off += chunk.len() as u32;
            }
        } else {
            // Slow path: copy through a word-aligned intermediate buffer.
            let mut sector_data = FlashSectorBuffer::uninit();
            let mut off = offset;
            for chunk in bytes.chunks(Self::SECTOR_SIZE as usize) {
                let mbuf = &mut sector_data.as_bytes_mut()[..chunk.len()];
                for (dst, &src) in mbuf.iter_mut().zip(chunk.iter()) {
                    dst.write(src);
                }
                // SAFETY: every byte in [..chunk.len()] was written in the loop above.
                let buf = unsafe { &sector_data.assume_init_bytes()[..chunk.len()] };
                self.internal_write(off, buf)?;
                off += chunk.len() as u32;
            }
        }
        Ok(())
    }

    /// Erase Flash in the range `[from, to)`.
    ///
    /// Both `from` and `to` must be multiples of 4096 (sector size).
    ///
    /// Uses a three-phase algorithm for maximum speed:
    /// 1. Sector-erase until the next 64 KiB block boundary.
    /// 2. Block-erase (much faster) while ≥ 64 KiB remains.
    /// 3. Sector-erase the trailing partial block.
    pub fn erase(&mut self, from: u32, to: u32) -> Result<(), FlashStorageError> {
        if !from.is_multiple_of(Self::SECTOR_SIZE) || !to.is_multiple_of(Self::SECTOR_SIZE) {
            return Err(FlashStorageError::NotAligned);
        }
        let len = to.checked_sub(from).ok_or(FlashStorageError::OutOfBounds)? as usize;
        self.check_bounds(from, len)?;

        let mut address = from;

        // Phase 1: sector-erase up to the next block boundary
        while address < to && !address.is_multiple_of(Self::BLOCK_SIZE) {
            self.internal_erase_sector(address / Self::SECTOR_SIZE)?;
            address += Self::SECTOR_SIZE;
        }

        // Phase 2: block-erase (faster)
        while to.saturating_sub(address) >= Self::BLOCK_SIZE {
            self.internal_erase_block(address / Self::BLOCK_SIZE)?;
            address += Self::BLOCK_SIZE;
        }

        // Phase 3: sector-erase tail
        while address < to {
            self.internal_erase_sector(address / Self::SECTOR_SIZE)?;
            address += Self::SECTOR_SIZE;
        }

        Ok(())
    }
}

// ── embedded-storage trait implementations ───────────────────────────────────

#[cfg(feature = "embedded-storage")]
mod traits {
    use super::*;
    use embedded_storage::nor_flash::{
        ErrorType, MultiwriteNorFlash, NorFlash, NorFlashError, NorFlashErrorKind, ReadNorFlash,
    };

    impl NorFlashError for FlashStorageError {
        fn kind(&self) -> NorFlashErrorKind {
            match self {
                FlashStorageError::OutOfBounds => NorFlashErrorKind::OutOfBounds,
                FlashStorageError::NotAligned => NorFlashErrorKind::NotAligned,
                _ => NorFlashErrorKind::Other,
            }
        }
    }

    // NOTE: ErrorType provides the Error associated type for NorFlash traits.
    // ReadStorage (from embedded_storage) has its own Error assoc type defined
    // in storage.rs; the two do not conflict because they are separate traits.
    impl ErrorType for FlashStorage {
        type Error = FlashStorageError;
    }

    impl ReadNorFlash for FlashStorage {
        const READ_SIZE: usize = FlashStorage::READ_SIZE;

        fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
            FlashStorage::read_nor(self, offset, bytes)
        }
        fn capacity(&self) -> usize {
            FlashStorage::capacity(self)
        }
    }

    impl NorFlash for FlashStorage {
        const WRITE_SIZE: usize = FlashStorage::WRITE_SIZE;
        const ERASE_SIZE: usize = FlashStorage::ERASE_SIZE;

        fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
            FlashStorage::write_nor(self, offset, bytes)
        }
        fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
            FlashStorage::erase(self, from, to)
        }
    }

    /// `MultiwriteNorFlash` marker: NOR Flash physically allows writing a byte
    /// multiple times (each write can only clear bits), so sequential partial
    /// writes to the same word are valid as long as the data is consistent.
    impl MultiwriteNorFlash for FlashStorage {}
}

#[cfg(all(test, feature = "emulation"))]
mod tests {
    use crate::common::tests::{fresh, make_pattern};

    // ── erase ─────────────────────────────────────────────────────────────────

    #[test]
    fn erase_sector_fills_ff() {
        let mut f = fresh();
        f.write_nor(0, &vec![0xABu8; 4096]).unwrap();
        f.erase(0, 4096).unwrap();
        let mut buf = vec![0u8; 4096];
        f.read_nor(0, &mut buf).unwrap();
        assert!(buf.iter().all(|&b| b == 0xFF), "sector not erased");
    }

    #[test]
    fn erase_two_sectors() {
        let mut f = fresh();
        f.write_nor(0, &vec![0x55u8; 8192]).unwrap();
        f.erase(0, 8192).unwrap();
        let mut buf = vec![0u8; 8192];
        f.read_nor(0, &mut buf).unwrap();
        assert!(buf.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn erase_block_fills_ff() {
        let mut f = fresh();
        f.write_nor(0, &vec![0x12u8; 65536]).unwrap();
        f.erase(0, 65536).unwrap();
        let mut buf = vec![0u8; 65536];
        f.read_nor(0, &mut buf).unwrap();
        assert!(buf.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn erase_not_sector_aligned_returns_error() {
        let mut f = fresh();
        assert!(f.erase(1, 4097).is_err());
    }

    #[test]
    fn erase_out_of_bounds_returns_error() {
        let mut f = fresh();
        let cap = f.capacity() as u32;
        assert!(f.erase(cap - 4096, cap + 4096).is_err());
    }

    #[test]
    fn erase_then_rewrite() {
        let mut f = fresh();
        let first = make_pattern(256, 0xAA);
        let second = make_pattern(256, 0x55);
        f.write_nor(0, &first).unwrap();
        f.erase(0, 4096).unwrap();
        f.write_nor(0, &second).unwrap();
        let mut buf = vec![0u8; 256];
        f.read_nor(0, &mut buf).unwrap();
        assert_eq!(buf, second);
    }

    // ── write_nor / read_nor ──────────────────────────────────────────────────

    #[test]
    fn write_nor_then_read_nor_roundtrip() {
        let mut f = fresh();
        let pattern = make_pattern(256, 0);
        f.write_nor(0, &pattern).unwrap();
        let mut buf = vec![0u8; 256];
        f.read_nor(0, &mut buf).unwrap();
        assert_eq!(buf, pattern);
    }

    #[test]
    fn write_nor_16b_at_offset() {
        let mut f = fresh();
        let pattern: Vec<u8> = vec![
            0x5A, 0xA5, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
            0x0D, 0x0E,
        ];
        f.write_nor(0x10, &pattern).unwrap();
        let mut buf = vec![0u8; 16];
        f.read_nor(0x10, &mut buf).unwrap();
        assert_eq!(buf, pattern);
        // Adjacent region must still be 0xFF
        let mut before = vec![0u8; 16];
        f.read_nor(0x00, &mut before).unwrap();
        assert!(before.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn write_nor_256b_aligned_page() {
        let mut f = fresh();
        let pattern = make_pattern(256, 0);
        f.write_nor(0, &pattern).unwrap();
        let mut buf = vec![0u8; 256];
        f.read_nor(0, &mut buf).unwrap();
        assert_eq!(&buf[..4], &[0x00, 0x01, 0x02, 0x03]);
        assert_eq!(&buf[252..], &[0xFC, 0xFD, 0xFE, 0xFF]);
    }

    #[test]
    fn write_nor_256b_cross_page_boundary() {
        let mut f = fresh();
        // offset 128 spans page 0 (bytes 128..255) and page 1 (bytes 0..127)
        let pattern = make_pattern(256, 0);
        f.write_nor(128, &pattern).unwrap();
        let mut buf = vec![0u8; 256];
        f.read_nor(128, &mut buf).unwrap();
        assert_eq!(buf, pattern);
    }

    #[test]
    fn write_nor_unaligned_offset_returns_error() {
        let mut f = fresh();
        assert!(f.write_nor(1, &vec![0u8; 4]).is_err());
    }

    #[test]
    fn write_nor_unaligned_length_returns_error() {
        let mut f = fresh();
        assert!(f.write_nor(0, &vec![0u8; 3]).is_err());
    }

    #[test]
    fn write_nor_out_of_bounds_returns_error() {
        let mut f = fresh();
        let cap = f.capacity() as u32;
        assert!(f.write_nor(cap - 4, &vec![0u8; 8]).is_err());
    }

    #[test]
    fn nor_flash_can_only_clear_bits() {
        // NOR Flash: write can clear bits (1→0) but NOT set bits (0→1 requires erase)
        let mut f = fresh();
        f.write_nor(0, &vec![0xF0u8; 4]).unwrap();
        f.write_nor(0, &vec![0x0Fu8; 4]).unwrap(); // ANDs with existing
        let mut buf = vec![0u8; 4];
        f.read_nor(0, &mut buf).unwrap();
        assert_eq!(buf, vec![0x00u8; 4]); // 0xF0 & 0x0F = 0x00
    }

    // ── embedded-storage NorFlash trait ──────────────────────────────────────

    #[cfg(feature = "embedded-storage")]
    #[test]
    fn nor_flash_trait_write_read() {
        use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
        let mut f = fresh();
        let data = make_pattern(64, 1);
        NorFlash::erase(&mut f, 0, 4096).unwrap();
        NorFlash::write(&mut f, 0, &data).unwrap();
        let mut buf = vec![0u8; 64];
        ReadNorFlash::read(&mut f, 0, &mut buf).unwrap();
        assert_eq!(buf, data);
    }
}
