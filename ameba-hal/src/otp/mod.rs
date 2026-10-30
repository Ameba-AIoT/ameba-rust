//! OTP / eFuse physical-map reader.
//!
//! De-ROM'd port of the chip ROM `OTP_Read8` (`sdk/.../rom/rom_common/
//! ameba_otp_rom.c : _OTP_Read8_Basic`). The OTPC is an indirect controller:
//! a read is a handshake through the `OTP_AS` access register, not a passive
//! shadow read.
//!
//! The `otpc_reg::RegisterBlock` now includes `otp_as` / `otp_as_sec` /
//! `otp_ctrl` fields at the correct offsets (0x008, 0x00C, 0x014) so the PAC
//! `OtpcReg::PTR` provides full type-safe access.

use embedded_hal::delay::DelayNs;
use rtl8721f_pac::{OtpcReg, otpc_reg};

#[inline(always)]
fn regs() -> &'static otpc_reg::RegisterBlock {
    unsafe { &*OtpcReg::PTR }
}

// ── General OTP constants ───────────────────────────────────────────────

/// Highest valid OTP physical byte address (exclusive).
/// SDK: ameba_otpc.h : `OTP_REAL_CONTENT_LEN`.
const OTP_REAL_CONTENT_LEN: u32 = 0x800;

/// Poll iterations before giving up (5 µs each → ~100 ms worst case).
/// SDK: ameba_otpc.h : `OTP_POLL_TIMES`.
const OTP_POLL_TIMES: u32 = 20000;

/// User read mode. SDK: ameba_otpc.h : `OTP_USER_MODE`.
const OTP_USER_MODE: u8 = 0;

/// OTP physical address of the chip cut version.
/// SDK: ameba_chipinfo_rom.h : `OTP_CHIPVER`.
const OTP_CHIPVER: u32 = 0x7F0;

// ── Public API ──────────────────────────────────────────────────────────

/// Read one OTP byte at physical `addr`.
///
/// Returns `0xFF` (the blank/erased value) on out-of-range addresses or a
/// poll timeout, matching the ROM's failure contract so callers can treat
/// `0xFF` as "no trim programmed".
pub fn read8(addr: u32) -> u8 {
    if addr >= OTP_REAL_CONTENT_LEN {
        return 0xFF;
    }

    // 1. OTP_AccessCmd(DISABLE): clear the program password so a plain
    //    read is armed.  SDK: ameba_otpc.c `OTP_AccessCmd(DISABLE)`.
    regs()
        .otp_ctrl()
        .modify(|_, w| unsafe { w.ef_pg_pwd().bits(0) });

    // 2. Trigger a read: write address + mode (EF_RD_WR defaults to 0).
    regs().otp_as().write(|w| unsafe {
        w.ef_addr()
            .bits(addr as u16)
            .ef_mode_sel()
            .bits(OTP_USER_MODE)
    });

    // 3. Poll EF_RD_WR until hardware sets it (data ready).
    let mut delay = crate::delay::Delay::new();
    let mut idx = 0u32;
    while idx < OTP_POLL_TIMES && regs().otp_as().read().ef_rd_wr().bit_is_clear() {
        delay.delay_us(5);
        idx += 1;
    }

    // 4. Read the data byte.
    if idx < OTP_POLL_TIMES {
        regs().otp_as().read().ef_data().bits()
    } else {
        0xFF
    }
}

/// Chip cut version (0 = A, 1 = B, ...).
///
/// De-ROM'd port of ROM `EFUSE_GetChipVersion`
/// (`OTP_Read8(OTP_CHIPVER) & 0x1F`).
pub fn chip_version() -> u8 {
    read8(OTP_CHIPVER) & 0x1F
}
