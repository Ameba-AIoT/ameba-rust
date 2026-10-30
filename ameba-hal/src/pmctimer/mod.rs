use bitflags::bitflags;
use rtl8721f_pac::{PmcTimerReg, pmc_timer_reg};

// ────────────────────────────────────────────────────────────────────────────
// Register block access — PmcTimerReg at 0x4081_8000
// ────────────────────────────────────────────────────────────────────────────

fn pmc_timer() -> &'static pmc_timer_reg::RegisterBlock {
    unsafe { &*PmcTimerReg::PTR }
}

// ────────────────────────────────────────────────────────────────────────────
// Public enums / bitflags
// ────────────────────────────────────────────────────────────────────────────

/// PMC timer channel index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerIdx {
    Timer0 = 0,
    Timer1 = 1,
    Timer2 = 2,
    Timer3 = 3,
}

bitflags! {
    /// Bitmask over all 4 PMC timer compare-enable bits (bits [3:0] of TIM_COM_EN).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct TimerCompare: u8 {
        const TIMER0 = 0b0001;
        const TIMER1 = 0b0010;
        const TIMER2 = 0b0100;
        const TIMER3 = 0b1000;
    }
}

impl TimerCompare {
    fn for_idx(idx: TimerIdx) -> Self {
        match idx {
            TimerIdx::Timer0 => Self::TIMER0,
            TimerIdx::Timer1 => Self::TIMER1,
            TimerIdx::Timer2 => Self::TIMER2,
            TimerIdx::Timer3 => Self::TIMER3,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Driver struct
// ────────────────────────────────────────────────────────────────────────────

/// PMC timer driver.
///
/// All 4 channels run at 32 kHz. PMCTimer_Init and PMCTimer_INT_Hdl are
/// intentionally omitted — they have RTOS (pmu_acquire_wakelock / InterruptRegister)
/// dependencies that do not belong in a bare-metal PAC driver.
pub struct PmcTimer(());

impl PmcTimer {
    /// # Safety
    /// Only one `PmcTimer` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        PmcTimer(())
    }

    /// Start or stop a timer channel.
    pub fn cmd(&self, idx: TimerIdx, enable: bool) {
        let regs = pmc_timer();
        match idx {
            TimerIdx::Timer0 => regs
                .tim_en()
                .modify(|_, w| w.cnt_start_timer0().bit(enable)),
            TimerIdx::Timer1 => regs
                .tim_en()
                .modify(|_, w| w.cnt_start_timer1().bit(enable)),
            TimerIdx::Timer2 => regs
                .tim_en()
                .modify(|_, w| w.cnt_start_timer2().bit(enable)),
            TimerIdx::Timer3 => regs
                .tim_en()
                .modify(|_, w| w.cnt_start_timer3().bit(enable)),
        };
    }

    /// Return `true` if the timer channel's counter value is valid.
    pub fn is_valid(&self, idx: TimerIdx) -> bool {
        let regs = pmc_timer();
        match idx {
            TimerIdx::Timer0 => regs.tim_en().read().config_valid_timer0().bit(),
            TimerIdx::Timer1 => regs.tim_en().read().config_valid_timer1().bit(),
            TimerIdx::Timer2 => regs.tim_en().read().config_valid_timer2().bit(),
            TimerIdx::Timer3 => regs.tim_en().read().config_valid_timer3().bit(),
        }
    }

    /// Read the current counter value of a timer channel.
    pub fn get_count(&self, idx: TimerIdx) -> u32 {
        let regs = pmc_timer();
        match idx {
            TimerIdx::Timer0 => regs.tim_cnt0().read().cnt0().bits(),
            TimerIdx::Timer1 => regs.tim_cnt1().read().cnt1().bits(),
            TimerIdx::Timer2 => regs.tim_cnt2().read().cnt2().bits(),
            TimerIdx::Timer3 => regs.tim_cnt3().read().cnt3().bits(),
        }
    }

    /// Program the countdown value for a timer channel.
    pub fn set_count(&self, idx: TimerIdx, value: u32) {
        let regs = pmc_timer();
        match idx {
            TimerIdx::Timer0 => regs
                .tim_set0()
                .modify(|_, w| unsafe { w.tim_set0().bits(value) }),
            TimerIdx::Timer1 => regs
                .tim_set1()
                .modify(|_, w| unsafe { w.tim_set1().bits(value) }),
            TimerIdx::Timer2 => regs
                .tim_set2()
                .modify(|_, w| unsafe { w.tim_set2().bits(value) }),
            TimerIdx::Timer3 => regs
                .tim_set3()
                .modify(|_, w| unsafe { w.tim_set3().bits(value) }),
        };
    }

    /// Reset a timer channel's counter.
    pub fn reset_count(&self, idx: TimerIdx) {
        let regs = pmc_timer();
        match idx {
            TimerIdx::Timer0 => regs.tim_en().modify(|_, w| w.reset_timer0().set_bit()),
            TimerIdx::Timer1 => regs.tim_en().modify(|_, w| w.reset_timer1().set_bit()),
            TimerIdx::Timer2 => regs.tim_en().modify(|_, w| w.reset_timer2().set_bit()),
            TimerIdx::Timer3 => regs.tim_en().modify(|_, w| w.reset_timer3().set_bit()),
        };
    }

    /// Enable or disable the interrupt for a timer channel.
    pub fn set_interrupt(&self, idx: TimerIdx, enabled: bool) {
        let regs = pmc_timer();
        match idx {
            TimerIdx::Timer0 => regs.tim_dier().modify(|_, w| w.uie_timer0().bit(enabled)),
            TimerIdx::Timer1 => regs.tim_dier().modify(|_, w| w.uie_timer1().bit(enabled)),
            TimerIdx::Timer2 => regs.tim_dier().modify(|_, w| w.uie_timer2().bit(enabled)),
            TimerIdx::Timer3 => regs.tim_dier().modify(|_, w| w.uie_timer3().bit(enabled)),
        };
    }

    /// Clear the interrupt flag for a timer channel (RW1C).
    pub fn clear_interrupt(&self, idx: TimerIdx) {
        // tim_isr is read-only in PAC; hardware supports RW1C.
        let bit = 1u32 << (idx as u32);
        unsafe {
            pmc_timer().tim_isr().as_ptr().write_volatile(bit);
        }
    }

    /// Enable or disable the compare function for a timer channel.
    pub fn set_compare(&self, idx: TimerIdx, enabled: bool) {
        let bit = TimerCompare::for_idx(idx);
        let mut current =
            TimerCompare::from_bits_truncate(pmc_timer().tim_com_en().read().com_en().bits());
        if enabled {
            current.insert(bit);
        } else {
            current.remove(bit);
        }
        pmc_timer()
            .tim_com_en()
            .modify(|_, w| unsafe { w.com_en().bits(current.bits()) });
    }
}
