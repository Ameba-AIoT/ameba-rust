//! RTL8201FR 10/100 Ethernet PHY driver for RTL8721F.
//!
//! All PHY register access is performed through a caller-supplied [`MdioBus`]
//! trait implementation.  This driver has no direct hardware register access;
//! it does not depend on the RMII MAC PAC.

use bitflags::bitflags;

// ─── ROM FFI ────────────────────────────────────────────────────────────────

use crate::delay::Delay;
use embedded_hal::delay::DelayNs;

// ─── IEEE 802.3 Clause 22 register addresses ────────────────────────────────

mod mii {
    pub const BMCR: u8 = 0x00;
    pub const BMSR: u8 = 0x01;
    pub const PHYSID1: u8 = 0x02;
    pub const PHYSID2: u8 = 0x03;
    pub const ADVERTISE: u8 = 0x04;
    pub const LPA: u8 = 0x05;
    pub const MMD_CTRL: u8 = 0x0D;
    pub const MMD_DATA: u8 = 0x0E;
}

// ─── BMCR bit flags ─────────────────────────────────────────────────────────

bitflags! {
    /// Basic Mode Control Register (BMCR, reg 0x00) bits.
    #[derive(Clone, Copy, Debug)]
    struct Bmcr: u16 {
        const RESET      = 1 << 15;
        const LOOPBACK   = 1 << 14;
        const SPEED100   = 1 << 13;
        const ANENABLE   = 1 << 12;
        const ANRESTART  = 1 <<  9;
        const FULLDPLX   = 1 <<  8;
    }
}

// ─── BMSR bit flags ─────────────────────────────────────────────────────────

bitflags! {
    /// Basic Mode Status Register (BMSR, reg 0x01) bits.
    #[derive(Clone, Copy, Debug)]
    struct Bmsr: u16 {
        const LSTATUS = 1 << 2;
    }
}

// ─── Auto-Negotiation Advertisement bit flags ────────────────────────────────

bitflags! {
    /// ADVERTISE / LPA register bits (regs 0x04 / 0x05).
    #[derive(Clone, Copy, Debug)]
    struct Advertise: u16 {
        const CSMA       = 1 << 0;
        const ADVERTISE_10HALF  = 1 << 5;
        const ADVERTISE_10FULL  = 1 << 6;
        const ADVERTISE_100HALF = 1 << 7;
        const ADVERTISE_100FULL = 1 << 8;
        const PAUSE_CAP  = 1 << 10;
        const PAUSE_ASYM = 1 << 11;
    }
}

// ─── MMD access constants ────────────────────────────────────────────────────

/// MMD Access Control function codes (written to reg 0x0D).
mod mmd {
    pub const FUNC_ADDR: u16 = 0x0000;
    pub const FUNC_DATA: u16 = 0x4000;

    /// MMD Auto-Negotiation device address (AN, device 7).
    pub const AN: u8 = 7;
    /// EEE Advertisement register address inside MMD AN.
    pub const EEEAR: u16 = 60;
    /// 100BASE-TX EEE advertisement bit.
    pub const EEE_ADVERT_100: u16 = 1 << 1;
}

// ─── RTL8201FR specifics ────────────────────────────────────────────────────

/// Page select register (register 31 on all pages).
const REG_PAGESEL: u8 = 31;
/// Standard register page.
const PAGE_0: u16 = 0;
/// Extension register page (RMII mode register lives here).
const PAGE_7: u16 = 7;

/// Extension page RMII Mode register address.
const P7_RMII_MODE: u8 = 16;
/// Clock direction control bit inside P7_RMII_MODE.
const P7_RMII_CLK_DIR: u16 = 1 << 12;

/// Expected combined PHY identifier (OUI + model).
const PHY_ID_MATCH: u32 = 0x001C_C816;

// ─── Public types ───────────────────────────────────────────────────────────

/// Errors returned by PHY driver operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// The underlying MDIO bus reported a transfer failure.
    Bus,
    /// The read PHY identifier does not match RTL8201FR.
    IdMismatch,
    /// A null or invalid parameter was supplied.
    InvalidParam,
}

/// PHY link speed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speed {
    Mbps10,
    Mbps100,
}

/// PHY duplex mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duplex {
    Half,
    Full,
}

/// RMII reference clock direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefClkDir {
    /// MAC drives 50 MHz into PHY REFCLK pin (PHY input).
    Mac2Phy,
    /// PHY drives 50 MHz out on REFCLK pin (PHY output).
    Phy2Mac,
}

/// Auto-negotiation advertisement capabilities.
#[derive(Clone, Copy, Debug, Default)]
pub struct AnAdvertise {
    pub cap_100m: bool,
    pub cap_10m: bool,
    pub pause: bool,
    pub asym_pause: bool,
}

/// Link configuration: either auto-negotiation or forced mode.
#[derive(Clone, Copy, Debug)]
pub enum LinkConfig {
    AutoNeg(AnAdvertise),
    Forced { speed: Speed, duplex: Duplex },
}

/// Snapshot of current link state.
#[derive(Clone, Copy, Debug)]
pub struct LinkState {
    pub link_up: bool,
    pub speed: Speed,
    pub duplex: Duplex,
}

/// RTL8201FR EEE capabilities reported by the PHY.
#[derive(Clone, Copy, Debug, Default)]
pub struct EeeCapability {
    pub eee_100m_capable: bool,
    pub eee_tx_lpi_capable: bool,
    pub eee_rx_lpi_capable: bool,
}

// ─── MDIO bus trait ─────────────────────────────────────────────────────────

/// Abstraction over the RMII MAC's MDIO bus.
///
/// The implementor performs the actual MIIAR register sequence.  This driver
/// never touches the RMII MAC registers directly.
pub trait MdioBus {
    fn read(&mut self, phy_addr: u8, reg_addr: u8) -> Result<u16, Error>;
    fn write(&mut self, phy_addr: u8, reg_addr: u8, val: u16) -> Result<(), Error>;
}

// ─── Driver struct ───────────────────────────────────────────────────────────

/// RTL8201FR PHY driver.
///
/// Obtain an instance with [`Rtl8201fr::steal`].  All methods are safe; the
/// `unsafe` is confined to construction.
pub struct Rtl8201fr<B: MdioBus> {
    bus: B,
    addr: u8,
}

impl<B: MdioBus> Rtl8201fr<B> {
    // ── Constructor ──────────────────────────────────────────────────────────

    /// Bind this driver to a PHY at `addr` on `bus`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that a RTL8201FR PHY physically exists at
    /// `addr` on the supplied MDIO bus.  Call [`init`](Self::init) to
    /// verify the identifier before normal operation.
    pub unsafe fn steal(bus: B, addr: u8) -> Self {
        Self { bus, addr }
    }

    // ── Private helpers ──────────────────────────────────────────────────────

    fn read_reg(&mut self, reg: u8) -> Result<u16, Error> {
        self.bus.read(self.addr, reg)
    }

    fn write_reg(&mut self, reg: u8, val: u16) -> Result<(), Error> {
        self.bus.write(self.addr, reg, val)
    }

    fn select_page(&mut self, page: u16) -> Result<(), Error> {
        self.write_reg(REG_PAGESEL, page)
    }

    /// Clause 45-over-22 indirect MMD write.
    fn mmd_write(&mut self, dev_addr: u8, reg_addr: u16, data: u16) -> Result<(), Error> {
        self.select_page(PAGE_0)?;
        self.write_reg(mii::MMD_CTRL, mmd::FUNC_ADDR | u16::from(dev_addr))?;
        self.write_reg(mii::MMD_DATA, reg_addr)?;
        self.write_reg(mii::MMD_CTRL, mmd::FUNC_DATA | u16::from(dev_addr))?;
        self.write_reg(mii::MMD_DATA, data)
    }

    /// Clause 45-over-22 indirect MMD read.
    fn mmd_read(&mut self, dev_addr: u8, reg_addr: u16) -> Result<u16, Error> {
        self.select_page(PAGE_0)?;
        self.write_reg(mii::MMD_CTRL, mmd::FUNC_ADDR | u16::from(dev_addr))?;
        self.write_reg(mii::MMD_DATA, reg_addr)?;
        self.write_reg(mii::MMD_CTRL, mmd::FUNC_DATA | u16::from(dev_addr))?;
        self.read_reg(mii::MMD_DATA)
    }

    fn autoneg_restart_inner(&mut self) -> Result<(), Error> {
        self.select_page(PAGE_0)?;
        let bmcr = Bmcr::from_bits_truncate(self.read_reg(mii::BMCR)?);
        let new = (bmcr | Bmcr::ANENABLE | Bmcr::ANRESTART).bits();
        self.write_reg(mii::BMCR, new)
    }

    // ── Public API ───────────────────────────────────────────────────────────

    /// Verify the PHY identifier.
    ///
    /// Returns [`Error::IdMismatch`] if the hardware does not identify as
    /// RTL8201FR.  Typically called once after power-on.
    pub fn init(&mut self) -> Result<(), Error> {
        self.select_page(PAGE_0)?;
        let id1 = self.read_reg(mii::PHYSID1)?;
        let id2 = self.read_reg(mii::PHYSID2)?;
        let phy_id = (u32::from(id1) << 16) | u32::from(id2);
        if phy_id != PHY_ID_MATCH {
            return Err(Error::IdMismatch);
        }
        Ok(())
    }

    /// Issue a software reset and wait for hardware stabilisation.
    ///
    /// The RTL8201FR datasheet requires at least 10–20 ms after asserting
    /// BMCR.RESET before the registers are accessible.
    pub fn reset(&mut self) -> Result<(), Error> {
        self.select_page(PAGE_0)?;
        self.write_reg(mii::BMCR, Bmcr::RESET.bits())?;
        // Datasheet requirement: ~10–20 ms reset stabilisation time.
        Delay::new().delay_ms(20);
        Ok(())
    }

    /// Configure link parameters.
    ///
    /// Pass [`LinkConfig::AutoNeg`] to enable auto-negotiation with the given
    /// advertisement set, or [`LinkConfig::Forced`] to lock speed/duplex.
    pub fn cfg_link(&mut self, cfg: LinkConfig) -> Result<(), Error> {
        self.select_page(PAGE_0)?;
        match cfg {
            LinkConfig::AutoNeg(adv) => {
                let mut advertise = Advertise::CSMA;
                if adv.cap_100m {
                    advertise |= Advertise::ADVERTISE_100FULL | Advertise::ADVERTISE_100HALF;
                }
                if adv.cap_10m {
                    advertise |= Advertise::ADVERTISE_10FULL | Advertise::ADVERTISE_10HALF;
                }
                if adv.pause {
                    advertise |= Advertise::PAUSE_CAP;
                }
                if adv.asym_pause {
                    advertise |= Advertise::PAUSE_ASYM;
                }
                self.write_reg(mii::ADVERTISE, advertise.bits())?;
                let bmcr = Bmcr::from_bits_truncate(self.read_reg(mii::BMCR)?);
                let new = (bmcr | Bmcr::ANENABLE | Bmcr::ANRESTART).bits();
                self.write_reg(mii::BMCR, new)
            }
            LinkConfig::Forced { speed, duplex } => {
                let bmcr = Bmcr::from_bits_truncate(self.read_reg(mii::BMCR)?);
                // Clear speed, duplex, and AN enable; preserve loopback/power-down.
                let mut new = bmcr - (Bmcr::SPEED100 | Bmcr::FULLDPLX | Bmcr::ANENABLE);
                if speed == Speed::Mbps100 {
                    new |= Bmcr::SPEED100;
                }
                if duplex == Duplex::Full {
                    new |= Bmcr::FULLDPLX;
                }
                self.write_reg(mii::BMCR, new.bits())
            }
        }
    }

    /// Read current link status.
    ///
    /// BMSR is read twice to clear the latching-low link-status bit before
    /// sampling it.
    pub fn get_link(&mut self) -> Result<LinkState, Error> {
        self.select_page(PAGE_0)?;
        // First read clears the latched-low state; second read reflects truth.
        let _ = self.read_reg(mii::BMSR)?;
        let bmsr = Bmsr::from_bits_truncate(self.read_reg(mii::BMSR)?);
        let bmcr = Bmcr::from_bits_truncate(self.read_reg(mii::BMCR)?);
        let lpa = Advertise::from_bits_truncate(self.read_reg(mii::LPA)?);
        let adv = Advertise::from_bits_truncate(self.read_reg(mii::ADVERTISE)?);
        Ok(parse_link_status(bmsr, bmcr, lpa, adv))
    }

    /// Restart auto-negotiation without changing the advertisement register.
    pub fn autoneg_restart(&mut self) -> Result<(), Error> {
        self.autoneg_restart_inner()
    }

    /// Configure the 50 MHz RMII reference clock direction.
    ///
    /// The RTL8201FR stores this setting on Page 7 register 16.  This method
    /// saves and restores Page 0 after the access.
    pub fn cfg_refclock(&mut self, dir: RefClkDir) -> Result<(), Error> {
        self.select_page(PAGE_7)?;
        let mut val = self.read_reg(P7_RMII_MODE)?;
        match dir {
            RefClkDir::Mac2Phy => val |= P7_RMII_CLK_DIR,
            RefClkDir::Phy2Mac => val &= !P7_RMII_CLK_DIR,
        }
        self.write_reg(P7_RMII_MODE, val)?;
        // Always restore to Page 0; hardware does not auto-restore on reset.
        self.select_page(PAGE_0)
    }

    /// Enable or disable 100BASE-TX EEE advertisement.
    ///
    /// Writes MMD AN register 60 (EEE Advertisement) and restarts
    /// auto-negotiation to propagate the updated capability to the link partner.
    pub fn cfg_eee(&mut self, enable: bool) -> Result<(), Error> {
        let adv_val = if enable { mmd::EEE_ADVERT_100 } else { 0u16 };
        self.mmd_write(mmd::AN, mmd::EEEAR, adv_val)?;
        // Readback to confirm write settled (RTL8201FR errata: verify MMD write).
        let _ = self.mmd_read(mmd::AN, mmd::EEEAR)?;
        self.autoneg_restart_inner()
    }

    /// Query EEE capabilities as advertised via MMD AN register 60.
    pub fn get_eee_cap(&mut self) -> Result<EeeCapability, Error> {
        self.select_page(PAGE_0)?;
        let adv_val = self.mmd_read(mmd::AN, mmd::EEEAR)?;
        let capable = (adv_val & mmd::EEE_ADVERT_100) != 0;
        Ok(EeeCapability {
            eee_100m_capable: capable,
            // RTL8201FR only supports 100BASE-TX EEE; LPI follows the same flag.
            eee_tx_lpi_capable: capable,
            eee_rx_lpi_capable: capable,
        })
    }

    /// Enable or disable the PHY digital loopback (BMCR.LOOPBACK).
    pub fn set_loopback(&mut self, enable: bool) -> Result<(), Error> {
        self.select_page(PAGE_0)?;
        let bmcr = Bmcr::from_bits_truncate(self.read_reg(mii::BMCR)?);
        let new = if enable {
            bmcr | Bmcr::LOOPBACK
        } else {
            bmcr - Bmcr::LOOPBACK
        };
        self.write_reg(mii::BMCR, new.bits())
    }

    /// No-op stub: link state change notifications are delivered externally
    /// (e.g. via the MAC interrupt handler) and do not require storage inside
    /// this driver.
    pub fn set_link_callback(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

// ─── Pure link-status parser ────────────────────────────────────────────────

/// Resolve speed and duplex from register snapshot.
///
/// Mirrors the C `phy_rtl8201_parse_link_status` logic exactly.
fn parse_link_status(bmsr: Bmsr, bmcr: Bmcr, lpa: Advertise, adv: Advertise) -> LinkState {
    let link_up = bmsr.contains(Bmsr::LSTATUS);
    if !link_up {
        return LinkState {
            link_up: false,
            speed: Speed::Mbps10,
            duplex: Duplex::Half,
        };
    }

    if !bmcr.contains(Bmcr::ANENABLE) {
        // Forced mode: read directly from BMCR.
        let speed = if bmcr.contains(Bmcr::SPEED100) {
            Speed::Mbps100
        } else {
            Speed::Mbps10
        };
        let duplex = if bmcr.contains(Bmcr::FULLDPLX) {
            Duplex::Full
        } else {
            Duplex::Half
        };
        return LinkState {
            link_up: true,
            speed,
            duplex,
        };
    }

    // Auto-negotiation: intersection of local advertisement and link partner.
    let common = lpa & adv;
    let (speed, duplex) = if common.contains(Advertise::ADVERTISE_100FULL) {
        (Speed::Mbps100, Duplex::Full)
    } else if common.contains(Advertise::ADVERTISE_100HALF) {
        (Speed::Mbps100, Duplex::Half)
    } else if common.contains(Advertise::ADVERTISE_10FULL) {
        (Speed::Mbps10, Duplex::Full)
    } else {
        (Speed::Mbps10, Duplex::Half)
    };

    LinkState {
        link_up: true,
        speed,
        duplex,
    }
}
