//! Firmware image packaging: `app.bin` and `ota_all.bin`.
//!
//! `app.bin` is a plain concatenation of four fixed-size / variable-size parts:
//!
//! ```text
//! cert.bin (4 KiB) | manifest.bin (4 KiB) | km4ns_image2_all.bin | km4tz_image2_all.bin
//! ```
//!
//! The certificate and the manifest are fixed-layout structures that the
//! bootloader reads to decide which slot to boot and to check the image
//! version. Both are 4 KiB buffers filled with `0xFF`, with a handful of
//! fields written into the first few dozen bytes; every byte this module does
//! not explicitly write stays `0xFF`, which is what the bootloader treats as
//! "field absent".
//!
//! # Scope: no secure boot, no flash encryption
//!
//! This implementation only covers images built with secure boot and flash
//! encryption (RSIP/RDP) **disabled**, which is what the fields below encode:
//! `AuthAlg`, `HashAlg`, `ImgHash`, `SBPubKey`, `Signature`, `RsipCfg` and
//! `RsipIV` are all left at `0xFF`, and the two payload images are appended
//! verbatim rather than encrypted.
//!
//! Enabling either feature is **not** a configuration change that this module
//! can detect and reject at run time — nothing it reads carries that flag. It
//! requires implementing the hashing, signing and encryption steps here, and
//! an image packed by this code for a part provisioned to require a signature
//! will fail to boot with no diagnostic. Treat this module as the place to
//! revisit if the security configuration ever changes.

use std::path::Path;

use anyhow::{Context, Result};

use crate::IMG2_SIGN;

// ── manifest / certificate constants ────────────────────────────────────────

/// Size of both the certificate and the manifest region.
const BLOCK_SIZE: usize = 4096;

/// Image format version. Selects the 4 KiB manifest/certificate layout
/// implemented here; version 2 is a different, larger layout.
const IMG_VERSION: u8 = 1;

/// Image ID of the application image.
const IMG_ID_APP: u8 = 1;

/// Application image version, as reported to the bootloader. The bootloader
/// compares the *certificate*'s key version across the two OTA slots to pick
/// which one to boot, so bumping these is what makes a freshly flashed slot win
/// over the other one.
const IMG_VER_MAJOR: u16 = 1;
const IMG_VER_MINOR: u16 = 3;

/// Hardware unique key epoch. Only meaningful with secure boot enabled; kept
/// at the value the bootloader expects for an unsigned image.
const HUK_EPOCH: u32 = 1;

/// Certificate key table: one entry, holding the SHA-256 hash of the public key
/// that would verify the application image's signature. With secure boot
/// disabled the bootloader never uses it, but the field is part of the layout
/// and is written to keep the structure well-formed.
const CERT_KEY_ID: u32 = 0;
const CERT_KEY_HASH: [u8; 32] = [
    0x1D, 0xFC, 0x2F, 0xE0, 0x1C, 0xA8, 0x27, 0x4F, 0x06, 0xE2, 0xE1, 0x12, 0xD0, 0x27, 0xC3, 0xC6,
    0xFF, 0x9C, 0xED, 0x59, 0xEE, 0x79, 0x94, 0x4B, 0xED, 0x46, 0xAD, 0xE3, 0x5C, 0x44, 0xB4, 0x22,
];

/// Byte offset of the key table inside the certificate: everything up to and
/// including the 144-byte public key field.
const CERT_KEY_TABLE_OFFSET: usize = 176;
/// Size of one key table entry: `KeyID` (4) + `Hash` (32).
const CERT_KEY_ENTRY_SIZE: usize = 36;

// ── OTA header constants ────────────────────────────────────────────────────

/// Total OTA header size for a single-image update: an 8-byte preamble plus one
/// 24-byte per-image descriptor.
const OTA_HEADER_SIZE: u32 = 32;
/// Size of the per-image descriptor that follows the preamble.
const OTA_DESC_SIZE: u32 = 24;
/// `OTA1`, stored big-endian so it reads as ASCII in a hex dump.
const OTA_MAGIC: u32 = 0x4F54_4131;

// ── app.bin ─────────────────────────────────────────────────────────────────

/// Build `app.bin` from the two per-core images, writing the certificate and
/// the manifest alongside it in `build_dir` for inspection.
pub fn fw_pack(app_bin: &Path, km4ns_bin: &Path, km4tz_bin: &Path, build_dir: &Path) -> Result<()> {
    let km4ns = read(km4ns_bin)?;
    let km4tz = read(km4tz_bin)?;

    // The manifest's ImgSize covers the payload only — the two images it
    // describes, not the certificate and manifest blocks in front of them.
    let img_size = u32::try_from(km4ns.len() + km4tz.len())
        .context("payload images exceed the 4 GiB the manifest's ImgSize field can express")?;

    let cert = build_cert();
    let manifest = build_manifest(img_size);

    let cert_bin = build_dir.join("app_cert.bin");
    let manifest_bin = build_dir.join("manifest_app.bin");
    write(&cert_bin, &cert)?;
    write(&manifest_bin, &manifest)?;

    let mut app = Vec::with_capacity(cert.len() + manifest.len() + km4ns.len() + km4tz.len());
    app.extend_from_slice(&cert);
    app.extend_from_slice(&manifest);
    app.extend_from_slice(&km4ns);
    app.extend_from_slice(&km4tz);
    write(app_bin, &app)?;

    println!(
        "[pack] cert={}B  manifest={}B  km4ns={}B  km4tz={}B  app={}B",
        cert.len(),
        manifest.len(),
        km4ns.len(),
        km4tz.len(),
        app.len()
    );
    Ok(())
}

/// Certificate block: image identity plus the one-entry key table.
///
/// ```text
/// off  size  field
///   0     8  Pattern
///   8     8  Rsvd1
///  16     1  Ver
///  17     1  ImgID
///  18     1  AuthAlg      (0xFF: unsigned)
///  19     1  HashAlg      (0xFF: unsigned)
///  20     2  MajorKeyVer
///  22     2  MinorKeyVer
///  24     4  TableSize
///  28     4  EntryNum
///  32   144  SBPubKey     (0xFF: unsigned)
/// 176     4  KeyID        \ one key table entry
/// 180    32  Hash         /
/// 212   144  Signature    (0xFF: unsigned)
/// 356    ..  padding to 4 KiB
/// ```
fn build_cert() -> Vec<u8> {
    let mut buf = vec![0xFF_u8; BLOCK_SIZE];

    buf[0..8].copy_from_slice(&IMG2_SIGN.to_be_bytes());
    buf[16] = IMG_VERSION;
    buf[17] = IMG_ID_APP;
    buf[20..22].copy_from_slice(&IMG_VER_MAJOR.to_le_bytes());
    buf[22..24].copy_from_slice(&IMG_VER_MINOR.to_le_bytes());

    // TableSize spans the fixed fields plus the entries actually present, so
    // the bootloader knows where the key table ends; it excludes the trailing
    // signature field.
    let table_size = u32::try_from(CERT_KEY_TABLE_OFFSET + CERT_KEY_ENTRY_SIZE).unwrap();
    buf[24..28].copy_from_slice(&table_size.to_le_bytes());
    buf[28..32].copy_from_slice(&1_u32.to_le_bytes()); // EntryNum

    let key = CERT_KEY_TABLE_OFFSET;
    buf[key..key + 4].copy_from_slice(&CERT_KEY_ID.to_le_bytes());
    buf[key + 4..key + 4 + CERT_KEY_HASH.len()].copy_from_slice(&CERT_KEY_HASH);

    buf
}

/// Manifest block: the image version and size the bootloader validates before
/// handing control to the application.
///
/// ```text
/// off  size  field
///   0     8  Pattern
///   8     7  Rsvd1
///  15     1  RsipCfg      (0xFF: no flash encryption)
///  16     1  Ver
///  17     1  ImgID
///  18     1  AuthAlg      (0xFF: unsigned)
///  19     1  HashAlg      (0xFF: unsigned)
///  20     2  MajorImgVer
///  22     2  MinorImgVer
///  24     4  ImgSize
///  28     4  HukEpoch
///  32    16  RsipIV       (0xFF: no flash encryption)
///  48    64  ImgHash      (0xFF: unsigned)
/// 112   144  SBPubKey     (0xFF: unsigned)
/// 256   144  Signature    (0xFF: unsigned)
/// 400    ..  padding to 4 KiB
/// ```
fn build_manifest(img_size: u32) -> Vec<u8> {
    let mut buf = vec![0xFF_u8; BLOCK_SIZE];

    buf[0..8].copy_from_slice(&IMG2_SIGN.to_be_bytes());
    buf[16] = IMG_VERSION;
    buf[17] = IMG_ID_APP;
    buf[20..22].copy_from_slice(&IMG_VER_MAJOR.to_le_bytes());
    buf[22..24].copy_from_slice(&IMG_VER_MINOR.to_le_bytes());
    buf[24..28].copy_from_slice(&img_size.to_le_bytes());
    buf[28..32].copy_from_slice(&HUK_EPOCH.to_le_bytes());

    buf
}

// ── ota_all.bin ─────────────────────────────────────────────────────────────

/// Prepend the OTA header to `app_bin`, producing an image ready to serve to an
/// OTA client.
///
/// This runs on every build rather than as an opt-in step: `ota_all.bin` is the
/// same content as `app.bin` with a 32-byte header describing it, so there is
/// no separate "package for OTA" phase to invoke.
///
/// ```text
/// off  size  field
///   0     4  version      (0xFFFFFFFF)
///   4     4  ota_num      (1: single image)
///   8     4  magic        ("OTA1", big-endian)
///  12     4  header_length
///  16     4  checksum     (sum of the image's bytes)
///  20     4  image_size
///  24     4  offset       (where the image starts)
///  28     4  img_id
///  32    ..  app.bin, verbatim
/// ```
pub fn ota_prepend_header(app_bin: &Path, ota_bin: &Path) -> Result<()> {
    let app = read(app_bin)?;

    let image_size = u32::try_from(app.len())
        .context("app.bin exceeds the 4 GiB the OTA header's image_size field can express")?;
    let checksum = app
        .iter()
        .fold(0_u32, |acc, &b| acc.wrapping_add(u32::from(b)));

    let mut ota = Vec::with_capacity(OTA_HEADER_SIZE as usize + app.len());
    ota.extend_from_slice(&u32::MAX.to_le_bytes()); // version
    ota.extend_from_slice(&1_u32.to_le_bytes()); // ota_num
    ota.extend_from_slice(&OTA_MAGIC.to_be_bytes());
    ota.extend_from_slice(&OTA_DESC_SIZE.to_le_bytes()); // header_length
    ota.extend_from_slice(&checksum.to_le_bytes());
    ota.extend_from_slice(&image_size.to_le_bytes());
    ota.extend_from_slice(&OTA_HEADER_SIZE.to_le_bytes()); // offset
    ota.extend_from_slice(&u32::from(IMG_ID_APP).to_le_bytes());
    debug_assert_eq!(ota.len(), OTA_HEADER_SIZE as usize);
    ota.extend_from_slice(&app);

    write(ota_bin, &ota)?;

    println!("[pack] ota image_size={image_size}B  checksum={checksum:08x}");
    Ok(())
}

// ── helpers ─────────────────────────────────────────────────────────────────

fn read(path: &Path) -> Result<Vec<u8>> {
    std::fs::read(path).with_context(|| format!("reading {}", path.display()))
}

fn write(path: &Path, data: &[u8]) -> Result<()> {
    std::fs::write(path, data).with_context(|| format!("writing {}", path.display()))
}
