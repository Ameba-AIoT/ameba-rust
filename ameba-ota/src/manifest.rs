//! OTA firmware-file header and manifest parsing.
//!
//! These structs describe the byte layout of headers embedded in the OTA
//! firmware file (as downloaded/streamed), not a hardware register block —
//! parsed by field offset from a byte slice rather than cast through a
//! `#[repr(C)]` pointer, so there is no reliance on this chip's struct
//! padding rules.
//!
//! SDK: `ota_internal.h` (`ota_hdr_t`, `ota_sub_hdr_t`, `ota_manifest_t`),
//! `ameba.h` (`AuthHeader_TypeDef`, of which `ota_manifest_t` is a type
//! alias). All field offsets below were verified by compiling an
//! `offsetof`/`sizeof` probe against these exact struct definitions with
//! the project's own `arm-none-eabi-gcc-12.3.1` toolchain and reading the
//! result back with `nm -S` — not hand-computed.

/// Byte length of [`FileHeader`] on the wire.
/// SDK: `ota_internal.h` : `OTA_HEADER_LEN`
pub const FILE_HEADER_LEN: usize = 8;

/// Byte length of one [`SubHeader`] entry on the wire.
/// SDK: `ota_internal.h` : `OTA_SUB_HEADER_LEN`
pub const SUB_HEADER_LEN: usize = 24;

/// Byte length of [`Manifest`] on the wire (`sizeof(ota_manifest_t)`, i.e.
/// `sizeof(AuthHeader_TypeDef)`).
pub const MANIFEST_LEN: usize = 24;

/// Byte length of the bootloader validity-marker `Pattern` field within
/// [`Manifest`].
/// SDK: `ota_internal.h` : `OTA_MANIFEST_PATTERN_LEN`
pub const MANIFEST_PATTERN_LEN: usize = 8;

/// Top-level OTA firmware file header.
/// SDK: `ota_internal.h` : `ota_hdr_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileHeader {
    /// OTA firmware version (first `u32` in the file).
    pub fw_ver: u32,
    /// Number of [`SubHeader`] entries following this header.
    pub hdr_num: u32,
}

impl FileHeader {
    /// Parse from the first [`FILE_HEADER_LEN`] bytes of the OTA file.
    /// Returns `None` if `buf` is too short.
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let fw_ver = u32::from_le_bytes(buf.get(0..4)?.try_into().ok()?);
        let hdr_num = u32::from_le_bytes(buf.get(4..8)?.try_into().ok()?);
        Some(Self { fw_ver, hdr_num })
    }
}

/// Per-image sub-header, one per image carried in the OTA file.
/// SDK: `ota_internal.h` : `ota_sub_hdr_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubHeader {
    /// Must read `"OTA"` (3 ASCII bytes, 4th byte unused) to be valid.
    pub signature: [u8; 4],
    /// Length of this image's header section.
    pub img_hdr_len: u32,
    /// Checksum of this image's body (used by [`crate::layout`]-relative
    /// verification, see SDK: `ota_storage.c` : `ota_storage_verify_checksum()`).
    pub checksum: u32,
    /// Total length of this image (manifest + body).
    pub img_len: u32,
    /// This image's byte offset within the overall OTA file.
    pub offset: u32,
    /// Image ID (`OTA_IMGID_BOOT` / `OTA_IMGID_APP`, see [`crate::slot::OtaImageId`]).
    pub img_id: u32,
}

impl SubHeader {
    /// Parse from a [`SUB_HEADER_LEN`]-byte slice. Returns `None` if `buf`
    /// is too short or the signature doesn't read `"OTA"`.
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let signature: [u8; 4] = buf.get(0..4)?.try_into().ok()?;
        if &signature[..3] != b"OTA" {
            return None;
        }
        let img_hdr_len = u32::from_le_bytes(buf.get(4..8)?.try_into().ok()?);
        let checksum = u32::from_le_bytes(buf.get(8..12)?.try_into().ok()?);
        let img_len = u32::from_le_bytes(buf.get(12..16)?.try_into().ok()?);
        let offset = u32::from_le_bytes(buf.get(16..20)?.try_into().ok()?);
        let img_id = u32::from_le_bytes(buf.get(20..24)?.try_into().ok()?);
        Some(Self {
            signature,
            img_hdr_len,
            checksum,
            img_len,
            offset,
            img_id,
        })
    }
}

/// Per-image manifest — the small validity/versioning header that prefixes
/// each image's body in the OTA stream (distinct from the much larger
/// signature-verification manifest the bootloader itself consumes).
/// SDK: `ameba.h` : `AuthHeader_TypeDef`; `ota_internal.h` : `ota_manifest_t`
/// (`typedef AuthHeader_TypeDef ota_manifest_t;`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Manifest {
    /// Bootloader validity marker (first [`MANIFEST_PATTERN_LEN`] bytes).
    pub pattern: [u8; MANIFEST_PATTERN_LEN],
    pub ver: u8,
    pub img_id: u8,
    pub auth_alg: u8,
    pub hash_alg: u8,
    pub major_key_ver: u16,
    pub minor_key_ver: u16,
}

impl Manifest {
    /// Parse from a [`MANIFEST_LEN`]-byte slice. Returns `None` if `buf` is
    /// too short.
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let pattern: [u8; MANIFEST_PATTERN_LEN] = buf.get(0..8)?.try_into().ok()?;
        let ver = *buf.get(16)?;
        let img_id = *buf.get(17)?;
        let auth_alg = *buf.get(18)?;
        let hash_alg = *buf.get(19)?;
        let major_key_ver = u16::from_le_bytes(buf.get(20..22)?.try_into().ok()?);
        let minor_key_ver = u16::from_le_bytes(buf.get(22..24)?.try_into().ok()?);
        Some(Self {
            pattern,
            ver,
            img_id,
            auth_alg,
            hash_alg,
            major_key_ver,
            minor_key_ver,
        })
    }

    /// 32-bit full version, matching the bootloader's own comparison value.
    /// SDK: `boot_ota_km4tz.c` : `BOOT_OTA_SlotSelect()` —
    /// `(MajorKeyVer << 16) | MinorKeyVer`.
    pub fn version(&self) -> u32 {
        (self.major_key_ver as u32) << 16 | self.minor_key_ver as u32
    }
}
