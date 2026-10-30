#[doc = "Register `REG_AX_PWR_TSSI_TABLE13` reader"]
pub type R = crate::R<RegAxPwrTssiTable13Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE13` writer"]
pub type W = crate::W<RegAxPwrTssiTable13Spec>;
#[doc = "Field `r_f_tssi_diff_out_seg0_db` reader - "]
pub type RFTssiDiffOutSeg0DbR = crate::FieldReader;
#[doc = "Field `r_f_tssi_diff_out_seg0_db` writer - "]
pub type RFTssiDiffOutSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_heer_init_diff_seg0_db` reader - "]
pub type RHeerInitDiffSeg0DbR = crate::FieldReader;
#[doc = "Field `r_heer_init_diff_seg0_db` writer - "]
pub type RHeerInitDiffSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_heer_init_ofst_seg0_db` reader - "]
pub type RHeerInitOfstSeg0DbR = crate::FieldReader;
#[doc = "Field `r_heer_init_ofst_seg0_db` writer - "]
pub type RHeerInitOfstSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_hemu_init_diff_seg0_db` reader - "]
pub type RHemuInitDiffSeg0DbR = crate::FieldReader;
#[doc = "Field `r_hemu_init_diff_seg0_db` writer - "]
pub type RHemuInitDiffSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_f_tssi_diff_out_seg0_db(&self) -> RFTssiDiffOutSeg0DbR {
        RFTssiDiffOutSeg0DbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_heer_init_diff_seg0_db(&self) -> RHeerInitDiffSeg0DbR {
        RHeerInitDiffSeg0DbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_heer_init_ofst_seg0_db(&self) -> RHeerInitOfstSeg0DbR {
        RHeerInitOfstSeg0DbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_hemu_init_diff_seg0_db(&self) -> RHemuInitDiffSeg0DbR {
        RHemuInitDiffSeg0DbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_f_tssi_diff_out_seg0_db(
        &mut self,
    ) -> RFTssiDiffOutSeg0DbW<'_, RegAxPwrTssiTable13Spec> {
        RFTssiDiffOutSeg0DbW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_heer_init_diff_seg0_db(
        &mut self,
    ) -> RHeerInitDiffSeg0DbW<'_, RegAxPwrTssiTable13Spec> {
        RHeerInitDiffSeg0DbW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_heer_init_ofst_seg0_db(
        &mut self,
    ) -> RHeerInitOfstSeg0DbW<'_, RegAxPwrTssiTable13Spec> {
        RHeerInitOfstSeg0DbW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_hemu_init_diff_seg0_db(
        &mut self,
    ) -> RHemuInitDiffSeg0DbW<'_, RegAxPwrTssiTable13Spec> {
        RHemuInitDiffSeg0DbW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable13Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table13::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table13::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE13 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable13Spec {}
