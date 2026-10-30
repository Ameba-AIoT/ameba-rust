//! Driver operating-mode markers — blocking (polling) vs interrupt-driven async.
//!
//! A peripheral driver is generic over one of these zero-sized markers (e.g.
//! `Uart<P, Blocking>` / `Uart<P, Async>`) and converts between them with
//! `into_async()` / `into_blocking()`. Keeping the mode as a *type parameter*
//! (rather than two separate driver structs) means blocking-only and
//! async-only methods are gated at compile time on the same handle, matching
//! the chip-vendor HAL convention.
//!
//! The [`dma`](crate::dma) module re-exports these same markers so the whole
//! HAL shares one definition of "blocking" and "async".

mod sealed {
    pub trait Sealed {}
}

/// Marker trait for a driver operating mode.
///
/// Sealed: only [`Blocking`] and [`Async`] implement it, so a driver's mode
/// type parameter can never be instantiated with anything else.
#[allow(private_bounds)]
pub trait DriverMode: sealed::Sealed {
    /// `true` for [`Async`], `false` for [`Blocking`].
    ///
    /// A driver `Drop` impl covers `<P, Dm>` uniformly (Rust forbids a `Drop`
    /// specialised to one mode), so it guards its interrupt teardown with
    /// `if Dm::IS_ASYNC`. Because this is an associated `const`, monomorphising
    /// the `Blocking` instance folds the guard to `if false` and the teardown
    /// is eliminated — so dropping a `Blocking` handle (including the many
    /// short-lived `steal()` handles the async/DMA code creates internally)
    /// costs nothing and never touches interrupt state.
    const IS_ASYNC: bool;
}

/// Blocking (polling) driver mode. The default mode for every driver.
pub struct Blocking;

/// Interrupt-driven async driver mode.
pub struct Async;

impl sealed::Sealed for Blocking {}
impl sealed::Sealed for Async {}
impl DriverMode for Blocking {
    const IS_ASYNC: bool = false;
}
impl DriverMode for Async {
    const IS_ASYNC: bool = true;
}
