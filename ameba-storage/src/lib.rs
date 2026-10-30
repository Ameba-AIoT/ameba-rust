//! NOR Flash storage driver for Realtek Ameba series chips.
//!
//! # Architecture
//!
//! ```text
//! User / upper-layer crate
//!   ↓  embedded_storage traits (Storage, NorFlash)
//! storage.rs / nor_flash.rs
//!   ↓  internal_* methods
//! common.rs  (FlashStorage, bounds/alignment checks)
//!   ↓  chip_specific routing (compile-time, not runtime)
//! hardware.rs   (real HW — FFI to ROM FLASH_TxData / FLASH_Erase)
//! stub.rs       (emulation — in-memory array, #[cfg(feature="emulation")])
//! ll.rs         (low-level raw Flash helpers — not used in default path)
//! ```
//!
//! # Locking
//!
//! All write/erase operations are wrapped in `with_flash_lock` (hardware.rs),
//! which acquires the IPC hardware semaphore and disables AP interrupts.
//! Read operations use the XIP memory-mapped path and need no lock.
//!
//! # SRAM placement
//!
//! All Flash-touching code in `hardware.rs` carries `#[link_section = ".sram.only.text"]`
//! to ensure execution from SRAM during the SPIC user-mode period.

// Allow std in test builds so that we can use the test harness on the host.
#![cfg_attr(not(test), no_std)]

// ── Compile-time backend selection ───────────────────────────────────────────
// Both modules expose the same `chip_specific::*` interface; the rest of
// the crate never references `hardware` or `stub` directly.
#[cfg_attr(not(feature = "emulation"), path = "hardware.rs")]
#[cfg_attr(feature = "emulation", path = "stub.rs")]
mod chip_specific;

pub mod buffer;
mod common;
pub mod ll;
mod nor_flash;
mod storage;

pub use common::{Flash, FlashStorage, FlashStorageError};

// ── Re-exports for embedded-storage trait impls ───────────────────────────────
#[cfg(feature = "embedded-storage")]
pub use embedded_storage;
