//! Debug timer — free-running monotonic microsecond counter.
//!
//! The debug timer is a free-running 64-bit up-counter clocked at 1 MHz. It is
//! enabled by hardware reset default (`DBGT_CTRL.ENABLE` reset value = 1), so no
//! init is required — the low 32 bits (`DBGT_CRV0`) simply increment once per
//! microsecond and wrap every ~71.6 minutes.
//!
//! This is the PAC-register reimplementation of the ROM `DTimestamp_Get()`
//! inline (`return DTIMER_DEV->DBGT_CRV0;`), used by drivers for wrapping-safe
//! busy-wait timeout polling.

use rtl8721f_pac::{DebugtimerReg, debugtimer_reg};

#[inline(always)]
fn regs() -> &'static debugtimer_reg::RegisterBlock {
    unsafe { &*DebugtimerReg::PTR }
}

/// Read the free-running debug-timer counter (low 32 bits, 1 µs per tick).
///
/// Monotonic modulo `u32::MAX`; use `wrapping_sub` on two samples to measure
/// elapsed microseconds. Equivalent to the ROM `DTimestamp_Get()`.
#[inline(always)]
pub fn timestamp() -> u32 {
    regs().dbgt_crv0().read().bits()
}
