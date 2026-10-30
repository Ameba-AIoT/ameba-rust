use rtl8721f_pac::Interrupt as PacIrq;

/// Type-level interrupt tags — one ZST per hardware interrupt line.
///
/// Used as the `I` parameter in [`Handler<I>`] and [`Binding<I, H>`] to encode
/// at the type level which interrupt a handler / binding is associated with.
/// Every ZST also implements [`Irq`], which carries the PAC interrupt number.
#[allow(non_camel_case_types)]
pub mod typelevel {
    use rtl8721f_pac::Interrupt as PacIrq;

    macro_rules! irq_types {
        ($(($name:ident, $pac:expr_2021)),* $(,)?) => {
            $(
                #[derive(Copy, Clone)]
                pub struct $name;
                impl super::Irq for $name {
                    const PAC: PacIrq = $pac;
                }
            )*
        };
    }
    irq_types!(
        (IPC_CPU0_IRQ, PacIrq::IPC_CPU0_IRQ),
        (TIMER0_IRQ, PacIrq::TIMER0_IRQ),
        (TIMER1_IRQ, PacIrq::TIMER1_IRQ),
        (TIMER2_IRQ, PacIrq::TIMER2_IRQ),
        (TIMER3_IRQ, PacIrq::TIMER3_IRQ),
        (TIMER4_IRQ, PacIrq::TIMER4_IRQ),
        (TIMER5_IRQ, PacIrq::TIMER5_IRQ),
        (TIMER6_IRQ, PacIrq::TIMER6_IRQ),
        (TIMER7_IRQ, PacIrq::TIMER7_IRQ),
        (TIMER8_IRQ, PacIrq::TIMER8_IRQ),
        (UART0_IRQ, PacIrq::UART0_IRQ),
        (UART1_IRQ, PacIrq::UART1_IRQ),
        (UART2_IRQ, PacIrq::UART2_IRQ),
        (UART3_IRQ, PacIrq::UART3_IRQ),
        (SPI0_IRQ, PacIrq::SPI0_IRQ),
        (SPI1_IRQ, PacIrq::SPI1_IRQ),
        (I2C0_IRQ, PacIrq::I2C0_IRQ),
        (I2C1_IRQ, PacIrq::I2C1_IRQ),
        (GPIOA_IRQ, PacIrq::GPIOA_IRQ),
        (GPIOB_IRQ, PacIrq::GPIOB_IRQ),
        (GPIOC_IRQ, PacIrq::GPIOC_IRQ),
        (GDMA0_CHANNEL0_IRQ, PacIrq::GDMA0_CHANNEL0_IRQ),
        (GDMA0_CHANNEL1_IRQ, PacIrq::GDMA0_CHANNEL1_IRQ),
        (GDMA0_CHANNEL2_IRQ, PacIrq::GDMA0_CHANNEL2_IRQ),
        (GDMA0_CHANNEL3_IRQ, PacIrq::GDMA0_CHANNEL3_IRQ),
        (GDMA0_CHANNEL4_IRQ, PacIrq::GDMA0_CHANNEL4_IRQ),
        (GDMA0_CHANNEL5_IRQ, PacIrq::GDMA0_CHANNEL5_IRQ),
        (GDMA0_CHANNEL6_IRQ, PacIrq::GDMA0_CHANNEL6_IRQ),
        (GDMA0_CHANNEL7_IRQ, PacIrq::GDMA0_CHANNEL7_IRQ),
    );

    /// Placeholder IRQ for type-erased peripherals (AnyUart etc.) that require an
    /// `Irq` bound but are never bound to a real interrupt vector.
    #[derive(Copy, Clone)]
    pub struct IrqAny;
    impl super::Irq for IrqAny {
        const PAC: PacIrq = PacIrq::IPC_CPU0_IRQ; // value unused -- no Binding<IrqAny, _> exists
    }
}

/// Links a typelevel IRQ tag to its PAC [`Interrupt`](rtl8721f_pac::Interrupt) number.
///
/// Implemented by every ZST in [`typelevel`]. Peripheral instance traits
/// (`UartInstance`, `SpiInstance`, …) require `type IrqType: Irq` so that all
/// IRQ operations go through this module rather than calling NVIC directly.
pub trait Irq {
    const PAC: PacIrq;
}

/// Enable an interrupt in the NVIC.
///
/// The `B: Binding<I, H>` bound is a compile-time proof that the vector-table
/// entry for `I` already points to `H::on_interrupt()`. Passing this token is
/// the only way to reach this function, so the caller cannot forget to register
/// the handler first.
pub fn enable<I: Irq, H: Handler<I>, B: Binding<I, H>>() {
    // SAFETY: Binding<I, H> guarantees the IRQ vector is wired to H::on_interrupt().
    unsafe { cortex_m::peripheral::NVIC::unmask(I::PAC) }
}

/// Disable (mask) an interrupt in the NVIC.
///
/// The symmetric counterpart to [`enable`]. Unlike `enable`, no [`Binding`] token
/// is required: masking a line is always sound — it can only *prevent* a handler
/// from running, never invoke an unregistered one.
///
/// This masks the NVIC line for the whole peripheral. It is therefore intended for
/// the async **handle** teardown (mirroring the single `enable` in `into_async`),
/// **not** for per-transfer future `Drop` — a future cancelled mid-way must clear
/// only its *peripheral-level* interrupt-enable bits so that later transfers on the
/// same handle keep working. Masking the shared NVIC line per-transfer would stall
/// every subsequent transfer.
pub fn disable<I: Irq>() {
    cortex_m::peripheral::NVIC::mask(I::PAC)
}

/// Set the NVIC priority for interrupt `I`.
///
/// `priority` is a 0-based level in the HAL convention (0 = highest).
/// The value is shifted to fill the top `NVIC_PRIO_BITS` of the 8-bit field,
/// matching CMSIS / SDK `irq_set_priority` behaviour.
///
/// # Safety
/// Must be called before `enable` for that interrupt. Changing priority while
/// the interrupt is pending may cause a spurious firing.
pub unsafe fn set_priority<I: Irq>(priority: u32) {
    unsafe {
        cortex_m::Peripherals::steal().NVIC.set_priority(
            I::PAC,
            (priority as u8) << (8 - rtl8721f_pac::NVIC_PRIO_BITS),
        );
    }
}

/// Peripheral interrupt handler. Implemented by HAL driver handler types.
///
/// # Safety contract for implementors
/// `on_interrupt` must be safe to call from the hardware interrupt vector for `I`.
/// It must not block, panic, or re-enable the same interrupt before returning.
pub trait Handler<I> {
    /// Body of the interrupt handler. Called in interrupt context.
    ///
    /// # Safety
    /// Must only be invoked from the correct hardware interrupt vector for `I`.
    unsafe fn on_interrupt();
}

/// Binding proof: implemented by the token type generated by [`bind_interrupts!`].
///
/// Proves that:
/// - The vector-table entry for interrupt `I` calls `H::on_interrupt()`.
///
/// # Safety
/// Must only be implemented when the above condition holds — i.e., when the
/// `#[no_mangle]` IRQ function for `I` is present in the binary and delegates
/// to `H::on_interrupt()`.
pub unsafe trait Binding<I, H: Handler<I>> {}

/// Helper: proves that GDMA channel `CH` is bound to `dma::InterruptHandler<CH>`.
///
/// Automatically derived from any `Binding<GDMA0_CHANNELn_IRQ, dma::InterruptHandler<n>>`
/// via the blanket impls below, so callers only need `T: DmaChannelBinding<CH>` as a bound.
///
/// # Safety
/// Must only be implemented when the GDMA0_CHANNELn_IRQ handler calls
/// `dma::InterruptHandler<CH>::on_interrupt()`.
pub unsafe trait DmaChannelBinding<const CH: u8> {}
