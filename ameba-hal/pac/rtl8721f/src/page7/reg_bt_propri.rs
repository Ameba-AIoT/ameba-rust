#[doc = "Register `REG_BT_PROPRI` reader"]
pub type R = crate::R<RegBtPropriSpec>;
#[doc = "Register `REG_BT_PROPRI` writer"]
pub type W = crate::W<RegBtPropriSpec>;
#[doc = "Field `BIT_RTK_BT_Enable` reader - Enable RTK BT mode"]
pub type BitRtkBtEnableR = crate::BitReader;
#[doc = "Field `BIT_RTK_BT_Enable` writer - Enable RTK BT mode"]
pub type BitRtkBtEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GPIO2_GPIO3_EXANGE_or_NO_BT_CCA` reader - Enable GPIO2 and GPIO3 exchange. (Note: need to enable mode2 and enhance 3 wire) If Enable RTK BT mode, set this bit to 1 to disable granting to BT device when arbitration"]
pub type BitGpio2Gpio3ExangeOrNoBtCcaR = crate::BitReader;
#[doc = "Field `BIT_GPIO2_GPIO3_EXANGE_or_NO_BT_CCA` writer - Enable GPIO2 and GPIO3 exchange. (Note: need to enable mode2 and enhance 3 wire) If Enable RTK BT mode, set this bit to 1 to disable granting to BT device when arbitration"]
pub type BitGpio2Gpio3ExangeOrNoBtCcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_Enable_Coexist_Tab_In_TDMA` reader - 0: default, no matter which WLAN_ACT is high or low, PTA doesn't execute co-exist table scheme 1: enable, When WLAN_ACT is high, PTA execute xo-exist table scheme"]
pub type BitEnableCoexistTabInTdmaR = crate::BitReader;
#[doc = "Field `BIT_Enable_Coexist_Tab_In_TDMA` writer - 0: default, no matter which WLAN_ACT is high or low, PTA doesn't execute co-exist table scheme 1: enable, When WLAN_ACT is high, PTA execute xo-exist table scheme"]
pub type BitEnableCoexistTabInTdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_Enable_PTA_TDMA_Mode` reader - 0: default, disable PTA TDMA mode 1: enable TDMA mode: a. WLAN_ACT default is high. b. Hardware mode: When NULL packet for BT had sent, hardware will notify PTA to start TDMA mode. Firmware mode: When NULL packet for BT had sent, hardware will trigger an interrupt to notify firmware. Then, firmware need to set TDMA_BT_START_NOTIFY to notify PTA to start TDMA mode."]
pub type BitEnablePtaTdmaModeR = crate::BitReader;
#[doc = "Field `BIT_Enable_PTA_TDMA_Mode` writer - 0: default, disable PTA TDMA mode 1: enable TDMA mode: a. WLAN_ACT default is high. b. Hardware mode: When NULL packet for BT had sent, hardware will notify PTA to start TDMA mode. Firmware mode: When NULL packet for BT had sent, hardware will trigger an interrupt to notify firmware. Then, firmware need to set TDMA_BT_START_NOTIFY to notify PTA to start TDMA mode."]
pub type BitEnablePtaTdmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_Enable_TDMA_HW_Mode` reader - 0: default, PTA TDMA mode is hardware mode. 1: enable firmware mode."]
pub type BitEnableTdmaHwModeR = crate::BitReader;
#[doc = "Field `BIT_Enable_TDMA_HW_Mode` writer - 0: default, PTA TDMA mode is hardware mode. 1: enable firmware mode."]
pub type BitEnableTdmaHwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TDMA_BT_START_NOTIFY` reader - When the control of TDMA mode is firmware and firmware also receive signal that NULL packet for BT had sent, firmware can fill this bit to notify HW PTA to pull WLAN_ACT to low and the interval is Tbt"]
pub type BitTdmaBtStartNotifyR = crate::BitReader;
#[doc = "Field `BIT_TDMA_BT_START_NOTIFY` writer - When the control of TDMA mode is firmware and firmware also receive signal that NULL packet for BT had sent, firmware can fill this bit to notify HW PTA to pull WLAN_ACT to low and the interval is Tbt"]
pub type BitTdmaBtStartNotifyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BT_HID_ISR_SET` reader - When EN_BT_PROFILE_OR_HID is set \"1\", hardware also need to check this value to trigger an interrupt. 0: default, TX and RX 1: TX 2: RX 3: Reserved"]
pub type BitBtHidIsrSetR = crate::FieldReader;
#[doc = "Field `BIT_BT_HID_ISR_SET` writer - When EN_BT_PROFILE_OR_HID is set \"1\", hardware also need to check this value to trigger an interrupt. 0: default, TX and RX 1: TX 2: RX 3: Reserved"]
pub type BitBtHidIsrSetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RPT_FROM_BT` reader - "]
pub type BitRptFromBtR = crate::FieldReader;
#[doc = "Field `BIT_BT_CMD_RPT` reader - "]
pub type BitBtCmdRptR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable RTK BT mode"]
    #[inline(always)]
    pub fn bit_rtk_bt_enable(&self) -> BitRtkBtEnableR {
        BitRtkBtEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable GPIO2 and GPIO3 exchange. (Note: need to enable mode2 and enhance 3 wire) If Enable RTK BT mode, set this bit to 1 to disable granting to BT device when arbitration"]
    #[inline(always)]
    pub fn bit_gpio2_gpio3_exange_or_no_bt_cca(&self) -> BitGpio2Gpio3ExangeOrNoBtCcaR {
        BitGpio2Gpio3ExangeOrNoBtCcaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: default, no matter which WLAN_ACT is high or low, PTA doesn't execute co-exist table scheme 1: enable, When WLAN_ACT is high, PTA execute xo-exist table scheme"]
    #[inline(always)]
    pub fn bit_enable_coexist_tab_in_tdma(&self) -> BitEnableCoexistTabInTdmaR {
        BitEnableCoexistTabInTdmaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: default, disable PTA TDMA mode 1: enable TDMA mode: a. WLAN_ACT default is high. b. Hardware mode: When NULL packet for BT had sent, hardware will notify PTA to start TDMA mode. Firmware mode: When NULL packet for BT had sent, hardware will trigger an interrupt to notify firmware. Then, firmware need to set TDMA_BT_START_NOTIFY to notify PTA to start TDMA mode."]
    #[inline(always)]
    pub fn bit_enable_pta_tdma_mode(&self) -> BitEnablePtaTdmaModeR {
        BitEnablePtaTdmaModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: default, PTA TDMA mode is hardware mode. 1: enable firmware mode."]
    #[inline(always)]
    pub fn bit_enable_tdma_hw_mode(&self) -> BitEnableTdmaHwModeR {
        BitEnableTdmaHwModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When the control of TDMA mode is firmware and firmware also receive signal that NULL packet for BT had sent, firmware can fill this bit to notify HW PTA to pull WLAN_ACT to low and the interval is Tbt"]
    #[inline(always)]
    pub fn bit_tdma_bt_start_notify(&self) -> BitTdmaBtStartNotifyR {
        BitTdmaBtStartNotifyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - When EN_BT_PROFILE_OR_HID is set \"1\", hardware also need to check this value to trigger an interrupt. 0: default, TX and RX 1: TX 2: RX 3: Reserved"]
    #[inline(always)]
    pub fn bit_bt_hid_isr_set(&self) -> BitBtHidIsrSetR {
        BitBtHidIsrSetR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn bit_rpt_from_bt(&self) -> BitRptFromBtR {
        BitRptFromBtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn bit_bt_cmd_rpt(&self) -> BitBtCmdRptR {
        BitBtCmdRptR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RTK BT mode"]
    #[inline(always)]
    pub fn bit_rtk_bt_enable(&mut self) -> BitRtkBtEnableW<'_, RegBtPropriSpec> {
        BitRtkBtEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable GPIO2 and GPIO3 exchange. (Note: need to enable mode2 and enhance 3 wire) If Enable RTK BT mode, set this bit to 1 to disable granting to BT device when arbitration"]
    #[inline(always)]
    pub fn bit_gpio2_gpio3_exange_or_no_bt_cca(
        &mut self,
    ) -> BitGpio2Gpio3ExangeOrNoBtCcaW<'_, RegBtPropriSpec> {
        BitGpio2Gpio3ExangeOrNoBtCcaW::new(self, 1)
    }
    #[doc = "Bit 2 - 0: default, no matter which WLAN_ACT is high or low, PTA doesn't execute co-exist table scheme 1: enable, When WLAN_ACT is high, PTA execute xo-exist table scheme"]
    #[inline(always)]
    pub fn bit_enable_coexist_tab_in_tdma(
        &mut self,
    ) -> BitEnableCoexistTabInTdmaW<'_, RegBtPropriSpec> {
        BitEnableCoexistTabInTdmaW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: default, disable PTA TDMA mode 1: enable TDMA mode: a. WLAN_ACT default is high. b. Hardware mode: When NULL packet for BT had sent, hardware will notify PTA to start TDMA mode. Firmware mode: When NULL packet for BT had sent, hardware will trigger an interrupt to notify firmware. Then, firmware need to set TDMA_BT_START_NOTIFY to notify PTA to start TDMA mode."]
    #[inline(always)]
    pub fn bit_enable_pta_tdma_mode(&mut self) -> BitEnablePtaTdmaModeW<'_, RegBtPropriSpec> {
        BitEnablePtaTdmaModeW::new(self, 3)
    }
    #[doc = "Bit 4 - 0: default, PTA TDMA mode is hardware mode. 1: enable firmware mode."]
    #[inline(always)]
    pub fn bit_enable_tdma_hw_mode(&mut self) -> BitEnableTdmaHwModeW<'_, RegBtPropriSpec> {
        BitEnableTdmaHwModeW::new(self, 4)
    }
    #[doc = "Bit 5 - When the control of TDMA mode is firmware and firmware also receive signal that NULL packet for BT had sent, firmware can fill this bit to notify HW PTA to pull WLAN_ACT to low and the interval is Tbt"]
    #[inline(always)]
    pub fn bit_tdma_bt_start_notify(&mut self) -> BitTdmaBtStartNotifyW<'_, RegBtPropriSpec> {
        BitTdmaBtStartNotifyW::new(self, 5)
    }
    #[doc = "Bits 6:7 - When EN_BT_PROFILE_OR_HID is set \"1\", hardware also need to check this value to trigger an interrupt. 0: default, TX and RX 1: TX 2: RX 3: Reserved"]
    #[inline(always)]
    pub fn bit_bt_hid_isr_set(&mut self) -> BitBtHidIsrSetW<'_, RegBtPropriSpec> {
        BitBtHidIsrSetW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_propri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_propri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtPropriSpec;
impl crate::RegisterSpec for RegBtPropriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_propri::R`](R) reader structure"]
impl crate::Readable for RegBtPropriSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_propri::W`](W) writer structure"]
impl crate::Writable for RegBtPropriSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_PROPRI to value 0"]
impl crate::Resettable for RegBtPropriSpec {}
