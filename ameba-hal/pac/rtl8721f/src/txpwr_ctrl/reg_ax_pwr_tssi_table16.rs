#[doc = "Register `REG_AX_PWR_TSSI_TABLE16` reader"]
pub type R = crate::R<RegAxPwrTssiTable16Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE16` writer"]
pub type W = crate::W<RegAxPwrTssiTable16Spec>;
#[doc = "Field `r_ht_init_ofst_seg0_db` reader - "]
pub type RHtInitOfstSeg0DbR = crate::FieldReader;
#[doc = "Field `r_ht_init_ofst_seg0_db` writer - "]
pub type RHtInitOfstSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_legacy_init_diff_seg0_db` reader - "]
pub type RLegacyInitDiffSeg0DbR = crate::FieldReader;
#[doc = "Field `r_legacy_init_diff_seg0_db` writer - "]
pub type RLegacyInitDiffSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_legacy_init_ofst_seg0_db` reader - "]
pub type RLegacyInitOfstSeg0DbR = crate::FieldReader;
#[doc = "Field `r_legacy_init_ofst_seg0_db` writer - "]
pub type RLegacyInitOfstSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_vht_init_diff_seg0_db` reader - "]
pub type RVhtInitDiffSeg0DbR = crate::FieldReader;
#[doc = "Field `r_vht_init_diff_seg0_db` writer - "]
pub type RVhtInitDiffSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_ht_init_ofst_seg0_db(&self) -> RHtInitOfstSeg0DbR {
        RHtInitOfstSeg0DbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_legacy_init_diff_seg0_db(&self) -> RLegacyInitDiffSeg0DbR {
        RLegacyInitDiffSeg0DbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_legacy_init_ofst_seg0_db(&self) -> RLegacyInitOfstSeg0DbR {
        RLegacyInitOfstSeg0DbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_vht_init_diff_seg0_db(&self) -> RVhtInitDiffSeg0DbR {
        RVhtInitDiffSeg0DbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_ht_init_ofst_seg0_db(&mut self) -> RHtInitOfstSeg0DbW<'_, RegAxPwrTssiTable16Spec> {
        RHtInitOfstSeg0DbW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_legacy_init_diff_seg0_db(
        &mut self,
    ) -> RLegacyInitDiffSeg0DbW<'_, RegAxPwrTssiTable16Spec> {
        RLegacyInitDiffSeg0DbW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_legacy_init_ofst_seg0_db(
        &mut self,
    ) -> RLegacyInitOfstSeg0DbW<'_, RegAxPwrTssiTable16Spec> {
        RLegacyInitOfstSeg0DbW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_vht_init_diff_seg0_db(&mut self) -> RVhtInitDiffSeg0DbW<'_, RegAxPwrTssiTable16Spec> {
        RVhtInitDiffSeg0DbW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable16Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table16::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable16Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table16::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE16 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable16Spec {}
