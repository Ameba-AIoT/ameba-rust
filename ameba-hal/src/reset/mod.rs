use bitflags::bitflags;
use rtl8721f_pac::{LpAon, LpSys, lp_aon, lp_sys};

// ────────────────────────────────────────────────────────────────────────────
// Register block accessors
// ────────────────────────────────────────────────────────────────────────────

fn lp_aon() -> &'static lp_aon::RegisterBlock {
    unsafe { &*LpAon::PTR }
}

fn lp_sys() -> &'static lp_sys::RegisterBlock {
    unsafe { &*LpSys::PTR }
}

// ────────────────────────────────────────────────────────────────────────────
// Public enums
// ────────────────────────────────────────────────────────────────────────────

/// CHIPEN operating mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipenMode {
    HwReset = 0,
    IntReset = 1,
    PulseReset = 2,
}

/// CHIPEN debounce time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipenDebounce {
    Us0 = 0,
    Us100 = 1,
    Us500 = 2,
    Ms1 = 3,
    Ms2 = 4,
    Ms4 = 5,
    Ms8 = 6,
    Ms16 = 7,
}

/// CHIPEN long-press threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipenLongPress {
    S1 = 0,
    S1P5 = 1,
    S2 = 2,
    S2P5 = 3,
    S3 = 4,
    S3P5 = 5,
    S4 = 6,
    S4P5 = 7,
}

/// CHIPEN short-press threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipenShortPress {
    Ms0 = 0,
    Ms50 = 1,
    Ms100 = 2,
    Ms150 = 3,
    Ms200 = 4,
    Ms250 = 5,
    Ms300 = 6,
    Ms350 = 7,
}

/// CHIPEN interrupt-mode ACK timeout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChipenAckTime {
    Ms50 = 0,
    Ms100 = 1,
    Ms200 = 2,
    Ms400 = 3,
}

bitflags! {
    /// CHIPEN interrupt status flags.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ChipenInt: u8 {
        /// Short-press interrupt.
        const SHORT_PRESS = 0b01;
        /// Long-press interrupt.
        const LONG_PRESS  = 0b10;
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Driver struct
// ────────────────────────────────────────────────────────────────────────────

/// CHIPEN (power key) and boot-reason driver.
///
/// `System_Reset` is intentionally omitted — it calls `SYS_CPUID()` which is
/// not available in ameba-rom-sys.
pub struct Reset(());

impl Reset {
    /// # Safety
    /// Only one `Reset` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        Reset(())
    }

    /// Set the CHIPEN operating mode.
    pub fn set_chipen_mode(&self, mode: ChipenMode) {
        lp_aon()
            .reg_chipen_ctrl()
            .modify(|_, w| unsafe { w.chipen_intr_mode().bits(mode as u8) });
    }

    /// Return the current CHIPEN operating mode.
    pub fn get_chipen_mode(&self) -> ChipenMode {
        let raw = lp_aon().reg_chipen_ctrl().read().chipen_intr_mode().bits();
        match raw.min(2) {
            0 => ChipenMode::HwReset,
            1 => ChipenMode::IntReset,
            _ => ChipenMode::PulseReset,
        }
    }

    /// Set the CHIPEN pin debounce time.
    pub fn set_chipen_debounce(&self, dbnc: ChipenDebounce) {
        lp_aon()
            .reg_chipen_ctrl()
            .modify(|_, w| unsafe { w.chipen_tdbc().bits(dbnc as u8) });
    }

    /// Return `true` if the CHIPEN pin is currently pressed.
    ///
    /// Only meaningful in `ChipenMode::PulseReset`.
    pub fn is_chipen_pressed(&self) -> bool {
        !lp_aon().reg_chipen_ctrl().read().chipen_st().bit()
    }

    /// Set long-press and short-press thresholds.
    ///
    /// Only meaningful in `ChipenMode::IntReset`.
    pub fn set_chipen_thresholds(&self, lp: ChipenLongPress, sp: ChipenShortPress) {
        lp_aon()
            .reg_chipen_ctrl()
            .modify(|_, w| unsafe { w.chipen_tlp().bits(lp as u8).chipen_tsp().bits(sp as u8) });
    }

    /// Set the interrupt-mode ACK timeout.
    pub fn set_chipen_ack_time(&self, tack: ChipenAckTime) {
        lp_aon()
            .reg_chipen_ctrl()
            .modify(|_, w| unsafe { w.chipen_tack().bits(tack as u8) });
    }

    /// Clear the specified CHIPEN interrupt flags (RW1C).
    pub fn clear_chipen_interrupt(&self, flags: ChipenInt) {
        // reg_chipen_isr is read-only in PAC; hardware is RW1C.
        // Bit 0 = short-press ISR, bit 1 = long-press ISR.
        unsafe {
            lp_aon()
                .reg_chipen_isr()
                .as_ptr()
                .write_volatile(flags.bits() as u32);
        }
    }

    /// Return pending CHIPEN interrupt flags.
    pub fn get_chipen_interrupt(&self) -> ChipenInt {
        let r = lp_aon().reg_chipen_isr().read();
        let mut flags = ChipenInt::empty();
        if r.chipen_sp_isr().bit() {
            flags |= ChipenInt::SHORT_PRESS;
        }
        if r.chipen_lp_isr().bit() {
            flags |= ChipenInt::LONG_PRESS;
        }
        flags
    }

    /// Return the boot reason bitmask (17 bits, from REG_LSYS_BOOT_REASON_SW).
    ///
    /// The PAC names this field "dummy" but the register holds valid boot-reason bits.
    pub fn boot_reason(&self) -> u32 {
        lp_sys().reg_boot_reason_sw().read().bits() & 0x0001_FFFF
    }
}
