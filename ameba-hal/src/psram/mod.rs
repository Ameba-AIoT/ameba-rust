//! RTL8721F PSRAM controller + PHY driver.
//!
//! Converts `ameba_psram.c` to idiomatic `no_std` Rust using the PAC for all
//! register access.  Functions that require rtos/cache/IPC primitives are
//! intentionally omitted per the conversion rules.
//!
//! # Safety contract
//! * `Psram::steal()` is the only `unsafe` public entry-point.  The caller
//!   must ensure no other code holds a simultaneous reference to the PSRAM
//!   controller or PHY register blocks for the lifetime of the returned
//!   `Psram` handle.
//! * All other `pub` methods are safe (they encapsulate the required `unsafe`
//!   blocks internally).

use bitflags::bitflags;
use rtl8721f_pac::{PsramPhyReg, PsramReg, psram_phy_reg, psram_reg};

use crate::delay::Delay;
use embedded_hal::delay::DelayNs;

// ---------------------------------------------------------------------------
// Vendor / sizing constants  (mirrors C macros)
// ---------------------------------------------------------------------------

pub const MCM_PSRAM_VENDOR_WB: u8 = 1;
pub const MCM_PSRAM_VENDOR_APM: u8 = 0;
pub const MCM_PSRAM_VENDOR_NOTCARE: u8 = 0xff;

pub const MCM_PSRAM_DQ16: u8 = 1;
pub const MCM_PSRAM_DQ8: u8 = 0;

/// Calibration pattern used by SW calibration.
const CALIB_PATTERN: [u32; 6] = [
    0x1122_3344,
    0xa55a_a55a,
    0x5aa5_5aa5,
    0x4433_2211,
    0x9669_6996,
    0x6996_9669,
];

// ---------------------------------------------------------------------------
// APM write-latency lookup table  (index = latency_set - 3)
// ---------------------------------------------------------------------------
const APM_WR_INIT_LATENCY_SPEC: [u8; 6] = [
    0x00, // 3 clk
    0x40, // 4 clk
    0x80, // 5 clk
    0xc0, // 6 clk
    0x20, // 7 clk
    0x60, // 8 clk
];

// WB write-latency lookup (index = latency_set - 3, 8 entries, gaps are 0)
const WB_WR_INIT_LATENCY_SPEC: [u8; 8] = [
    0x00, // 3 clk
    0x20, // 4 clk
    0x40, // 5 clk
    0x60, // 6 clk
    0x80, // 7 clk
    0x00, // NULL
    0x00, // NULL
    0xe0, // 10 clk
];

// WB read/write command type per device_type index (0=WB955, 1=WB957, 2=WB958)
const WB_RD_CMD_TYPE: [u8; 3] = [0xe0, 0xe0, 0x20]; // wrapped/wrapped/linear
const WB_WR_CMD_TYPE: [u8; 3] = [0x60, 0x60, 0x20];

// APM command codes
const APM_RD_CMD: u8 = 0x20;
const APM_WR_CMD: u8 = 0xa0;

// ---------------------------------------------------------------------------
// Type-safe enumerations
// ---------------------------------------------------------------------------

/// PSRAM memory vendor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Vendor {
    Apm = MCM_PSRAM_VENDOR_APM,
    Wb = MCM_PSRAM_VENDOR_WB,
    NotCare = MCM_PSRAM_VENDOR_NOTCARE,
}

/// PSRAM device model (maps to `PSRAM_TYPE_*` in C).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DeviceType {
    Wb955 = 0,
    Wb957 = 1,
    Wb958 = 2,
    Apm64 = 3,
    Apm128 = 4,
}

/// Data bus width.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DqWidth {
    Dq8,
    Dq16,
}

/// Page size encoding for DEVICE_INFO.PAGE_SIZE.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PageSize {
    Page128 = 0,
    Page1024 = 2,
    Page2048 = 3,
}

/// Latency mode: fixed or variable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LatencyMode {
    Variable = 0,
    Fixed = 1,
}

/// PHY read-FIFO ready delay selection.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum RfifoReadyDelay {
    Variable = 2,
    Fixed = 7,
}

// ---------------------------------------------------------------------------
// PHY configuration builder
// ---------------------------------------------------------------------------

/// Configuration parameters for the PSRAM PHY.
#[derive(Clone, Copy, Debug)]
pub struct PhyConfig {
    pub pre_cal_phase: u8,
    pub cfg_cal_jmin: u8,
    pub cfg_cal_jmax: u8,
    pub cfg_cal_j: u8,
    pub cfg_cal_n: u8,
    pub sw_rst_n: bool,
    pub latency_mode: LatencyMode,
    pub rwds_chk_opt: bool,
    pub rwds_pd_en: bool,
    pub cal_rwds_phase: u8,
    pub dq16_en: bool,
    pub rfifo_rdy_dly: RfifoReadyDelay,
}

impl PhyConfig {
    /// Returns default `PhyConfig` suitable for post-silicon bring-up.
    ///
    /// Mirrors `PSRAM_PHY_StructInit` defaults with fixed latency selected.
    pub fn default_for(dq16: bool) -> Self {
        let latency_mode = LatencyMode::Fixed;
        let rfifo_rdy_dly = if latency_mode == LatencyMode::Fixed {
            RfifoReadyDelay::Fixed
        } else {
            RfifoReadyDelay::Variable
        };
        PhyConfig {
            pre_cal_phase: 0x1,
            cfg_cal_jmin: 0x2,
            cfg_cal_jmax: 0x3,
            cfg_cal_j: 0x3,
            cfg_cal_n: 0xa,
            sw_rst_n: true,
            latency_mode,
            rwds_chk_opt: true,
            rwds_pd_en: false,
            cal_rwds_phase: 0x1,
            dq16_en: dq16,
            rfifo_rdy_dly,
        }
    }
}

// ---------------------------------------------------------------------------
// Runtime PSRAM info (caller fills from ChipInfo + clock APIs)
// ---------------------------------------------------------------------------

/// All timing/model information needed to configure the PSRAM subsystem.
#[derive(Clone, Copy, Debug)]
pub struct PsramInfo {
    pub vendor: Vendor,
    pub device_type: DeviceType,
    pub dq_width: DqWidth,
    /// Total PSRAM size in bytes.
    pub size_bytes: u32,
    pub page_size: PageSize,
    /// Maximum device clock limit in Hz (informational only).
    pub clk_limit_hz: u32,
    /// Half of the PSRAM controller input clock (Tpsram = 2 * Tpsramc).
    pub clk_set_hz: u32,
    /// Clock period in picoseconds: `1_000_000_000_000 / input_clk_hz`.
    pub clk_unit_ps: u64,
    /// Latency setting in cycles (vendor-specific, typically 3..10).
    pub latency_set: u8,
    /// TRWR timing in nanoseconds (Winbond only).
    pub trwr_ns: u32,
    /// CSHI timing in nanoseconds (APM only).
    pub cshi_ns: u32,
    /// PDEX CMD0 command word (APM: PSRAM_APM_PDEX_CMD0; WB: PSRAM_WB_PDEX_CMD0).
    pub pdex_cmd0: u32,
    /// PDEX CMD1 command word (APM: PSRAM_APM_PDEX_CMD1; WB: PSRAM_WB_PDEX_CMD1).
    pub pdex_cmd1: u32,
    /// PDX (power-down exit) CS-high time in picoseconds.
    pub pdx_time_ps: u32,
    /// PDE (power-down entry) CS-high time in picoseconds.
    pub pde_time_ps: u32,
    /// Wakeup resume counter value in cycles.
    pub resume_cnt: u32,
}

bitflags! {
    /// Selects which DQ byte lanes are checked during pattern verification.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CalDqGroup: u8 {
        const DQ0_7   = 1 << 0;
        const DQ8_15  = 1 << 1;
        const DQ16_23 = 1 << 2;
        const DQ24_31 = 1 << 3;
    }
}

/// Which calibration slice is being calibrated.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CalSlice {
    /// Slice-0 for 8-bit bus
    Dq8Sl0,
    /// Slice-0 for 16-bit bus (`DQ[7:0]` + `DQ[23:16]`)
    Dq16Sl0,
    /// Slice-1 for 16-bit bus (`DQ[15:8]` + `DQ[31:24]`)
    Dq16Sl1,
}

// ---------------------------------------------------------------------------
// PSRAM driver handle
// ---------------------------------------------------------------------------

/// Handle to the PSRAM controller and PHY.
pub struct Psram {
    ctrl: &'static psram_reg::RegisterBlock,
    phy: &'static psram_phy_reg::RegisterBlock,
}

impl Psram {
    /// Steal ownership of the PSRAM peripheral registers.
    ///
    /// # Safety
    /// The caller must guarantee exclusive access to the PSRAM controller
    /// (0x4011_e000) and PHY (0x4011_f000) for the lifetime of this handle.
    pub unsafe fn steal() -> Self {
        let ctrl = unsafe { &*PsramReg::PTR };
        let phy = unsafe { &*PsramPhyReg::PTR };
        Self { ctrl, phy }
    }

    // -----------------------------------------------------------------------
    // PHY initialisation (PSRAM_PHY_Init / PSRAM_PHY_StructInit)
    // -----------------------------------------------------------------------

    /// Configure the PSRAM PHY according to `cfg`.
    pub fn phy_init(&self, cfg: &PhyConfig) {
        // CAL_PAR layout: [4:0]=N, [11:8]=J, [19:16]=JMAX, [26:24]=JMIN
        let cal_par_val = ((cfg.cfg_cal_jmin as u32) << 24)
            | ((cfg.cfg_cal_jmax as u32) << 16)
            | ((cfg.cfg_cal_j as u32) << 8)
            | (cfg.cfg_cal_n as u32);

        // 0x04 REG_CAL_PAR (SL0)
        self.phy
            .reg_cal_par()
            .write(|w| unsafe { w.bits(cal_par_val) });
        // 0x24 REG_SL1_CAL_PAR
        self.phy
            .reg_sl1_cal_par()
            .write(|w| unsafe { w.bits(cal_par_val) });

        // 0x0c REG_PRE_CAL_PAR — pre_cal_phase in bits[3:0]
        self.phy
            .reg_pre_cal_par()
            .write(|w| unsafe { w.pre_cal_phase().bits(cfg.pre_cal_phase & 0x0f) });
        self.phy
            .reg_sl1_pre_cal_par()
            .write(|w| unsafe { w.sl1_pre_cal_phase().bits(cfg.pre_cal_phase & 0x0f) });

        // 0x18 REG_PHY_CTRL
        self.phy.reg_phy_ctrl().write(|w| unsafe {
            w.psram_sw_rst_n()
                .bit(cfg.sw_rst_n)
                .rwds_chk_opt()
                .bit(cfg.rwds_chk_opt)
                .rwds_pd_en()
                .bit(cfg.rwds_pd_en)
                .psram_dq16_en()
                .bit(cfg.dq16_en)
                .latency_type()
                .bit(cfg.latency_mode == LatencyMode::Fixed)
                .cal_rwds_phase()
                .bits(cfg.cal_rwds_phase & 0x0f)
                .rfifo_rdy_dly()
                .bits(cfg.rfifo_rdy_dly as u8)
        });
    }

    // -----------------------------------------------------------------------
    // Controller initialisation (PSRAM_CTRL_Init)
    // -----------------------------------------------------------------------

    /// Configure the PSRAM controller according to `info`.
    ///
    /// Always uses fixed-latency mode (PSRSAM_FIX_LATENCY == 1 in C SDK).
    /// `chip_cut_a` should be `true` when `EFUSE_GetChipVersion()` returns
    /// `SYSCFG_CUT_VERSION_A`.
    pub fn ctrl_init(&self, info: &PsramInfo, chip_cut_a: bool) {
        let ctrl = self.ctrl;
        let dq16 = info.dq_width == DqWidth::Dq16;
        let is_wb = info.vendor == Vendor::Wb;
        // SDK default is always fixed latency
        const FIX_LAT: bool = true;
        let rfifo_dly = RfifoReadyDelay::Fixed as u32; // 7

        // 0x08 Disable controller
        ctrl.ssienr().write(|w| unsafe { w.bits(0) });

        // 0x00 CTRLR0: DDR on all channels, quad addr/cmd, CK_MTIMES=1
        // DDR_EN[15:13]=0b111, ADDR_CH[19:18]=0b11, CK_MTIMES[27:23]=0b00001,
        // CMD_CH[25:24]=0b11
        ctrl.ctrlr0().write(|w| unsafe {
            w.ddr_en()
                .bits(0b111)
                .addr_ch()
                .bits(0b11)
                .ck_mtimes()
                .bits(0x1)
                .cmd_ch()
                .bits(0b11)
        });

        // 0x14 BAUDR: divide-by-2  (Tpsram = 2*Tpsramc)
        ctrl.baudr().write(|w| unsafe { w.bits(2) });

        // 0x120 VALID_CMD: BIT_FRD_SINGLE(bit2) | BIT_CTRLR0_CH(bit0)
        ctrl.valid_cmd().write(|w| unsafe { w.bits(0x0000_0005) });

        // 0x200..0x208 SEC region 0: scramble whole PSRAM space
        // SEC_Rx_MODE(0)=bits[31:28]=0, BIT_SEC_Rx_EN=bit27
        let sec_end_val = info.size_bytes | (1 << 27);
        ctrl.sec_base_0().write(|w| unsafe { w.bits(0) });
        ctrl.sec_end_0().write(|w| unsafe { w.bits(sec_end_val) });
        // Nonce written with a placeholder; production code should call _rand()
        // via ROM (not available here without the forbidden rtos/ROM call rules).
        ctrl.sec_nonce_0().write(|w| unsafe { w.bits(0xDEAD_BEEF) });

        // 0x148 PDEX_CTRL0: PDE_CMD0
        ctrl.pdex_ctrl0()
            .write(|w| unsafe { w.bits(info.pdex_cmd0) });
        // 0x14c PDEX_CTRL1: PDE_CMD1
        ctrl.pdex_ctrl1()
            .write(|w| unsafe { w.bits(info.pdex_cmd1) });
        // 0x150 PDEX_CTRL2: PDE_CMD_LEN(1) in bits[15:8], PDX_CMD_LEN in bits[7:0]
        let pdx_cmd_len = pdex_cs_h_cnt(40_000u64, info.clk_unit_ps) & 0xff;
        ctrl.pdex_ctrl2()
            .write(|w| unsafe { w.bits((1u32 << 8) | pdx_cmd_len) });

        // 0x154 TPR2: PDX_CS_H_CNT[15:8], PDE_CS_H_CNT[7:0]
        let pde_csh = pdex_cs_h_cnt(info.pde_time_ps as u64, info.clk_unit_ps) & 0xff;
        let pdx_csh = pdex_cs_h_cnt(info.pdx_time_ps as u64, info.clk_unit_ps) & 0xff;
        ctrl.tpr2()
            .write(|w| unsafe { w.bits((pdx_csh << 8) | pde_csh) });

        // ICG_EN: disable CGATE_EN_PDEX (bit8) on cut-A to keep PDEX clk free-running
        if chip_cut_a {
            ctrl.icg_en()
                .modify(|r, w| unsafe { w.cgate_en_pdex().bit(false).bits(r.bits() & !(1 << 8)) });
        }

        if is_wb {
            self.ctrl_init_wb(info, dq16, FIX_LAT, rfifo_dly);
        } else {
            self.ctrl_init_apm(info, dq16, FIX_LAT, rfifo_dly);
        }
    }

    fn ctrl_init_wb(&self, info: &PsramInfo, dq16: bool, fix_lat: bool, rfifo_dly: u32) {
        let ctrl = self.ctrl;
        let page = (info.page_size as u32) << 16;

        if dq16 {
            // BIT_DATA_UNIT_4B(bit9), BIT_DQ16_DATA_CH(bit8), BIT_JEDEC_P2CMF(bit7),
            // BIT_PSRAM(bit5), ATOM_SIZE(2)=(0b10<<3), BIT_RD_PAGE_EN(bit2),
            // BIT_WR_PAGE_EN(bit1)
            ctrl.device_info().write(|w| unsafe {
                w.bits(
                    (1 << 9)
                        | (1 << 8)
                        | (1 << 7)
                        | (1 << 5)
                        | (0b10 << 3)
                        | (1 << 2)
                        | (1 << 1)
                        | page,
                )
            });
        } else {
            // BIT_DATA_UNIT_2B(bit8), BIT_JEDEC_P2CMF(bit7), BIT_PSRAM(bit5),
            // ATOM_SIZE(1)=(0b01<<3)
            ctrl.device_info().write(|w| unsafe {
                w.bits((1 << 8) | (1 << 7) | (1 << 5) | (0b01 << 3) | (1 << 2) | (1 << 1) | page)
            });
            // DATA_CH = octal (0b11 << 20) for 8-IO
            ctrl.ctrlr0()
                .modify(|r, w| unsafe { w.data_ch().bits(0b11).bits(r.bits() | (0b11 << 20)) });
        }

        // 0x110 CTRLR2
        // BIT_SO_DNUM(bit17)=1, TX_FIFO_ENTRY(5)=(5<<8), RX_FIFO_ENTRY(5)=(5<<0),
        // BIT_DM_ACT(bit19)=1, BIT_DM_CA_DIS(bit18)=1,
        // WR_VL_EN(bit16)=0 (fix), RD_VL_EN(bit15)=0, RD_WEIGHT(2)=(2<<13)
        ctrl.ctrlr2().write(|w| unsafe {
            w.bits(
                ((1 << 19)
                    | (1 << 18)
                    | (1 << 17)
                    | (0x5 << 8)
                    | 0x5) // RX_FIFO_ENTRY(bit0)=5; WR_VL_EN/RD_VL_EN(bit16/15)=0
                    | (0x2 << 13),
            )
        });

        // 0x140 TPR1: clear IDLE_WINDOW[19:16] and TPWR[15:8], set CS_ACTIVE_SETUP=2
        ctrl.tpr1().modify(|r, w| unsafe {
            let v = r.bits() & !(0x000f_ff00); // clear idle_window + tpwr
            w.bits(v | 0x2) // CS_ACTIVE_SETUP(2) at bits[1:0]
        });

        // 0x138 TPR0
        let tcem = tcem_cycles(85_000u64, info.clk_unit_ps);
        let csh_wr = trwr_cs_h_dum_len(info.trwr_ns, info.clk_unit_ps);
        // layout: CS_TCEM[31:24] CS_SEQ_TIMEOUT[23:16] CS_ACTIVE_HOLD[15:8]
        //         CS_H_WR_DUM_LEN[7:4] CS_H_RD_DUM_LEN[3:0]
        ctrl.tpr0().write(|w| unsafe {
            w.bits(
                ((tcem << 24)
                    | (0xffu32 << 16)) // CS_SEQ_TIMEOUT=0xff; CS_ACTIVE_HOLD[15:8]=0 (Psram_WB_CSH)
                    | (csh_wr << 4)
                    | csh_wr,
            )
        });

        // 0xe0 READ_FAST_SINGLE (FRD command byte)
        ctrl.read_fast_single()
            .write(|w| unsafe { w.bits(wb_rd_cmd(info.device_type) as u32) });
        // 0xf4 WRITE_SIGNLE
        ctrl.write_signle()
            .write(|w| unsafe { w.bits(wb_wr_cmd(info.device_type) as u32) });

        // AUTO_LENGTH base: AUTO_CMD_LENGTH(1)=bits[22:20], AUTO_ADDR_LENGTH(5)=bits[19:16]
        let auto_len_base: u32 = (0x1 << 20) | (0x5 << 16);
        // 0x118 USER_LENGTH base
        ctrl.user_length()
            .write(|w| unsafe { w.bits((0x5 << 16) | (0x1 << 20)) });

        if fix_lat {
            let rd_dum = 2 * 2 * (info.latency_set as u32) - 1;
            let in_phys = rfifo_dly + 2;
            // 0x11c AUTO_LENGTH: RD_DUMMY[15:8], IN_PHYSICAL_CYC[7:0]
            ctrl.auto_length()
                .write(|w| unsafe { w.bits(auto_len_base | (rd_dum << 8) | in_phys) });
            // 0x13c AUTO_LENGTH2: WR_DUMMY (WB 2xLC write latency)
            ctrl.auto_length2()
                .write(|w| unsafe { w.bits(2 * 2 * (info.latency_set as u32) - 2) });
            // USER_LENGTH: USER_RD_DUMMY_LENGTH[31:24]
            ctrl.user_length()
                .modify(|r, w| unsafe { w.bits(r.bits() | (rd_dum << 24)) });
        } else {
            let rd_dum = 2 * (info.latency_set as u32) - 1;
            ctrl.auto_length()
                .write(|w| unsafe { w.bits(auto_len_base | (rd_dum << 8)) });
            ctrl.auto_length2()
                .write(|w| unsafe { w.bits(2 * (info.latency_set as u32) - 2) });
            ctrl.user_length()
                .modify(|r, w| unsafe { w.bits(r.bits() | (rd_dum << 24)) });
        }
    }

    fn ctrl_init_apm(&self, info: &PsramInfo, dq16: bool, fix_lat: bool, rfifo_dly: u32) {
        let ctrl = self.ctrl;
        let page = (info.page_size as u32) << 16;

        if dq16 {
            // BIT_DATA_UNIT_2B(bit8), BIT_DQ16_DATA_CH(bit7), BIT_PSRAM(bit5),
            // ATOM_SIZE(2)=(0b10<<3)
            ctrl.device_info().write(|w| unsafe {
                w.bits((1 << 8) | (1 << 7) | (1 << 5) | (0b10 << 3) | (1 << 2) | (1 << 1) | page)
            });
        } else {
            // BIT_PSRAM(bit5), ATOM_SIZE(1)=(0b01<<3)
            ctrl.device_info()
                .write(|w| unsafe { w.bits((1 << 5) | (0b01 << 3) | (1 << 2) | (1 << 1) | page) });
            ctrl.ctrlr0()
                .modify(|r, w| unsafe { w.data_ch().bits(0b11).bits(r.bits() | (0b11 << 20)) });
        }

        // 0x110 CTRLR2 (no BIT_DM_CA_DIS for APM)
        ctrl.ctrlr2()
            .write(|w| unsafe { w.bits(((1 << 19) | (1 << 17) | (0x5 << 8) | 0x5) | (0x2 << 13)) });

        // 0x138 TPR0
        let tcem = tcem_cycles(85_000u64, info.clk_unit_ps);
        let csh_wr = apm_cshi_dum_len(info.cshi_ns, info.clk_unit_ps);
        ctrl.tpr0().write(|w| unsafe {
            w.bits(
                ((tcem << 24)
                    | (0xffu32 << 16)) // CS_SEQ_TIMEOUT=0xff; CS_ACTIVE_HOLD[15:8]=0 (Psram_APM_CSH)
                    | (csh_wr << 4)
                    | csh_wr,
            )
        });

        // 0xe0 / 0xf4
        ctrl.read_fast_single()
            .write(|w| unsafe { w.bits(APM_RD_CMD as u32) });
        ctrl.write_signle()
            .write(|w| unsafe { w.bits(APM_WR_CMD as u32) });

        // AUTO_LENGTH base: cmd=2 bytes, addr=4 bytes
        let auto_len_base: u32 = (0x2 << 20) | (0x4 << 16);
        ctrl.user_length()
            .write(|w| unsafe { w.bits((0x4 << 16) | (0x2 << 20)) });

        if fix_lat {
            let rd_dum = 2 * 2 * (info.latency_set as u32) - 1;
            let in_phys = rfifo_dly + 2;
            ctrl.auto_length()
                .write(|w| unsafe { w.bits(auto_len_base | (rd_dum << 8) | in_phys) });
            // APM: 1xLC write latency
            ctrl.auto_length2()
                .write(|w| unsafe { w.bits(2 * (info.latency_set as u32) - 2) });
            ctrl.user_length()
                .modify(|r, w| unsafe { w.bits(r.bits() | (rd_dum << 24)) });
        } else {
            let rd_dum = 2 * (info.latency_set as u32) - 1;
            ctrl.auto_length()
                .write(|w| unsafe { w.bits(auto_len_base | (rd_dum << 8)) });
            ctrl.auto_length2()
                .write(|w| unsafe { w.bits(2 * (info.latency_set as u32) - 2) });
            ctrl.user_length()
                .modify(|r, w| unsafe { w.bits(r.bits() | (rd_dum << 24)) });
        }
    }

    // -----------------------------------------------------------------------
    // PSRAM chip device register write
    // (PSRAM_APM_DEVIC_Init / PSRAM_WB_DEVIC_Init)
    // -----------------------------------------------------------------------

    /// Configure the APM PSRAM device registers (MR0, MR4, MR8).
    pub fn apm_device_init(&self, info: &PsramInfo) {
        let idx = (info.latency_set - 3) as usize;
        // PSRAM_READ_LATENCY_CODE(idx) = idx as 3-bit field
        // PSRAM_LT_SELECT(fix) = bit3 set for fixed latency
        // PSRAM_Drive_Strength(100Ω) = 0 in bits[5:4]
        let mr0 = (idx as u8 & 0x07) | (1 << 3);
        let wlc = APM_WR_INIT_LATENCY_SPEC[idx];
        // PSRAM_WRITE_LATENCY_CODE(wlc) | PSRAM_Refresh_Setting(slow=0)
        let mr4 = wlc & 0xe0;
        // PSRAM_ENBALE_DQ16 = bit2
        let mr8: u8 = 0x04;

        self.reg_write(Vendor::Apm, 0x0, &[mr0, mr0][..1], info.dq_width);
        self.reg_write(Vendor::Apm, 0x4, &[mr4, mr4][..1], info.dq_width);
        if info.dq_width == DqWidth::Dq16 {
            self.reg_write(Vendor::Apm, 0x8, &[mr8, mr8][..1], info.dq_width);
        }
    }

    /// Configure the Winbond PSRAM device register CR0.
    pub fn wb_device_init(&self, info: &PsramInfo) {
        let idx = (info.latency_set - 3) as usize;
        let wlc = WB_WR_INIT_LATENCY_SPEC[idx];

        let (burst_bit8, burst_len): (u8, u8) = if info.device_type == DeviceType::Wb957 {
            (0x00, 0x06) // BIT8_CLR, burst=2048B
        } else {
            (0x80, 0x00) // BIT8_SET, burst=128B
        };

        // data[0] = byte-0 of CR0: burst-bit8, drive-strength=010, DPD=off
        let data0 = burst_bit8 | (0x02 << 2); // WB_Drive_Strength_010 in bits[3:2]; DPD_EN(bit0)=0 (WB_NORMAL_OPE)
        // data[1] = byte-1 of CR0: burst-len, hyper-burst, fix-latency, write-latency
        let data1 = burst_len
            | (1 << 5) // PSRAM_WB_HyBURSE_EN
            | (1 << 3) // PSRAM_WB_FIX_LATENCY_EN = 1 (always fixed)
            | (wlc & 0x07); // PSRAM_WB_INIT_LATENCY lower 3 bits

        self.reg_write(Vendor::Wb, 0x0, &[data0, data1], info.dq_width);
    }

    // -----------------------------------------------------------------------
    // User-mode register read / write (PSRAM_REG_Read / PSRAM_REG_Write)
    // -----------------------------------------------------------------------

    /// Read PSRAM device configuration register(s).
    ///
    /// Returns a `[u8; 2]`; only the first `read_len` bytes are meaningful.
    /// `cr` selects identification (false) or configuration (true) register
    /// space for Winbond devices.
    pub fn reg_read(
        &self,
        vendor: Vendor,
        addr: u32,
        read_len: usize,
        cr: bool,
        dq: DqWidth,
    ) -> [u8; 2] {
        let ctrl = self.ctrl;
        let rx_ndf: u32 = if dq == DqWidth::Dq16 { 4 } else { 2 };
        let command: u32 = if vendor == Vendor::Apm {
            0x4040
        } else {
            0x00e0
        };
        let cr_sel: u32 = if cr { 0x0001 } else { 0x0000 };
        let address: u32 = (addr << 8) & 0xff00;

        // Wait until controller is idle
        while ctrl.sr().read().busy().bit_is_set() {}

        // Enter user mode: USER_MODE = bit31 of CTRLR0
        ctrl.ctrlr0()
            .modify(|r, w| unsafe { w.user_mode().set_bit().bits(r.bits() | (1 << 31)) });

        ctrl.ssienr().write(|w| unsafe { w.bits(0) });
        ctrl.rx_ndf().write(|w| unsafe { w.bits(rx_ndf) });
        ctrl.tx_ndf().write(|w| unsafe { w.bits(0) });
        // TMOD = 0b11 (receive) in bits[9:8]
        ctrl.ctrlr0()
            .modify(|r, w| unsafe { w.tmod().bits(0b11).bits(r.bits() | (0b11 << 8)) });

        // Push half-words into the DR FIFO
        ctrl.dr0()
            .write(|w| unsafe { w.dr0().bits(command & 0xffff) });
        ctrl.dr0()
            .write(|w| unsafe { w.dr0().bits(cr_sel & 0xffff) });
        ctrl.dr0()
            .write(|w| unsafe { w.dr0().bits(address & 0xffff) });

        // Start transaction
        ctrl.ssienr().write(|w| w.spic_en().set_bit());
        while ctrl.ssienr().read().spic_en().bit_is_set() {}

        // Drain RX FIFO
        let rxdata0 = (ctrl.dr0().read().dr0().bits() & 0xffff) as u16;
        let rxdata1: u16 = if rx_ndf >= 4 {
            (ctrl.dr0().read().dr0().bits() & 0xffff) as u16
        } else {
            0
        };

        while ctrl.sr().read().busy().bit_is_set() {}

        // Exit user mode
        ctrl.ctrlr0()
            .modify(|r, w| unsafe { w.user_mode().clear_bit().bits(r.bits() & !(1 << 31)) });

        let mut out = [0u8; 2];
        if vendor == Vendor::Apm {
            out[0] = (rxdata0 & 0xff) as u8;
        } else if dq == DqWidth::Dq16 {
            out[0] = (rxdata0 & 0xff) as u8;
            out[1] = (rxdata1 & 0xff) as u8;
        } else {
            // DQ8 WB: bytes are packed in the 16-bit FIFO entry
            out[0] = (rxdata0 & 0xff) as u8;
            if read_len > 1 {
                out[1] = ((rxdata0 >> 8) & 0xff) as u8;
            }
        }
        out
    }

    /// Write PSRAM device configuration register(s).
    pub fn reg_write(&self, vendor: Vendor, addr: u32, data: &[u8], dq: DqWidth) {
        let ctrl = self.ctrl;
        let command: u32 = if vendor == Vendor::Apm {
            0xc0c0
        } else {
            0x0060
        };
        let cr_sel: u32 = 0x0001;
        let addr_data: u32 = (addr << 8) & 0xff00;
        let tx_ndf: u32 = if dq == DqWidth::Dq16 { 4 } else { 2 };

        while ctrl.sr().read().busy().bit_is_set() {}

        // Enter user mode
        ctrl.ctrlr0()
            .modify(|r, w| unsafe { w.user_mode().set_bit().bits(r.bits() | (1 << 31)) });

        ctrl.ssienr().write(|w| unsafe { w.bits(0) });

        // TX mode: TMOD = 0b00
        ctrl.ctrlr0()
            .modify(|r, w| unsafe { w.tmod().bits(0b00).bits(r.bits() & !(0b11 << 8)) });
        ctrl.rx_ndf().write(|w| unsafe { w.bits(0) });
        ctrl.tx_ndf().write(|w| unsafe { w.bits(tx_ndf) });

        ctrl.dr0()
            .write(|w| unsafe { w.dr0().bits(command & 0xffff) });
        ctrl.dr0()
            .write(|w| unsafe { w.dr0().bits(cr_sel & 0xffff) });
        ctrl.dr0()
            .write(|w| unsafe { w.dr0().bits(addr_data & 0xffff) });

        let mut idx = 0usize;
        while idx < data.len() {
            let byte0 = data[idx] as u32;
            idx += 1;
            let word = if dq != DqWidth::Dq16 && vendor != Vendor::Apm && idx < data.len() {
                let byte1 = data[idx] as u32;
                idx += 1;
                byte0 | (byte1 << 8)
            } else {
                byte0
            };
            ctrl.dr0().write(|w| unsafe { w.dr0().bits(word & 0xffff) });
            // APM DQ16: each data half-word is followed by a zero padding half-word
            if vendor == Vendor::Apm && dq == DqWidth::Dq16 {
                ctrl.dr0().write(|w| unsafe { w.dr0().bits(0) });
            }
        }

        ctrl.ssienr().write(|w| w.spic_en().set_bit());
        while ctrl.ssienr().read().spic_en().bit_is_set() {}
        while ctrl.sr().read().busy().bit_is_set() {}

        // Exit user mode
        ctrl.ctrlr0()
            .modify(|r, w| unsafe { w.user_mode().clear_bit().bits(r.bits() & !(1 << 31)) });
    }

    // -----------------------------------------------------------------------
    // SW calibration (PSRAM_SW_Calibration / PSRAM_calibration)
    //
    // DCache and PSRAM memory writes/reads are delegated to the caller via a
    // closure (avoids the forbidden DCache_CleanInvalidate dependency).
    // -----------------------------------------------------------------------

    /// Software-calibrate one DQ slice.
    ///
    /// `read_fn(ptr, write_val) -> read_val` must:
    ///   1. Write `write_val` to the volatile `*mut u32` pointer.
    ///   2. Clean-invalidate the cache line covering that address.
    ///   3. Read and return the 32-bit value.
    ///
    /// `psram_base` is the virtual base of the PSRAM memory window.
    ///
    /// Returns `true` when a calibration window of at least 9 was found.
    ///
    /// # Safety
    /// `psram_base` must be a valid, mapped PSRAM window pointer, and `read_fn`
    /// must uphold its contract (volatile write, cache clean-invalidate, read
    /// back). This mutates PSRAM controller calibration registers, so the caller
    /// must ensure no concurrent PSRAM access occurs during calibration.
    pub unsafe fn sw_calibration<F>(
        &self,
        slice: CalSlice,
        read_fn: F,
        psram_base: *mut u32,
    ) -> bool
    where
        F: Fn(*mut u32, u32) -> u32,
    {
        let phy = self.phy;
        let is_sl0 = slice == CalSlice::Dq8Sl0 || slice == CalSlice::Dq16Sl0;

        // Disable HW calibration and snapshot baseline (mask CFG_CAL_N [4:0])
        let temp_par_base: u32 = if is_sl0 {
            phy.reg_cal_ctrl()
                .modify(|r, w| unsafe { w.cfg_cal_en().clear_bit().bits(r.bits() & !0x1) });
            let par = phy.reg_cal_par().read().bits();
            phy.reg_pre_cal_par()
                .modify(|r, w| unsafe { w.pre_cal_phase().bits(0).bits(r.bits() & !0x0f) });
            par & !0x1f // zero out CFG_CAL_N field
        } else {
            phy.reg_sl1_cal_ctrl()
                .modify(|r, w| unsafe { w.sl1_cfg_cal_en().clear_bit().bits(r.bits() & !0x1) });
            let par = phy.reg_sl1_cal_par().read().bits();
            phy.reg_sl1_pre_cal_par()
                .modify(|r, w| unsafe { w.sl1_pre_cal_phase().bits(0).bits(r.bits() & !0x0f) });
            par & !0x1f
        };

        let mut window_start: i32 = -1;
        let mut window_end: i32 = -1;
        let mut window_size: i32 = 0;
        let mut best_phase: i32 = -1;

        for phase in 0u32..8 {
            let mut wt_start: i32 = -1;
            let mut wt_size: i32 = 0;
            let mut wt_end: i32 = -1;

            for cal_n in 0u32..32 {
                // Program phase and N
                if is_sl0 {
                    phy.reg_pre_cal_par().modify(|r, w| unsafe {
                        w.pre_cal_phase()
                            .bits((phase & 0x0f) as u8)
                            .bits((r.bits() & !0x0f) | phase)
                    });
                    phy.reg_cal_par()
                        .write(|w| unsafe { w.bits(temp_par_base | cal_n) });
                } else {
                    phy.reg_sl1_pre_cal_par().modify(|r, w| unsafe {
                        w.sl1_pre_cal_phase()
                            .bits((phase & 0x0f) as u8)
                            .bits((r.bits() & !0x0f) | phase)
                    });
                    phy.reg_sl1_cal_par()
                        .write(|w| unsafe { w.bits(temp_par_base | cal_n) });
                }

                // Write + read-back each calibration word
                let mut data_rd = [0u32; 6];
                for i in 0..6usize {
                    // stride = 0x50000 bytes = 0x14000 u32 words
                    let ptr = unsafe { psram_base.add(i * 0x14000) };
                    data_rd[i] = read_fn(ptr, CALIB_PATTERN[i]);
                }

                let ok = match slice {
                    CalSlice::Dq16Sl0 => {
                        pattern_check_byte(0, &data_rd) && pattern_check_byte(2, &data_rd)
                    }
                    CalSlice::Dq16Sl1 => {
                        pattern_check_byte(1, &data_rd) && pattern_check_byte(3, &data_rd)
                    }
                    CalSlice::Dq8Sl0 => data_rd == CALIB_PATTERN,
                };

                if ok {
                    if wt_start < 0 {
                        wt_start = cal_n as i32;
                    }
                    wt_end = wt_start + wt_size;
                    wt_size += 1;

                    if cal_n == 31 && wt_size > window_size {
                        window_start = wt_start;
                        window_end = wt_end;
                        window_size = wt_size;
                        best_phase = phase as i32;
                    }
                } else if wt_start >= 0 {
                    if wt_size > window_size {
                        window_start = wt_start;
                        window_end = wt_end;
                        window_size = wt_size;
                        best_phase = phase as i32;
                    }
                    wt_start = -1;
                    wt_size = 0;
                    wt_end = -1;
                }
            }
        }

        if window_size < 9 {
            return false;
        }

        // Program optimal N/J into hardware
        let half = ((window_end - window_start) / 2 - 2).max(0) as u32;
        let mid_n = ((window_end + window_start) / 2) as u32;

        // Reconstruct par: keep upper bits (JMIN), overwrite JMAX/J/N
        let final_par = (temp_par_base & 0xff00_0000) // preserve JMIN
            | (half << 16)  // CFG_CAL_JMAX
            | (half << 8)   // CFG_CAL_J
            | (mid_n & 0x1f); // CFG_CAL_N

        if is_sl0 {
            phy.reg_cal_par().write(|w| unsafe { w.bits(final_par) });
            phy.reg_pre_cal_par()
                .write(|w| unsafe { w.bits(best_phase as u32 & 0x0f) });
            phy.reg_cal_ctrl()
                .modify(|r, w| unsafe { w.cfg_cal_en().set_bit().bits(r.bits() | 0x1) });
        } else {
            phy.reg_sl1_cal_par()
                .write(|w| unsafe { w.bits(final_par) });
            phy.reg_sl1_pre_cal_par()
                .write(|w| unsafe { w.bits(best_phase as u32 & 0x0f) });
            phy.reg_sl1_cal_ctrl()
                .modify(|r, w| unsafe { w.sl1_cfg_cal_en().set_bit().bits(r.bits() | 0x1) });
        }

        true
    }

    /// Calibrate all slices (DQ8 = 1 slice; DQ16 = 2 slices).
    ///
    /// Returns `true` only if every slice passes.
    ///
    /// # Safety
    /// Same requirements as [`Psram::sw_calibration`]: `psram_base` must be a
    /// valid mapped PSRAM window pointer, `read_fn` must uphold its contract,
    /// and no concurrent PSRAM access may occur during calibration.
    pub unsafe fn calibration<F>(&self, dq: DqWidth, read_fn: F, psram_base: *mut u32) -> bool
    where
        F: Fn(*mut u32, u32) -> u32,
    {
        unsafe {
            if dq == DqWidth::Dq16 {
                self.sw_calibration(CalSlice::Dq16Sl0, &read_fn, psram_base)
                    && self.sw_calibration(CalSlice::Dq16Sl1, &read_fn, psram_base)
            } else {
                self.sw_calibration(CalSlice::Dq8Sl0, &read_fn, psram_base)
            }
        }
    }

    // -----------------------------------------------------------------------
    // Sleep / wakeup helpers
    // (set_psram_sleep_mode / set_psram_wakeup_mode — pure register portions)
    //
    // NOTE: The C `set_psram_wakeup_mode` also calls DCache_Invalidate and
    // reads from PSRAM memory; those steps must be done by the caller before
    // invoking `phy_info_reset` and `*_post_wakeup_delay`.
    // -----------------------------------------------------------------------

    /// Freeze HW calibration by writing the ITT mask.
    ///
    /// Called at sleep entry to prevent spurious calibration updates.
    pub fn phy_cal_freeze(&self) {
        self.phy
            .reg_cal_ctrl()
            .write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    }

    /// APM: enable slow refresh and send half-sleep command.
    pub fn apm_enter_sleep(&self, dq: DqWidth) {
        let mut mr4 = self.reg_read(Vendor::Apm, 0x4, 1, true, dq);
        mr4[0] |= 0x08; // PSRAM_ENABLE_SLOW_REFRESH
        mr4[1] = mr4[0];
        self.reg_write(Vendor::Apm, 0x4, &mr4[..1], dq);
        self.reg_write(Vendor::Apm, 0x06, &[0xF0u8][..], dq);
    }

    /// Winbond: send half-sleep command to CR1.
    pub fn wb_enter_sleep(&self, dq: DqWidth) {
        self.reg_write(Vendor::Wb, 0x1, &[0xFFu8, 0x20u8], dq);
    }

    /// APM: delay 150 µs for PSRAM exit-sleep timing.
    pub fn apm_post_wakeup_delay(&self) {
        Delay::new().delay_us(150);
    }

    /// Winbond: delay 100 µs for PSRAM exit-sleep timing.
    pub fn wb_post_wakeup_delay(&self) {
        Delay::new().delay_us(100);
    }

    /// Workaround for PHY hang after PG wakeup (read-back-write PSPHY_PHY_INFO).
    pub fn phy_info_reset(&self) {
        let v = self.phy.reg_phy_info().read().bits();
        self.phy.reg_phy_info().write(|w| unsafe { w.bits(v) });
    }

    /// APM: disable slow refresh bit in MR4 after wakeup.
    pub fn apm_disable_slow_refresh(&self, dq: DqWidth) {
        let mut mr4 = self.reg_read(Vendor::Apm, 0x4, 1, true, dq);
        mr4[0] &= !0x08;
        mr4[1] = mr4[0];
        self.reg_write(Vendor::Apm, 0x4, &mr4[..1], dq);
    }
}

// ---------------------------------------------------------------------------
// Private arithmetic helpers
// ---------------------------------------------------------------------------

/// CS_TCEM divider: floor(tcem_ns * 1000 / clk_unit_ps / 32) - 2.
fn tcem_cycles(tcem_ns: u64, clk_unit_ps: u64) -> u32 {
    if clk_unit_ps == 0 {
        return 0;
    }
    (tcem_ns * 1000 / clk_unit_ps / 32).saturating_sub(2) as u32
}

/// CS high dummy cycles for Winbond TRWR timing.
///
/// Returns 2 when TRWR fits within 8 clock cycles; otherwise computes
/// ceil(TRWR_ps / clk_unit_ps) - 3(cmd+addr) - 3(Tcs) - 2.
fn trwr_cs_h_dum_len(trwr_ns: u32, clk_unit_ps: u64) -> u32 {
    let trwr_ps = (trwr_ns as u64) * 1000;
    if clk_unit_ps == 0 || 8 * clk_unit_ps > trwr_ps {
        return 2;
    }
    let clks = trwr_ps.div_ceil(clk_unit_ps);
    clks.saturating_sub(3 + 3 + 2) as u32
}

/// CS high dummy cycles for APM CSHI timing.
///
/// Returns ceil(cshi_ns * 1000 / clk_unit_ps) - 2, floored to 1.
fn apm_cshi_dum_len(cshi_ns: u32, clk_unit_ps: u64) -> u32 {
    if clk_unit_ps == 0 || cshi_ns == 0 {
        return 1;
    }
    let cshi_ps = (cshi_ns as u64) * 1000;
    if cshi_ps < clk_unit_ps {
        return 1;
    }
    let clks = cshi_ps.div_ceil(clk_unit_ps);
    clks.saturating_sub(2).max(1) as u32
}

/// ceil(time_ps / clk_unit_ps).
fn pdex_cs_h_cnt(time_ps: u64, clk_unit_ps: u64) -> u32 {
    if clk_unit_ps == 0 {
        return 0;
    }
    time_ps.div_ceil(clk_unit_ps) as u32
}

/// WB auto-read command byte for a given device type.
fn wb_rd_cmd(dt: DeviceType) -> u8 {
    WB_RD_CMD_TYPE[match dt {
        DeviceType::Wb955 => 0,
        DeviceType::Wb957 => 1,
        DeviceType::Wb958 => 2,
        _ => 0,
    }]
}

/// WB auto-write command byte for a given device type.
fn wb_wr_cmd(dt: DeviceType) -> u8 {
    WB_WR_CMD_TYPE[match dt {
        DeviceType::Wb955 => 0,
        DeviceType::Wb957 => 1,
        DeviceType::Wb958 => 2,
        _ => 0,
    }]
}

/// Check if all 6 calibration words match on byte lane `byte_idx`
/// (0=DQ[7:0], 1=DQ[15:8], 2=DQ[23:16], 3=DQ[31:24]).
fn pattern_check_byte(byte_idx: u32, data: &[u32; 6]) -> bool {
    for i in 0..6 {
        let got = (data[i] >> (byte_idx * 8)) & 0xff;
        let exp = (CALIB_PATTERN[i] >> (byte_idx * 8)) & 0xff;
        if got != exp {
            return false;
        }
    }
    true
}
