//! Prebuilt images flashed alongside every Rust application.

/// First-stage bootloader (image1).
pub const BOOT_BIN: &[u8] = include_bytes!("../../ameba-rom-sys/bin/rtl8721f/boot.bin");
/// Prebuilt NP-core (KM4NS) WiFi firmware.
pub const NP_IMAGE: &[u8] = include_bytes!("../../ameba-rom-sys/bin/rtl8721f/km4ns_image2_all.bin");
/// RAM-resident flash loader used by UART flashing.
pub const FLOADER_BIN: &[u8] =
    include_bytes!("../../ameba-rom-sys/bin/rtl8721f/floader_rtl8721f.bin");
