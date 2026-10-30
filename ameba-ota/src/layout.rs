//! Flash layout constants.
//!
//! This project's OTA partition table is fixed by menuconfig (SDK:
//! `build_RTL8721F/menuconfig/.config_km4tz`, `CONFIG_FLASH_*` keys), not
//! computed at runtime — consumed by both slot detection ([`crate::slot`])
//! and the future storage read/write layer.

/// XIP memory-mapped base address that `CONFIG_FLASH_*_OFFSET` values are
/// relative to.
/// SDK: `hal_platform.h` : `SPI_FLASH_BASE`
pub const SPI_FLASH_BASE: u32 = 0x0800_0000;

/// Bootloader OTA1 slot absolute start address.
/// SDK config key: `CONFIG_FLASH_BOOT_OFFSET`
pub const BOOT_OTA1_START: u32 = 0x0800_0000;
/// Bootloader OTA1 slot size in bytes.
/// SDK config key: `CONFIG_FLASH_BOOT_SIZE`
pub const BOOT_OTA1_SIZE: u32 = 0x0004_0000;

/// Bootloader OTA2 slot absolute start address.
/// SDK config key: `CONFIG_FLASH_BOOT_OTA2_OFFSET`
pub const BOOT_OTA2_START: u32 = 0x081C_0000;
/// Bootloader OTA2 slot size in bytes.
/// SDK config key: `CONFIG_FLASH_BOOT_OTA2_SIZE`
pub const BOOT_OTA2_SIZE: u32 = 0x0004_0000;

/// Application OTA1 slot absolute start address.
/// SDK config key: `CONFIG_FLASH_OTA1_OFFSET`
pub const APP_OTA1_START: u32 = 0x0804_0000;
/// Application OTA1 slot size in bytes.
/// SDK config key: `CONFIG_FLASH_OTA1_SIZE`
pub const APP_OTA1_SIZE: u32 = 0x0018_0000;

/// Application OTA2 slot absolute start address.
/// SDK config key: `CONFIG_FLASH_OTA2_OFFSET`
pub const APP_OTA2_START: u32 = 0x0820_0000;
/// Application OTA2 slot size in bytes.
/// SDK config key: `CONFIG_FLASH_OTA2_SIZE`
pub const APP_OTA2_SIZE: u32 = 0x0018_0000;
