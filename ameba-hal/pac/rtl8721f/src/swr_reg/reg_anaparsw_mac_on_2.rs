#[doc = "Register `REG_ANAPARSW_MAC_ON_2` reader"]
pub type R = crate::R<RegAnaparswMacOn2Spec>;
#[doc = "Register `REG_ANAPARSW_MAC_ON_2` writer"]
pub type W = crate::W<RegAnaparswMacOn2Spec>;
#[doc = "Field `REG_VC_CLAMP_SEL_L` reader - 0:unity gain feedback clamp VC=VREF 1.orignal clamp(roll back option)"]
pub type RegVcClampSelLR = crate::BitReader;
#[doc = "Field `REG_VC_CLAMP_SEL_L` writer - 0:unity gain feedback clamp VC=VREF 1.orignal clamp(roll back option)"]
pub type RegVcClampSelLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_BG_DIG_L` reader - 1:Local SWR BG switch on"]
pub type RegBgDigLR = crate::BitReader;
#[doc = "Field `REG_BG_DIG_L` writer - 1:Local SWR BG switch on"]
pub type RegBgDigLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_BG_ANA_L` reader - 1:External SWR BG switch on"]
pub type RegBgAnaLR = crate::BitReader;
#[doc = "Field `REG_BG_ANA_L` writer - 1:External SWR BG switch on"]
pub type RegBgAnaLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bg_current_L` reader - 00:23.9n 01:54.6n 10:117.8n"]
pub type RegBgCurrentLR = crate::FieldReader;
#[doc = "Field `reg_bg_current_L` writer - 00:23.9n 01:54.6n 10:117.8n"]
pub type RegBgCurrentLW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `External_clk_sel` reader - 0:internal clock 1:external clock"]
pub type ExternalClkSelR = crate::BitReader;
#[doc = "Field `External_clk_sel` writer - 0:internal clock 1:external clock"]
pub type ExternalClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1_L1` reader - 00 => 1.03p 01 => 2.06p 10 => 3.1p 11 => 4.12p"]
pub type C1L1R = crate::FieldReader;
#[doc = "Field `C1_L1` writer - 00 => 1.03p 01 => 2.06p 10 => 3.1p 11 => 4.12p"]
pub type C1L1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C3_L1` reader - 00 => 1.59p 01 => 3.2p 10 => 4.76p 11 => 6.34p"]
pub type C3L1R = crate::FieldReader;
#[doc = "Field `C3_L1` writer - 00 => 1.59p 01 => 3.2p 10 => 4.76p 11 => 6.34p"]
pub type C3L1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R2_L1` reader - 00 => 0.4M 01 => 0.3M 10 => 0.2M 11 => 0.1M"]
pub type R2L1R = crate::FieldReader;
#[doc = "Field `R2_L1` writer - 00 => 0.4M 01 => 0.3M 10 => 0.2M 11 => 0.1M"]
pub type R2L1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_prss_H` reader - 0:default 1:res+cap improve psrr"]
pub type RegPrssHR = crate::BitReader;
#[doc = "Field `ZCD_Ioffset_b` reader - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
pub type ZcdIoffsetBR = crate::FieldReader;
#[doc = "Field `BG_R1_tune` reader - BG R1 resistor tuning"]
pub type BgR1TuneR = crate::FieldReader;
#[doc = "Field `BG_R1_tune` writer - BG R1 resistor tuning"]
pub type BgR1TuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BG_R2_tuen` reader - BG R2 resistor tuning"]
pub type BgR2TuenR = crate::FieldReader;
#[doc = "Field `BG_R2_tuen` writer - BG R2 resistor tuning"]
pub type BgR2TuenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BG_VR1200_tune` reader - BG voltage tuning"]
pub type BgVr1200TuneR = crate::FieldReader;
#[doc = "Field `BG_VR1200_tune` writer - BG voltage tuning"]
pub type BgVr1200TuneW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_SWR_ZCD_CTRL` reader - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
pub type RegSwrZcdCtrlR = crate::FieldReader;
#[doc = "Field `PWM_VREF_SEL` reader - 00:VREF=0.600 01:VREF=0.635 10:VREF=0.835 11:VREF=0.865"]
pub type PwmVrefSelR = crate::FieldReader;
#[doc = "Field `VOFB_SEL` reader - 0:Feedback from REGU Internal PAD for 1.25V/1.35V 1:Feedback from External PAD for 0.9V/1.0V"]
pub type VofbSelR = crate::BitReader;
#[doc = "Field `VOFB_SEL` writer - 0:Feedback from REGU Internal PAD for 1.25V/1.35V 1:Feedback from External PAD for 0.9V/1.0V"]
pub type VofbSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDIV_PFM_SEL` reader - Dummy"]
pub type VdivPfmSelR = crate::BitReader;
#[doc = "Field `VDIV_PFM_SEL` writer - Dummy"]
pub type VdivPfmSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLOAD_PLUS` reader - Dummy"]
pub type HloadPlusR = crate::BitReader;
#[doc = "Field `HLOAD_PLUS` writer - Dummy"]
pub type HloadPlusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLOAD_CUNT_SEL` reader - Dummy"]
pub type HloadCuntSelR = crate::BitReader;
#[doc = "Field `HLOAD_CUNT_SEL` writer - Dummy"]
pub type HloadCuntSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:unity gain feedback clamp VC=VREF 1.orignal clamp(roll back option)"]
    #[inline(always)]
    pub fn reg_vc_clamp_sel_l(&self) -> RegVcClampSelLR {
        RegVcClampSelLR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:Local SWR BG switch on"]
    #[inline(always)]
    pub fn reg_bg_dig_l(&self) -> RegBgDigLR {
        RegBgDigLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:External SWR BG switch on"]
    #[inline(always)]
    pub fn reg_bg_ana_l(&self) -> RegBgAnaLR {
        RegBgAnaLR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 00:23.9n 01:54.6n 10:117.8n"]
    #[inline(always)]
    pub fn reg_bg_current_l(&self) -> RegBgCurrentLR {
        RegBgCurrentLR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 0:internal clock 1:external clock"]
    #[inline(always)]
    pub fn external_clk_sel(&self) -> ExternalClkSelR {
        ExternalClkSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 00 => 1.03p 01 => 2.06p 10 => 3.1p 11 => 4.12p"]
    #[inline(always)]
    pub fn c1_l1(&self) -> C1L1R {
        C1L1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 00 => 1.59p 01 => 3.2p 10 => 4.76p 11 => 6.34p"]
    #[inline(always)]
    pub fn c3_l1(&self) -> C3L1R {
        C3L1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 00 => 0.4M 01 => 0.3M 10 => 0.2M 11 => 0.1M"]
    #[inline(always)]
    pub fn r2_l1(&self) -> R2L1R {
        R2L1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 0:default 1:res+cap improve psrr"]
    #[inline(always)]
    pub fn reg_prss_h(&self) -> RegPrssHR {
        RegPrssHR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 00=>+600nA 01=>+400nA 10=>+200nA 11=>0nA"]
    #[inline(always)]
    pub fn zcd_ioffset_b(&self) -> ZcdIoffsetBR {
        ZcdIoffsetBR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:17 - BG R1 resistor tuning"]
    #[inline(always)]
    pub fn bg_r1_tune(&self) -> BgR1TuneR {
        BgR1TuneR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - BG R2 resistor tuning"]
    #[inline(always)]
    pub fn bg_r2_tuen(&self) -> BgR2TuenR {
        BgR2TuenR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - BG voltage tuning"]
    #[inline(always)]
    pub fn bg_vr1200_tune(&self) -> BgVr1200TuneR {
        BgVr1200TuneR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - // SWR ZCD CTRL control <1>1:double current <0>1'b1: force in CCM/DCM boundary mode(default), 1'b0 : no boundary"]
    #[inline(always)]
    pub fn reg_swr_zcd_ctrl(&self) -> RegSwrZcdCtrlR {
        RegSwrZcdCtrlR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 00:VREF=0.600 01:VREF=0.635 10:VREF=0.835 11:VREF=0.865"]
    #[inline(always)]
    pub fn pwm_vref_sel(&self) -> PwmVrefSelR {
        PwmVrefSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 0:Feedback from REGU Internal PAD for 1.25V/1.35V 1:Feedback from External PAD for 0.9V/1.0V"]
    #[inline(always)]
    pub fn vofb_sel(&self) -> VofbSelR {
        VofbSelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Dummy"]
    #[inline(always)]
    pub fn vdiv_pfm_sel(&self) -> VdivPfmSelR {
        VdivPfmSelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Dummy"]
    #[inline(always)]
    pub fn hload_plus(&self) -> HloadPlusR {
        HloadPlusR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Dummy"]
    #[inline(always)]
    pub fn hload_cunt_sel(&self) -> HloadCuntSelR {
        HloadCuntSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:unity gain feedback clamp VC=VREF 1.orignal clamp(roll back option)"]
    #[inline(always)]
    pub fn reg_vc_clamp_sel_l(&mut self) -> RegVcClampSelLW<'_, RegAnaparswMacOn2Spec> {
        RegVcClampSelLW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:Local SWR BG switch on"]
    #[inline(always)]
    pub fn reg_bg_dig_l(&mut self) -> RegBgDigLW<'_, RegAnaparswMacOn2Spec> {
        RegBgDigLW::new(self, 1)
    }
    #[doc = "Bit 2 - 1:External SWR BG switch on"]
    #[inline(always)]
    pub fn reg_bg_ana_l(&mut self) -> RegBgAnaLW<'_, RegAnaparswMacOn2Spec> {
        RegBgAnaLW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 00:23.9n 01:54.6n 10:117.8n"]
    #[inline(always)]
    pub fn reg_bg_current_l(&mut self) -> RegBgCurrentLW<'_, RegAnaparswMacOn2Spec> {
        RegBgCurrentLW::new(self, 3)
    }
    #[doc = "Bit 5 - 0:internal clock 1:external clock"]
    #[inline(always)]
    pub fn external_clk_sel(&mut self) -> ExternalClkSelW<'_, RegAnaparswMacOn2Spec> {
        ExternalClkSelW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 00 => 1.03p 01 => 2.06p 10 => 3.1p 11 => 4.12p"]
    #[inline(always)]
    pub fn c1_l1(&mut self) -> C1L1W<'_, RegAnaparswMacOn2Spec> {
        C1L1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - 00 => 1.59p 01 => 3.2p 10 => 4.76p 11 => 6.34p"]
    #[inline(always)]
    pub fn c3_l1(&mut self) -> C3L1W<'_, RegAnaparswMacOn2Spec> {
        C3L1W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 00 => 0.4M 01 => 0.3M 10 => 0.2M 11 => 0.1M"]
    #[inline(always)]
    pub fn r2_l1(&mut self) -> R2L1W<'_, RegAnaparswMacOn2Spec> {
        R2L1W::new(self, 10)
    }
    #[doc = "Bits 15:17 - BG R1 resistor tuning"]
    #[inline(always)]
    pub fn bg_r1_tune(&mut self) -> BgR1TuneW<'_, RegAnaparswMacOn2Spec> {
        BgR1TuneW::new(self, 15)
    }
    #[doc = "Bits 18:20 - BG R2 resistor tuning"]
    #[inline(always)]
    pub fn bg_r2_tuen(&mut self) -> BgR2TuenW<'_, RegAnaparswMacOn2Spec> {
        BgR2TuenW::new(self, 18)
    }
    #[doc = "Bits 21:23 - BG voltage tuning"]
    #[inline(always)]
    pub fn bg_vr1200_tune(&mut self) -> BgVr1200TuneW<'_, RegAnaparswMacOn2Spec> {
        BgVr1200TuneW::new(self, 21)
    }
    #[doc = "Bit 28 - 0:Feedback from REGU Internal PAD for 1.25V/1.35V 1:Feedback from External PAD for 0.9V/1.0V"]
    #[inline(always)]
    pub fn vofb_sel(&mut self) -> VofbSelW<'_, RegAnaparswMacOn2Spec> {
        VofbSelW::new(self, 28)
    }
    #[doc = "Bit 29 - Dummy"]
    #[inline(always)]
    pub fn vdiv_pfm_sel(&mut self) -> VdivPfmSelW<'_, RegAnaparswMacOn2Spec> {
        VdivPfmSelW::new(self, 29)
    }
    #[doc = "Bit 30 - Dummy"]
    #[inline(always)]
    pub fn hload_plus(&mut self) -> HloadPlusW<'_, RegAnaparswMacOn2Spec> {
        HloadPlusW::new(self, 30)
    }
    #[doc = "Bit 31 - Dummy"]
    #[inline(always)]
    pub fn hload_cunt_sel(&mut self) -> HloadCuntSelW<'_, RegAnaparswMacOn2Spec> {
        HloadCuntSelW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anaparsw_mac_on_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anaparsw_mac_on_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparswMacOn2Spec;
impl crate::RegisterSpec for RegAnaparswMacOn2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anaparsw_mac_on_2::R`](R) reader structure"]
impl crate::Readable for RegAnaparswMacOn2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anaparsw_mac_on_2::W`](W) writer structure"]
impl crate::Writable for RegAnaparswMacOn2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPARSW_MAC_ON_2 to value 0x4092_4f5a"]
impl crate::Resettable for RegAnaparswMacOn2Spec {
    const RESET_VALUE: u32 = 0x4092_4f5a;
}
