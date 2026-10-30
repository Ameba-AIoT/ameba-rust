//! Image packaging and UART flashing for the Ameba Rust SDK.
//!
//! - [`elf`] — ELF → `km4tz_image2_all.bin` section extraction.
//! - [`pack`] — `app.bin` / `ota_all.bin` assembly.
//! - [`flash`] — UART flashing protocol and the per-session driver.
//! - [`monitor`] — serial log monitor.
//! - [`prebuilt`] — bootloader, NP image and flash loader embedded in the crate.

pub mod elf;
pub mod flash;
pub mod monitor;
pub mod pack;
pub mod prebuilt;

/// Magic at the start of every image sub-header.
pub(crate) const IMG2_SIGN: u64 = 0x3831393538373131;
