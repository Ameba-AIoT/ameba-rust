/// Bind interrupt vectors to HAL handler types and generate a compile-time proof token.
///
/// ## Usage
///
/// ```rust
/// use ameba_hal::bind_interrupts;
/// use ameba_hal::uart;
/// use ameba_hal::dma;
///
/// bind_interrupts!(struct Irqs {
///     UART0_IRQ => uart::InterruptHandler<uart::Uart0>;
///     GDMA0_CHANNEL0_IRQ => dma::InterruptHandler<0>;
/// });
/// ```
///
/// This generates:
/// - A `Copy + Clone` struct `Irqs`.
/// - One `#[unsafe(no_mangle)] unsafe extern "C"` IRQ function per entry that calls the
///   corresponding `Handler::on_interrupt()`. This function IS the vector-table
///   entry for that interrupt — it overrides `DefaultHandler`.
/// - `unsafe impl Binding<typelevel::IRQ_NAME, HandlerType> for Irqs` for each entry.
///
/// The token (e.g. `Irqs`) is then passed to driver constructors such as
/// `uart.into_async(Irqs)`, which call [`crate::interrupt::enable`] internally
/// knowing the handler is in place.
#[macro_export]
macro_rules! bind_interrupts {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident {
            $($irq:ident => $handler:ty;)*
        }
    ) => {
        $(#[$outer])*
        #[derive(Copy, Clone)]
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            unsafe extern "C" fn $irq() {
                <$handler as $crate::interrupt::Handler<
                    $crate::interrupt::typelevel::$irq
                >>::on_interrupt();
            }

            // SAFETY: the #[unsafe(no_mangle)] function above IS the vector-table entry for $irq,
            // and it calls H::on_interrupt() as required by Binding's contract.
            unsafe impl $crate::interrupt::Binding<
                $crate::interrupt::typelevel::$irq,
                $handler,
            > for $name {}
        )*
    };
}

/// Generate a type-erased `AnyFoo<'d>` wrapper for a set of peripheral singletons.
///
/// Expands to:
/// - `pub mod any` with `Inner` enum, `delegate!` helper, and `Degrade` trait
/// - `pub struct $name<'d>` — the type-erased wrapper
/// - `From<ConcreteN> for $name` and `TryFrom<$name> for ConcreteN` for each variant
#[macro_export]
macro_rules! any_peripheral {
    (
        $(#[$meta:meta])* $vis:vis peripheral $name:ident<'d> {
            $(
                $(#[cfg($variant_cfg:meta)])*
                $variant:ident($inner:ty)
            ),* $(,)?
        }
    ) => {
        #[doc(hidden)]
        pub mod any {
            #[allow(unused_imports)]
            use super::*;

            /// Delegate a method call to the active concrete variant.
            ///
            /// Usage: `any::delegate!(self, inner => { inner.method() })`
            #[allow(unused_macros)]
            macro_rules! delegate {
                ($any:ident, $inner_ident:ident => $code:tt) => {
                    match &$any.__inner {
                        $(
                            $(#[cfg($variant_cfg)])*
                            any::Inner::$variant($inner_ident) => $code,
                        )*
                    }
                }
            }
            #[allow(unused_imports)]
            pub(crate) use delegate;

            #[derive(Debug)]
            pub(crate) enum Inner {
                $(
                    $(#[cfg($variant_cfg)])*
                    $variant($inner),
                )*
            }

            /// Implemented by concrete peripheral tokens and by the `AnyFoo` wrapper itself.
            pub trait Degrade: Sized + $crate::private::Sealed {
                fn degrade<'a>(self) -> super::$name<'a>
                where
                    Self: 'a;
            }
        }

        $(#[$meta])*
        #[derive(Debug)]
        $vis struct $name<'d> {
            pub(crate) __inner: any::Inner,
            pub(crate) __phantom: core::marker::PhantomData<&'d ()>,
        }

        impl $crate::private::Sealed for $name<'_> {}

        impl<'d> any::Degrade for $name<'d> {
            #[inline]
            fn degrade<'a>(self) -> $name<'a>
            where
                Self: 'a,
            {
                self
            }
        }

        $(
            $(#[cfg($variant_cfg)])*
            impl any::Degrade for $inner {
                #[inline]
                fn degrade<'a>(self) -> $name<'a>
                where
                    Self: 'a,
                {
                    $name::from(self)
                }
            }

            $(#[cfg($variant_cfg)])*
            impl<'d> From<$inner> for $name<'d> {
                #[inline]
                fn from(inner: $inner) -> Self {
                    Self {
                        __inner: any::Inner::$variant(inner),
                        __phantom: core::marker::PhantomData,
                    }
                }
            }

            $(#[cfg($variant_cfg)])*
            impl<'d> TryFrom<$name<'d>> for $inner {
                type Error = $name<'d>;
                #[inline]
                fn try_from(any_peri: $name<'d>) -> Result<Self, $name<'d>> {
                    let $name { __inner: inner, __phantom } = any_peri;
                    match inner {
                        any::Inner::$variant(val) => Ok(val),
                        other => Err($name { __inner: other, __phantom }),
                    }
                }
            }
        )*
    };
}
