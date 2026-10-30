#[doc = "Register `REG_AX_PWR_TSSI_TABLE14` reader"]
pub type R = crate::R<RegAxPwrTssiTable14Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE14` writer"]
pub type W = crate::W<RegAxPwrTssiTable14Spec>;
#[doc = "Field `r_hemu_init_ofst_seg0_db` reader - "]
pub type RHemuInitOfstSeg0DbR = crate::FieldReader;
#[doc = "Field `r_hemu_init_ofst_seg0_db` writer - "]
pub type RHemuInitOfstSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_hesu_init_diff_seg0_db` reader - "]
pub type RHesuInitDiffSeg0DbR = crate::FieldReader;
#[doc = "Field `r_hesu_init_diff_seg0_db` writer - "]
pub type RHesuInitDiffSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_hesu_init_ofst_seg0_db` reader - "]
pub type RHesuInitOfstSeg0DbR = crate::FieldReader;
#[doc = "Field `r_hesu_init_ofst_seg0_db` writer - "]
pub type RHesuInitOfstSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_hetb_init_diff_athestf_seg0_db` reader - "]
pub type RHetbInitDiffAthestfSeg0DbR = crate::FieldReader;
#[doc = "Field `r_hetb_init_diff_athestf_seg0_db` writer - "]
pub type RHetbInitDiffAthestfSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_hemu_init_ofst_seg0_db(&self) -> RHemuInitOfstSeg0DbR {
        RHemuInitOfstSeg0DbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_hesu_init_diff_seg0_db(&self) -> RHesuInitDiffSeg0DbR {
        RHesuInitDiffSeg0DbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_hesu_init_ofst_seg0_db(&self) -> RHesuInitOfstSeg0DbR {
        RHesuInitOfstSeg0DbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_hetb_init_diff_athestf_seg0_db(&self) -> RHetbInitDiffAthestfSeg0DbR {
        RHetbInitDiffAthestfSeg0DbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_hemu_init_ofst_seg0_db(
        &mut self,
    ) -> RHemuInitOfstSeg0DbW<'_, RegAxPwrTssiTable14Spec> {
        RHemuInitOfstSeg0DbW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_hesu_init_diff_seg0_db(
        &mut self,
    ) -> RHesuInitDiffSeg0DbW<'_, RegAxPwrTssiTable14Spec> {
        RHesuInitDiffSeg0DbW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_hesu_init_ofst_seg0_db(
        &mut self,
    ) -> RHesuInitOfstSeg0DbW<'_, RegAxPwrTssiTable14Spec> {
        RHesuInitOfstSeg0DbW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_hetb_init_diff_athestf_seg0_db(
        &mut self,
    ) -> RHetbInitDiffAthestfSeg0DbW<'_, RegAxPwrTssiTable14Spec> {
        RHetbInitDiffAthestfSeg0DbW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable14Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table14::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table14::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE14 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable14Spec {}
