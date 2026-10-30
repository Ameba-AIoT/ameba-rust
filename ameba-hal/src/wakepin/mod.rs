use bitflags::bitflags;
use rtl8721f_pac::{LpAon, lp_aon};

// ────────────────────────────────────────────────────────────────────────────
// Register block access
// ────────────────────────────────────────────────────────────────────────────

fn lp_aon() -> &'static lp_aon::RegisterBlock {
    // SAFETY: LpAon is always-on hardware at a fixed address, valid for 'static.
    unsafe { &*LpAon::PTR }
}

// ────────────────────────────────────────────────────────────────────────────
// Public enums
// ────────────────────────────────────────────────────────────────────────────

/// AON wakeup pin index (maps to physical pads PA3/PA2/PA1/PA0).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakePinIdx {
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
}

/// Level that triggers wakeup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeLevel {
    Low,
    High,
}

bitflags! {
    /// Bitmask of wakeup-pin event flags returned by `WakePin::get_triggered`.
    ///
    /// Bits correspond to hardware bits [7:4] of REG_AON_AON_WAK_EVT shifted to [3:0].
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WakePinEvt: u8 {
        const PIN0 = 0b0001;
        const PIN1 = 0b0010;
        const PIN2 = 0b0100;
        const PIN3 = 0b1000;
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Driver struct
// ────────────────────────────────────────────────────────────────────────────

/// AON wakeup-pin driver.
pub struct WakePin(());

impl WakePin {
    /// # Safety
    /// Only one `WakePin` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        WakePin(())
    }

    /// Configure a wakeup pin: polarity and enable.
    pub fn configure(&self, pin: WakePinIdx, level: WakeLevel) {
        let bit = 1u8 << (pin as u8);

        // Disable the pin first.
        lp_aon().reg_gpio_ctrl().modify(|r, w| {
            let fen = r.gpio_wake_fen().bits() & !bit;
            unsafe { w.gpio_wake_fen().bits(fen) }
        });

        // Set polarity.
        lp_aon().reg_gpio_ctrl().modify(|r, w| {
            let poly = match level {
                WakeLevel::Low => r.gpio_wakdet_poly().bits() & !bit,
                WakeLevel::High => r.gpio_wakdet_poly().bits() | bit,
            };
            unsafe { w.gpio_wakdet_poly().bits(poly) }
        });

        // Re-enable the pin.
        lp_aon().reg_gpio_ctrl().modify(|r, w| {
            let fen = r.gpio_wake_fen().bits() | bit;
            unsafe { w.gpio_wake_fen().bits(fen) }
        });
    }

    /// Configure the shared debounce circuit.
    ///
    /// When `enabled` is `false`, `cycle` is ignored and debounce is disabled.
    /// `cycle` must be ≤ 0x7FFF; unit is 100k/128k clock cycles.
    pub fn configure_debounce(&self, cycle: u16, enabled: bool) {
        let regs = lp_aon();

        // Save and clear gpio_wake_fen.
        let fen_backup = regs.reg_gpio_ctrl().read().gpio_wake_fen().bits();
        regs.reg_gpio_ctrl()
            .modify(|_, w| unsafe { w.gpio_wake_fen().bits(0) });

        // Set debounce parameters.
        if enabled {
            regs.reg_gpio_ctrl()
                .modify(|_, w| unsafe { w.gpio_dbnc_cyc().bits(cycle).gpio_dbnc_fen().set_bit() });
        } else {
            regs.reg_gpio_ctrl()
                .modify(|_, w| w.gpio_dbnc_fen().clear_bit());
        }

        // Restore gpio_wake_fen.
        regs.reg_gpio_ctrl().modify(|r, w| unsafe {
            w.gpio_wake_fen()
                .bits(r.gpio_wake_fen().bits() | fen_backup)
        });
    }

    /// Return bitmask of wakeup pins that triggered an event.
    pub fn get_triggered(&self) -> WakePinEvt {
        // GPIO PIN0-3 wakeup event bits are at [7:4] of reg_aon_wak_evt.
        // The PAC does not expose these bits as named fields.
        let raw = lp_aon().reg_aon_wak_evt().read().bits();
        WakePinEvt::from_bits_truncate(((raw >> 4) & 0xF) as u8)
    }

    /// Clear the wakeup-pin interrupt flags (RW1C).
    ///
    /// Pass the `WakePinEvt` bits whose interrupt should be cleared.
    pub fn clear_interrupt(&self, pins: WakePinEvt) {
        // PAC marks reg_gpio_wake_isr as read-only; hardware is RW1C.
        unsafe {
            lp_aon()
                .reg_gpio_wake_isr()
                .as_ptr()
                .write_volatile(pins.bits() as u32);
        }
    }
}
