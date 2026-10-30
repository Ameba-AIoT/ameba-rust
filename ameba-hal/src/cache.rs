//! D-Cache and I-Cache maintenance helpers.
//! Mirrors the SDK cache API in ameba_cache.h (DCache_Enable/Disable/Clean/Invalidate etc.)
//!
//! ## D-Cache maintenance is a no-op when the cache is disabled
//!
//! `dcache_clean` / `dcache_invalidate` / `dcache_clean_invalidate` return early
//! when [`dcache_is_enabled`] is false: with D-Cache off there are no cache lines
//! to write back or discard. This mirrors the SDK, whose `DCache_Clean`,
//! `DCache_Invalidate` and `DCache_CleanInvalidate` all open with
//! `if (DCache_IsEnabled() == 0) { return; }` (`ameba_cache.h`). The early return
//! is deliberate, not a missing code path — keep it when editing these functions.

use cortex_m::peripheral::{Peripherals, SCB};

// SDK: ameba_cache.h : CACHE_LINE_ADDR_MSK — round down to 32-byte cache line boundary.
const CACHE_LINE_MASK: usize = !(32 - 1);

/// Returns `true` if D-Cache is currently enabled.
/// SDK: ameba_cache.h : DCache_IsEnabled
#[inline]
pub fn dcache_is_enabled() -> bool {
    SCB::dcache_enabled()
}

/// Enable D-Cache; no-op if already enabled.
/// SDK: ameba_cache.h : DCache_Enable
pub fn dcache_enable() {
    if !dcache_is_enabled() {
        unsafe {
            let mut cp = Peripherals::steal();
            cp.SCB.enable_dcache(&mut cp.CPUID);
        }
    }
}

/// Disable D-Cache (clean+invalidate then disable).
/// SDK: ameba_cache.h : DCache_Disable
pub fn dcache_disable() {
    unsafe {
        let mut cp = Peripherals::steal();
        cp.SCB.disable_dcache(&mut cp.CPUID);
    }
}

/// Enable I-Cache.
/// SDK: ameba_cache.h : ICache_Enable
pub fn icache_enable() {
    unsafe {
        Peripherals::steal().SCB.enable_icache();
    }
}

/// Disable I-Cache.
/// SDK: ameba_cache.h : ICache_Disable
pub fn icache_disable() {
    unsafe {
        Peripherals::steal().SCB.disable_icache();
    }
}

/// Invalidate entire I-Cache.
/// SDK: ameba_cache.h : ICache_Invalidate
pub fn icache_invalidate() {
    unsafe {
        Peripherals::steal().SCB.invalidate_icache();
    }
}

/// Clean (write-back) dirty cache lines covering `[ptr, ptr+len)` to SRAM.
/// No-op if D-Cache is disabled.
/// SDK: ameba_cache.h : DCache_Clean
///
/// Address is rounded down to the 32-byte cache-line boundary as the SDK does.
///
/// # Safety
/// `[ptr, ptr + len)` must lie within a valid, cacheable SRAM region. The
/// operation writes back whole cache lines, so the effective range is widened
/// to the enclosing 32-byte line boundaries — the caller must ensure that
/// widening does not clobber unrelated live data.
#[inline]
pub unsafe fn dcache_clean(ptr: *const u8, len: usize) {
    unsafe {
        if !dcache_is_enabled() {
            return;
        }
        let addr = ptr as usize & CACHE_LINE_MASK;
        let adjusted = (ptr as usize - addr) + len;
        Peripherals::steal()
            .SCB
            .clean_dcache_by_address(addr, adjusted);
    }
}

/// Invalidate (discard) cache lines covering `[ptr, ptr+len)`, forcing re-read from SRAM.
/// No-op if D-Cache is disabled.
/// SDK: ameba_cache.h : DCache_Invalidate
///
/// Address is rounded down to the 32-byte cache-line boundary as the SDK does.
///
/// # Safety
/// `[ptr, ptr + len)` must lie within a valid, cacheable SRAM region. Any dirty
/// lines in the (line-aligned, widened) range are discarded without write-back,
/// so the caller must ensure no unflushed data in those lines is still needed.
#[inline]
pub unsafe fn dcache_invalidate(ptr: *const u8, len: usize) {
    unsafe {
        if !dcache_is_enabled() {
            return;
        }
        let addr = ptr as usize & CACHE_LINE_MASK;
        let adjusted = (ptr as usize - addr) + len;
        Peripherals::steal()
            .SCB
            .invalidate_dcache_by_address(addr, adjusted);
    }
}

/// Clean then invalidate cache lines covering `[ptr, ptr+len)`.
/// No-op if D-Cache is disabled.
/// SDK: ameba_cache.h : DCache_CleanInvalidate
///
/// Address is rounded down to the 32-byte cache-line boundary as the SDK does.
///
/// # Safety
/// `[ptr, ptr + len)` must lie within a valid, cacheable SRAM region. The
/// operation writes back then discards whole cache lines, so the effective
/// range is widened to the enclosing 32-byte line boundaries — the caller must
/// ensure that widening does not affect unrelated live data.
#[inline]
pub unsafe fn dcache_clean_invalidate(ptr: *const u8, len: usize) {
    unsafe {
        if !dcache_is_enabled() {
            return;
        }
        let addr = ptr as usize & CACHE_LINE_MASK;
        let adjusted = (ptr as usize - addr) + len;
        Peripherals::steal()
            .SCB
            .clean_invalidate_dcache_by_address(addr, adjusted);
    }
}
