use crate::interrupt::{Binding, Handler};
use core::marker::PhantomData;
use core::sync::atomic::{AtomicUsize, Ordering};

use bitflags::bitflags;
use rtl8721f_pac::timer0_reg;

// ────────────────────────────────────────────────────────────────────────────
// Raw ISR callback (register_irq) — mirrors ameba_hal::uart::Uart::register_irq
// ────────────────────────────────────────────────────────────────────────────

/// Raw timer interrupt event, decoded from the SR snapshot taken by
/// `dispatch_irq` — already read-and-cleared (W1C) by the time this is
/// delivered, mirroring UART/SPI's "already consumed" convention.
#[derive(Clone, Copy, Debug, Default)]
pub struct TimIrqEvent {
    pub flags: TimFlag,
}

/// User-registered raw ISR callback for a timer instance. Runs directly in
/// the timer's NVIC interrupt context. Register via [`Tim::register_irq`].
pub trait TimIrqHandler: Send {
    fn on_irq(&mut self, event: TimIrqEvent);
}

/// Type-erased trampoline stored in `TIM_STATE::irq_handler`, resolved back to
/// `H::on_irq` via the matching `irq_data` pointer. Mirrors GDMA's
/// `gdma_handler_trampoline`.
unsafe fn tim_handler_trampoline<H: TimIrqHandler>(data: usize, event: TimIrqEvent) {
    let handler = unsafe { &mut *(data as *mut H) };
    handler.on_irq(event);
}

struct TimState {
    irq_handler: AtomicUsize, // unsafe fn(usize, TimIrqEvent) as usize; 0 = none registered
    irq_data: AtomicUsize,    // *mut H as usize, paired with irq_handler
}

impl TimState {
    const fn new() -> Self {
        Self {
            irq_handler: AtomicUsize::new(0),
            irq_data: AtomicUsize::new(0),
        }
    }
}

static TIM_STATE: [TimState; 9] = [
    TimState::new(),
    TimState::new(),
    TimState::new(),
    TimState::new(),
    TimState::new(),
    TimState::new(),
    TimState::new(),
    TimState::new(),
    TimState::new(),
];

fn dispatch_user_irq(idx: usize, event: TimIrqEvent) {
    let state = &TIM_STATE[idx];
    let handler_fn = state.irq_handler.load(Ordering::Acquire);
    if handler_fn == 0 {
        return;
    }
    let data = state.irq_data.load(Ordering::Acquire);
    let trampoline: unsafe fn(usize, TimIrqEvent) = unsafe { core::mem::transmute(handler_fn) };
    unsafe { trampoline(data, event) };
}

// Called from each named TIMERn_IRQ handler defined below. Snapshots and
// clears SR (W1C, with the 32kHz cross-domain sync poll), then hands the
// decoded flags to the registered callback, if any.
// SDK: ameba_tim_rom.c : RTIM_INTClear, TIMER_POLLING_TIMES = 4000
#[inline(always)]
unsafe fn dispatch_irq(idx: usize, regs: &timer0_reg::RegisterBlock) {
    let sr = regs.tim_sr().read().bits();
    unsafe { regs.tim_sr().as_ptr().write_volatile(sr) };
    for _ in 0..4000u32 {
        if regs.tim_sr().read().bits() & 0x00FF_FFFF == 0 {
            break;
        }
    }
    dispatch_user_irq(
        idx,
        TimIrqEvent {
            flags: TimFlag::from_bits_truncate(sr),
        },
    );
}

// ────────────────────────────────────────────────────────────────────────────
// Interrupt handler type
// ────────────────────────────────────────────────────────────────────────────

/// Embassy-style interrupt handler for a typed timer instance.
///
/// Register it via [`crate::bind_interrupts!`], then pass the resulting token
/// to [`Tim::register_irq`]:
///
/// ```rust,ignore
/// bind_interrupts!(struct Irqs {
///     TIMER0_IRQ => tim::InterruptHandler<Timer0Reg>;
/// });
///
/// let tim = TimBuilder::new(pac).period(239_999).init();
/// tim.register_irq(my_handler, Irqs);
/// ```
pub struct InterruptHandler<P: TimInstance>(PhantomData<P>);

impl<P: TimInstance> Handler<P::IrqType> for InterruptHandler<P> {
    unsafe fn on_interrupt() {
        unsafe {
            dispatch_irq(P::IDX, P::regs());
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Type states
// ────────────────────────────────────────────────────────────────────────────

pub struct Basic;
pub struct Pwm;
pub struct Inpulse;

// ────────────────────────────────────────────────────────────────────────────
// Sealed trait + TimInstance
// ────────────────────────────────────────────────────────────────────────────

mod sealed {
    pub trait Sealed {}
}

/// Core trait implemented by each of the 9 timer peripheral types.
/// `DefaultMode` encodes the hardware capability (Basic/Pwm/Inpulse).
pub trait TimInstance: sealed::Sealed {
    const IDX: usize;
    /// true for PWM/Inpulse timers (4-8) which have a functional prescaler;
    /// false for Basic timers (0-3) which ignore the prescaler (hardware forces PSC=0).
    /// SDK: ameba_tim_rom.c : RTIM_TimeBaseInit, IS_TIM_CCM_TIM
    const IS_CCM: bool;
    /// RCC clock/function gate for this timer instance.
    /// SDK: ameba_tim.c : APBPeriph_TIMx / APBPeriph_TIMx_CLOCK arrays
    const RCC: crate::rcc::Peripheral;
    /// The natural mode for this timer (Basic, Pwm, or Inpulse).
    type DefaultMode;
    /// Typelevel IRQ tag used with [`crate::interrupt::Binding`].
    type IrqType: crate::interrupt::Irq;
    fn regs() -> &'static timer0_reg::RegisterBlock;
    /// Steal this peripheral instance without going through `Peripherals::take()`.
    ///
    /// # Safety
    /// Caller must guarantee the peripheral is not already owned by another `Tim`.
    unsafe fn steal() -> Self;
}

pub trait BasicTimInst: TimInstance<DefaultMode = Basic> {}
pub trait PwmTimInst: TimInstance<DefaultMode = Pwm> {}
pub trait InpulseTimInst: TimInstance<DefaultMode = Inpulse> {}

// ────────────────────────────────────────────────────────────────────────────
// Macro: instantiate the 9 timer impls
// All 9 PAC RegisterBlocks have identical layout; cast to timer0_reg::RegisterBlock.
// ────────────────────────────────────────────────────────────────────────────

macro_rules! impl_tim {
    ($pac_type:ty, $pac_field:ident, $irq_type:ty, $rcc:expr_2021, $is_ccm:literal, $idx:literal, $kind:ident, $mode:ty) => {
        impl sealed::Sealed for $pac_type {}
        impl TimInstance for $pac_type {
            const IDX: usize = $idx;
            const IS_CCM: bool = $is_ccm;
            const RCC: crate::rcc::Peripheral = $rcc;
            type DefaultMode = $mode;
            type IrqType = $irq_type;
            #[inline(always)]
            fn regs() -> &'static timer0_reg::RegisterBlock {
                unsafe { &*(<$pac_type>::PTR as *const timer0_reg::RegisterBlock) }
            }
            #[inline(always)]
            unsafe fn steal() -> Self {
                unsafe { crate::pac::Peripherals::steal().$pac_field }
            }
        }
        impl $kind for $pac_type {}
    };
}

use rtl8721f_pac::{
    Timer0Reg, Timer1Reg, Timer2Reg, Timer3Reg, Timer4Reg, Timer5Reg, Timer6Reg, Timer7Reg,
    Timer8Reg,
};

use crate::rcc::Peripheral;
// SDK: ameba_tim.c : APBPeriph_TIMx[] / APBPeriph_TIMx_CLOCK[] arrays
impl_tim!(
    Timer0Reg,
    timer0_reg,
    crate::interrupt::typelevel::TIMER0_IRQ,
    Peripheral::LTIM0,
    false,
    0,
    BasicTimInst,
    Basic
);
impl_tim!(
    Timer1Reg,
    timer1_reg,
    crate::interrupt::typelevel::TIMER1_IRQ,
    Peripheral::LTIM1,
    false,
    1,
    BasicTimInst,
    Basic
);
impl_tim!(
    Timer2Reg,
    timer2_reg,
    crate::interrupt::typelevel::TIMER2_IRQ,
    Peripheral::LTIM2,
    false,
    2,
    BasicTimInst,
    Basic
);
impl_tim!(
    Timer3Reg,
    timer3_reg,
    crate::interrupt::typelevel::TIMER3_IRQ,
    Peripheral::LTIM3,
    false,
    3,
    BasicTimInst,
    Basic
);
// Timer4 (PWM0) is reserved by default (feature "time-driver-tim4") as the embassy-time driver.
// Switch to time-driver-tim5/6/7 to free Timer4; see ameba-hal/Cargo.toml [features].
impl_tim!(
    Timer4Reg,
    timer4_reg,
    crate::interrupt::typelevel::TIMER4_IRQ,
    Peripheral::PWM0,
    true,
    4,
    PwmTimInst,
    Pwm
);
impl_tim!(
    Timer5Reg,
    timer5_reg,
    crate::interrupt::typelevel::TIMER5_IRQ,
    Peripheral::PWM1,
    true,
    5,
    PwmTimInst,
    Pwm
);
impl_tim!(
    Timer6Reg,
    timer6_reg,
    crate::interrupt::typelevel::TIMER6_IRQ,
    Peripheral::PWM2,
    true,
    6,
    PwmTimInst,
    Pwm
);
impl_tim!(
    Timer7Reg,
    timer7_reg,
    crate::interrupt::typelevel::TIMER7_IRQ,
    Peripheral::PWM3,
    true,
    7,
    PwmTimInst,
    Pwm
);
impl_tim!(
    Timer8Reg,
    timer8_reg,
    crate::interrupt::typelevel::TIMER8_IRQ,
    Peripheral::PWM4,
    true,
    8,
    InpulseTimInst,
    Inpulse
);

// ────────────────────────────────────────────────────────────────────────────
// bitflags types
// ────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// Interrupt enable bits (DIER register)
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TimIt: u32 {
        const UIE   = 1 << 0;
        const CC0IE = 1 << 1;
        const CC1IE = 1 << 2;
        const CC2IE = 1 << 3;
        const CC3IE = 1 << 4;
        const UIE0  = 1 << 16;
        const UIE1  = 1 << 17;
        const UIE2  = 1 << 18;
        const UIE3  = 1 << 19;
    }
}

bitflags! {
    /// Status flags (SR register, RW1C)
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct TimFlag: u32 {
        const UIF   = 1 << 0;
        const CC0IF = 1 << 1;
        const CC1IF = 1 << 2;
        const CC2IF = 1 << 3;
        const CC3IF = 1 << 4;
        const UIF0  = 1 << 16;
        const UIF1  = 1 << 17;
        const UIF2  = 1 << 18;
        const UIF3  = 1 << 19;
    }
}

bitflags! {
    /// Software event generation (EGR register)
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TimEvent: u32 {
        const UG   = 1 << 0;
        const CC0G = 1 << 1;
        const CC1G = 1 << 2;
        const CC2G = 1 << 3;
        const CC3G = 1 << 4;
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Enums
// ────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateSource {
    Global,
    Overflow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgePolarity {
    Rising = 0,
    Falling = 1,
    BothEdge = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    Ch0 = 0,
    Ch1 = 1,
    Ch2 = 2,
    Ch3 = 3,
}

// ────────────────────────────────────────────────────────────────────────────
// Builder
// ────────────────────────────────────────────────────────────────────────────

pub struct TimBuilder<P: TimInstance> {
    _pac: P,
    prescaler: u16,
    period: u32,
}

impl<P: TimInstance> TimBuilder<P> {
    pub fn new(pac: P) -> Self {
        TimBuilder {
            _pac: pac,
            prescaler: 0,
            period: 0xFFFF,
        }
    }

    pub fn prescaler(mut self, psc: u16) -> Self {
        self.prescaler = psc;
        self
    }

    pub fn period(mut self, arr: u32) -> Self {
        self.period = arr;
        self
    }

    /// Initialize the timer. The mode is determined by the peripheral type
    /// (Basic/Pwm/Inpulse). Use [`Tim::register_irq`] afterwards to hook up a
    /// raw ISR callback.
    pub fn init(self) -> Tim<P, P::DefaultMode> {
        init_timer(self._pac, self.prescaler, self.period)
    }
}

fn init_timer<P: TimInstance, Mode>(pac: P, prescaler: u16, period: u32) -> Tim<P, Mode> {
    // SDK: ameba_timer_test.c : RCC_PeriphClockCmd(APBPeriph_TIMx[idx], ...)
    crate::rcc::enable(P::RCC);

    let regs = P::regs();

    // Disable counter, clear DIER, clear SR before reconfiguring.
    // SDK: ameba_tim_rom.c : RTIM_TimeBaseInit (reset sequence)
    if regs.tim_en().read().cen().bit_is_set() {
        regs.tim_en().write(|w| w.cnt_stop().set_bit());
        while regs.tim_en().read().cen().bit_is_set() {}
    }
    regs.tim_dier().write(|w| unsafe { w.bits(0) });
    // SR is W1C; PAC marks it read-only so a raw write is required.
    let sr = regs.tim_sr().read().bits();
    unsafe { regs.tim_sr().as_ptr().write_volatile(sr) };
    // Poll until SR clears (32kHz cross-domain sync).
    // SDK: ameba_tim_rom.c : RTIM_INTClear, TIMER_POLLING_TIMES = 4000
    for _ in 0..4000u32 {
        if regs.tim_sr().read().bits() & 0x00FF_FFFF == 0 {
            break;
        }
    }

    // ARR width is instance-dependent hardware: basic timers (0-3) have a
    // 32-bit auto-reload (SDK TIMBasic_ARR, mask 0xFFFF_FFFF), CCM/PWM timers
    // (4-8) only 16 bits (SDK TIM_ARR, mask 0xFFFF) — same register offset,
    // different physical width. The SVD can only model one field per register,
    // so the PAC exposes a single 16-bit `arr` field for every timer; the
    // basic-timer high word has no named accessor and must be written raw
    // (the register spec is u32 and ARR is its only field).
    // SDK: ameba_pwmtimer.h : TIMBasic_ARR vs TIM_ARR / ameba_tim_rom.c : RTIM_TimeBaseInit
    if P::IS_CCM {
        regs.tim_arr()
            .write(|w| unsafe { w.arr().bits(period as u16) });
    } else {
        unsafe { regs.tim_arr().as_ptr().write_volatile(period) };
    }

    // PSC: functional only on CCM timers (Timer4-8); basic timers (0-3) hardware
    // ignores the prescaler and the SDK forces PSC=0 for them.
    // SDK: ameba_tim_rom.c : RTIM_TimeBaseInit, IS_TIM_CCM_TIM
    let psc = if P::IS_CCM { prescaler } else { 0 };
    regs.tim_psc().write(|w| unsafe { w.psc().bits(psc) });

    // CR defaults: ARPE=0 (ARR updates immediately), URS=0 (global), UDIS=0 (UEV enabled)
    regs.tim_cr()
        .modify(|_, w| w.arpe().clear_bit().urs().clear_bit().udis().clear_bit());

    // Trigger UG to reload PSC and ARR immediately, reset counter.
    // SDK: ameba_tim_rom.c : RTIM_TimeBaseInit — unbounded poll; ug_done asserts
    // ~71.936 µs (2×32kHz cycles) after EGR write for all timer types.
    regs.tim_egr().write(|w| w.ug().set_bit());
    while !regs.tim_sr().read().ug_done().bit_is_set() {}
    // Clear flags raised by the UG event.
    let sr = regs.tim_sr().read().bits();
    unsafe { regs.tim_sr().as_ptr().write_volatile(sr) };

    Tim {
        _pac: pac,
        _mode: PhantomData,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Timer struct
// ────────────────────────────────────────────────────────────────────────────

pub struct Tim<P: TimInstance, Mode> {
    _pac: P,
    _mode: PhantomData<Mode>,
}

// ────────────────────────────────────────────────────────────────────────────
// Common methods (all modes)
// ────────────────────────────────────────────────────────────────────────────

impl<P: TimInstance, Mode> Tim<P, Mode> {
    fn regs(&self) -> &'static timer0_reg::RegisterBlock {
        P::regs()
    }

    pub fn start(&self) {
        let regs = self.regs();
        if !regs.tim_en().read().cen().bit_is_set() {
            regs.tim_en().write(|w| w.cnt_start().set_bit());
            while !regs.tim_en().read().cen().bit_is_set() {}
        }
    }

    pub fn stop(&self) {
        let regs = self.regs();
        if regs.tim_en().read().cen().bit_is_set() {
            regs.tim_en().write(|w| w.cnt_stop().set_bit());
            while regs.tim_en().read().cen().bit_is_set() {}
        }
    }

    /// Returns the current counter value (16-bit hardware CNT register).
    pub fn get_count(&self) -> u32 {
        self.regs().tim_cnt().read().cnt().bits() as u32
    }

    pub fn enable_interrupt(&self, it: TimIt) {
        self.regs()
            .tim_dier()
            .modify(|r, w| unsafe { w.bits(r.bits() | it.bits()) });
    }

    pub fn disable_interrupt(&self, it: TimIt) {
        self.regs()
            .tim_dier()
            .modify(|r, w| unsafe { w.bits(r.bits() & !it.bits()) });
    }

    /// Clear all interrupt flags and wait for the 32kHz cross-domain sync to complete.
    /// SDK: ameba_tim_rom.c : RTIM_INTClear, TIMER_POLLING_TIMES = 4000
    pub fn clear_all_interrupt_flags(&self) {
        let regs = self.regs();
        let sr = regs.tim_sr().read().bits();
        unsafe { regs.tim_sr().as_ptr().write_volatile(sr) };
        for _ in 0..4000u32 {
            if regs.tim_sr().read().bits() & 0x00FF_FFFF == 0 {
                break;
            }
        }
    }

    pub fn get_flags(&self) -> TimFlag {
        let bits = self.regs().tim_sr().read().bits();
        TimFlag::from_bits_truncate(bits)
    }

    pub fn clear_flags(&self, flags: TimFlag) {
        unsafe { self.regs().tim_sr().as_ptr().write_volatile(flags.bits()) }
    }

    pub fn generate_event(&self, events: TimEvent) {
        self.regs()
            .tim_egr()
            .write(|w| unsafe { w.bits(events.bits()) });
    }

    pub fn set_arr_preload(&self, enabled: bool) {
        if enabled {
            self.regs().tim_cr().modify(|_, w| w.arpe().set_bit());
        } else {
            self.regs().tim_cr().modify(|_, w| w.arpe().clear_bit());
        }
    }

    pub fn set_update_disabled(&self, disabled: bool) {
        if disabled {
            self.regs().tim_cr().modify(|_, w| w.udis().set_bit());
        } else {
            self.regs().tim_cr().modify(|_, w| w.udis().clear_bit());
        }
    }

    pub fn set_update_source(&self, src: UpdateSource) {
        match src {
            UpdateSource::Overflow => {
                self.regs().tim_cr().modify(|_, w| w.urs().set_bit());
            }
            UpdateSource::Global => {
                self.regs().tim_cr().modify(|_, w| w.urs().clear_bit());
            }
        }
    }

    pub fn set_prescaler(&self, psc: u16) {
        self.regs()
            .tim_psc()
            .write(|w| unsafe { w.psc().bits(psc) });
        // Apply immediately via UG event
        self.regs().tim_egr().write(|w| w.ug().set_bit());
    }

    pub fn set_period(&self, period: u16) {
        self.regs()
            .tim_arr()
            .write(|w| unsafe { w.arr().bits(period) });
    }

    /// Set period and apply immediately (clears ARPE, triggers UG, restores ARPE).
    pub fn set_period_immediate(&self, period: u16) {
        self.regs().tim_cr().modify(|_, w| w.arpe().clear_bit());
        self.regs()
            .tim_arr()
            .write(|w| unsafe { w.arr().bits(period) });
        self.regs().tim_egr().write(|w| w.ug().set_bit());
        while !self.regs().tim_sr().read().ug_done().bit_is_set() {}
        let sr = self.regs().tim_sr().read().bits();
        unsafe { self.regs().tim_sr().as_ptr().write_volatile(sr) };
        self.regs().tim_cr().modify(|_, w| w.arpe().set_bit());
    }

    /// Software reset: triggers UG event and polls for completion.
    pub fn reset(&self) {
        self.regs().tim_egr().write(|w| w.ug().set_bit());
        while !self.regs().tim_sr().read().ug_done().bit_is_set() {}
        let sr = self.regs().tim_sr().read().bits();
        unsafe { self.regs().tim_sr().as_ptr().write_volatile(sr) };
    }

    /// Register a raw ISR callback for this timer's SR events.
    ///
    /// Requires a [`crate::interrupt::Binding`] token proving `InterruptHandler<P>`
    /// is wired to `P::IrqType`, obtained via [`crate::bind_interrupts!`]; this
    /// call itself unmasks the NVIC line. Which SR bits actually fire is up to
    /// the peripheral-level enable bits set separately via
    /// [`Tim::enable_interrupt`].
    ///
    /// `handler.on_irq()` runs directly in interrupt context; by the time it is
    /// called, SR has already been snapshotted and cleared (W1C).
    pub fn register_irq<H, B>(&self, handler: &'static mut H, _binding: B)
    where
        H: TimIrqHandler + 'static,
        B: Binding<P::IrqType, InterruptHandler<P>>,
    {
        let state = &TIM_STATE[P::IDX];
        let trampoline: unsafe fn(usize, TimIrqEvent) = tim_handler_trampoline::<H>;
        state
            .irq_handler
            .store(trampoline as usize, Ordering::Release);
        state
            .irq_data
            .store(handler as *mut H as usize, Ordering::Release);
        crate::interrupt::enable::<P::IrqType, InterruptHandler<P>, B>();
    }

    /// Deregister the raw ISR callback: clears the state slot. Does not mask
    /// the shared NVIC line or touch DIER — call
    /// [`Tim::disable_interrupt`](Tim::disable_interrupt) first if you want to
    /// fully quiesce (mirrors GDMA's `unregister_irq`).
    pub fn unregister_irq(&self) {
        let state = &TIM_STATE[P::IDX];
        state.irq_handler.store(0, Ordering::Release);
        state.irq_data.store(0, Ordering::Release);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// PWM-specific methods (Tim4-7)
// ────────────────────────────────────────────────────────────────────────────

impl<P: PwmTimInst> Tim<P, Pwm> {
    pub fn set_compare(&self, ch: Channel, ccr: u16) {
        match ch {
            Channel::Ch0 => {
                self.regs()
                    .tim_ccr0()
                    .modify(|_, w| unsafe { w.ccr0().bits(ccr) });
            }
            Channel::Ch1 => {
                self.regs()
                    .tim_ccr1()
                    .modify(|_, w| unsafe { w.ccr1().bits(ccr) });
            }
            Channel::Ch2 => {
                self.regs()
                    .tim_ccr2()
                    .modify(|_, w| unsafe { w.ccr2().bits(ccr) });
            }
            Channel::Ch3 => {
                self.regs()
                    .tim_ccr3()
                    .modify(|_, w| unsafe { w.ccr3().bits(ccr) });
            }
        }
    }

    pub fn get_compare(&self, ch: Channel) -> u16 {
        match ch {
            Channel::Ch0 => self.regs().tim_ccr0().read().ccr0().bits(),
            Channel::Ch1 => self.regs().tim_ccr1().read().ccr1().bits(),
            Channel::Ch2 => self.regs().tim_ccr2().read().ccr2().bits(),
            Channel::Ch3 => self.regs().tim_ccr3().read().ccr3().bits(),
        }
    }

    pub fn enable_channel(&self, ch: Channel) {
        match ch {
            Channel::Ch0 => {
                self.regs().tim_ccr0().modify(|_, w| w.cc0e().set_bit());
            }
            Channel::Ch1 => {
                self.regs().tim_ccr1().modify(|_, w| w.cc1e().set_bit());
            }
            Channel::Ch2 => {
                self.regs().tim_ccr2().modify(|_, w| w.cc2e().set_bit());
            }
            Channel::Ch3 => {
                self.regs().tim_ccr3().modify(|_, w| w.cc3e().set_bit());
            }
        }
    }

    pub fn disable_channel(&self, ch: Channel) {
        match ch {
            Channel::Ch0 => {
                self.regs().tim_ccr0().modify(|_, w| w.cc0e().clear_bit());
            }
            Channel::Ch1 => {
                self.regs().tim_ccr1().modify(|_, w| w.cc1e().clear_bit());
            }
            Channel::Ch2 => {
                self.regs().tim_ccr2().modify(|_, w| w.cc2e().clear_bit());
            }
            Channel::Ch3 => {
                self.regs().tim_ccr3().modify(|_, w| w.cc3e().clear_bit());
            }
        }
    }

    pub fn set_channel_preload(&self, ch: Channel, enabled: bool) {
        match (ch, enabled) {
            (Channel::Ch0, true) => {
                self.regs().tim_ccr0().modify(|_, w| w.oc0pe().set_bit());
            }
            (Channel::Ch0, false) => {
                self.regs().tim_ccr0().modify(|_, w| w.oc0pe().clear_bit());
            }
            (Channel::Ch1, true) => {
                self.regs().tim_ccr1().modify(|_, w| w.oc1pe().set_bit());
            }
            (Channel::Ch1, false) => {
                self.regs().tim_ccr1().modify(|_, w| w.oc1pe().clear_bit());
            }
            (Channel::Ch2, true) => {
                self.regs().tim_ccr2().modify(|_, w| w.oc2pe().set_bit());
            }
            (Channel::Ch2, false) => {
                self.regs().tim_ccr2().modify(|_, w| w.oc2pe().clear_bit());
            }
            (Channel::Ch3, true) => {
                self.regs().tim_ccr3().modify(|_, w| w.oc3pe().set_bit());
            }
            (Channel::Ch3, false) => {
                self.regs().tim_ccr3().modify(|_, w| w.oc3pe().clear_bit());
            }
        }
    }

    pub fn set_channel_polarity_active_low(&self, ch: Channel, active_low: bool) {
        match (ch, active_low) {
            (Channel::Ch0, true) => {
                self.regs().tim_ccr0().modify(|_, w| w.cc0p().set_bit());
            }
            (Channel::Ch0, false) => {
                self.regs().tim_ccr0().modify(|_, w| w.cc0p().clear_bit());
            }
            (Channel::Ch1, true) => {
                self.regs().tim_ccr1().modify(|_, w| w.cc1p().set_bit());
            }
            (Channel::Ch1, false) => {
                self.regs().tim_ccr1().modify(|_, w| w.cc1p().clear_bit());
            }
            (Channel::Ch2, true) => {
                self.regs().tim_ccr2().modify(|_, w| w.cc2p().set_bit());
            }
            (Channel::Ch2, false) => {
                self.regs().tim_ccr2().modify(|_, w| w.cc2p().clear_bit());
            }
            (Channel::Ch3, true) => {
                self.regs().tim_ccr3().modify(|_, w| w.cc3p().set_bit());
            }
            (Channel::Ch3, false) => {
                self.regs().tim_ccr3().modify(|_, w| w.cc3p().clear_bit());
            }
        }
    }

    pub fn set_channel_mode(&self, ch: Channel, mode: bool) {
        match (ch, mode) {
            (Channel::Ch0, true) => {
                self.regs().tim_ccr0().modify(|_, w| w.cc0m().set_bit());
            }
            (Channel::Ch0, false) => {
                self.regs().tim_ccr0().modify(|_, w| w.cc0m().clear_bit());
            }
            (Channel::Ch1, true) => {
                self.regs().tim_ccr1().modify(|_, w| w.cc1m().set_bit());
            }
            (Channel::Ch1, false) => {
                self.regs().tim_ccr1().modify(|_, w| w.cc1m().clear_bit());
            }
            (Channel::Ch2, true) => {
                self.regs().tim_ccr2().modify(|_, w| w.cc2m().set_bit());
            }
            (Channel::Ch2, false) => {
                self.regs().tim_ccr2().modify(|_, w| w.cc2m().clear_bit());
            }
            (Channel::Ch3, true) => {
                self.regs().tim_ccr3().modify(|_, w| w.cc3m().set_bit());
            }
            (Channel::Ch3, false) => {
                self.regs().tim_ccr3().modify(|_, w| w.cc3m().clear_bit());
            }
        }
    }

    /// Init a CC channel: writes CCR, triggers UG event, clears SR flags.
    pub fn init_channel(&self, ch: Channel, ccr: u16) {
        self.set_compare(ch, ccr);
        match ch {
            Channel::Ch0 => {
                self.regs().tim_egr().write(|w| w.cc0g().set_bit());
            }
            Channel::Ch1 => {
                self.regs().tim_egr().write(|w| w.cc1g().set_bit());
            }
            Channel::Ch2 => {
                self.regs().tim_egr().write(|w| w.cc2g().set_bit());
            }
            Channel::Ch3 => {
                self.regs().tim_egr().write(|w| w.cc3g().set_bit());
            }
        }
        let flag = match ch {
            Channel::Ch0 => TimFlag::CC0IF,
            Channel::Ch1 => TimFlag::CC1IF,
            Channel::Ch2 => TimFlag::CC2IF,
            Channel::Ch3 => TimFlag::CC3IF,
        };
        unsafe { self.regs().tim_sr().as_ptr().write_volatile(flag.bits()) }
    }

    pub fn set_sync_phase(&self, ch: Channel, phase: u16) {
        match ch {
            Channel::Ch0 => {
                self.regs()
                    .tim_psync0()
                    .modify(|_, w| unsafe { w.syncphase0().bits(phase) });
            }
            Channel::Ch1 => {
                self.regs()
                    .tim_psync1()
                    .modify(|_, w| unsafe { w.syncphase1().bits(phase) });
            }
            Channel::Ch2 => {
                self.regs()
                    .tim_psync2()
                    .modify(|_, w| unsafe { w.syncphase2().bits(phase) });
            }
            Channel::Ch3 => {
                self.regs()
                    .tim_psync3()
                    .modify(|_, w| unsafe { w.syncphase3().bits(phase) });
            }
        }
    }

    pub fn get_sync_phase(&self, ch: Channel) -> u16 {
        match ch {
            Channel::Ch0 => self.regs().tim_psync0().read().syncphase0().bits(),
            Channel::Ch1 => self.regs().tim_psync1().read().syncphase1().bits(),
            Channel::Ch2 => self.regs().tim_psync2().read().syncphase2().bits(),
            Channel::Ch3 => self.regs().tim_psync3().read().syncphase3().bits(),
        }
    }

    pub fn set_sync_preload(&self, ch: Channel, enabled: bool) {
        match (ch, enabled) {
            (Channel::Ch0, true) => {
                self.regs()
                    .tim_psync0()
                    .modify(|_, w| w.syncpe0().set_bit());
            }
            (Channel::Ch0, false) => {
                self.regs()
                    .tim_psync0()
                    .modify(|_, w| w.syncpe0().clear_bit());
            }
            (Channel::Ch1, true) => {
                self.regs()
                    .tim_psync1()
                    .modify(|_, w| w.syncpe1().set_bit());
            }
            (Channel::Ch1, false) => {
                self.regs()
                    .tim_psync1()
                    .modify(|_, w| w.syncpe1().clear_bit());
            }
            (Channel::Ch2, true) => {
                self.regs()
                    .tim_psync2()
                    .modify(|_, w| w.syncpe2().set_bit());
            }
            (Channel::Ch2, false) => {
                self.regs()
                    .tim_psync2()
                    .modify(|_, w| w.syncpe2().clear_bit());
            }
            (Channel::Ch3, true) => {
                self.regs()
                    .tim_psync3()
                    .modify(|_, w| w.syncpe3().set_bit());
            }
            (Channel::Ch3, false) => {
                self.regs()
                    .tim_psync3()
                    .modify(|_, w| w.syncpe3().clear_bit());
            }
        }
    }

    pub fn set_sync_dir_count_down(&self, ch: Channel, count_down: bool) {
        match (ch, count_down) {
            (Channel::Ch0, true) => {
                self.regs()
                    .tim_psync0()
                    .modify(|_, w| w.syncdir0().set_bit());
            }
            (Channel::Ch0, false) => {
                self.regs()
                    .tim_psync0()
                    .modify(|_, w| w.syncdir0().clear_bit());
            }
            (Channel::Ch1, true) => {
                self.regs()
                    .tim_psync1()
                    .modify(|_, w| w.syncdir1().set_bit());
            }
            (Channel::Ch1, false) => {
                self.regs()
                    .tim_psync1()
                    .modify(|_, w| w.syncdir1().clear_bit());
            }
            (Channel::Ch2, true) => {
                self.regs()
                    .tim_psync2()
                    .modify(|_, w| w.syncdir2().set_bit());
            }
            (Channel::Ch2, false) => {
                self.regs()
                    .tim_psync2()
                    .modify(|_, w| w.syncdir2().clear_bit());
            }
            (Channel::Ch3, true) => {
                self.regs()
                    .tim_psync3()
                    .modify(|_, w| w.syncdir3().set_bit());
            }
            (Channel::Ch3, false) => {
                self.regs()
                    .tim_psync3()
                    .modify(|_, w| w.syncdir3().clear_bit());
            }
        }
    }

    pub fn enable_sync(&self, ch: Channel) {
        match ch {
            Channel::Ch0 => {
                self.regs()
                    .tim_psync0()
                    .modify(|_, w| w.syncen0().set_bit());
            }
            Channel::Ch1 => {
                self.regs()
                    .tim_psync1()
                    .modify(|_, w| w.syncen1().set_bit());
            }
            Channel::Ch2 => {
                self.regs()
                    .tim_psync2()
                    .modify(|_, w| w.syncen2().set_bit());
            }
            Channel::Ch3 => {
                self.regs()
                    .tim_psync3()
                    .modify(|_, w| w.syncen3().set_bit());
            }
        }
    }

    pub fn disable_sync(&self, ch: Channel) {
        match ch {
            Channel::Ch0 => {
                self.regs()
                    .tim_psync0()
                    .modify(|_, w| w.syncen0().clear_bit());
            }
            Channel::Ch1 => {
                self.regs()
                    .tim_psync1()
                    .modify(|_, w| w.syncen1().clear_bit());
            }
            Channel::Ch2 => {
                self.regs()
                    .tim_psync2()
                    .modify(|_, w| w.syncen2().clear_bit());
            }
            Channel::Ch3 => {
                self.regs()
                    .tim_psync3()
                    .modify(|_, w| w.syncen3().clear_bit());
            }
        }
    }

    pub fn get_phase_count(&self, ch: Channel) -> u16 {
        match ch {
            Channel::Ch0 => self.regs().tim_phasecnt0().read().cnt0().bits(),
            Channel::Ch1 => self.regs().tim_phasecnt1().read().cnt1().bits(),
            Channel::Ch2 => self.regs().tim_phasecnt2().read().cnt2().bits(),
            Channel::Ch3 => self.regs().tim_phasecnt3().read().cnt3().bits(),
        }
    }
}

pub mod eh;

// ────────────────────────────────────────────────────────────────────────────
// Inpulse-specific methods (Tim8)
// ────────────────────────────────────────────────────────────────────────────

impl<P: InpulseTimInst> Tim<P, Inpulse> {
    pub fn set_one_pulse_mode(&self, enabled: bool) {
        if enabled {
            self.regs().tim_cr().modify(|_, w| w.opm().set_bit());
        } else {
            self.regs().tim_cr().modify(|_, w| w.opm().clear_bit());
        }
    }

    pub fn set_edge_trigger_polarity(&self, polarity: EdgePolarity) {
        self.regs()
            .tim_cr()
            .modify(|_, w| unsafe { w.etp().bits(polarity as u8) });
    }

    pub fn set_opm_default_level(&self, high: bool) {
        if high {
            self.regs().tim_ccr0().modify(|_, w| w.opm_dl0().set_bit());
        } else {
            self.regs()
                .tim_ccr0()
                .modify(|_, w| w.opm_dl0().clear_bit());
        }
    }

    pub fn set_compare(&self, ccr: u16) {
        self.regs()
            .tim_ccr0()
            .modify(|_, w| unsafe { w.ccr0().bits(ccr) });
    }

    pub fn get_phase_count(&self) -> u16 {
        self.regs().tim_phasecnt0().read().cnt0().bits()
    }

    pub fn set_sync_phase(&self, phase: u16) {
        self.regs()
            .tim_psync0()
            .modify(|_, w| unsafe { w.syncphase0().bits(phase) });
    }

    pub fn enable_sync(&self) {
        self.regs()
            .tim_psync0()
            .modify(|_, w| w.syncen0().set_bit());
    }

    pub fn disable_sync(&self) {
        self.regs()
            .tim_psync0()
            .modify(|_, w| w.syncen0().clear_bit());
    }
}
