/// Delay provider implementing both the blocking and async `DelayNs` traits.
///
/// **Blocking** (`embedded_hal::delay::DelayNs`): backed by SYST_CVR polling for
/// delays ≥ 1 µs, and `asm::delay` for sub-µs delays. Minimum reliable granularity
/// is ~1 µs; shorter requests use a tight 2-instruction loop whose overhead is small
/// but not zero. DWT CYCCNT is unavailable (DWT_CTRL.NOCYCCNT = 1). Safe in any
/// context including critical sections (no ISR dependency).
///
/// **Async** (`embedded_hal_async::delay::DelayNs`): backed by `embassy_time`,
/// whose tick is 1 MHz (see `sys_timer`, Timer4 CCR0 alarm). Resolution is ~1 µs
/// (hardware compare, no polling). `delay_ns` rounds up to the next µs tick.
///
/// Obtain via `Delay::new()` after `ameba_hal::init()`.
pub struct Delay;

impl Default for Delay {
    fn default() -> Self {
        Self::new()
    }
}

impl Delay {
    pub fn new() -> Self {
        Delay
    }
}

// SDK: ameba_boot.h : SocClk_Info_TypeDef (boot.bin: USB PLL 960 MHz / KM4TZ CLKDIV=4 → 240 MHz)
const CYCLES_PER_NS_NUM: u64 = 6; // 240_000_000 / GCD(240M, 1000M) = 6
const CYCLES_PER_NS_DEN: u64 = 25; // 1_000_000_000 / GCD(240M, 1000M) = 25
const CYCLES_PER_US: u64 = 240; // 240_000_000 / 1_000_000
const CYCLES_PER_MS: u64 = 240_000; // 240_000_000 / 1_000

#[inline(always)]
fn spin_cycles(cycles: u64) {
    if cycles == 0 {
        return;
    }
    // Fast path: fits within one SysTick period (~70 ms). Use u32 arithmetic and compare
    // elapsed against the target directly — avoids u64 saturating_sub per iteration.
    const RELOAD: u32 = crate::sys_timer::SYSTICK_RELOAD;
    if cycles <= RELOAD as u64 {
        let target = cycles as u32;
        let start = cortex_m::peripheral::SYST::get_current();
        loop {
            let curr = cortex_m::peripheral::SYST::get_current();
            let elapsed = if curr <= start {
                start - curr
            } else {
                // CVR wrapped: start + (RELOAD + 1) - curr.
                // max value = RELOAD + 1 + RELOAD - 0 = 2*RELOAD+1 = 33_554_431, fits in u32.
                start + (RELOAD + 1 - curr)
            };
            if elapsed >= target {
                return;
            }
        }
    }
    // Slow path: multiple SysTick periods. Track remaining as u64.
    let mut remaining = cycles;
    let mut prev = cortex_m::peripheral::SYST::get_current();
    while remaining > 0 {
        let curr = cortex_m::peripheral::SYST::get_current();
        let elapsed = if curr <= prev {
            (prev - curr) as u64
        } else {
            prev as u64 + 1 + (RELOAD as u64 - curr as u64)
        };
        remaining = remaining.saturating_sub(elapsed);
        prev = curr;
    }
}

impl embedded_hal::delay::DelayNs for Delay {
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        // 240 MHz / 1 GHz = 6/25. Dividing by small constant 25 → LLVM multiply-by-reciprocal,
        // no __aeabi_uldivmod (which fired with the original / 1_000_000_000 divisor).
        spin_cycles(ns as u64 * CYCLES_PER_NS_NUM / CYCLES_PER_NS_DEN);
    }

    #[inline]
    fn delay_us(&mut self, us: u32) {
        spin_cycles(us as u64 * CYCLES_PER_US);
    }

    #[inline]
    fn delay_ms(&mut self, ms: u32) {
        spin_cycles(ms as u64 * CYCLES_PER_MS);
    }
}

impl embedded_hal_async::delay::DelayNs for Delay {
    // embassy-time tick = 1 µs (Timer4 @ 1 MHz); async delays have ~1 µs precision.
    async fn delay_ns(&mut self, ns: u32) {
        embassy_time::Timer::after_nanos(ns as u64).await;
    }

    async fn delay_us(&mut self, us: u32) {
        embassy_time::Timer::after_micros(us as u64).await;
    }

    async fn delay_ms(&mut self, ms: u32) {
        embassy_time::Timer::after_millis(ms as u64).await;
    }
}
