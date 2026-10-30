#[doc = "Register `REG_SSC_CTRL3` reader"]
pub type R = crate::R<RegSscCtrl3Spec>;
#[doc = "Register `REG_SSC_CTRL3` writer"]
pub type W = crate::W<RegSscCtrl3Spec>;
#[doc = "Field `REG_BYPASS_PI` reader - bypass PI =0: the phase selsect signal sent to the PLL Loop PI toggles normally =1: the phase selsect signal sent to the PLL Loop PI ==0"]
pub type RegBypassPiR = crate::BitReader;
#[doc = "Field `REG_BYPASS_PI` writer - bypass PI =0: the phase selsect signal sent to the PLL Loop PI toggles normally =1: the phase selsect signal sent to the PLL Loop PI ==0"]
pub type RegBypassPiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_SSC` reader - SSC enable: =0: disable SSC =1: enable SSC"]
pub type EnSscR = crate::BitReader;
#[doc = "Field `EN_SSC` writer - SSC enable: =0: disable SSC =1: enable SSC"]
pub type EnSscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDM_ORDER` reader - Sigma-delta order: =0: 1st-order sigma-delta =1: 2nd-order sigma-delta"]
pub type RegSdmOrderR = crate::BitReader;
#[doc = "Field `REG_SDM_ORDER` writer - Sigma-delta order: =0: 1st-order sigma-delta =1: 2nd-order sigma-delta"]
pub type RegSdmOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRAN_EST` reader - SSC step (see the ppt for the SSC depth&frequence calculation formula)"]
pub type GranEstR = crate::FieldReader<u32>;
#[doc = "Field `GRAN_EST` writer - SSC step (see the ppt for the SSC depth&frequence calculation formula)"]
pub type GranEstW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `REG_DOT_GRAN` reader - Controls the decimal point position of GRAN_EST\\[20:0\\]"]
pub type RegDotGranR = crate::FieldReader;
#[doc = "Field `REG_DOT_GRAN` writer - Controls the decimal point position of GRAN_EST\\[20:0\\]"]
pub type RegDotGranW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GRAN_AUTO_RST` reader - SSC step auto reset control: =0: not auto reset =1: the SSC step resets to 0 at every triangle-wave turning point, then GRAN_EST and REG_DOT_GRAN are assigned to the SSC step"]
pub type RegGranAutoRstR = crate::BitReader;
#[doc = "Field `REG_GRAN_AUTO_RST` writer - SSC step auto reset control: =0: not auto reset =1: the SSC step resets to 0 at every triangle-wave turning point, then GRAN_EST and REG_DOT_GRAN are assigned to the SSC step"]
pub type RegGranAutoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_EN_WDOG` reader - PLL watchdog enable =0: disable PLL watchdog =1: enable PLL watchdog"]
pub type RegEnWdogR = crate::BitReader;
#[doc = "Field `REG_EN_WDOG` writer - PLL watchdog enable =0: disable PLL watchdog =1: enable PLL watchdog"]
pub type RegEnWdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TIME0_CK` reader - PLL watchdog timer: 3'b000: watchdog compare CKREF&CKBK frequency Every 2 CKREF period 3'b001: watchdog compare CKREF&CKBK frequency Every 200 CKREF period 3'b010: watchdog compare CKREF&CKBK frequency Every 600 CKREF period 3'b011: watchdog compare CKREF&CKBK frequency Every 1200 CKREF period 3'b100: watchdog compare CKREF&CKBK frequency Every 3000 CKREF period 3'b101: watchdog compare CKREF&CKBK frequency Every 6400 CKREF period 3'b110: watchdog compare CKREF&CKBK frequency Every 12800 CKREF period 3'b111: watchdog compare CKREF&CKBK frequency Every 16000 CKREF period"]
pub type RegTime0CkR = crate::FieldReader;
#[doc = "Field `REG_TIME0_CK` writer - PLL watchdog timer: 3'b000: watchdog compare CKREF&CKBK frequency Every 2 CKREF period 3'b001: watchdog compare CKREF&CKBK frequency Every 200 CKREF period 3'b010: watchdog compare CKREF&CKBK frequency Every 600 CKREF period 3'b011: watchdog compare CKREF&CKBK frequency Every 1200 CKREF period 3'b100: watchdog compare CKREF&CKBK frequency Every 3000 CKREF period 3'b101: watchdog compare CKREF&CKBK frequency Every 6400 CKREF period 3'b110: watchdog compare CKREF&CKBK frequency Every 12800 CKREF period 3'b111: watchdog compare CKREF&CKBK frequency Every 16000 CKREF period"]
pub type RegTime0CkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - bypass PI =0: the phase selsect signal sent to the PLL Loop PI toggles normally =1: the phase selsect signal sent to the PLL Loop PI ==0"]
    #[inline(always)]
    pub fn reg_bypass_pi(&self) -> RegBypassPiR {
        RegBypassPiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSC enable: =0: disable SSC =1: enable SSC"]
    #[inline(always)]
    pub fn en_ssc(&self) -> EnSscR {
        EnSscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sigma-delta order: =0: 1st-order sigma-delta =1: 2nd-order sigma-delta"]
    #[inline(always)]
    pub fn reg_sdm_order(&self) -> RegSdmOrderR {
        RegSdmOrderR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:22 - SSC step (see the ppt for the SSC depth&frequence calculation formula)"]
    #[inline(always)]
    pub fn gran_est(&self) -> GranEstR {
        GranEstR::new((self.bits >> 4) & 0x0007_ffff)
    }
    #[doc = "Bits 23:25 - Controls the decimal point position of GRAN_EST\\[20:0\\]"]
    #[inline(always)]
    pub fn reg_dot_gran(&self) -> RegDotGranR {
        RegDotGranR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - SSC step auto reset control: =0: not auto reset =1: the SSC step resets to 0 at every triangle-wave turning point, then GRAN_EST and REG_DOT_GRAN are assigned to the SSC step"]
    #[inline(always)]
    pub fn reg_gran_auto_rst(&self) -> RegGranAutoRstR {
        RegGranAutoRstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL watchdog enable =0: disable PLL watchdog =1: enable PLL watchdog"]
    #[inline(always)]
    pub fn reg_en_wdog(&self) -> RegEnWdogR {
        RegEnWdogR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - PLL watchdog timer: 3'b000: watchdog compare CKREF&CKBK frequency Every 2 CKREF period 3'b001: watchdog compare CKREF&CKBK frequency Every 200 CKREF period 3'b010: watchdog compare CKREF&CKBK frequency Every 600 CKREF period 3'b011: watchdog compare CKREF&CKBK frequency Every 1200 CKREF period 3'b100: watchdog compare CKREF&CKBK frequency Every 3000 CKREF period 3'b101: watchdog compare CKREF&CKBK frequency Every 6400 CKREF period 3'b110: watchdog compare CKREF&CKBK frequency Every 12800 CKREF period 3'b111: watchdog compare CKREF&CKBK frequency Every 16000 CKREF period"]
    #[inline(always)]
    pub fn reg_time0_ck(&self) -> RegTime0CkR {
        RegTime0CkR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - bypass PI =0: the phase selsect signal sent to the PLL Loop PI toggles normally =1: the phase selsect signal sent to the PLL Loop PI ==0"]
    #[inline(always)]
    pub fn reg_bypass_pi(&mut self) -> RegBypassPiW<'_, RegSscCtrl3Spec> {
        RegBypassPiW::new(self, 0)
    }
    #[doc = "Bit 1 - SSC enable: =0: disable SSC =1: enable SSC"]
    #[inline(always)]
    pub fn en_ssc(&mut self) -> EnSscW<'_, RegSscCtrl3Spec> {
        EnSscW::new(self, 1)
    }
    #[doc = "Bit 2 - Sigma-delta order: =0: 1st-order sigma-delta =1: 2nd-order sigma-delta"]
    #[inline(always)]
    pub fn reg_sdm_order(&mut self) -> RegSdmOrderW<'_, RegSscCtrl3Spec> {
        RegSdmOrderW::new(self, 2)
    }
    #[doc = "Bits 4:22 - SSC step (see the ppt for the SSC depth&frequence calculation formula)"]
    #[inline(always)]
    pub fn gran_est(&mut self) -> GranEstW<'_, RegSscCtrl3Spec> {
        GranEstW::new(self, 4)
    }
    #[doc = "Bits 23:25 - Controls the decimal point position of GRAN_EST\\[20:0\\]"]
    #[inline(always)]
    pub fn reg_dot_gran(&mut self) -> RegDotGranW<'_, RegSscCtrl3Spec> {
        RegDotGranW::new(self, 23)
    }
    #[doc = "Bit 26 - SSC step auto reset control: =0: not auto reset =1: the SSC step resets to 0 at every triangle-wave turning point, then GRAN_EST and REG_DOT_GRAN are assigned to the SSC step"]
    #[inline(always)]
    pub fn reg_gran_auto_rst(&mut self) -> RegGranAutoRstW<'_, RegSscCtrl3Spec> {
        RegGranAutoRstW::new(self, 26)
    }
    #[doc = "Bit 27 - PLL watchdog enable =0: disable PLL watchdog =1: enable PLL watchdog"]
    #[inline(always)]
    pub fn reg_en_wdog(&mut self) -> RegEnWdogW<'_, RegSscCtrl3Spec> {
        RegEnWdogW::new(self, 27)
    }
    #[doc = "Bits 28:30 - PLL watchdog timer: 3'b000: watchdog compare CKREF&CKBK frequency Every 2 CKREF period 3'b001: watchdog compare CKREF&CKBK frequency Every 200 CKREF period 3'b010: watchdog compare CKREF&CKBK frequency Every 600 CKREF period 3'b011: watchdog compare CKREF&CKBK frequency Every 1200 CKREF period 3'b100: watchdog compare CKREF&CKBK frequency Every 3000 CKREF period 3'b101: watchdog compare CKREF&CKBK frequency Every 6400 CKREF period 3'b110: watchdog compare CKREF&CKBK frequency Every 12800 CKREF period 3'b111: watchdog compare CKREF&CKBK frequency Every 16000 CKREF period"]
    #[inline(always)]
    pub fn reg_time0_ck(&mut self) -> RegTime0CkW<'_, RegSscCtrl3Spec> {
        RegTime0CkW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ssc_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ssc_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSscCtrl3Spec;
impl crate::RegisterSpec for RegSscCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ssc_ctrl3::R`](R) reader structure"]
impl crate::Readable for RegSscCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ssc_ctrl3::W`](W) writer structure"]
impl crate::Writable for RegSscCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SSC_CTRL3 to value 0x2a00_56a1"]
impl crate::Resettable for RegSscCtrl3Spec {
    const RESET_VALUE: u32 = 0x2a00_56a1;
}
