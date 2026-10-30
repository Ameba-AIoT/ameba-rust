// SysTick: free-running counter (no ISR) for blocking delay.rs spin_cycles().
// Max 24-bit reload → ~70 ms wrap period at 240 MHz.
pub(crate) const SYSTICK_RELOAD: u32 = 0x00FF_FFFF;

/// Initialize SysTick as a free-running counter for `delay.rs` blocking spin loops.
///
/// Called automatically and unconditionally from `chip_pre_init()`, before any
/// other chip-init step — those steps use blocking `Delay`, which depends on
/// this counter already running.
pub fn init() {
    let mut syst = unsafe { cortex_m::Peripherals::steal().SYST };
    syst.set_reload(SYSTICK_RELOAD);
    syst.clear_current();
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    syst.enable_counter();
}
