use bitflags::bitflags;
use rtl8721f_pac::{LdoReg, LpAon, Pmc, ldo_reg, lp_aon, pmc};

// ────────────────────────────────────────────────────────────────────────────
// Register block accessors
// ────────────────────────────────────────────────────────────────────────────

fn ldo_reg() -> &'static ldo_reg::RegisterBlock {
    unsafe { &*LdoReg::PTR }
}

fn lp_aon() -> &'static lp_aon::RegisterBlock {
    unsafe { &*LpAon::PTR }
}

fn pmc() -> &'static pmc::RegisterBlock {
    unsafe { &*Pmc::PTR }
}

// ────────────────────────────────────────────────────────────────────────────
// Public enums
// ────────────────────────────────────────────────────────────────────────────

/// BOR operating mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorMode {
    /// Hardware reset on under-voltage.
    Reset,
    /// NMI interrupt on under-voltage.
    Interrupt,
}

/// BOR interrupt-mode debounce time (unit: ANA4M clock cycles).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BorDebounce {
    Us0 = 0b000,
    Us100 = 0b001,
    Us500 = 0b010,
    Us1000 = 0b011,
    Us2000 = 0b100,
    Us4000 = 0b101,
    Us8000 = 0b110,
    Us16000 = 0b111,
}

bitflags! {
    /// PMC NP wakeup source mask bits (from sysreg_pmc.h).
    struct WakeSource: u32 {
        const BOR = 1 << 26;
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Driver struct
// ────────────────────────────────────────────────────────────────────────────

/// Brown-Out Reset (BOR) driver.
pub struct Bor(());

impl Bor {
    /// # Safety
    /// Only one `Bor` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        Bor(())
    }

    /// Set the BOD falling and rising voltage thresholds (5-bit values 0x00–0x1F).
    ///
    /// Higher values correspond to lower voltage thresholds.
    pub fn set_threshold(&self, fall: u8, rise: u8) {
        ldo_reg().reg_bod().modify(|_, w| unsafe {
            w.reg_bod_f()
                .bits(fall & 0x1F)
                .reg_bod_r()
                .bits(rise & 0x1F)
        });
    }

    /// Enable or disable the BOD comparator.
    pub fn enable(&self, enabled: bool) {
        ldo_reg()
            .reg_bod()
            .modify(|_, w| w.reg_bod_r_en().bit(enabled));
    }

    /// Set the BOR mode (reset or interrupt).
    ///
    /// When switching to `Interrupt` mode, also enables BOR as a PMC NP wakeup source.
    pub fn set_mode(&self, mode: BorMode) {
        match mode {
            BorMode::Interrupt => {
                lp_aon()
                    .reg_bor_ctrl()
                    .modify(|_, w| w.bor_intr_mode().set_bit());

                let current = WakeSource::from_bits_truncate(
                    pmc().reg_wak_mask0_np().read().wak_np_imr_29_0().bits(),
                );
                let new_val = (current | WakeSource::BOR).bits();
                pmc()
                    .reg_wak_mask0_np()
                    .modify(|_, w| unsafe { w.wak_np_imr_29_0().bits(new_val) });
            }
            BorMode::Reset => {
                lp_aon()
                    .reg_bor_ctrl()
                    .modify(|_, w| w.bor_intr_mode().clear_bit());
            }
        }
    }

    /// Set the interrupt-mode debounce time.
    ///
    /// Only takes effect in `BorMode::Interrupt`.
    pub fn set_debounce(&self, dbnc: BorDebounce) {
        lp_aon()
            .reg_bor_ctrl()
            .modify(|_, w| unsafe { w.bor_tdbc().bits(dbnc as u8) });
    }

    /// Clear the BOR interrupt flag (RW1C).
    pub fn clear_interrupt(&self) {
        // PAC marks reg_bor_isr as read-only; hardware is RW1C.
        unsafe {
            lp_aon().reg_bor_isr().as_ptr().write_volatile(1);
        }
    }
}
