//! Transport abstraction for the OTA download source.
//!
//! The core state machine ([`crate::core`]) never talks to a network stack
//! or a filesystem directly — it only pulls bytes from whatever implements
//! [`Transport`]. This mirrors how the C SDK separates `ota_transport_read()`
//! from `ota_core.c`'s programming loop (SDK: `ota_internal.h` declares the
//! `ota_transport_*` family independently of the core), and keeps the crate
//! usable with HTTP, a local filesystem, or a caller-driven byte feed
//! without the core knowing which.
//!
//! The trait is deliberately narrower than `embedded_io_async::Read`: an
//! OTA source is read once, front to back, and never written to or seeked.

/// A sequential, read-once source of OTA image bytes.
pub trait Transport {
    /// Transport-specific failure (socket error, HTTP status, short read…).
    type Error;

    /// Fill as much of `buf` as is currently available, returning the byte
    /// count. A return of `Ok(0)` means end of stream — the transport has no
    /// more bytes and will not produce any later.
    ///
    /// Implementations must not busy-wait: this is the point where an async
    /// transport yields to the executor while the next packet arrives.
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> impl core::future::Future<Output = Result<usize, Self::Error>>;
}
