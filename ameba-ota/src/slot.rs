//! Active-slot detection.
//!
//! Rust port of SDK: `ameba_ota.c` : `ota_get_cur_index()`.

use crate::layout;

/// OTA image identifier.
/// SDK: `ameba_ota.h` : `OTA_IMGID_BOOT` / `OTA_IMGID_APP`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtaImageId {
    /// KM4 bootloader image.
    Boot = 0,
    /// Application image.
    App = 1,
}

/// Which physical A/B slot an image currently occupies.
/// SDK: `ameba_boot.h` : `OTA_INDEX_1` / `OTA_INDEX_2`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtaSlot {
    /// OTA1 (the slot mapped by `CONFIG_FLASH_*_OFFSET` without an `_OTA2` suffix).
    Slot1 = 0,
    /// OTA2 (the `_OTA2`-suffixed slot).
    Slot2 = 1,
}

/// `RSIP_REMAPxOR` "remap region x enable" bit, as retained by the
/// bootloader into `RRAM_DEV->OTA_IMG_REMAP[]`.
/// SDK: `ameba_rsip.h` : `RSIP_BIT_REMAP_x_ENABLE`
const RSIP_BIT_REMAP_X_ENABLE: u32 = 1 << 0;

/// Extra bytes prepended to the application image (certificate + manifest)
/// that the remap physical address must be corrected for.
/// SDK: `ameba_ota.c` : `ota_get_cur_index()`, `img_id == OTA_IMGID_APP` branch.
const APP_HEADER_SIZE: u32 = 0x2000;

/// Extra bytes prepended to a non-application image (manifest only).
/// SDK: `ameba_ota.c` : `ota_get_cur_index()`, `else` branch.
const MANIFEST_ONLY_SIZE: u32 = 0x1000;

/// Return the OTA1/OTA2 physical start address for `img_id`.
fn ota1_start(img_id: OtaImageId) -> u32 {
    match img_id {
        OtaImageId::Boot => layout::BOOT_OTA1_START,
        OtaImageId::App => layout::APP_OTA1_START,
    }
}

/// Which slot a new download must be written to — always the one the chip is
/// *not* currently running from, so a failed or interrupted update leaves the
/// running image intact.
/// SDK: `ota_internal.h` : `ota_get_target_slot()`
pub fn ota_get_target_slot(img_id: OtaImageId) -> OtaSlot {
    match ota_get_cur_index(img_id) {
        OtaSlot::Slot1 => OtaSlot::Slot2,
        OtaSlot::Slot2 => OtaSlot::Slot1,
    }
}

/// Determine which physical slot `img_id` is currently running from.
///
/// Reads the bootloader's boot-time remap snapshot (see [`ameba_hal::rram::ota_img_remap`])
/// rather than any live register, matching the SDK's own data source.
pub fn ota_get_cur_index(img_id: OtaImageId) -> OtaSlot {
    let addr_start = ameba_hal::rram::ota_img_remap(img_id as usize);

    if addr_start & RSIP_BIT_REMAP_X_ENABLE != 0 {
        let phy_addr = (addr_start >> 9) << 5;
        let header_size = match img_id {
            OtaImageId::App => APP_HEADER_SIZE,
            OtaImageId::Boot => MANIFEST_ONLY_SIZE,
        };
        let phy_addr = phy_addr - header_size;

        if phy_addr == ota1_start(img_id) {
            return OtaSlot::Slot1;
        }
        return OtaSlot::Slot2;
    }

    OtaSlot::Slot1
}
