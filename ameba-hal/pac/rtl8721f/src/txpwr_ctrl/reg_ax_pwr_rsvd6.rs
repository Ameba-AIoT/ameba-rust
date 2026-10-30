#[doc = "Register `REG_AX_PWR_RSVD6` reader"]
pub type R = crate::R<RegAxPwrRsvd6Spec>;
#[doc = "Register `REG_AX_PWR_RSVD6` writer"]
pub type W = crate::W<RegAxPwrRsvd6Spec>;
#[doc = "Field `r_txbias_legacy_region_2` reader - txbias region"]
pub type RTxbiasLegacyRegion2R = crate::FieldReader;
#[doc = "Field `r_txbias_legacy_region_2` writer - txbias region"]
pub type RTxbiasLegacyRegion2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_legacy_region_3` reader - txbias region"]
pub type RTxbiasLegacyRegion3R = crate::FieldReader;
#[doc = "Field `r_txbias_legacy_region_3` writer - txbias region"]
pub type RTxbiasLegacyRegion3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_ht_region_2` reader - txbias region"]
pub type RTxbiasHtRegion2R = crate::FieldReader;
#[doc = "Field `r_txbias_ht_region_2` writer - txbias region"]
pub type RTxbiasHtRegion2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_ht_region_3` reader - txbias region"]
pub type RTxbiasHtRegion3R = crate::FieldReader;
#[doc = "Field `r_txbias_ht_region_3` writer - txbias region"]
pub type RTxbiasHtRegion3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_vht_region_2` reader - txbias region"]
pub type RTxbiasVhtRegion2R = crate::FieldReader;
#[doc = "Field `r_txbias_vht_region_2` writer - txbias region"]
pub type RTxbiasVhtRegion2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_vht_region_3` reader - txbias region"]
pub type RTxbiasVhtRegion3R = crate::FieldReader;
#[doc = "Field `r_txbias_vht_region_3` writer - txbias region"]
pub type RTxbiasVhtRegion3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_he_region_2` reader - txbias region"]
pub type RTxbiasHeRegion2R = crate::FieldReader;
#[doc = "Field `r_txbias_he_region_2` writer - txbias region"]
pub type RTxbiasHeRegion2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_txbias_he_region_3` reader - txbias region"]
pub type RTxbiasHeRegion3R = crate::FieldReader;
#[doc = "Field `r_txbias_he_region_3` writer - txbias region"]
pub type RTxbiasHeRegion3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_region_2(&self) -> RTxbiasLegacyRegion2R {
        RTxbiasLegacyRegion2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_region_3(&self) -> RTxbiasLegacyRegion3R {
        RTxbiasLegacyRegion3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_region_2(&self) -> RTxbiasHtRegion2R {
        RTxbiasHtRegion2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_region_3(&self) -> RTxbiasHtRegion3R {
        RTxbiasHtRegion3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_region_2(&self) -> RTxbiasVhtRegion2R {
        RTxbiasVhtRegion2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_region_3(&self) -> RTxbiasVhtRegion3R {
        RTxbiasVhtRegion3R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_region_2(&self) -> RTxbiasHeRegion2R {
        RTxbiasHeRegion2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_region_3(&self) -> RTxbiasHeRegion3R {
        RTxbiasHeRegion3R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_region_2(&mut self) -> RTxbiasLegacyRegion2W<'_, RegAxPwrRsvd6Spec> {
        RTxbiasLegacyRegion2W::new(self, 0)
    }
    #[doc = "Bits 2:3 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_legacy_region_3(&mut self) -> RTxbiasLegacyRegion3W<'_, RegAxPwrRsvd6Spec> {
        RTxbiasLegacyRegion3W::new(self, 2)
    }
    #[doc = "Bits 4:5 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_region_2(&mut self) -> RTxbiasHtRegion2W<'_, RegAxPwrRsvd6Spec> {
        RTxbiasHtRegion2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_ht_region_3(&mut self) -> RTxbiasHtRegion3W<'_, RegAxPwrRsvd6Spec> {
        RTxbiasHtRegion3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_region_2(&mut self) -> RTxbiasVhtRegion2W<'_, RegAxPwrRsvd6Spec> {
        RTxbiasVhtRegion2W::new(self, 8)
    }
    #[doc = "Bits 10:11 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_vht_region_3(&mut self) -> RTxbiasVhtRegion3W<'_, RegAxPwrRsvd6Spec> {
        RTxbiasVhtRegion3W::new(self, 10)
    }
    #[doc = "Bits 12:13 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_region_2(&mut self) -> RTxbiasHeRegion2W<'_, RegAxPwrRsvd6Spec> {
        RTxbiasHeRegion2W::new(self, 12)
    }
    #[doc = "Bits 14:15 - txbias region"]
    #[inline(always)]
    pub fn r_txbias_he_region_3(&mut self) -> RTxbiasHeRegion3W<'_, RegAxPwrRsvd6Spec> {
        RTxbiasHeRegion3W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd6Spec;
impl crate::RegisterSpec for RegAxPwrRsvd6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd6::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd6::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD6 to value 0"]
impl crate::Resettable for RegAxPwrRsvd6Spec {}
