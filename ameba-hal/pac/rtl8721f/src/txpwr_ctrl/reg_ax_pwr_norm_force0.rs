#[doc = "Register `REG_AX_PWR_NORM_FORCE0` reader"]
pub type R = crate::R<RegAxPwrNormForce0Spec>;
#[doc = "Register `REG_AX_PWR_NORM_FORCE0` writer"]
pub type W = crate::W<RegAxPwrNormForce0Spec>;
#[doc = "Field `r_force_max_rate_value` reader - (RTL_only) Force input from MAC"]
pub type RForceMaxRateValueR = crate::FieldReader<u16>;
#[doc = "Field `r_force_max_rate_value` writer - (RTL_only) Force input from MAC"]
pub type RForceMaxRateValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `r_force_max_rate_en` reader - (RTL_only) Force input from MAC"]
pub type RForceMaxRateEnR = crate::BitReader;
#[doc = "Field `r_force_max_rate_en` writer - (RTL_only) Force input from MAC"]
pub type RForceMaxRateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_DBW_idx_value` reader - (RTL_only) Force input from MAC"]
pub type RForceDbwIdxValueR = crate::FieldReader;
#[doc = "Field `r_force_DBW_idx_value` writer - (RTL_only) Force input from MAC"]
pub type RForceDbwIdxValueW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_DBW_idx_en` reader - (RTL_only) Force input from MAC"]
pub type RForceDbwIdxEnR = crate::BitReader;
#[doc = "Field `r_force_DBW_idx_en` writer - (RTL_only) Force input from MAC"]
pub type RForceDbwIdxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_dcm_value` reader - (RTL_only) Force input from MAC"]
pub type RForceDcmValueR = crate::BitReader;
#[doc = "Field `r_force_dcm_value` writer - (RTL_only) Force input from MAC"]
pub type RForceDcmValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_dcm_en` reader - (RTL_only) Force input from MAC"]
pub type RForceDcmEnR = crate::BitReader;
#[doc = "Field `r_force_dcm_en` writer - (RTL_only) Force input from MAC"]
pub type RForceDcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_RF_BW_idx_value` reader - (RTL_only) Force input from MAC"]
pub type RForceRfBwIdxValueR = crate::FieldReader;
#[doc = "Field `r_force_RF_BW_idx_value` writer - (RTL_only) Force input from MAC"]
pub type RForceRfBwIdxValueW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_force_RF_BW_idx_en` reader - (RTL_only) Force input from MAC"]
pub type RForceRfBwIdxEnR = crate::BitReader;
#[doc = "Field `r_force_RF_BW_idx_en` writer - (RTL_only) Force input from MAC"]
pub type RForceRfBwIdxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_BW80p80_en_value` reader - (RTL_only) Force input from MAC"]
pub type RForceBw80p80EnValueR = crate::BitReader;
#[doc = "Field `r_force_BW80p80_en_value` writer - (RTL_only) Force input from MAC"]
pub type RForceBw80p80EnValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_BW80p80_en_en` reader - (RTL_only) Force input from MAC"]
pub type RForceBw80p80EnEnR = crate::BitReader;
#[doc = "Field `r_force_BW80p80_en_en` writer - (RTL_only) Force input from MAC"]
pub type RForceBw80p80EnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_macid_value` reader - (RTL_only) Force input from MAC"]
pub type RForceMacidValueR = crate::FieldReader;
#[doc = "Field `r_force_macid_value` writer - (RTL_only) Force input from MAC"]
pub type RForceMacidValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_force_macid_en` reader - (RTL_only) Force input from MAC"]
pub type RForceMacidEnR = crate::BitReader;
#[doc = "Field `r_force_macid_en` writer - (RTL_only) Force input from MAC"]
pub type RForceMacidEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_max_rate_value(&self) -> RForceMaxRateValueR {
        RForceMaxRateValueR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_max_rate_en(&self) -> RForceMaxRateEnR {
        RForceMaxRateEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_dbw_idx_value(&self) -> RForceDbwIdxValueR {
        RForceDbwIdxValueR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_dbw_idx_en(&self) -> RForceDbwIdxEnR {
        RForceDbwIdxEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_dcm_value(&self) -> RForceDcmValueR {
        RForceDcmValueR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_dcm_en(&self) -> RForceDcmEnR {
        RForceDcmEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rf_bw_idx_value(&self) -> RForceRfBwIdxValueR {
        RForceRfBwIdxValueR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rf_bw_idx_en(&self) -> RForceRfBwIdxEnR {
        RForceRfBwIdxEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_bw80p80_en_value(&self) -> RForceBw80p80EnValueR {
        RForceBw80p80EnValueR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_bw80p80_en_en(&self) -> RForceBw80p80EnEnR {
        RForceBw80p80EnEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:29 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_macid_value(&self) -> RForceMacidValueR {
        RForceMacidValueR::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_macid_en(&self) -> RForceMacidEnR {
        RForceMacidEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_max_rate_value(&mut self) -> RForceMaxRateValueW<'_, RegAxPwrNormForce0Spec> {
        RForceMaxRateValueW::new(self, 0)
    }
    #[doc = "Bit 12 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_max_rate_en(&mut self) -> RForceMaxRateEnW<'_, RegAxPwrNormForce0Spec> {
        RForceMaxRateEnW::new(self, 12)
    }
    #[doc = "Bits 13:14 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_dbw_idx_value(&mut self) -> RForceDbwIdxValueW<'_, RegAxPwrNormForce0Spec> {
        RForceDbwIdxValueW::new(self, 13)
    }
    #[doc = "Bit 15 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_dbw_idx_en(&mut self) -> RForceDbwIdxEnW<'_, RegAxPwrNormForce0Spec> {
        RForceDbwIdxEnW::new(self, 15)
    }
    #[doc = "Bit 16 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_dcm_value(&mut self) -> RForceDcmValueW<'_, RegAxPwrNormForce0Spec> {
        RForceDcmValueW::new(self, 16)
    }
    #[doc = "Bit 17 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_dcm_en(&mut self) -> RForceDcmEnW<'_, RegAxPwrNormForce0Spec> {
        RForceDcmEnW::new(self, 17)
    }
    #[doc = "Bits 18:19 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rf_bw_idx_value(&mut self) -> RForceRfBwIdxValueW<'_, RegAxPwrNormForce0Spec> {
        RForceRfBwIdxValueW::new(self, 18)
    }
    #[doc = "Bit 20 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_rf_bw_idx_en(&mut self) -> RForceRfBwIdxEnW<'_, RegAxPwrNormForce0Spec> {
        RForceRfBwIdxEnW::new(self, 20)
    }
    #[doc = "Bit 21 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_bw80p80_en_value(
        &mut self,
    ) -> RForceBw80p80EnValueW<'_, RegAxPwrNormForce0Spec> {
        RForceBw80p80EnValueW::new(self, 21)
    }
    #[doc = "Bit 22 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_bw80p80_en_en(&mut self) -> RForceBw80p80EnEnW<'_, RegAxPwrNormForce0Spec> {
        RForceBw80p80EnEnW::new(self, 22)
    }
    #[doc = "Bits 23:29 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_macid_value(&mut self) -> RForceMacidValueW<'_, RegAxPwrNormForce0Spec> {
        RForceMacidValueW::new(self, 23)
    }
    #[doc = "Bit 30 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_macid_en(&mut self) -> RForceMacidEnW<'_, RegAxPwrNormForce0Spec> {
        RForceMacidEnW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_norm_force0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_norm_force0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrNormForce0Spec;
impl crate::RegisterSpec for RegAxPwrNormForce0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_norm_force0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrNormForce0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_norm_force0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrNormForce0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_NORM_FORCE0 to value 0x0020_0000"]
impl crate::Resettable for RegAxPwrNormForce0Spec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
