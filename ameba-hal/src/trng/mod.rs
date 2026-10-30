//! True Random Number Generator (TRNG) driver.
//!
//! SDK: `ameba_trng.h` / `rom/rom_common/ameba_trng.c` : `_rand()`.
//!
//! This currently covers a basic polling read via the NS-aliased `TrngReg` PAC struct.
//! `TRNG_CTRL.SECURITY_CONTROL` resets to 0, which restricts non-secure bus
//! access to the FIFO_STATUS(0x0C)/DATAx(0x10-0x2C)/STATUS(0x30) window —
//! exactly what a basic read interface needs (SDK: `ameba_trng.h` doc comment
//! on `TRNG_CTRL`). APT-failure recovery (`TRNG_SW_RST`, offset 0x038) needs
//! secure-only bus access that the single-instance PAC `TrngReg` (NS alias)
//! doesn't expose, so it's deferred: [`Trng::read_u32`] returns `None`
//! instead of retrying forever like the SDK's `_rand()`. Wrapping this as
//! `rand_core::RngCore` is also deferred to a later revision.

use crate::rcc::{self, Peripheral};
use rtl8721f_pac::{TrngReg, trng_reg};

fn regs() -> &'static trng_reg::RegisterBlock {
    unsafe { &*TrngReg::PTR }
}

/// APT (Adaptive Proportion Test) failure code. SDK: `ameba_trng.h` : `TRNG_FIFO_LEVEL_INVALID`.
const TRNG_FIFO_LEVEL_INVALID: u8 = 0xF;
/// Buffer empty / APT not yet done. SDK: `ameba_trng.h` : `TRNG_FIFO_LEVEL_EMPTY`.
const TRNG_FIFO_LEVEL_EMPTY: u8 = 0x0;
/// Poll-loop iteration bound. SDK: `ameba_trng.h` : `TRNG_POLLING_TIMEOUT`.
const TRNG_POLLING_TIMEOUT: u32 = 0xFFFF;

/// TRNG driver.
pub struct Trng(());

impl Trng {
    /// # Safety
    /// Only one `Trng` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        Trng(())
    }

    /// Enable the TRNG function/bus clock.
    /// SDK: `ameba_trng.c` : `TRNG_Init` (the rest of `TRNG_Init` is a
    /// FPGA/Palladium-only pseudo-random-model setup, a no-op on real silicon).
    pub fn init(&self) {
        rcc::enable(Peripheral::TRNG);
    }

    /// Read one 32-bit random word, polling the FIFO valid level.
    ///
    /// Returns `None` on an APT self-test failure or after
    /// `TRNG_POLLING_TIMEOUT` empty polls. The SDK's `_rand()` instead issues
    /// `TRNG_SW_RST` and retries forever in both cases, but that register
    /// needs secure-only bus access this driver doesn't have (see module docs).
    pub fn read_u32(&self) -> Option<u32> {
        for _ in 0..=TRNG_POLLING_TIMEOUT {
            let level = regs().valid().read().valid().bits();
            if level == TRNG_FIFO_LEVEL_INVALID {
                return None;
            }
            if level != TRNG_FIFO_LEVEL_EMPTY {
                // Read-triggers-pop register. SDK: ameba_trng.h : TRNG_DATAx[0].
                return Some(regs().data0().read().random_number().bits());
            }
        }
        None
    }
}
