//! D-Cache maintenance helpers.
//!
//! Thin `usize`-address forwarding layer over [`ameba_hal::cache`], which owns
//! the single implementation (cache-line-aligned range widening matching the
//! SDK's `ameba_cache.h`). Keeping the WiFi call sites on `usize` addresses
//! avoids churning ~35 IPC/TRX sites that already carry raw addresses.

/// Invalidate (discard) cache lines covering `[addr, addr + len)`, forcing a
/// re-read from SRAM.
///
/// # Safety
/// `[addr, addr + len)` must lie within a valid, cacheable SRAM region. Any
/// dirty lines in the (line-aligned, widened) range are discarded without
/// write-back, so the caller must ensure no unflushed data in those lines is
/// still needed.
#[inline]
pub unsafe fn invalidate(addr: usize, len: usize) {
    unsafe {
        ameba_hal::cache::dcache_invalidate(addr as *const u8, len);
    }
}

/// Clean (write-back) dirty cache lines covering `[addr, addr + len)` to SRAM.
///
/// # Safety
/// `[addr, addr + len)` must lie within a valid, cacheable SRAM region. The
/// operation writes back whole cache lines, so the effective range is widened
/// to the enclosing 32-byte line boundaries — the caller must ensure that
/// widening does not clobber unrelated live data.
#[inline]
pub unsafe fn clean(addr: usize, len: usize) {
    unsafe {
        ameba_hal::cache::dcache_clean(addr as *const u8, len);
    }
}
