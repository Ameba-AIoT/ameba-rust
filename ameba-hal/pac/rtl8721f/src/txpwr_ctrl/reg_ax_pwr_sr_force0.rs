#[doc = "Register `REG_AX_PWR_SR_FORCE0` reader"]
pub type R = crate::R<RegAxPwrSrForce0Spec>;
#[doc = "Register `REG_AX_PWR_SR_FORCE0` writer"]
pub type W = crate::W<RegAxPwrSrForce0Spec>;
#[doc = "Field `r_force_SR_txpwr_tolerance_value` reader - (RTL_only) Force input from MAC"]
pub type RForceSrTxpwrToleranceValueR = crate::FieldReader;
#[doc = "Field `r_force_SR_txpwr_tolerance_value` writer - (RTL_only) Force input from MAC"]
pub type RForceSrTxpwrToleranceValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_SR_txpwr_tolerance_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceSrTxpwrToleranceEnR = crate::BitReader;
#[doc = "Field `r_force_SR_txpwr_tolerance_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceSrTxpwrToleranceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_SR_rate_idx_value` reader - (RTL_only) Force input from MAC"]
pub type RForceSrRateIdxValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_SR_rate_idx_value` writer - (RTL_only) Force input from MAC"]
pub type RForceSrRateIdxValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_force_SR_rate_idx_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceSrRateIdxEnR = crate::BitReader;
#[doc = "Field `r_force_SR_rate_idx_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceSrRateIdxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_SR_txpwr_PD_value` reader - (RTL_only) Force input from MAC"]
pub type RForceSrTxpwrPdValueR = crate::FieldReader;
#[doc = "Field `r_force_SR_txpwr_PD_value` writer - (RTL_only) Force input from MAC"]
pub type RForceSrTxpwrPdValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_SR_txpwr_PD_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceSrTxpwrPdEnR = crate::BitReader;
#[doc = "Field `r_force_SR_txpwr_PD_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceSrTxpwrPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_SR_rate_dcm_value` reader - (RTL_only) Force input from MAC"]
pub type RForceSrRateDcmValueR = crate::BitReader;
#[doc = "Field `r_force_SR_rate_dcm_value` writer - (RTL_only) Force input from MAC"]
pub type RForceSrRateDcmValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_SR_rate_dcm_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceSrRateDcmEnR = crate::BitReader;
#[doc = "Field `r_force_SR_rate_dcm_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceSrRateDcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_SR_resp_dcm_value` reader - (RTL_only) Force input from MAC"]
pub type RForceSrRespDcmValueR = crate::BitReader;
#[doc = "Field `r_force_SR_resp_dcm_value` writer - (RTL_only) Force input from MAC"]
pub type RForceSrRespDcmValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_SR_resp_dcm_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceSrRespDcmEnR = crate::BitReader;
#[doc = "Field `r_force_SR_resp_dcm_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceSrRespDcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_txpwr_tolerance_value(&self) -> RForceSrTxpwrToleranceValueR {
        RForceSrTxpwrToleranceValueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_txpwr_tolerance_en(&self) -> RForceSrTxpwrToleranceEnR {
        RForceSrTxpwrToleranceEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:18 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_rate_idx_value(&self) -> RForceSrRateIdxValueR {
        RForceSrRateIdxValueR::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bit 19 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_rate_idx_en(&self) -> RForceSrRateIdxEnR {
        RForceSrRateIdxEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_txpwr_pd_value(&self) -> RForceSrTxpwrPdValueR {
        RForceSrTxpwrPdValueR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_txpwr_pd_en(&self) -> RForceSrTxpwrPdEnR {
        RForceSrTxpwrPdEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_rate_dcm_value(&self) -> RForceSrRateDcmValueR {
        RForceSrRateDcmValueR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_rate_dcm_en(&self) -> RForceSrRateDcmEnR {
        RForceSrRateDcmEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_resp_dcm_value(&self) -> RForceSrRespDcmValueR {
        RForceSrRespDcmValueR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_resp_dcm_en(&self) -> RForceSrRespDcmEnR {
        RForceSrRespDcmEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_txpwr_tolerance_value(
        &mut self,
    ) -> RForceSrTxpwrToleranceValueW<'_, RegAxPwrSrForce0Spec> {
        RForceSrTxpwrToleranceValueW::new(self, 0)
    }
    #[doc = "Bit 6 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_txpwr_tolerance_en(
        &mut self,
    ) -> RForceSrTxpwrToleranceEnW<'_, RegAxPwrSrForce0Spec> {
        RForceSrTxpwrToleranceEnW::new(self, 6)
    }
    #[doc = "Bits 7:18 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_rate_idx_value(&mut self) -> RForceSrRateIdxValueW<'_, RegAxPwrSrForce0Spec> {
        RForceSrRateIdxValueW::new(self, 7)
    }
    #[doc = "Bit 19 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_rate_idx_en(&mut self) -> RForceSrRateIdxEnW<'_, RegAxPwrSrForce0Spec> {
        RForceSrRateIdxEnW::new(self, 19)
    }
    #[doc = "Bits 20:25 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_txpwr_pd_value(&mut self) -> RForceSrTxpwrPdValueW<'_, RegAxPwrSrForce0Spec> {
        RForceSrTxpwrPdValueW::new(self, 20)
    }
    #[doc = "Bit 26 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_txpwr_pd_en(&mut self) -> RForceSrTxpwrPdEnW<'_, RegAxPwrSrForce0Spec> {
        RForceSrTxpwrPdEnW::new(self, 26)
    }
    #[doc = "Bit 27 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_rate_dcm_value(&mut self) -> RForceSrRateDcmValueW<'_, RegAxPwrSrForce0Spec> {
        RForceSrRateDcmValueW::new(self, 27)
    }
    #[doc = "Bit 28 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_rate_dcm_en(&mut self) -> RForceSrRateDcmEnW<'_, RegAxPwrSrForce0Spec> {
        RForceSrRateDcmEnW::new(self, 28)
    }
    #[doc = "Bit 29 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_resp_dcm_value(&mut self) -> RForceSrRespDcmValueW<'_, RegAxPwrSrForce0Spec> {
        RForceSrRespDcmValueW::new(self, 29)
    }
    #[doc = "Bit 30 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_resp_dcm_en(&mut self) -> RForceSrRespDcmEnW<'_, RegAxPwrSrForce0Spec> {
        RForceSrRespDcmEnW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_force0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_force0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrForce0Spec;
impl crate::RegisterSpec for RegAxPwrSrForce0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_force0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrForce0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_force0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrForce0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_FORCE0 to value 0"]
impl crate::Resettable for RegAxPwrSrForce0Spec {}
