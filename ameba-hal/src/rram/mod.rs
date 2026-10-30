//! Retention RAM (RRAM) raw-memory reader.
//!
//! `RETENTION_RAM_BASE` is plain SRAM that survives an application-only
//! reset (not a register-block peripheral) — grepping the PAC (`pac/rtl8721f/src/`)
//! and the raw SVD files for "RRAM"/"RETENTION" turns up only unrelated
//! `LP_AON`/`LP_SYS`/`SYSTEM_MEM_CTRL` register blocks and the
//! `KM4[NT]Z_RETENTION_MODE` control bits, never a peripheral covering this
//! address. There is no PAC accessor for this region, matching the existing
//! `RRAM_DEV->clk_info_bk` "no PAC equivalent" precedent in
//! [`crate::clk`]'s module doc — so this module reads it with a raw pointer.

/// Base address of the chip's retention RAM block.
/// SDK: `hal_platform.h` : `RETENTION_RAM_BASE`
const RETENTION_RAM_BASE: u32 = 0x4080_AE00;

/// Byte offset of `RRAM_TypeDef.OTA_IMG_REMAP` within the struct.
///
/// SDK: `sysreg_lsys.h` : `RRAM_TypeDef` (member `OTA_IMG_REMAP`). The
/// struct mixes packed/aligned sub-members (`CLK_Info_Backup` is
/// `__attribute__((packed, aligned(1)))` while the outer struct is not),
/// so this offset was not hand-computed — it was verified by compiling an
/// `offsetof` probe against the real struct definition with the project's
/// own `arm-none-eabi-gcc-12.3.1` toolchain and reading the result back
/// with `nm`/`objdump`.
const OTA_IMG_REMAP_OFFSET: u32 = 0x11C;

/// Number of entries in `RRAM_TypeDef.OTA_IMG_REMAP[]`.
/// SDK: `sysreg_lsys.h` : `RRAM_TypeDef` (`OTA_IMG_REMAP[2]`)
const OTA_IMG_REMAP_LEN: usize = 2;

/// Read the bootloader's slot-remap snapshot for one OTA image ID
/// (`img_id` is `OTA_IMGID_BOOT` = 0 or `OTA_IMGID_APP` = 1 per SDK:
/// `ameba_ota.h`).
///
/// This is a boot-time snapshot, not a live register: the bootloader
/// copies `RSIP_REMAPxOR` into `RRAM_DEV->OTA_IMG_REMAP[]` before handing
/// off to the application (SDK: `boot_ota_km4tz.c`, the `RRAM_DEV->OTA_IMG_REMAP[...] =
/// RSIP_BASE->FLASH_MMU[...].RSIP_REMAPxOR` assignments). The application-side
/// slot detector (SDK: `ameba_ota.c` : `ota_get_cur_index()`) reads this same
/// retained field rather than the live `rsip_reg` block, which this port
/// follows.
///
/// # Panics
/// Panics if `img_id >= 2` (out of bounds for `OTA_IMG_REMAP[2]`).
pub fn ota_img_remap(img_id: usize) -> u32 {
    assert!(
        img_id < OTA_IMG_REMAP_LEN,
        "OTA_IMG_REMAP index out of bounds"
    );
    let addr = RETENTION_RAM_BASE + OTA_IMG_REMAP_OFFSET + (img_id as u32) * 4;
    // Safety: `addr` stays within the 512-byte RRAM_TypeDef region (Compile_Assert
    // in sysreg_lsys.h fixes its size at 512 bytes), is naturally 4-byte aligned
    // (base and offset are both multiples of 4), and this field is written once
    // by the bootloader before the application starts — no other bus master
    // writes it afterward, so a plain volatile read is race-free.
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}
