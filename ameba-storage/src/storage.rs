//! High-level `Storage` / `ReadStorage` trait implementation.
//!
//! Handles **arbitrary alignment and length** via a Read-Modify-Write
//! (RMW) strategy: even a 1-byte write reads the containing 4 KiB sector,
//! patches the byte in a stack buffer, erases the sector, then writes the
//! entire sector back.
//!
//! This is the recommended API for application code that does not care about
//! flash wear levelling (use sequential-storage or LittleFS on top for that).

use crate::{
    buffer::FlashSectorBuffer,
    common::{FlashStorage, FlashStorageError},
};

// ── Read ──────────────────────────────────────────────────────────────────────

impl FlashStorage {
    /// Read `bytes.len()` bytes from Flash at `offset`.
    ///
    /// Any alignment is accepted; internally the read is expanded to
    /// 4-byte word boundaries and the relevant slice is copied out.
    pub fn read(&mut self, offset: u32, mut bytes: &mut [u8]) -> Result<(), FlashStorageError> {
        self.check_bounds(offset, bytes.len())?;

        let mut data_offset = offset % Self::WORD_SIZE;
        let mut aligned_offset = offset - data_offset;

        let mut sector_data = FlashSectorBuffer::uninit();

        while !bytes.is_empty() {
            // How many bytes of this sector we need
            let len = bytes.len().min((Self::SECTOR_SIZE - data_offset) as usize);

            // Round up to nearest word
            let aligned_end =
                ((data_offset as usize + len + 3) & !3).min(Self::SECTOR_SIZE as usize);
            let buf = &mut sector_data.as_bytes_mut()[..aligned_end];

            self.internal_read(aligned_offset, buf)?;

            let sector_bytes = unsafe { &sector_data.assume_init_bytes()[..aligned_end] };
            bytes[..len].copy_from_slice(&sector_bytes[data_offset as usize..][..len]);

            aligned_offset += Self::SECTOR_SIZE;
            data_offset = 0;
            bytes = &mut bytes[len..];
        }
        Ok(())
    }
}

// ── Write (RMW) ───────────────────────────────────────────────────────────────

impl FlashStorage {
    /// Write `bytes` to Flash at `offset`.
    ///
    /// Performs Read-Modify-Write at sector granularity (4 KiB): each
    /// affected sector is read, patched in RAM, erased, and written back.
    ///
    /// Any alignment is accepted for both `offset` and `bytes.len()`.
    ///
    /// ⚠️ No wear levelling: avoid calling this at high frequency on the same
    /// sector.  Use sequential-storage or LittleFS on top for wear levelling.
    pub fn write(&mut self, offset: u32, mut bytes: &[u8]) -> Result<(), FlashStorageError> {
        self.check_bounds(offset, bytes.len())?;

        let mut data_offset = offset % Self::SECTOR_SIZE;
        let mut aligned_offset = offset - data_offset;

        let mut sector_data = FlashSectorBuffer::uninit();

        while !bytes.is_empty() {
            // 1. Read the full sector
            self.internal_read(aligned_offset, sector_data.as_bytes_mut())?;
            let sector_bytes = unsafe { sector_data.assume_init_bytes_mut() };

            let len = bytes.len().min((Self::SECTOR_SIZE - data_offset) as usize);

            // 2. Patch the sector buffer
            sector_bytes[data_offset as usize..][..len].copy_from_slice(&bytes[..len]);

            // 3. Erase sector
            self.internal_erase_sector(aligned_offset / Self::SECTOR_SIZE)?;

            // 4. Write back the full sector
            self.internal_write(aligned_offset, sector_bytes)?;

            aligned_offset += Self::SECTOR_SIZE;
            data_offset = 0;
            bytes = &bytes[len..];
        }
        Ok(())
    }
}

// ── embedded-storage trait implementations ───────────────────────────────────

#[cfg(feature = "embedded-storage")]
mod traits {
    use super::*;
    use embedded_storage::{ReadStorage, Storage};

    impl ReadStorage for FlashStorage {
        type Error = FlashStorageError;

        fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
            FlashStorage::read(self, offset, bytes)
        }
        fn capacity(&self) -> usize {
            FlashStorage::capacity(self)
        }
    }

    impl Storage for FlashStorage {
        fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
            FlashStorage::write(self, offset, bytes)
        }
    }
}

#[cfg(all(test, feature = "emulation"))]
mod tests {
    use crate::common::tests::fresh;

    #[test]
    fn rmw_write_arbitrary_alignment() {
        let mut f = fresh();
        f.write(3, b"hello").unwrap();
        let mut buf = vec![0u8; 5];
        f.read(3, &mut buf).unwrap();
        assert_eq!(&buf, b"hello");
    }

    #[test]
    fn rmw_write_preserves_surrounding_bytes() {
        let mut f = fresh();
        let mut init = vec![0xAAu8; 4096];
        init[8..12].fill(0xBB);
        f.write_nor(0, &init).unwrap();

        f.write(9, &[0x11, 0x22]).unwrap();

        let mut buf = vec![0u8; 4];
        f.read(8, &mut buf).unwrap();
        assert_eq!(buf[0], 0xBB, "byte before write region must be preserved");
        assert_eq!(buf[1], 0x11, "first written byte");
        assert_eq!(buf[2], 0x22, "second written byte");
        assert_eq!(buf[3], 0xBB, "byte after write region must be preserved");
    }

    #[test]
    fn rmw_write_cross_sector_boundary() {
        let mut f = fresh();
        // 8 bytes straddling sector 0 and sector 1
        f.write(4092, &vec![0x42u8; 8]).unwrap();
        let mut buf = vec![0u8; 8];
        f.read(4092, &mut buf).unwrap();
        assert_eq!(buf, vec![0x42u8; 8]);
    }

    // ── embedded-storage Storage trait ───────────────────────────────────────

    #[cfg(feature = "embedded-storage")]
    #[test]
    fn storage_trait_write_read() {
        use embedded_storage::{ReadStorage, Storage};
        let mut f = fresh();
        Storage::write(&mut f, 7, b"world").unwrap();
        let mut buf = vec![0u8; 5];
        ReadStorage::read(&mut f, 7, &mut buf).unwrap();
        assert_eq!(&buf, b"world");
    }
}
