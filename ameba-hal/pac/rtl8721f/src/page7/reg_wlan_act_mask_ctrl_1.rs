#[doc = "Register `REG_WLAN_ACT_MASK_CTRL_1` reader"]
pub type R = crate::R<RegWlanActMaskCtrl1Spec>;
#[doc = "Register `REG_WLAN_ACT_MASK_CTRL_1` writer"]
pub type W = crate::W<RegWlanActMaskCtrl1Spec>;
#[doc = "Field `BIT_EN_WLACT_MASK_SYNC` reader - When NAV update is beyond this value, then NAV update is aborted. When this field is 0x00, NAV_UPPER function is disable. The unit is 128us."]
pub type BitEnWlactMaskSyncR = crate::BitReader;
#[doc = "Field `BIT_EN_WLACT_MASK_SYNC` writer - When NAV update is beyond this value, then NAV update is aborted. When this field is 0x00, NAV_UPPER function is disable. The unit is 128us."]
pub type BitEnWlactMaskSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLACT_HIGH_GNTBT_EN_1` reader - When WLACT_HIGH_GNTBT_EN = 1, the TRX of WLAN are blocked according to the BTCOEX table arbitration during MASK ON duration. When WLACT_HIGH_ GNTBT_EN = 0, the TRX of WLAN are not paused during MASK OFF duration."]
pub type BitWlactHighGntbtEn1R = crate::BitReader;
#[doc = "Field `BIT_WLACT_HIGH_GNTBT_EN_1` writer - When WLACT_HIGH_GNTBT_EN = 1, the TRX of WLAN are blocked according to the BTCOEX table arbitration during MASK ON duration. When WLACT_HIGH_ GNTBT_EN = 0, the TRX of WLAN are not paused during MASK OFF duration."]
pub type BitWlactHighGntbtEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLACT_LOW_GNTWL_EN_1` reader - When WLACT_LOW_GNTWL_EN = 1, the TRX of WLAN are blocked according to the BTCOEX table arbitration during MASK OFF duration. WLACT_LOW_GNTWL_EN = 0, the TRX of WLAN are paused during MASK OFF duration.."]
pub type BitWlactLowGntwlEn1R = crate::BitReader;
#[doc = "Field `BIT_WLACT_LOW_GNTWL_EN_1` writer - When WLACT_LOW_GNTWL_EN = 1, the TRX of WLAN are blocked according to the BTCOEX table arbitration during MASK OFF duration. WLACT_LOW_GNTWL_EN = 0, the TRX of WLAN are paused during MASK OFF duration.."]
pub type BitWlactLowGntwlEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ANTSEL_FOR_BT_CTRL_EN_1` reader - Enable the control of antenna selection through GNT_BT: GNT_BT=1: ANT_SEL for BT device; GNT_BT=0: ANT_SEL for WL device."]
pub type BitAntselForBtCtrlEn1R = crate::BitReader;
#[doc = "Field `BIT_ANTSEL_FOR_BT_CTRL_EN_1` writer - Enable the control of antenna selection through GNT_BT: GNT_BT=1: ANT_SEL for BT device; GNT_BT=0: ANT_SEL for WL device."]
pub type BitAntselForBtCtrlEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ANT_DIVERSITY_SEL_1` reader - Antenna diversity for 2 antenna mode: MAC_ANT_SEL = GNT_BT ^ ANT_DIVERDITY_SEL"]
pub type BitAntDiversitySel1R = crate::BitReader;
#[doc = "Field `BIT_ANT_DIVERSITY_SEL_1` writer - Antenna diversity for 2 antenna mode: MAC_ANT_SEL = GNT_BT ^ ANT_DIVERDITY_SEL"]
pub type BitAntDiversitySel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLRX_TER_BY_AD_1` reader - When wlan receive the packet without fit address, the signal indicating wlan rx can be terminated as soon as the address check finish. This feature can be enabled by this bit. 1: en"]
pub type BitWlrxTerByAd1R = crate::BitReader;
#[doc = "Field `BIT_WLRX_TER_BY_AD_1` writer - When wlan receive the packet without fit address, the signal indicating wlan rx can be terminated as soon as the address check finish. This feature can be enabled by this bit. 1: en"]
pub type BitWlrxTerByAd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLRX_TER_BY_CTL_1` reader - When wlan receive a control packet without fit address, whether the signal indicating wlan rx can be terminated as soon as the address check finish is not only controlled by r_WLRX_TER_BY_AD, but also should be refereed to this bit. 1: enable; 0: disable"]
pub type BitWlrxTerByCtl1R = crate::BitReader;
#[doc = "Field `BIT_WLRX_TER_BY_CTL_1` writer - When wlan receive a control packet without fit address, whether the signal indicating wlan rx can be terminated as soon as the address check finish is not only controlled by r_WLRX_TER_BY_AD, but also should be refereed to this bit. 1: enable; 0: disable"]
pub type BitWlrxTerByCtl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENHBT` reader - Enable Enhanced BT Interface (we suggest to use with BT mode 2)"]
pub type BitEnhbtR = crate::BitReader;
#[doc = "Field `BIT_ENHBT` writer - Enable Enhanced BT Interface (we suggest to use with BT mode 2)"]
pub type BitEnhbtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_WL_ACT_MASK` reader - Enable WLAN_ACT Mask Function (related to 0x768h-0x76Dh)"]
pub type BitEnWlActMaskR = crate::BitReader;
#[doc = "Field `BIT_EN_WL_ACT_MASK` writer - Enable WLAN_ACT Mask Function (related to 0x768h-0x76Dh)"]
pub type BitEnWlActMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STATIS_BT_EN` reader - When this bit is set, the BT_ACT_STATISTICS counters starts counting"]
pub type BitStatisBtEnR = crate::BitReader;
#[doc = "Field `BIT_STATIS_BT_EN` writer - When this bit is set, the BT_ACT_STATISTICS counters starts counting"]
pub type BitStatisBtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STATIS_BT_RSTB` reader - clear BT static info"]
pub type BitStatisBtRstbR = crate::BitReader;
#[doc = "Field `BIT_STATIS_BT_RSTB` writer - clear BT static info"]
pub type BitStatisBtRstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BT_PRI_DETECT_TO` reader - After BT_PRI asserting, if BT_STAT is asserted within this TO duration, BT will initiate high priority activities; otherwise, if BT_STAT is not asserted within this duration, it would be low priority activities. Units 1us."]
pub type BitBtPriDetectToR = crate::FieldReader;
#[doc = "Field `BIT_BT_PRI_DETECT_TO` writer - After BT_PRI asserting, if BT_STAT is asserted within this TO duration, BT will initiate high priority activities; otherwise, if BT_STAT is not asserted within this duration, it would be low priority activities. Units 1us."]
pub type BitBtPriDetectToW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_BT_TRX_INIT_DETECT` reader - After BT_PRI asserting for BT_TRX_INIT_DETECT us, if BT_STAT is low, then BT will receive packets; otherwise, if BT goes high, BT will TX data. Unit 4us."]
pub type BitBtTrxInitDetectR = crate::FieldReader;
#[doc = "Field `BIT_BT_TRX_INIT_DETECT` writer - After BT_PRI asserting for BT_TRX_INIT_DETECT us, if BT_STAT is low, then BT will receive packets; otherwise, if BT goes high, BT will TX data. Unit 4us."]
pub type BitBtTrxInitDetectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_BT_STAT_DELAY` reader - For CSR, BT_PRI and BT_STAT go high form low at the same time. Thus, BT_STAT need delay BT_STAT_DELAY time to trigger hardware. Unit: 1us"]
pub type BitBtStatDelayR = crate::FieldReader;
#[doc = "Field `BIT_BT_STAT_DELAY` writer - For CSR, BT_PRI and BT_STAT go high form low at the same time. Thus, BT_STAT need delay BT_STAT_DELAY time to trigger hardware. Unit: 1us"]
pub type BitBtStatDelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - When NAV update is beyond this value, then NAV update is aborted. When this field is 0x00, NAV_UPPER function is disable. The unit is 128us."]
    #[inline(always)]
    pub fn bit_en_wlact_mask_sync(&self) -> BitEnWlactMaskSyncR {
        BitEnWlactMaskSyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When WLACT_HIGH_GNTBT_EN = 1, the TRX of WLAN are blocked according to the BTCOEX table arbitration during MASK ON duration. When WLACT_HIGH_ GNTBT_EN = 0, the TRX of WLAN are not paused during MASK OFF duration."]
    #[inline(always)]
    pub fn bit_wlact_high_gntbt_en_1(&self) -> BitWlactHighGntbtEn1R {
        BitWlactHighGntbtEn1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When WLACT_LOW_GNTWL_EN = 1, the TRX of WLAN are blocked according to the BTCOEX table arbitration during MASK OFF duration. WLACT_LOW_GNTWL_EN = 0, the TRX of WLAN are paused during MASK OFF duration.."]
    #[inline(always)]
    pub fn bit_wlact_low_gntwl_en_1(&self) -> BitWlactLowGntwlEn1R {
        BitWlactLowGntwlEn1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable the control of antenna selection through GNT_BT: GNT_BT=1: ANT_SEL for BT device; GNT_BT=0: ANT_SEL for WL device."]
    #[inline(always)]
    pub fn bit_antsel_for_bt_ctrl_en_1(&self) -> BitAntselForBtCtrlEn1R {
        BitAntselForBtCtrlEn1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Antenna diversity for 2 antenna mode: MAC_ANT_SEL = GNT_BT ^ ANT_DIVERDITY_SEL"]
    #[inline(always)]
    pub fn bit_ant_diversity_sel_1(&self) -> BitAntDiversitySel1R {
        BitAntDiversitySel1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When wlan receive the packet without fit address, the signal indicating wlan rx can be terminated as soon as the address check finish. This feature can be enabled by this bit. 1: en"]
    #[inline(always)]
    pub fn bit_wlrx_ter_by_ad_1(&self) -> BitWlrxTerByAd1R {
        BitWlrxTerByAd1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When wlan receive a control packet without fit address, whether the signal indicating wlan rx can be terminated as soon as the address check finish is not only controlled by r_WLRX_TER_BY_AD, but also should be refereed to this bit. 1: enable; 0: disable"]
    #[inline(always)]
    pub fn bit_wlrx_ter_by_ctl_1(&self) -> BitWlrxTerByCtl1R {
        BitWlrxTerByCtl1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Enhanced BT Interface (we suggest to use with BT mode 2)"]
    #[inline(always)]
    pub fn bit_enhbt(&self) -> BitEnhbtR {
        BitEnhbtR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable WLAN_ACT Mask Function (related to 0x768h-0x76Dh)"]
    #[inline(always)]
    pub fn bit_en_wl_act_mask(&self) -> BitEnWlActMaskR {
        BitEnWlActMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When this bit is set, the BT_ACT_STATISTICS counters starts counting"]
    #[inline(always)]
    pub fn bit_statis_bt_en(&self) -> BitStatisBtEnR {
        BitStatisBtEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - clear BT static info"]
    #[inline(always)]
    pub fn bit_statis_bt_rstb(&self) -> BitStatisBtRstbR {
        BitStatisBtRstbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - After BT_PRI asserting, if BT_STAT is asserted within this TO duration, BT will initiate high priority activities; otherwise, if BT_STAT is not asserted within this duration, it would be low priority activities. Units 1us."]
    #[inline(always)]
    pub fn bit_bt_pri_detect_to(&self) -> BitBtPriDetectToR {
        BitBtPriDetectToR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - After BT_PRI asserting for BT_TRX_INIT_DETECT us, if BT_STAT is low, then BT will receive packets; otherwise, if BT goes high, BT will TX data. Unit 4us."]
    #[inline(always)]
    pub fn bit_bt_trx_init_detect(&self) -> BitBtTrxInitDetectR {
        BitBtTrxInitDetectR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - For CSR, BT_PRI and BT_STAT go high form low at the same time. Thus, BT_STAT need delay BT_STAT_DELAY time to trigger hardware. Unit: 1us"]
    #[inline(always)]
    pub fn bit_bt_stat_delay(&self) -> BitBtStatDelayR {
        BitBtStatDelayR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When NAV update is beyond this value, then NAV update is aborted. When this field is 0x00, NAV_UPPER function is disable. The unit is 128us."]
    #[inline(always)]
    pub fn bit_en_wlact_mask_sync(&mut self) -> BitEnWlactMaskSyncW<'_, RegWlanActMaskCtrl1Spec> {
        BitEnWlactMaskSyncW::new(self, 0)
    }
    #[doc = "Bit 1 - When WLACT_HIGH_GNTBT_EN = 1, the TRX of WLAN are blocked according to the BTCOEX table arbitration during MASK ON duration. When WLACT_HIGH_ GNTBT_EN = 0, the TRX of WLAN are not paused during MASK OFF duration."]
    #[inline(always)]
    pub fn bit_wlact_high_gntbt_en_1(
        &mut self,
    ) -> BitWlactHighGntbtEn1W<'_, RegWlanActMaskCtrl1Spec> {
        BitWlactHighGntbtEn1W::new(self, 1)
    }
    #[doc = "Bit 2 - When WLACT_LOW_GNTWL_EN = 1, the TRX of WLAN are blocked according to the BTCOEX table arbitration during MASK OFF duration. WLACT_LOW_GNTWL_EN = 0, the TRX of WLAN are paused during MASK OFF duration.."]
    #[inline(always)]
    pub fn bit_wlact_low_gntwl_en_1(
        &mut self,
    ) -> BitWlactLowGntwlEn1W<'_, RegWlanActMaskCtrl1Spec> {
        BitWlactLowGntwlEn1W::new(self, 2)
    }
    #[doc = "Bit 8 - Enable the control of antenna selection through GNT_BT: GNT_BT=1: ANT_SEL for BT device; GNT_BT=0: ANT_SEL for WL device."]
    #[inline(always)]
    pub fn bit_antsel_for_bt_ctrl_en_1(
        &mut self,
    ) -> BitAntselForBtCtrlEn1W<'_, RegWlanActMaskCtrl1Spec> {
        BitAntselForBtCtrlEn1W::new(self, 8)
    }
    #[doc = "Bit 9 - Antenna diversity for 2 antenna mode: MAC_ANT_SEL = GNT_BT ^ ANT_DIVERDITY_SEL"]
    #[inline(always)]
    pub fn bit_ant_diversity_sel_1(&mut self) -> BitAntDiversitySel1W<'_, RegWlanActMaskCtrl1Spec> {
        BitAntDiversitySel1W::new(self, 9)
    }
    #[doc = "Bit 10 - When wlan receive the packet without fit address, the signal indicating wlan rx can be terminated as soon as the address check finish. This feature can be enabled by this bit. 1: en"]
    #[inline(always)]
    pub fn bit_wlrx_ter_by_ad_1(&mut self) -> BitWlrxTerByAd1W<'_, RegWlanActMaskCtrl1Spec> {
        BitWlrxTerByAd1W::new(self, 10)
    }
    #[doc = "Bit 11 - When wlan receive a control packet without fit address, whether the signal indicating wlan rx can be terminated as soon as the address check finish is not only controlled by r_WLRX_TER_BY_AD, but also should be refereed to this bit. 1: enable; 0: disable"]
    #[inline(always)]
    pub fn bit_wlrx_ter_by_ctl_1(&mut self) -> BitWlrxTerByCtl1W<'_, RegWlanActMaskCtrl1Spec> {
        BitWlrxTerByCtl1W::new(self, 11)
    }
    #[doc = "Bit 16 - Enable Enhanced BT Interface (we suggest to use with BT mode 2)"]
    #[inline(always)]
    pub fn bit_enhbt(&mut self) -> BitEnhbtW<'_, RegWlanActMaskCtrl1Spec> {
        BitEnhbtW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable WLAN_ACT Mask Function (related to 0x768h-0x76Dh)"]
    #[inline(always)]
    pub fn bit_en_wl_act_mask(&mut self) -> BitEnWlActMaskW<'_, RegWlanActMaskCtrl1Spec> {
        BitEnWlActMaskW::new(self, 17)
    }
    #[doc = "Bit 18 - When this bit is set, the BT_ACT_STATISTICS counters starts counting"]
    #[inline(always)]
    pub fn bit_statis_bt_en(&mut self) -> BitStatisBtEnW<'_, RegWlanActMaskCtrl1Spec> {
        BitStatisBtEnW::new(self, 18)
    }
    #[doc = "Bit 19 - clear BT static info"]
    #[inline(always)]
    pub fn bit_statis_bt_rstb(&mut self) -> BitStatisBtRstbW<'_, RegWlanActMaskCtrl1Spec> {
        BitStatisBtRstbW::new(self, 19)
    }
    #[doc = "Bits 20:23 - After BT_PRI asserting, if BT_STAT is asserted within this TO duration, BT will initiate high priority activities; otherwise, if BT_STAT is not asserted within this duration, it would be low priority activities. Units 1us."]
    #[inline(always)]
    pub fn bit_bt_pri_detect_to(&mut self) -> BitBtPriDetectToW<'_, RegWlanActMaskCtrl1Spec> {
        BitBtPriDetectToW::new(self, 20)
    }
    #[doc = "Bits 24:27 - After BT_PRI asserting for BT_TRX_INIT_DETECT us, if BT_STAT is low, then BT will receive packets; otherwise, if BT goes high, BT will TX data. Unit 4us."]
    #[inline(always)]
    pub fn bit_bt_trx_init_detect(&mut self) -> BitBtTrxInitDetectW<'_, RegWlanActMaskCtrl1Spec> {
        BitBtTrxInitDetectW::new(self, 24)
    }
    #[doc = "Bits 28:31 - For CSR, BT_PRI and BT_STAT go high form low at the same time. Thus, BT_STAT need delay BT_STAT_DELAY time to trigger hardware. Unit: 1us"]
    #[inline(always)]
    pub fn bit_bt_stat_delay(&mut self) -> BitBtStatDelayW<'_, RegWlanActMaskCtrl1Spec> {
        BitBtStatDelayW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wlan_act_mask_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wlan_act_mask_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlanActMaskCtrl1Spec;
impl crate::RegisterSpec for RegWlanActMaskCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wlan_act_mask_ctrl_1::R`](R) reader structure"]
impl crate::Readable for RegWlanActMaskCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_wlan_act_mask_ctrl_1::W`](W) writer structure"]
impl crate::Writable for RegWlanActMaskCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WLAN_ACT_MASK_CTRL_1 to value 0"]
impl crate::Resettable for RegWlanActMaskCtrl1Spec {}
