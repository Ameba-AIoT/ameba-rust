//! OTA (dual A/B slot) firmware update for RTL8721F.
//!
//! Rust port of `component/ota/ota_core.c` + `ota_storage.c` (SDK). Runs
//! entirely in the application; the bootloader (`boot.bin`) independently
//! performs slot selection and ECC signature verification before handing
//! off to this image — this crate does not re-implement that step.
#![no_std]

pub mod core;
#[cfg(feature = "http-reqwless")]
pub mod http;
pub mod layout;
pub mod manifest;
pub mod slot;
pub mod storage;
pub mod transport;
