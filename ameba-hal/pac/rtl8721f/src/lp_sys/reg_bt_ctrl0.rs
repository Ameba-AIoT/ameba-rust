#[doc = "Register `REG_BT_CTRL0` reader"]
pub type R = crate::R<RegBtCtrl0Spec>;
#[doc = "Register `REG_BT_CTRL0` writer"]
pub type W = crate::W<RegBtCtrl0Spec>;
#[doc = "Field `BT_SWR_STS_LPS` reader - SET SWR state when BT in LPS mode 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
pub type BtSwrStsLpsR = crate::FieldReader;
#[doc = "Field `BT_SWR_STS_LPS` writer - SET SWR state when BT in LPS mode 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
pub type BtSwrStsLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSWR_REF_BT` reader - core swr parameter refer BT status"]
pub type DswrRefBtR = crate::BitReader;
#[doc = "Field `DSWR_REF_BT` writer - core swr parameter refer BT status"]
pub type DswrRefBtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_SYS_BTDBG_SEL` reader - 0: select debug port from BTTOP 1: debug_port\\[31:18\\] from bt_misc; else from BTTOP"]
pub type BtSysBtdbgSelR = crate::BitReader;
#[doc = "Field `BT_SYS_BTDBG_SEL` writer - 0: select debug port from BTTOP 1: debug_port\\[31:18\\] from bt_misc; else from BTTOP"]
pub type BtSysBtdbgSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_BT_OFF_PARAM` reader - level -> pulse. Write to 1 to disable XTAL and SWR, before BT OFF. Before Write to 1 or after BT OFF,need to write to 0"]
pub type SetBtOffParamR = crate::BitReader;
#[doc = "Field `SET_BT_OFF_PARAM` writer - level -> pulse. Write to 1 to disable XTAL and SWR, before BT OFF. Before Write to 1 or after BT OFF,need to write to 0"]
pub type SetBtOffParamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_MEM_PWRC_LPS` reader - BT DMEM/BFMEM power mode control in SLEEP. Setted before chip goes to SLEEP.Used to controll power mode of BT memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. 000 - active 001 - light sleep 010 - deep sleep 100 - shut down"]
pub type BtMemPwrcLpsR = crate::FieldReader;
#[doc = "Field `BT_MEM_PWRC_LPS` writer - BT DMEM/BFMEM power mode control in SLEEP. Setted before chip goes to SLEEP.Used to controll power mode of BT memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. 000 - active 001 - light sleep 010 - deep sleep 100 - shut down"]
pub type BtMemPwrcLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BT_I2C_TEST_EN` reader - SW control BT enter I2C test mode"]
pub type BtI2cTestEnR = crate::BitReader;
#[doc = "Field `BT_I2C_TEST_EN` writer - SW control BT enter I2C test mode"]
pub type BtI2cTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_XTAL_MODE_LPS` reader - SET XTAL state when BT in LPS mode 000: XTAL OFF 001: XTAL LPS, gating XTAL 40M 010: XTAL LPS, enable XTAL 40M 011: XTAL NM mode 100: XTAL HP mode"]
pub type BtXtalModeLpsR = crate::FieldReader;
#[doc = "Field `BT_XTAL_MODE_LPS` writer - SET XTAL state when BT in LPS mode 000: XTAL OFF 001: XTAL LPS, gating XTAL 40M 010: XTAL LPS, enable XTAL 40M 011: XTAL NM mode 100: XTAL HP mode"]
pub type BtXtalModeLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HOST_WAKE_BT` reader - CPU set this bit to 1 to wake BT"]
pub type HostWakeBtR = crate::BitReader;
#[doc = "Field `HOST_WAKE_BT` writer - CPU set this bit to 1 to wake BT"]
pub type HostWakeBtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_WAKE_BT_POL` reader - 1:HOST_WAKE_BT to BT and pinmux 0:pinmux to BT for external trig"]
pub type HostWakeBtPolR = crate::BitReader;
#[doc = "Field `HOST_WAKE_BT_POL` writer - 1:HOST_WAKE_BT to BT and pinmux 0:pinmux to BT for external trig"]
pub type HostWakeBtPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_WAKE_HOST_POL` reader - 1:BT_WAKE_HOST to CPU and pinmux 0:pinmux to CPU for external trig"]
pub type BtWakeHostPolR = crate::BitReader;
#[doc = "Field `BT_WAKE_HOST_POL` writer - 1:BT_WAKE_HOST to CPU and pinmux 0:pinmux to CPU for external trig"]
pub type BtWakeHostPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_RSVD_BT_SW2` reader - RSVD for sotfware"]
pub type BootRsvdBtSw2R = crate::BitReader;
#[doc = "Field `BOOT_RSVD_BT_SW2` writer - RSVD for sotfware"]
pub type BootRsvdBtSw2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_RSVD_BT_SW1` reader - RSVD for sotfware"]
pub type BootRsvdBtSw1R = crate::BitReader;
#[doc = "Field `BOOT_RSVD_BT_SW1` writer - RSVD for sotfware"]
pub type BootRsvdBtSw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_CTRL_RFC_SEL_SI` reader - In Normal mode or Test Mode 2, this register selects the RFC control source. BT_CTRL_RFC_SEL_SI = 1 hands the RFC-side SI 3 wire interface over to the dedicated BT_SI_*_TST lines; BT_CTRL_RFC_SEL_SI = 0 hands the RFC-side SI 3 wire interface over to I2C control. When BT_CTRL_RFC_SEL_SI = 1, it can also be handed over to PI control by pulling BT_SI_ENB_TST high"]
pub type BtCtrlRfcSelSiR = crate::BitReader;
#[doc = "Field `BT_CTRL_RFC_SEL_SI` writer - In Normal mode or Test Mode 2, this register selects the RFC control source. BT_CTRL_RFC_SEL_SI = 1 hands the RFC-side SI 3 wire interface over to the dedicated BT_SI_*_TST lines; BT_CTRL_RFC_SEL_SI = 0 hands the RFC-side SI 3 wire interface over to I2C control. When BT_CTRL_RFC_SEL_SI = 1, it can also be handed over to PI control by pulling BT_SI_ENB_TST high"]
pub type BtCtrlRfcSelSiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_USE_EXT_UART` reader - 1: BT use external uart 0: BT use internal uart"]
pub type BtUseExtUartR = crate::BitReader;
#[doc = "Field `BT_USE_EXT_UART` writer - 1: BT use external uart 0: BT use internal uart"]
pub type BtUseExtUartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_EN_TST_SI_CTRL_RFC` reader - Only when Test Mode 8 is entered and BT_EN_TST_SI_CTRL_RFC=1 at the same time is the RFC-side SI 3 wire interface handed over to the dedicated BT_SI_*_TST lines for control; If I2C wants to access RFC, set BT_EN_TST_SI_CTRL_RFC=0 or pull BT_SI_ENB_TST high (if BT_SI_ENB_TST has a broken-out PIN)"]
pub type BtEnTstSiCtrlRfcR = crate::BitReader;
#[doc = "Field `BT_EN_TST_SI_CTRL_RFC` writer - Only when Test Mode 8 is entered and BT_EN_TST_SI_CTRL_RFC=1 at the same time is the RFC-side SI 3 wire interface handed over to the dedicated BT_SI_*_TST lines for control; If I2C wants to access RFC, set BT_EN_TST_SI_CTRL_RFC=0 or pull BT_SI_ENB_TST high (if BT_SI_ENB_TST has a broken-out PIN)"]
pub type BtEnTstSiCtrlRfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_XTAL_MODE_NM` reader - SET XTAL state when BT in active mode 000: XTAL OFF 001: XTAL LPS, gating XTAL 40M 010: XTAL LPS, enable XTAL 40M 011: XTAL NM mode 100: XTAL HP mode"]
pub type BtXtalModeNmR = crate::FieldReader;
#[doc = "Field `BT_XTAL_MODE_NM` writer - SET XTAL state when BT in active mode 000: XTAL OFF 001: XTAL LPS, gating XTAL 40M 010: XTAL LPS, enable XTAL 40M 011: XTAL NM mode 100: XTAL HP mode"]
pub type BtXtalModeNmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FORCE_LOGUART_USE_LOGUART_PAD_B` reader - Force wifi soc to use loguart pad mandatory if necessary 0: loguart use loguart trx pad 1: who uses loguart trx pad depends on BTWL_USE_LOGUART_PAD_ST"]
pub type ForceLoguartUseLoguartPadBR = crate::BitReader;
#[doc = "Field `FORCE_LOGUART_USE_LOGUART_PAD_B` writer - Force wifi soc to use loguart pad mandatory if necessary 0: loguart use loguart trx pad 1: who uses loguart trx pad depends on BTWL_USE_LOGUART_PAD_ST"]
pub type ForceLoguartUseLoguartPadBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL_USE_REQ` reader - who uses loguart trx pad depends on the xor of WL_USE_REQ and BT_USE_REQ, wifi can send use request by change the value of WL_USE_REQ"]
pub type WlUseReqR = crate::BitReader;
#[doc = "Field `WL_USE_REQ` writer - who uses loguart trx pad depends on the xor of WL_USE_REQ and BT_USE_REQ, wifi can send use request by change the value of WL_USE_REQ"]
pub type WlUseReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_USE_EXT_BT` reader - 1: select external BT 0: select internal BT"]
pub type BtUseExtBtR = crate::BitReader;
#[doc = "Field `BT_USE_EXT_BT` writer - 1: select external BT 0: select internal BT"]
pub type BtUseExtBtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL_MP_STATUS` reader - Wi-Fi MP status"]
pub type WlMpStatusR = crate::BitReader;
#[doc = "Field `WL_MP_STATUS` writer - Wi-Fi MP status"]
pub type WlMpStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTWL_USE_LOGUART_PAD_ST` reader - The state of xor of WL_USE_REQ and BT_USE_REQ. 0: loguart use loguart trx pad 1: bt hci use loguart trx pad"]
pub type BtwlUseLoguartPadStR = crate::BitReader;
#[doc = "Field `BT_SWR_STS_NM` reader - SET SWR state when BT in active mode 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
pub type BtSwrStsNmR = crate::FieldReader;
#[doc = "Field `BT_SWR_STS_NM` writer - SET SWR state when BT in active mode 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
pub type BtSwrStsNmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SET SWR state when BT in LPS mode 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
    #[inline(always)]
    pub fn bt_swr_sts_lps(&self) -> BtSwrStsLpsR {
        BtSwrStsLpsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - core swr parameter refer BT status"]
    #[inline(always)]
    pub fn dswr_ref_bt(&self) -> DswrRefBtR {
        DswrRefBtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: select debug port from BTTOP 1: debug_port\\[31:18\\] from bt_misc; else from BTTOP"]
    #[inline(always)]
    pub fn bt_sys_btdbg_sel(&self) -> BtSysBtdbgSelR {
        BtSysBtdbgSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - level -> pulse. Write to 1 to disable XTAL and SWR, before BT OFF. Before Write to 1 or after BT OFF,need to write to 0"]
    #[inline(always)]
    pub fn set_bt_off_param(&self) -> SetBtOffParamR {
        SetBtOffParamR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - BT DMEM/BFMEM power mode control in SLEEP. Setted before chip goes to SLEEP.Used to controll power mode of BT memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. 000 - active 001 - light sleep 010 - deep sleep 100 - shut down"]
    #[inline(always)]
    pub fn bt_mem_pwrc_lps(&self) -> BtMemPwrcLpsR {
        BtMemPwrcLpsR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - SW control BT enter I2C test mode"]
    #[inline(always)]
    pub fn bt_i2c_test_en(&self) -> BtI2cTestEnR {
        BtI2cTestEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - SET XTAL state when BT in LPS mode 000: XTAL OFF 001: XTAL LPS, gating XTAL 40M 010: XTAL LPS, enable XTAL 40M 011: XTAL NM mode 100: XTAL HP mode"]
    #[inline(always)]
    pub fn bt_xtal_mode_lps(&self) -> BtXtalModeLpsR {
        BtXtalModeLpsR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - CPU set this bit to 1 to wake BT"]
    #[inline(always)]
    pub fn host_wake_bt(&self) -> HostWakeBtR {
        HostWakeBtR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1:HOST_WAKE_BT to BT and pinmux 0:pinmux to BT for external trig"]
    #[inline(always)]
    pub fn host_wake_bt_pol(&self) -> HostWakeBtPolR {
        HostWakeBtPolR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1:BT_WAKE_HOST to CPU and pinmux 0:pinmux to CPU for external trig"]
    #[inline(always)]
    pub fn bt_wake_host_pol(&self) -> BtWakeHostPolR {
        BtWakeHostPolR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RSVD for sotfware"]
    #[inline(always)]
    pub fn boot_rsvd_bt_sw2(&self) -> BootRsvdBtSw2R {
        BootRsvdBtSw2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RSVD for sotfware"]
    #[inline(always)]
    pub fn boot_rsvd_bt_sw1(&self) -> BootRsvdBtSw1R {
        BootRsvdBtSw1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - In Normal mode or Test Mode 2, this register selects the RFC control source. BT_CTRL_RFC_SEL_SI = 1 hands the RFC-side SI 3 wire interface over to the dedicated BT_SI_*_TST lines; BT_CTRL_RFC_SEL_SI = 0 hands the RFC-side SI 3 wire interface over to I2C control. When BT_CTRL_RFC_SEL_SI = 1, it can also be handed over to PI control by pulling BT_SI_ENB_TST high"]
    #[inline(always)]
    pub fn bt_ctrl_rfc_sel_si(&self) -> BtCtrlRfcSelSiR {
        BtCtrlRfcSelSiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: BT use external uart 0: BT use internal uart"]
    #[inline(always)]
    pub fn bt_use_ext_uart(&self) -> BtUseExtUartR {
        BtUseExtUartR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Only when Test Mode 8 is entered and BT_EN_TST_SI_CTRL_RFC=1 at the same time is the RFC-side SI 3 wire interface handed over to the dedicated BT_SI_*_TST lines for control; If I2C wants to access RFC, set BT_EN_TST_SI_CTRL_RFC=0 or pull BT_SI_ENB_TST high (if BT_SI_ENB_TST has a broken-out PIN)"]
    #[inline(always)]
    pub fn bt_en_tst_si_ctrl_rfc(&self) -> BtEnTstSiCtrlRfcR {
        BtEnTstSiCtrlRfcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - SET XTAL state when BT in active mode 000: XTAL OFF 001: XTAL LPS, gating XTAL 40M 010: XTAL LPS, enable XTAL 40M 011: XTAL NM mode 100: XTAL HP mode"]
    #[inline(always)]
    pub fn bt_xtal_mode_nm(&self) -> BtXtalModeNmR {
        BtXtalModeNmR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Force wifi soc to use loguart pad mandatory if necessary 0: loguart use loguart trx pad 1: who uses loguart trx pad depends on BTWL_USE_LOGUART_PAD_ST"]
    #[inline(always)]
    pub fn force_loguart_use_loguart_pad_b(&self) -> ForceLoguartUseLoguartPadBR {
        ForceLoguartUseLoguartPadBR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - who uses loguart trx pad depends on the xor of WL_USE_REQ and BT_USE_REQ, wifi can send use request by change the value of WL_USE_REQ"]
    #[inline(always)]
    pub fn wl_use_req(&self) -> WlUseReqR {
        WlUseReqR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: select external BT 0: select internal BT"]
    #[inline(always)]
    pub fn bt_use_ext_bt(&self) -> BtUseExtBtR {
        BtUseExtBtR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wi-Fi MP status"]
    #[inline(always)]
    pub fn wl_mp_status(&self) -> WlMpStatusR {
        WlMpStatusR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The state of xor of WL_USE_REQ and BT_USE_REQ. 0: loguart use loguart trx pad 1: bt hci use loguart trx pad"]
    #[inline(always)]
    pub fn btwl_use_loguart_pad_st(&self) -> BtwlUseLoguartPadStR {
        BtwlUseLoguartPadStR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - SET SWR state when BT in active mode 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
    #[inline(always)]
    pub fn bt_swr_sts_nm(&self) -> BtSwrStsNmR {
        BtSwrStsNmR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SET SWR state when BT in LPS mode 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
    #[inline(always)]
    pub fn bt_swr_sts_lps(&mut self) -> BtSwrStsLpsW<'_, RegBtCtrl0Spec> {
        BtSwrStsLpsW::new(self, 0)
    }
    #[doc = "Bit 3 - core swr parameter refer BT status"]
    #[inline(always)]
    pub fn dswr_ref_bt(&mut self) -> DswrRefBtW<'_, RegBtCtrl0Spec> {
        DswrRefBtW::new(self, 3)
    }
    #[doc = "Bit 4 - 0: select debug port from BTTOP 1: debug_port\\[31:18\\] from bt_misc; else from BTTOP"]
    #[inline(always)]
    pub fn bt_sys_btdbg_sel(&mut self) -> BtSysBtdbgSelW<'_, RegBtCtrl0Spec> {
        BtSysBtdbgSelW::new(self, 4)
    }
    #[doc = "Bit 5 - level -> pulse. Write to 1 to disable XTAL and SWR, before BT OFF. Before Write to 1 or after BT OFF,need to write to 0"]
    #[inline(always)]
    pub fn set_bt_off_param(&mut self) -> SetBtOffParamW<'_, RegBtCtrl0Spec> {
        SetBtOffParamW::new(self, 5)
    }
    #[doc = "Bits 6:8 - BT DMEM/BFMEM power mode control in SLEEP. Setted before chip goes to SLEEP.Used to controll power mode of BT memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. 000 - active 001 - light sleep 010 - deep sleep 100 - shut down"]
    #[inline(always)]
    pub fn bt_mem_pwrc_lps(&mut self) -> BtMemPwrcLpsW<'_, RegBtCtrl0Spec> {
        BtMemPwrcLpsW::new(self, 6)
    }
    #[doc = "Bit 9 - SW control BT enter I2C test mode"]
    #[inline(always)]
    pub fn bt_i2c_test_en(&mut self) -> BtI2cTestEnW<'_, RegBtCtrl0Spec> {
        BtI2cTestEnW::new(self, 9)
    }
    #[doc = "Bits 10:12 - SET XTAL state when BT in LPS mode 000: XTAL OFF 001: XTAL LPS, gating XTAL 40M 010: XTAL LPS, enable XTAL 40M 011: XTAL NM mode 100: XTAL HP mode"]
    #[inline(always)]
    pub fn bt_xtal_mode_lps(&mut self) -> BtXtalModeLpsW<'_, RegBtCtrl0Spec> {
        BtXtalModeLpsW::new(self, 10)
    }
    #[doc = "Bit 13 - CPU set this bit to 1 to wake BT"]
    #[inline(always)]
    pub fn host_wake_bt(&mut self) -> HostWakeBtW<'_, RegBtCtrl0Spec> {
        HostWakeBtW::new(self, 13)
    }
    #[doc = "Bit 14 - 1:HOST_WAKE_BT to BT and pinmux 0:pinmux to BT for external trig"]
    #[inline(always)]
    pub fn host_wake_bt_pol(&mut self) -> HostWakeBtPolW<'_, RegBtCtrl0Spec> {
        HostWakeBtPolW::new(self, 14)
    }
    #[doc = "Bit 15 - 1:BT_WAKE_HOST to CPU and pinmux 0:pinmux to CPU for external trig"]
    #[inline(always)]
    pub fn bt_wake_host_pol(&mut self) -> BtWakeHostPolW<'_, RegBtCtrl0Spec> {
        BtWakeHostPolW::new(self, 15)
    }
    #[doc = "Bit 16 - RSVD for sotfware"]
    #[inline(always)]
    pub fn boot_rsvd_bt_sw2(&mut self) -> BootRsvdBtSw2W<'_, RegBtCtrl0Spec> {
        BootRsvdBtSw2W::new(self, 16)
    }
    #[doc = "Bit 17 - RSVD for sotfware"]
    #[inline(always)]
    pub fn boot_rsvd_bt_sw1(&mut self) -> BootRsvdBtSw1W<'_, RegBtCtrl0Spec> {
        BootRsvdBtSw1W::new(self, 17)
    }
    #[doc = "Bit 18 - In Normal mode or Test Mode 2, this register selects the RFC control source. BT_CTRL_RFC_SEL_SI = 1 hands the RFC-side SI 3 wire interface over to the dedicated BT_SI_*_TST lines; BT_CTRL_RFC_SEL_SI = 0 hands the RFC-side SI 3 wire interface over to I2C control. When BT_CTRL_RFC_SEL_SI = 1, it can also be handed over to PI control by pulling BT_SI_ENB_TST high"]
    #[inline(always)]
    pub fn bt_ctrl_rfc_sel_si(&mut self) -> BtCtrlRfcSelSiW<'_, RegBtCtrl0Spec> {
        BtCtrlRfcSelSiW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: BT use external uart 0: BT use internal uart"]
    #[inline(always)]
    pub fn bt_use_ext_uart(&mut self) -> BtUseExtUartW<'_, RegBtCtrl0Spec> {
        BtUseExtUartW::new(self, 19)
    }
    #[doc = "Bit 20 - Only when Test Mode 8 is entered and BT_EN_TST_SI_CTRL_RFC=1 at the same time is the RFC-side SI 3 wire interface handed over to the dedicated BT_SI_*_TST lines for control; If I2C wants to access RFC, set BT_EN_TST_SI_CTRL_RFC=0 or pull BT_SI_ENB_TST high (if BT_SI_ENB_TST has a broken-out PIN)"]
    #[inline(always)]
    pub fn bt_en_tst_si_ctrl_rfc(&mut self) -> BtEnTstSiCtrlRfcW<'_, RegBtCtrl0Spec> {
        BtEnTstSiCtrlRfcW::new(self, 20)
    }
    #[doc = "Bits 21:23 - SET XTAL state when BT in active mode 000: XTAL OFF 001: XTAL LPS, gating XTAL 40M 010: XTAL LPS, enable XTAL 40M 011: XTAL NM mode 100: XTAL HP mode"]
    #[inline(always)]
    pub fn bt_xtal_mode_nm(&mut self) -> BtXtalModeNmW<'_, RegBtCtrl0Spec> {
        BtXtalModeNmW::new(self, 21)
    }
    #[doc = "Bit 24 - Force wifi soc to use loguart pad mandatory if necessary 0: loguart use loguart trx pad 1: who uses loguart trx pad depends on BTWL_USE_LOGUART_PAD_ST"]
    #[inline(always)]
    pub fn force_loguart_use_loguart_pad_b(
        &mut self,
    ) -> ForceLoguartUseLoguartPadBW<'_, RegBtCtrl0Spec> {
        ForceLoguartUseLoguartPadBW::new(self, 24)
    }
    #[doc = "Bit 25 - who uses loguart trx pad depends on the xor of WL_USE_REQ and BT_USE_REQ, wifi can send use request by change the value of WL_USE_REQ"]
    #[inline(always)]
    pub fn wl_use_req(&mut self) -> WlUseReqW<'_, RegBtCtrl0Spec> {
        WlUseReqW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: select external BT 0: select internal BT"]
    #[inline(always)]
    pub fn bt_use_ext_bt(&mut self) -> BtUseExtBtW<'_, RegBtCtrl0Spec> {
        BtUseExtBtW::new(self, 26)
    }
    #[doc = "Bit 27 - Wi-Fi MP status"]
    #[inline(always)]
    pub fn wl_mp_status(&mut self) -> WlMpStatusW<'_, RegBtCtrl0Spec> {
        WlMpStatusW::new(self, 27)
    }
    #[doc = "Bits 29:31 - SET SWR state when BT in active mode 000: RSVD 001: state1 0.7/PC/PFM 010: state2 0.8/PC/PFM 011: state3 0.9/PC/PFM 100: state4 0.9/SLP/PFM 101: state5 0.9/SLP/PWM 110: state6 0.9/NORM/PWM 111: state7 1.0/NORM/PWM"]
    #[inline(always)]
    pub fn bt_swr_sts_nm(&mut self) -> BtSwrStsNmW<'_, RegBtCtrl0Spec> {
        BtSwrStsNmW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtCtrl0Spec;
impl crate::RegisterSpec for RegBtCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegBtCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegBtCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_CTRL0 to value 0xc090_c488"]
impl crate::Resettable for RegBtCtrl0Spec {
    const RESET_VALUE: u32 = 0xc090_c488;
}
