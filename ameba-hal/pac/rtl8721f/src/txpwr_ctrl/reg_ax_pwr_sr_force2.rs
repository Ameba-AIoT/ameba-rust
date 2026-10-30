#[doc = "Register `REG_AX_PWR_SR_FORCE2` reader"]
pub type R = crate::R<RegAxPwrSrForce2Spec>;
#[doc = "Register `REG_AX_PWR_SR_FORCE2` writer"]
pub type W = crate::W<RegAxPwrSrForce2Spec>;
#[doc = "Field `r_force_RPL_upper_value` reader - (RTL_only) Force input from MAC"]
pub type RForceRplUpperValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_RPL_upper_value` writer - (RTL_only) Force input from MAC"]
pub type RForceRplUpperValueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_force_RPL_upper_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceRplUpperEnR = crate::BitReader;
#[doc = "Field `r_force_RPL_upper_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceRplUpperEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_RPL_upper_en_value` reader - (RTL_only) Force input from MAC"]
pub type RForceRplUpperEnValueR = crate::BitReader;
#[doc = "Field `r_force_RPL_upper_en_value` writer - (RTL_only) Force input from MAC"]
pub type RForceRplUpperEnValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_RPL_upper_en_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceRplUpperEnEnR = crate::BitReader;
#[doc = "Field `r_force_RPL_upper_en_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceRplUpperEnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_TF_rate_idx_value` reader - (RTL_only) Force input from MAC"]
pub type RForceTfRateIdxValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_TF_rate_idx_value` writer - (RTL_only) Force input from MAC"]
pub type RForceTfRateIdxValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_force_TF_rate_idx_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceTfRateIdxEnR = crate::BitReader;
#[doc = "Field `r_force_TF_rate_idx_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceTfRateIdxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_TF_AP_tx_PWR_value` reader - (RTL_only) Force input from MAC"]
pub type RForceTfApTxPwrValueR = crate::FieldReader;
#[doc = "Field `r_force_TF_AP_tx_PWR_value` writer - (RTL_only) Force input from MAC"]
pub type RForceTfApTxPwrValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_force_TF_AP_tx_PWR_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceTfApTxPwrEnR = crate::BitReader;
#[doc = "Field `r_force_TF_AP_tx_PWR_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceTfApTxPwrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rpl_upper_value(&self) -> RForceRplUpperValueR {
        RForceRplUpperValueR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_rpl_upper_en(&self) -> RForceRplUpperEnR {
        RForceRplUpperEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rpl_upper_en_value(&self) -> RForceRplUpperEnValueR {
        RForceRplUpperEnValueR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_rpl_upper_en_en(&self) -> RForceRplUpperEnEnR {
        RForceRplUpperEnEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:23 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tf_rate_idx_value(&self) -> RForceTfRateIdxValueR {
        RForceTfRateIdxValueR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tf_rate_idx_en(&self) -> RForceTfRateIdxEnR {
        RForceTfRateIdxEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tf_ap_tx_pwr_value(&self) -> RForceTfApTxPwrValueR {
        RForceTfApTxPwrValueR::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tf_ap_tx_pwr_en(&self) -> RForceTfApTxPwrEnR {
        RForceTfApTxPwrEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rpl_upper_value(&mut self) -> RForceRplUpperValueW<'_, RegAxPwrSrForce2Spec> {
        RForceRplUpperValueW::new(self, 0)
    }
    #[doc = "Bit 9 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_rpl_upper_en(&mut self) -> RForceRplUpperEnW<'_, RegAxPwrSrForce2Spec> {
        RForceRplUpperEnW::new(self, 9)
    }
    #[doc = "Bit 10 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rpl_upper_en_value(
        &mut self,
    ) -> RForceRplUpperEnValueW<'_, RegAxPwrSrForce2Spec> {
        RForceRplUpperEnValueW::new(self, 10)
    }
    #[doc = "Bit 11 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_rpl_upper_en_en(&mut self) -> RForceRplUpperEnEnW<'_, RegAxPwrSrForce2Spec> {
        RForceRplUpperEnEnW::new(self, 11)
    }
    #[doc = "Bits 12:23 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tf_rate_idx_value(&mut self) -> RForceTfRateIdxValueW<'_, RegAxPwrSrForce2Spec> {
        RForceTfRateIdxValueW::new(self, 12)
    }
    #[doc = "Bit 24 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tf_rate_idx_en(&mut self) -> RForceTfRateIdxEnW<'_, RegAxPwrSrForce2Spec> {
        RForceTfRateIdxEnW::new(self, 24)
    }
    #[doc = "Bits 25:30 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tf_ap_tx_pwr_value(
        &mut self,
    ) -> RForceTfApTxPwrValueW<'_, RegAxPwrSrForce2Spec> {
        RForceTfApTxPwrValueW::new(self, 25)
    }
    #[doc = "Bit 31 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tf_ap_tx_pwr_en(&mut self) -> RForceTfApTxPwrEnW<'_, RegAxPwrSrForce2Spec> {
        RForceTfApTxPwrEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_sr_force2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_sr_force2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrSrForce2Spec;
impl crate::RegisterSpec for RegAxPwrSrForce2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_sr_force2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrSrForce2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_sr_force2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrSrForce2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_SR_FORCE2 to value 0"]
impl crate::Resettable for RegAxPwrSrForce2Spec {}
