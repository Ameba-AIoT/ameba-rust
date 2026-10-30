#[doc = "Register `REG_AX_PWR_SR_FORCE1` reader"]
pub type R = crate::R<RegAxPwrSrForce1Spec>;
#[doc = "Register `REG_AX_PWR_SR_FORCE1` writer"]
pub type W = crate::W<RegAxPwrSrForce1Spec>;
#[doc = "Field `r_force_SR_resp_txpwr_PD_value` reader - (RTL_only) Force input from MAC"]
pub type RForceSrRespTxpwrPdValueR = crate::FieldReader;
#[doc = "Field `r_force_SR_resp_txpwr_PD_value` writer - (RTL_only) Force input from MAC"]
pub type RForceSrRespTxpwrPdValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_SR_resp_txpwr_PD_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceSrRespTxpwrPdEnR = crate::BitReader;
#[doc = "Field `r_force_SR_resp_txpwr_PD_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceSrRespTxpwrPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_SR_resp_rate_idx_value` reader - (RTL_only) Force input from MAC"]
pub type RForceSrRespRateIdxValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_SR_resp_rate_idx_value` writer - (RTL_only) Force input from MAC"]
pub type RForceSrRespRateIdxValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_force_SR_resp_rate_idx_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceSrRespRateIdxEnR = crate::BitReader;
#[doc = "Field `r_force_SR_resp_rate_idx_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceSrRespRateIdxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_RPL_value` reader - (RTL_only) Force input from MAC"]
pub type RForceRplValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_RPL_value` writer - (RTL_only) Force input from MAC"]
pub type RForceRplValueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_RPL_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceRplEnR = crate::BitReader;
#[doc = "Field `r_force_RPL_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceRplEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_resp_txpwr_pd_value(&self) -> RForceSrRespTxpwrPdValueR {
        RForceSrRespTxpwrPdValueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_resp_txpwr_pd_en(&self) -> RForceSrRespTxpwrPdEnR {
        RForceSrRespTxpwrPdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:18 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_resp_rate_idx_value(&self) -> RForceSrRespRateIdxValueR {
        RForceSrRespRateIdxValueR::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bit 19 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_resp_rate_idx_en(&self) -> RForceSrRespRateIdxEnR {
        RForceSrRespRateIdxEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rpl_value(&self) -> RForceRplValueR {
        RForceRplValueR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_rpl_en(&self) -> RForceRplEnR {
        RForceRplEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_resp_txpwr_pd_value(
        &mut self,
    ) -> RForceSrRespTxpwrPdValueW<'_, RegAxPwrSrForce1Spec> {
        RForceSrRespTxpwrPdValueW::new(self, 0)
    }
    #[doc = "Bit 6 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_resp_txpwr_pd_en(
        &mut self,
    ) -> RForceSrRespTxpwrPdEnW<'_, RegAxPwrSrForce1Spec> {
        RForceSrRespTxpwrPdEnW::new(self, 6)
    }
    #[doc = "Bits 7:18 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_sr_resp_rate_idx_value(
        &mut self,
    ) -> RForceSrRespRateIdxValueW<'_, RegAxPwrSrForce1Spec> {
        RForceSrRespRateIdxValueW::new(self, 7)
    }
    #[doc = "Bit 19 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_sr_resp_rate_idx_en(
        &mut self,
    ) -> RForceSrRespRateIdxEnW<'_, RegAxPwrSrForce1Spec> {
        RForceSrRespRateIdxEnW::new(self, 19)
    }
    #[doc = "Bits 20:28 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rpl_value(&mut self) -> RForceRplValueW<'_, RegAxPwrSrForce1Spec> {
        RForceRplValueW::new(self, 20)
    }
    #[doc = "Bit 29 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_rpl_en(&mut self) -> RForceRplEnW<'_, RegAxPwrSrForce1Spec> {
        RForceRplEnW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_force1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_force1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrForce1Spec;
impl crate::RegisterSpec for RegAxPwrSrForce1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_force1::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrForce1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_force1::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrForce1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_FORCE1 to value 0"]
impl crate::Resettable for RegAxPwrSrForce1Spec {}
