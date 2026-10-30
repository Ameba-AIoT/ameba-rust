//! OTA download state machine.
//!
//! Rust port of the programming loop in SDK: `ota_core.c` —
//! `ota_core_recv_header()` / `download_parameter_init()` /
//! `manifest_accumulate()` / `download_packet_process()` /
//! `download_finish()`. Pulls bytes from a [`Transport`] and drives
//! [`OtaWriter`] to land them in the inactive slot.
//!
//! Scope versus the C original:
//!
//! - **Pull path only.** The C API also exposes a push path
//!   (`ota_set_image_info()` + `ota_write()` + `ota_finish()`) that skips
//!   header parsing and leaves integrity checking to the caller. That path
//!   has no consumer here yet; the byte-feed use case it exists for is
//!   already covered by writing a [`Transport`] impl.
//! - **Single image per file.** The C core walks `HdrNum` sub-headers and
//!   programs each in turn, buffering any bytes of image *N+1* that arrived
//!   inside image *N*'s last packet (`NextImgBuf`/`NextImgFg`). Here a file
//!   carrying more than one image is rejected up front
//!   ([`OtaError::MultiImageUnsupported`]) rather than half-supported —
//!   this project's images are built and shipped one at a time, and the
//!   spillover buffer only earns its complexity once that changes.
//! - **No bootloader-OTA2 special case.** The C core consults
//!   `ota_check_bootloader_ota2()` and silently drops the payload
//!   (`SkipBootOTAFg`) when the bootloader's second slot is unusable. That
//!   check reads chip state this crate does not model yet, so bootloader
//!   images are refused outright instead of being skipped quietly.

use crate::manifest::{self, FileHeader, SubHeader};
use crate::slot::{OtaImageId, ota_get_target_slot};
use crate::storage::{OtaStorageError, OtaWriteBuf, OtaWriter};
use crate::transport::Transport;
use ameba_storage::FlashStorage;

/// Byte buffer size for one transport read.
///
/// Matches the C core's own download buffer (SDK: `ota_internal.h` :
/// `OTA_BUF_SIZE`), which `ota_core_process()` heap-allocates per session.
/// Here the caller owns it as a plain array — see [`OtaSession::run`].
pub const OTA_BUF_SIZE: usize = 2048;

/// Failure modes of an OTA session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtaError<T> {
    /// The transport failed or ended early.
    Transport(T),
    /// Stream ended before the declared image length was received.
    UnexpectedEof,
    /// Flash write/erase/read failed.
    Storage(OtaStorageError),
    /// The leading [`FileHeader`] could not be parsed.
    BadFileHeader,
    /// A [`SubHeader`] was malformed or carried a foreign signature.
    BadSubHeader,
    /// The file declares no images at all.
    NoImages,
    /// The file carries more than one image — see the module note.
    MultiImageUnsupported,
    /// `ImgID` names an image this crate cannot program.
    UnsupportedImageId(u32),
    /// `ImgLen` is smaller than the manifest that must precede the body.
    ImageTooShort,
    /// `ImgLen` exceeds the target slot's capacity.
    ImageTooLarge,
    /// Body checksum did not match the sub-header's `Checksum`.
    ChecksumMismatch,
}

impl<T> From<OtaStorageError> for OtaError<T> {
    fn from(e: OtaStorageError) -> Self {
        OtaError::Storage(e)
    }
}

/// Outcome of a completed session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OtaReport {
    /// Firmware version from the file header.
    pub fw_ver: u32,
    /// Which image was programmed.
    pub img_id: OtaImageId,
    /// Body bytes written (image length minus its manifest).
    pub body_len: u32,
}

/// Slot capacity for `img_id`'s target slot, in bytes.
fn slot_size(img_id: OtaImageId, slot: crate::slot::OtaSlot) -> u32 {
    use crate::layout;
    use crate::slot::OtaSlot;
    match (img_id, slot) {
        (OtaImageId::Boot, OtaSlot::Slot1) => layout::BOOT_OTA1_SIZE,
        (OtaImageId::Boot, OtaSlot::Slot2) => layout::BOOT_OTA2_SIZE,
        (OtaImageId::App, OtaSlot::Slot1) => layout::APP_OTA1_SIZE,
        (OtaImageId::App, OtaSlot::Slot2) => layout::APP_OTA2_SIZE,
    }
}

/// Reads ahead over a [`Transport`], handing out contiguous byte runs.
///
/// The core needs to consume a few small fixed-size records (file header,
/// sub-header, manifest) that a transport is free to split across reads, then
/// stream the rest. This buffers just enough to make those records
/// contiguous without copying the body a second time.
struct Reader<'b, T: Transport> {
    transport: T,
    buf: &'b mut [u8; OTA_BUF_SIZE],
    /// Valid bytes live in `buf[pos..len]`.
    pos: usize,
    len: usize,
    eof: bool,
}

impl<'b, T: Transport> Reader<'b, T> {
    fn new(transport: T, buf: &'b mut [u8; OTA_BUF_SIZE]) -> Self {
        Self {
            transport,
            buf,
            pos: 0,
            len: 0,
            eof: false,
        }
    }

    /// Pull one transport read into the buffer, compacting first. Returns
    /// `false` once the transport reports end of stream.
    async fn refill(&mut self) -> Result<bool, OtaError<T::Error>> {
        if self.eof {
            return Ok(false);
        }
        if self.pos > 0 {
            self.buf.copy_within(self.pos..self.len, 0);
            self.len -= self.pos;
            self.pos = 0;
        }
        if self.len == self.buf.len() {
            // Callers only ever request records far smaller than the buffer,
            // so a full buffer means there is nothing to wait for.
            return Ok(true);
        }
        let n = self
            .transport
            .read(&mut self.buf[self.len..])
            .await
            .map_err(OtaError::Transport)?;
        if n == 0 {
            self.eof = true;
            return Ok(false);
        }
        self.len += n;
        Ok(true)
    }

    /// Borrow exactly `n` contiguous bytes, reading more if needed.
    /// `n` must not exceed [`OTA_BUF_SIZE`].
    async fn exact(&mut self, n: usize) -> Result<&[u8], OtaError<T::Error>> {
        debug_assert!(n <= OTA_BUF_SIZE);
        while self.len - self.pos < n {
            if !self.refill().await? {
                return Err(OtaError::UnexpectedEof);
            }
        }
        let out = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(out)
    }

    /// Borrow whatever is buffered, refilling if empty. Returns an empty
    /// slice only at end of stream.
    async fn chunk(&mut self) -> Result<&[u8], OtaError<T::Error>> {
        if self.pos == self.len && !self.refill().await? {
            return Ok(&[]);
        }
        let out = &self.buf[self.pos..self.len];
        self.pos = self.len;
        Ok(out)
    }
}

/// One OTA download.
///
/// Owns nothing but the transport: the flash handle is borrowed per call so a
/// caller can keep using flash for other purposes between sessions.
pub struct OtaSession<T: Transport> {
    transport: T,
}

impl<T: Transport> OtaSession<T> {
    /// Wrap `transport` as the byte source for one download.
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    /// Download, verify, and commit the image the transport is serving.
    ///
    /// On success the target slot holds a fully written image with a valid
    /// manifest, and the bootloader will select it on the next reset. On any
    /// error the target slot is left partially written but *uncommitted* —
    /// its manifest `Pattern` is never placed, so the bootloader keeps
    /// booting the current image (SDK: `ota_storage.c` :
    /// `update_manifest_nor()` relies on the same ordering).
    ///
    /// `buf` is the transport read buffer ([`OTA_BUF_SIZE`]); `write_buf` is the
    /// flash staging buffer ([`crate::storage::OtaWriteBuf`]). Both are borrowed
    /// rather than owned so they can live in `static`s instead of inflating this
    /// future.
    pub async fn run(
        self,
        flash: &mut FlashStorage,
        buf: &mut [u8; OTA_BUF_SIZE],
        write_buf: &mut OtaWriteBuf,
    ) -> Result<OtaReport, OtaError<T::Error>> {
        let mut reader = Reader::new(self.transport, buf);

        // ── File header: version + image count ───────────────────────────
        // SDK: `ota_core.c` : `ota_core_recv_header()`
        let file_hdr = {
            let bytes = reader.exact(manifest::FILE_HEADER_LEN).await?;
            FileHeader::parse(bytes).ok_or(OtaError::BadFileHeader)?
        };
        match file_hdr.hdr_num {
            0 => return Err(OtaError::NoImages),
            1 => {}
            _ => return Err(OtaError::MultiImageUnsupported),
        }

        // ── Sub-header: where and how big this image is ──────────────────
        // SDK: `ota_core.c` : `ota_core_get_target_header()`
        let sub_hdr = {
            let bytes = reader.exact(manifest::SUB_HEADER_LEN).await?;
            SubHeader::parse(bytes).ok_or(OtaError::BadSubHeader)?
        };

        // Only the application image is programmable here — see the module
        // note on the bootloader-OTA2 check this crate does not model.
        let img_id = match sub_hdr.img_id {
            id if id == OtaImageId::App as u32 => OtaImageId::App,
            id => return Err(OtaError::UnsupportedImageId(id)),
        };

        let manifest_len = manifest::MANIFEST_LEN as u32;
        let body_len = sub_hdr
            .img_len
            .checked_sub(manifest_len)
            .ok_or(OtaError::ImageTooShort)?;

        // ── Slot selection and capacity check ────────────────────────────
        // SDK: `ota_core.c` : `download_parameter_init()` picks the inactive
        // slot; `ota_storage_check_layout()` is the size guard.
        let slot = ota_get_target_slot(img_id);
        if sub_hdr.img_len > slot_size(img_id, slot) {
            return Err(OtaError::ImageTooLarge);
        }

        // ── Manifest: buffered in RAM, committed last ────────────────────
        // SDK: `ota_core.c` : `manifest_accumulate()` — the manifest is held
        // back so a slot is never marked valid before its body verifies.
        let mut manifest_buf = [0u8; manifest::MANIFEST_LEN];
        manifest_buf.copy_from_slice(reader.exact(manifest::MANIFEST_LEN).await?);

        // ── Body: stream straight to flash ───────────────────────────────
        // SDK: `ota_core.c` : `download_packet_process()`
        let mut writer = OtaWriter::begin(img_id, slot, write_buf);
        let mut remaining = body_len;
        while remaining > 0 {
            let chunk = reader.chunk().await?;
            if chunk.is_empty() {
                return Err(OtaError::UnexpectedEof);
            }
            // A server may append bytes past the declared length; program
            // only what the header accounts for, matching the C core's
            // `RemainBytes` clamp.
            let take = (chunk.len() as u32).min(remaining) as usize;
            writer.write_body(flash, &chunk[..take])?;
            remaining -= take as u32;
        }
        writer.flush(flash)?;

        // ── Verify, then commit ──────────────────────────────────────────
        // SDK: `ota_core.c` : `download_finish()` — checksum first, manifest
        // second, so an interrupted commit cannot arm a bad image.
        if !writer.verify_checksum(flash, &manifest_buf, sub_hdr.checksum)? {
            return Err(OtaError::ChecksumMismatch);
        }
        writer.commit_manifest(flash, &manifest_buf)?;

        Ok(OtaReport {
            fw_ver: file_hdr.fw_ver,
            img_id,
            body_len,
        })
    }
}
