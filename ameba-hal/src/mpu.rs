//! MPU (Memory Protection Unit) initialization for the KM4TZ core.
//!
//! Mirrors `mpu_init()` / `mpu_region_cfg()` (`ameba_mpu_ram.c`) and
//! `app_mpu_nocache_init()` (`ameba_app_start.c`, `ram_km4tz`) from the
//! ameba-rtos C SDK. Without this, [`crate::cache`] enables DCache with no
//! MPU region backing it: the ROM null-pointer range is not read-only
//! protected, and there is no non-cacheable region for DMA buffers that
//! aren't manually cache-maintained (UART/SPI DMA in this HAL already do
//! per-transfer `dcache_clean`/`dcache_invalidate` and don't depend on this).
//!
//! Region numbers are hardcoded (0, 1) rather than porting the SDK's
//! `mpu_entry_alloc`/`mpu_entry_free` round-robin allocator — nothing else in
//! this HAL currently allocates MPU regions.

use cortex_m::peripheral::Peripherals;
use cortex_m::peripheral::scb::Exception;

// SDK: mpu_config.h — Outer/Inner memory-attribute encodings (unshifted).
const NORMAL_WT_T_RA: u8 = 0x02;
const NORMAL_WB_T_RWA: u8 = 0x07;
const NORMAL_NC: u8 = 0x04;
const ARM_MPU_ATTR_DEVICE: u8 = 0x00;
const ARM_MPU_ATTR_DEVICE_NGNRNE: u8 = 0x00;

/// SDK: `cmsis/m-profile/armv8m_mpu.h` : `ARM_MPU_ATTR(O, I)`.
const fn arm_mpu_attr(outer: u8, inner: u8) -> u8 {
    if outer & 0xF != 0 {
        ((outer & 0xF) << 4) | (inner & 0xF)
    } else {
        (inner & 0x3) << 2
    }
}

// SDK: mpu_config.h : MPU_MEM_ATTR0..3 — Attr0 = fully non-cacheable (used
// for the ROM RO region and the nocache DMA region below), Attr1/Attr2 are
// unused by this port but kept so MAIR0 matches the SDK bit-for-bit, Attr3 =
// Device nGnRnE (also reused for Attr4..7, all Device on this chip).
const MPU_MEM_ATTR0: u32 = arm_mpu_attr(NORMAL_NC, NORMAL_NC) as u32;
const MPU_MEM_ATTR1: u32 = arm_mpu_attr(NORMAL_WT_T_RA, NORMAL_WT_T_RA) as u32;
const MPU_MEM_ATTR2: u32 = arm_mpu_attr(NORMAL_WB_T_RWA, NORMAL_WB_T_RWA) as u32;
const MPU_MEM_ATTR_DEVICE: u32 =
    arm_mpu_attr(ARM_MPU_ATTR_DEVICE, ARM_MPU_ATTR_DEVICE_NGNRNE) as u32;

/// SDK: mpu_config.h : `MPU_MEM_ATTR_IDX_NC` — MAIR attribute index used for
/// the non-cacheable regions this port configures.
const MPU_MEM_ATTR_IDX_NC: u8 = 0;

// SDK: core_cm33_km4.h — PMSAv8 MAIR0/MAIR1 attribute-byte shifts.
const MAIR0_ATTR1_POS: u32 = 8;
const MAIR0_ATTR2_POS: u32 = 16;
const MAIR0_ATTR3_POS: u32 = 24;
const MAIR1_ATTR5_POS: u32 = 8;
const MAIR1_ATTR6_POS: u32 = 16;
const MAIR1_ATTR7_POS: u32 = 24;

// SDK: core_cm33_km4.h — PMSAv8 RBAR/RLAR/CTRL bit positions and masks.
const MPU_RBAR_BASE_MSK: u32 = 0x7FF_FFFF << 5;
const MPU_RBAR_SH_POS: u32 = 3;
const MPU_RBAR_AP_POS: u32 = 1;
const MPU_RBAR_XN_POS: u32 = 0;
const MPU_RLAR_LIMIT_MSK: u32 = 0x7FF_FFFF << 5;
const MPU_RLAR_ATTRINDX_POS: u32 = 1;
const MPU_RLAR_EN_MSK: u32 = 1;
const MPU_CTRL_ENABLE_MSK: u32 = 1;
const MPU_CTRL_HFNMIENA_MSK: u32 = 1 << 1;
const MPU_CTRL_PRIVDEFENA_MSK: u32 = 1 << 2;

/// SDK: mpu_config.h : `mpu_region_config_ap_define` — access-permission
/// encodings, pre-shifted into RBAR[2:1] like the SDK's `MPU_*` constants.
const MPU_UN_PRIV_RO: u32 = 3 << MPU_RBAR_AP_POS;
const MPU_UN_PRIV_RW: u32 = 1 << MPU_RBAR_AP_POS;

/// SDK: mpu_config.h : `mpu_region_config_xn_define`.
const MPU_EXEC_ALLOW: u32 = 0 << MPU_RBAR_XN_POS;

/// SDK: mpu_config.h : `mpu_region_config_sh_define` (`ARM_MPU_SH_NON`) — the
/// only shareability value `app_mpu_nocache_init()` actually uses.
const MPU_NON_SHAREABLE: u32 = 0 << MPU_RBAR_SH_POS;

// SDK: ameba_layout.ld (KM4TZ) — nocache RAM region bounds. Defined in
// ameba.x, 32-byte aligned at both ends to match the PMSAv8 RBAR/RLAR granule;
// empty (start == end) until a driver needs a static nocache buffer, matching
// app_mpu_nocache_init()'s own `region_size >= 32` guard below.
unsafe extern "C" {
    static __ram_nocache_start__: u8;
    static __ram_nocache_end__: u8;
}

/// Mirrors the SDK's `mpu_region_config` struct; `ap`/`sh`/`xn` take the
/// pre-shifted `MPU_*` constants above.
struct MpuRegionConfig {
    region_base: u32,
    region_size: u32,
    xn: u32,
    ap: u32,
    sh: u32,
    attr_idx: u8,
}

/// SDK: `armv8m_mpu.h` : `ARM_MPU_Disable()`.
fn arm_mpu_disable() {
    cortex_m::asm::dmb();
    unsafe {
        let mut cp = Peripherals::steal();
        cp.SCB.disable(Exception::MemoryManagement);
        cp.MPU.ctrl.modify(|ctrl| ctrl & !MPU_CTRL_ENABLE_MSK);
    }
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}

/// SDK: `armv8m_mpu.h` : `ARM_MPU_Enable(MPU_Control)`.
fn arm_mpu_enable(mpu_control: u32) {
    cortex_m::asm::dmb();
    unsafe {
        let mut cp = Peripherals::steal();
        cp.MPU.ctrl.write(mpu_control | MPU_CTRL_ENABLE_MSK);
        cp.SCB.enable(Exception::MemoryManagement);
    }
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
}

/// Program MAIR0/MAIR1 with the SDK's typical attribute set and enable the
/// MPU with the default (unmapped-address) background region active.
///
/// SDK: `ameba_mpu_ram.c` : `mpu_init`
pub fn mpu_init() {
    arm_mpu_disable();

    let mair0 = MPU_MEM_ATTR0
        | MPU_MEM_ATTR1 << MAIR0_ATTR1_POS
        | MPU_MEM_ATTR2 << MAIR0_ATTR2_POS
        | MPU_MEM_ATTR_DEVICE << MAIR0_ATTR3_POS;
    let mair1 = MPU_MEM_ATTR_DEVICE
        | MPU_MEM_ATTR_DEVICE << MAIR1_ATTR5_POS
        | MPU_MEM_ATTR_DEVICE << MAIR1_ATTR6_POS
        | MPU_MEM_ATTR_DEVICE << MAIR1_ATTR7_POS;
    unsafe {
        let cp = Peripherals::steal();
        cp.MPU.mair[0].write(mair0);
        cp.MPU.mair[1].write(mair1);
    }

    // PRIVDEFENA=1 (unmapped privileged accesses fall back to the default
    // memory map instead of faulting), HFNMIENA=0 (HardFault/NMI handlers
    // bypass the MPU).
    arm_mpu_enable(MPU_CTRL_PRIVDEFENA_MSK & !MPU_CTRL_HFNMIENA_MSK);
}

/// SDK: `ameba_mpu_ram.c` : `mpu_region_cfg`
fn mpu_region_cfg(region_num: u8, cfg: &MpuRegionConfig) {
    let region_limit = cfg.region_base + cfg.region_size - 1;

    arm_mpu_disable();
    let ctrl = unsafe {
        let cp = Peripherals::steal();
        cp.MPU.rnr.write(region_num as u32);
        cp.MPU
            .rbar
            .write((cfg.region_base & MPU_RBAR_BASE_MSK) | cfg.sh | cfg.ap | cfg.xn);
        cp.MPU.rlar.write(
            (region_limit & MPU_RLAR_LIMIT_MSK)
                | ((cfg.attr_idx as u32) << MPU_RLAR_ATTRINDX_POS)
                | MPU_RLAR_EN_MSK,
        );
        cp.MPU.ctrl.read()
    };
    arm_mpu_enable(ctrl);
}

/// Protect the ROM null-pointer range as read-only, and — if the linker's
/// `__ram_nocache_start__..__ram_nocache_end__` region is non-empty — mark it
/// non-cacheable for DMA buffers that aren't manually cache-maintained.
///
/// SDK: `ameba_app_start.c` : `app_mpu_nocache_init`
pub fn mpu_nocache_init() {
    // ROM code does not enter the cache; map [0, 0xF0000) read-only so a
    // null-pointer dereference faults instead of silently reading ROM.
    mpu_region_cfg(
        0,
        &MpuRegionConfig {
            region_base: 0,
            region_size: 0x000F_0000,
            xn: MPU_EXEC_ALLOW,
            ap: MPU_UN_PRIV_RO,
            sh: MPU_NON_SHAREABLE,
            attr_idx: MPU_MEM_ATTR_IDX_NC,
        },
    );

    let region_base = unsafe { &__ram_nocache_start__ as *const u8 as u32 };
    let region_end = unsafe { &__ram_nocache_end__ as *const u8 as u32 };
    let region_size = region_end - region_base;
    if region_size >= 32 {
        mpu_region_cfg(
            1,
            &MpuRegionConfig {
                region_base,
                region_size,
                xn: MPU_EXEC_ALLOW,
                ap: MPU_UN_PRIV_RW,
                sh: MPU_NON_SHAREABLE,
                attr_idx: MPU_MEM_ATTR_IDX_NC,
            },
        );
    }
}
