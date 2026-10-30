#[doc = "Register `REG_AX_PWR_RSVD3` reader"]
pub type R = crate::R<RegAxPwrRsvd3Spec>;
#[doc = "Register `REG_AX_PWR_RSVD3` writer"]
pub type W = crate::W<RegAxPwrRsvd3Spec>;
#[doc = "Field `r_sta_ul_log_norm_BW20_2tx` reader - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw20_2txR = crate::FieldReader;
#[doc = "Field `r_sta_ul_log_norm_BW20_2tx` writer - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw20_2txW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_sta_ul_log_norm_BW40_2tx` reader - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw40_2txR = crate::FieldReader;
#[doc = "Field `r_sta_ul_log_norm_BW40_2tx` writer - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw40_2txW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_sta_ul_log_norm_BW80_2tx` reader - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw80_2txR = crate::FieldReader;
#[doc = "Field `r_sta_ul_log_norm_BW80_2tx` writer - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw80_2txW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_sta_ul_log_norm_BW260_2tx` reader - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw260_2txR = crate::FieldReader;
#[doc = "Field `r_sta_ul_log_norm_BW260_2tx` writer - s(8,2) sta ul txpwr log norm"]
pub type RStaUlLogNormBw260_2txW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw20_2tx(&self) -> RStaUlLogNormBw20_2txR {
        RStaUlLogNormBw20_2txR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw40_2tx(&self) -> RStaUlLogNormBw40_2txR {
        RStaUlLogNormBw40_2txR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw80_2tx(&self) -> RStaUlLogNormBw80_2txR {
        RStaUlLogNormBw80_2txR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw260_2tx(&self) -> RStaUlLogNormBw260_2txR {
        RStaUlLogNormBw260_2txR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw20_2tx(&mut self) -> RStaUlLogNormBw20_2txW<'_, RegAxPwrRsvd3Spec> {
        RStaUlLogNormBw20_2txW::new(self, 0)
    }
    #[doc = "Bits 8:15 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw40_2tx(&mut self) -> RStaUlLogNormBw40_2txW<'_, RegAxPwrRsvd3Spec> {
        RStaUlLogNormBw40_2txW::new(self, 8)
    }
    #[doc = "Bits 16:23 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw80_2tx(&mut self) -> RStaUlLogNormBw80_2txW<'_, RegAxPwrRsvd3Spec> {
        RStaUlLogNormBw80_2txW::new(self, 16)
    }
    #[doc = "Bits 24:31 - s(8,2) sta ul txpwr log norm"]
    #[inline(always)]
    pub fn r_sta_ul_log_norm_bw260_2tx(
        &mut self,
    ) -> RStaUlLogNormBw260_2txW<'_, RegAxPwrRsvd3Spec> {
        RStaUlLogNormBw260_2txW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd3Spec;
impl crate::RegisterSpec for RegAxPwrRsvd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD3 to value 0"]
impl crate::Resettable for RegAxPwrRsvd3Spec {}
