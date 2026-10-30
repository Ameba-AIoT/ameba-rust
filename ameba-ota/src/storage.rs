//! OTA image write/verify/commit over NOR flash.
//!
//! Scope: NOR only. SDK: `ota_storage.c` — the same write path additionally
//! branches on `FLASH_CAP_BADBLOCK` to support NAND (page-buffered writes,
//! bad-block skip/mark, deferred whole-page manifest commit). NOR has no
//! bad-block concept (SDK: `ameba_nor_dev.c` : `nor_dev_is_bad()` always
//! reports good) and is byte/word writable, so this port erases lazily at
//! sector granularity and writes body bytes directly instead of buffering
//! whole pages — the NAND-only branches have no NOR equivalent to port.
//!
//! The manifest ([`crate::manifest::MANIFEST_LEN`] bytes at the head of the
//! image) is still deferred and, once written, still lands with its
//! `Pattern` field last — SDK: `ota_storage.c` : `update_manifest_nor()` —
//! since that ordering is what makes a slot commit power-safe on NOR, not a
//! NAND-only concern.

use crate::{
    layout, manifest,
    slot::{OtaImageId, OtaSlot},
};
use ameba_storage::{FlashStorage, FlashStorageError};

/// Errors from [`OtaWriter`] operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtaStorageError {
    /// The write cursor reached the slot's end before the image did.
    SlotExhausted,
    /// Underlying flash erase failed.
    Erase(FlashStorageError),
    /// Underlying flash write failed.
    Write(FlashStorageError),
    /// Underlying flash read failed.
    Read(FlashStorageError),
}

/// Bytes buffered in RAM before a flash write is issued.
///
/// One sector. Every `write_nor`/`erase` call takes the flash lock, which
/// acquires an IPC hardware semaphore and **disables interrupts** for the whole
/// operation (`ameba-storage`: `hardware.rs` : `with_flash_lock()`). Writing in
/// small increments therefore does not merely cost throughput — it starves the
/// core. Programming a ~380 KB image four bytes at a time takes the lock ~98000
/// times, which on hardware stalled the WiFi driver long enough to miss beacons
/// (`no bcn long, disconnect/roaming`) and wedge the cross-core IPC channel.
/// Buffering a full sector cuts that to one lock per sector.
///
/// Measured on hardware at this size (RTL8721F, 100 MHz flash clock, full
/// ~380 KB image = 96 sectors): sector erase 0.9–1.4 ms, sector write
/// 4.8–7.1 ms — so the longest single stretch with interrupts disabled is
/// ~7 ms, and one erase+write round trip is at worst ~8.5 ms. That bound is
/// what makes a watchdog feasible here without any hook into this crate: a
/// refresh task on a period comfortably above ~10 ms (e.g. the 500 ms / 5 s
/// timeout pairing in `examples/wdg`) cannot be starved past its deadline by
/// flash access. Shrinking `CHUNK_LEN` would multiply the lock count without
/// shortening any single stretch; growing it past one sector would lengthen
/// the individual stretch.
pub const CHUNK_LEN: usize = FlashStorage::SECTOR_SIZE as usize;

/// Staging buffer for [`OtaWriter::begin`]. Place it in a `static`.
pub type OtaWriteBuf = [u8; CHUNK_LEN];

/// Sequential writer for one OTA image slot.
///
/// Usage: [`OtaWriter::begin`], repeated [`OtaWriter::write_body`],
/// [`OtaWriter::flush`], [`OtaWriter::verify_checksum`], then — only if the
/// checksum matches — [`OtaWriter::commit_manifest`].
///
/// The [`CHUNK_LEN`]-byte staging buffer is borrowed rather than inlined: at
/// this size it would bloat the enclosing async future past what an embassy
/// task arena comfortably holds, so the caller places it in a `static`.
pub struct OtaWriter<'b> {
    slot_start: u32,
    slot_end: u32,
    erased_to: u32,
    write_cursor: u32,
    body_len: u32,
    /// Staging buffer; `buf[..buf_len]` is pending data not yet programmed.
    buf: &'b mut [u8; CHUNK_LEN],
    buf_len: usize,
}

impl<'b> OtaWriter<'b> {
    /// Start writing `img_id`'s `slot` — resets the write cursor to the slot
    /// start, past the reserved manifest bytes. `buf` is the staging buffer
    /// described on [`OtaWriter`].
    /// SDK: `ota_storage.c` : `ota_storage_begin_image()`.
    pub fn begin(img_id: OtaImageId, slot: OtaSlot, buf: &'b mut [u8; CHUNK_LEN]) -> Self {
        let (start, size) = match (img_id, slot) {
            (OtaImageId::Boot, OtaSlot::Slot1) => (layout::BOOT_OTA1_START, layout::BOOT_OTA1_SIZE),
            (OtaImageId::Boot, OtaSlot::Slot2) => (layout::BOOT_OTA2_START, layout::BOOT_OTA2_SIZE),
            (OtaImageId::App, OtaSlot::Slot1) => (layout::APP_OTA1_START, layout::APP_OTA1_SIZE),
            (OtaImageId::App, OtaSlot::Slot2) => (layout::APP_OTA2_START, layout::APP_OTA2_SIZE),
        };
        let slot_start = start - layout::SPI_FLASH_BASE;
        Self {
            slot_start,
            slot_end: slot_start + size,
            erased_to: slot_start,
            write_cursor: slot_start + manifest::MANIFEST_LEN as u32,
            body_len: 0,
            buf,
            buf_len: 0,
        }
    }

    /// Erase every not-yet-erased sector in `[erased_to, up_to)`, rounding
    /// `up_to` up to the next sector boundary. Lazy — called right before the
    /// bytes it covers are written — the shared behavior SDK: `ota_storage.c`
    /// : `prepare_block()` implements at (NAND) erase-block granularity; here
    /// it is plain sector erase since NOR has no larger program unit to
    /// respect.
    fn ensure_erased(
        &mut self,
        flash: &mut FlashStorage,
        up_to: u32,
    ) -> Result<(), OtaStorageError> {
        if up_to > self.slot_end {
            return Err(OtaStorageError::SlotExhausted);
        }
        let target = up_to.div_ceil(FlashStorage::SECTOR_SIZE) * FlashStorage::SECTOR_SIZE;
        let target = target.min(self.slot_end);
        if target > self.erased_to {
            flash
                .erase(self.erased_to, target)
                .map_err(OtaStorageError::Erase)?;
            self.erased_to = target;
        }
        Ok(())
    }

    /// Program `self.buf[..len]` at the cursor and advance it.
    ///
    /// `len` must be a multiple of [`FlashStorage::WORD_SIZE`]; the tail case is
    /// handled by [`OtaWriter::flush`], which pads first.
    fn commit_buf(&mut self, flash: &mut FlashStorage, len: usize) -> Result<(), OtaStorageError> {
        if len == 0 {
            return Ok(());
        }
        self.ensure_erased(flash, self.write_cursor + len as u32)?;
        flash
            .write_nor(self.write_cursor, &self.buf[..len])
            .map_err(OtaStorageError::Write)?;
        self.write_cursor += len as u32;
        // Retain any bytes past `len` (a padded tail write never has them).
        self.buf.copy_within(len..self.buf_len, 0);
        self.buf_len -= len;
        Ok(())
    }

    /// Append image body bytes. Storage owns the physical cursor, lazy sector
    /// erase, and buffering — see [`CHUNK_LEN`] for why writes are batched.
    /// SDK: `ota_storage.c` : `ota_storage_write_stream()`.
    pub fn write_body(
        &mut self,
        flash: &mut FlashStorage,
        mut data: &[u8],
    ) -> Result<(), OtaStorageError> {
        while !data.is_empty() {
            let take = (CHUNK_LEN - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&data[..take]);
            self.buf_len += take;
            self.body_len += take as u32;
            data = &data[take..];

            if self.buf_len == CHUNK_LEN {
                self.commit_buf(flash, CHUNK_LEN)?;
            }
        }
        Ok(())
    }

    /// Program whatever is still buffered, padding the final word with `0xFF`
    /// (the flash-erased value, so it costs nothing to leave programmed). Call
    /// once after the last [`OtaWriter::write_body`].
    /// SDK: `ota_storage.c` : `ota_storage_flush()`.
    pub fn flush(&mut self, flash: &mut FlashStorage) -> Result<(), OtaStorageError> {
        if self.buf_len == 0 {
            return Ok(());
        }
        let word = FlashStorage::WORD_SIZE as usize;
        let padded = self.buf_len.next_multiple_of(word);
        for b in &mut self.buf[self.buf_len..padded] {
            *b = 0xFF;
        }
        // Report only the padded span as consumed: `buf_len` may be < `padded`,
        // and the padding bytes are not image data.
        self.buf_len = padded;
        self.commit_buf(flash, padded)
    }

    /// Sum `manifest` plus every body byte written so far and compare
    /// against `expected`. The sum is a plain byte-wise `u32` accumulator
    /// (wrapping on overflow, no CRC) — matching SDK: `ota_storage.c` :
    /// `ota_storage_verify_checksum()`, which sums the same way (all 24
    /// manifest bytes including `Pattern`, then body bytes, ascending
    /// address order). Call after [`OtaWriter::flush`], before
    /// [`OtaWriter::commit_manifest`] — the manifest itself is not yet on
    /// flash at this point.
    pub fn verify_checksum(
        &self,
        flash: &mut FlashStorage,
        manifest: &[u8; manifest::MANIFEST_LEN],
        expected: u32,
    ) -> Result<bool, OtaStorageError> {
        let mut sum: u32 = 0;
        for &b in manifest {
            sum = sum.wrapping_add(b as u32);
        }

        let mut off = self.slot_start + manifest::MANIFEST_LEN as u32;
        let end = off + self.body_len;
        let mut buf = [0u8; 256];
        while off < end {
            let remaining = (end - off) as usize;
            let chunk = remaining.min(buf.len());
            let read_len = (chunk + 3) & !3;
            flash
                .read_nor(off, &mut buf[..read_len])
                .map_err(OtaStorageError::Read)?;
            for &b in &buf[..chunk] {
                sum = sum.wrapping_add(b as u32);
            }
            off += chunk as u32;
        }

        Ok(sum == expected)
    }

    /// Commit `manifest` — the deferred header at the slot's head — writing
    /// its body first and its `Pattern` field (the bootloader validity
    /// marker) last, so a power loss mid-write leaves the old slot's pattern
    /// intact rather than a half-written new one. Call only after
    /// [`OtaWriter::verify_checksum`] returns `Ok(true)`.
    /// SDK: `ota_storage.c` : `update_manifest_nor()`.
    pub fn commit_manifest(
        &mut self,
        flash: &mut FlashStorage,
        manifest: &[u8; manifest::MANIFEST_LEN],
    ) -> Result<(), OtaStorageError> {
        self.ensure_erased(flash, self.slot_start + manifest::MANIFEST_LEN as u32)?;
        flash
            .write_nor(
                self.slot_start + manifest::MANIFEST_PATTERN_LEN as u32,
                &manifest[manifest::MANIFEST_PATTERN_LEN..],
            )
            .map_err(OtaStorageError::Write)?;
        flash
            .write_nor(self.slot_start, &manifest[..manifest::MANIFEST_PATTERN_LEN])
            .map_err(OtaStorageError::Write)?;
        Ok(())
    }
}
