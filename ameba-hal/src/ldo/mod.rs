use rtl8721f_pac::{OtpcReg, Pmc, otpc_reg, pmc};

fn pmc() -> &'static pmc::RegisterBlock {
    unsafe { &*Pmc::PTR }
}

fn otpc() -> &'static otpc_reg::RegisterBlock {
    unsafe { &*OtpcReg::PTR }
}

/// Core LDO voltage selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoreVol {
    V0p9,
    V1p0,
}

/// Mem LDO operating mode (normal or sleep configuration).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MldoMode {
    Off = 0b00,
    Sleep = 0b01,
    Normal0 = 0b10,
    Normal1 = 0b11,
}

/// LDO (core and memory regulator) driver.
pub struct Ldo(());

impl Ldo {
    /// # Safety
    /// Only one `Ldo` instance may exist at a time.
    pub unsafe fn steal() -> Self {
        Ldo(())
    }

    /// Set core LDO voltage. Mirrors `LDO_CoreVolSet`.
    ///
    /// For V1p0: programs OTPC boot_vol_sel and triggers regu state transition.
    /// For V0p9: default hardware state, no action needed.
    pub fn set_core_vol(&self, vol: CoreVol) {
        if vol == CoreVol::V1p0 {
            otpc()
                .reg_otp_hwcfg()
                .modify(|_, w| w.boot_vol_sel().set_bit());
            pmc()
                .reg_aip_trigger()
                .modify(|_, w| w.trg_sys_regu_sts().set_bit());
        }
    }

    /// Set mem LDO mode during normal (active) state. Mirrors `LDO_MemSetInNormal`.
    ///
    /// Returns `true` on success, `false` on timeout waiting for MLDO ready.
    pub fn set_mem_mode_normal(&self, mode: MldoMode) -> bool {
        pmc()
            .reg_aip_trigger()
            .modify(|_, w| unsafe { w.sys_mldo_norm().bits(mode as u8) });
        pmc()
            .reg_aip_trigger()
            .modify(|_, w| w.trg_sys_mldo().set_bit());

        let mut timeout: u32 = 10_000;
        loop {
            if pmc().reg_aip_trigger().read().sys_mldo_rdy().bit() {
                return true;
            }
            timeout -= 1;
            if timeout == 0 {
                return false;
            }
        }
    }

    /// Set mem LDO mode during sleep state. Mirrors `LDO_MemSetInSleep`.
    pub fn set_mem_mode_sleep(&self, mode: MldoMode) {
        pmc()
            .reg_aip_trigger()
            .modify(|_, w| unsafe { w.sys_mldo_slp().bits(mode as u8) });
    }
}
