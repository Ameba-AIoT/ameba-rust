#[doc = "Register `REG_AX_PWR_RSVD2` reader"]
pub type R = crate::R<RegAxPwrRsvd2Spec>;
#[doc = "Register `REG_AX_PWR_RSVD2` writer"]
pub type W = crate::W<RegAxPwrRsvd2Spec>;
#[doc = "Field `r_sta_ul_log_norm_BW20_1tx` reader - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw20_1txR = crate::FieldReader;
#[doc = "Field `r_sta_ul_log_norm_BW20_1tx` writer - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw20_1txW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_sta_ul_log_norm_BW40_1tx` reader - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw40_1txR = crate::FieldReader;
#[doc = "Field `r_sta_ul_log_norm_BW40_1tx` writer - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw40_1txW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_sta_ul_log_norm_BW80_1tx` reader - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw80_1txR = crate::FieldReader;
#[doc = "Field `r_sta_ul_log_norm_BW80_1tx` writer - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw80_1txW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_sta_ul_log_norm_BW160_1tx` reader - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw160_1txR = crate::FieldReader;
#[doc = "Field `r_sta_ul_log_norm_BW160_1tx` writer - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw160_1txW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw20_1tx(&self) -> RStaUlLogNormBw20_1txR {
        RStaUlLogNormBw20_1txR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw40_1tx(&self) -> RStaUlLogNormBw40_1txR {
        RStaUlLogNormBw40_1txR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw80_1tx(&self) -> RStaUlLogNormBw80_1txR {
        RStaUlLogNormBw80_1txR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw160_1tx(&self) -> RStaUlLogNormBw160_1txR {
        RStaUlLogNormBw160_1txR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw20_1tx(&mut self) -> RStaUlLogNormBw20_1txW<'_, RegAxPwrRsvd2Spec> {
        RStaUlLogNormBw20_1txW::new(self, 0)
    }
    #[doc = "Bits 8:15 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw40_1tx(&mut self) -> RStaUlLogNormBw40_1txW<'_, RegAxPwrRsvd2Spec> {
        RStaUlLogNormBw40_1txW::new(self, 8)
    }
    #[doc = "Bits 16:23 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw80_1tx(&mut self) -> RStaUlLogNormBw80_1txW<'_, RegAxPwrRsvd2Spec> {
        RStaUlLogNormBw80_1txW::new(self, 16)
    }
    #[doc = "Bits 24:31 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw160_1tx(
        &mut self,
    ) -> RStaUlLogNormBw160_1txW<'_, RegAxPwrRsvd2Spec> {
        RStaUlLogNormBw160_1txW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd2Spec;
impl crate::RegisterSpec for RegAxPwrRsvd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD2 to value 0"]
impl crate::Resettable for RegAxPwrRsvd2Spec {}
