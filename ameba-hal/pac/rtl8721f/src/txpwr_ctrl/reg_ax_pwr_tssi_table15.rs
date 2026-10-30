#[doc = "Register `REG_AX_PWR_TSSI_TABLE15` reader"]
pub type R = crate::R<RegAxPwrTssiTable15Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE15` writer"]
pub type W = crate::W<RegAxPwrTssiTable15Spec>;
#[doc = "Field `r_hetb_init_diff_seg0_db` reader - "]
pub type RHetbInitDiffSeg0DbR = crate::FieldReader;
#[doc = "Field `r_hetb_init_diff_seg0_db` writer - "]
pub type RHetbInitDiffSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_hetb_init_ofst_athestf_seg0_db` reader - "]
pub type RHetbInitOfstAthestfSeg0DbR = crate::FieldReader;
#[doc = "Field `r_hetb_init_ofst_athestf_seg0_db` writer - "]
pub type RHetbInitOfstAthestfSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_hetb_init_ofst_seg0_db` reader - "]
pub type RHetbInitOfstSeg0DbR = crate::FieldReader;
#[doc = "Field `r_hetb_init_ofst_seg0_db` writer - "]
pub type RHetbInitOfstSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_ht_init_diff_seg0_db` reader - "]
pub type RHtInitDiffSeg0DbR = crate::FieldReader;
#[doc = "Field `r_ht_init_diff_seg0_db` writer - "]
pub type RHtInitDiffSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_hetb_init_diff_seg0_db(&self) -> RHetbInitDiffSeg0DbR {
        RHetbInitDiffSeg0DbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_hetb_init_ofst_athestf_seg0_db(&self) -> RHetbInitOfstAthestfSeg0DbR {
        RHetbInitOfstAthestfSeg0DbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_hetb_init_ofst_seg0_db(&self) -> RHetbInitOfstSeg0DbR {
        RHetbInitOfstSeg0DbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_ht_init_diff_seg0_db(&self) -> RHtInitDiffSeg0DbR {
        RHtInitDiffSeg0DbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_hetb_init_diff_seg0_db(
        &mut self,
    ) -> RHetbInitDiffSeg0DbW<'_, RegAxPwrTssiTable15Spec> {
        RHetbInitDiffSeg0DbW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_hetb_init_ofst_athestf_seg0_db(
        &mut self,
    ) -> RHetbInitOfstAthestfSeg0DbW<'_, RegAxPwrTssiTable15Spec> {
        RHetbInitOfstAthestfSeg0DbW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_hetb_init_ofst_seg0_db(
        &mut self,
    ) -> RHetbInitOfstSeg0DbW<'_, RegAxPwrTssiTable15Spec> {
        RHetbInitOfstSeg0DbW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_ht_init_diff_seg0_db(&mut self) -> RHtInitDiffSeg0DbW<'_, RegAxPwrTssiTable15Spec> {
        RHtInitDiffSeg0DbW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable15Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table15::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table15::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE15 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable15Spec {}
