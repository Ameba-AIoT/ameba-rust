use rtl8721f_pac::{LpAon, lp_aon};

// ────────────────────────────────────────────────────────────────────────────
// Register block access — LpAon at 0x4080_A000
// ────────────────────────────────────────────────────────────────────────────

fn lp_aon() -> &'static lp_aon::RegisterBlock {
    // SAFETY: LpAon is always-on hardware at a fixed address, valid for 'static.
    unsafe { &*LpAon::PTR }
}

// ────────────────────────────────────────────────────────────────────────────
// Driver struct
// ────────────────────────────────────────────────────────────────────────────

/// AON (always-on) timer driver.
///
/// Hardware runs at 100 kHz (100 ticks/ms). Maximum duration ≈ 715 min.
pub struct AonTimer(());

impl AonTimer {
    /// Obtain the AON timer driver. Caller must ensure single ownership.
    ///
    /// # Safety
    /// Only one `AonTimer` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        AonTimer(())
    }

    /// Program a countdown of `ms` milliseconds before the AON wakeup fires.
    ///
    /// A value of 0 is ignored (matches C driver behaviour).
    pub fn set_duration(&self, ms: u32) {
        if ms == 0 {
            return;
        }
        let cnt = ms * 100;
        lp_aon()
            .reg_tim_set()
            .write(|w| unsafe { w.aon_tim_cnt_set().bits(cnt) });
    }

    /// Return the current AON timer counter in milliseconds.
    pub fn get_count(&self) -> u32 {
        lp_aon().reg_tim_cnt().read().aon_tim_cnt_cur().bits() / 100
    }

    /// Enable or disable the AON timer interrupt.
    pub fn set_interrupt(&self, enabled: bool) {
        lp_aon()
            .reg_tim_ier()
            .modify(|_, w| w.aon_tim_ie().bit(enabled));
    }

    /// Clear the AON timer interrupt flag (RW1C).
    pub fn clear_interrupt(&self) {
        // PAC marks reg_tim_isr as read-only; hardware is RW1C.
        // Write bit 0 = 1 to clear, using PAC-provided address.
        unsafe {
            lp_aon().reg_tim_isr().as_ptr().write_volatile(1);
        }
    }
}
