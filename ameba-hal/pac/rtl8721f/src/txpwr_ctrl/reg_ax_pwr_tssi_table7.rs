#[doc = "Register `REG_AX_PWR_TSSI_TABLE7` reader"]
pub type R = crate::R<RegAxPwrTssiTable7Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE7` writer"]
pub type W = crate::W<RegAxPwrTssiTable7Spec>;
#[doc = "Field `r_bandedge_zero_cfg_sbw80_2` reader - "]
pub type RBandedgeZeroCfgSbw80_2R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw80_2` writer - "]
pub type RBandedgeZeroCfgSbw80_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_zero_cfg_sbw80_3` reader - "]
pub type RBandedgeZeroCfgSbw80_3R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw80_3` writer - "]
pub type RBandedgeZeroCfgSbw80_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bmode_init_diff_seg0_db` reader - "]
pub type RBmodeInitDiffSeg0DbR = crate::FieldReader;
#[doc = "Field `r_bmode_init_diff_seg0_db` writer - "]
pub type RBmodeInitDiffSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bmode_init_ofst_seg0_db` reader - "]
pub type RBmodeInitOfstSeg0DbR = crate::FieldReader;
#[doc = "Field `r_bmode_init_ofst_seg0_db` writer - "]
pub type RBmodeInitOfstSeg0DbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw80_2(&self) -> RBandedgeZeroCfgSbw80_2R {
        RBandedgeZeroCfgSbw80_2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw80_3(&self) -> RBandedgeZeroCfgSbw80_3R {
        RBandedgeZeroCfgSbw80_3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bmode_init_diff_seg0_db(&self) -> RBmodeInitDiffSeg0DbR {
        RBmodeInitDiffSeg0DbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bmode_init_ofst_seg0_db(&self) -> RBmodeInitOfstSeg0DbR {
        RBmodeInitOfstSeg0DbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw80_2(
        &mut self,
    ) -> RBandedgeZeroCfgSbw80_2W<'_, RegAxPwrTssiTable7Spec> {
        RBandedgeZeroCfgSbw80_2W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw80_3(
        &mut self,
    ) -> RBandedgeZeroCfgSbw80_3W<'_, RegAxPwrTssiTable7Spec> {
        RBandedgeZeroCfgSbw80_3W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bmode_init_diff_seg0_db(
        &mut self,
    ) -> RBmodeInitDiffSeg0DbW<'_, RegAxPwrTssiTable7Spec> {
        RBmodeInitDiffSeg0DbW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bmode_init_ofst_seg0_db(
        &mut self,
    ) -> RBmodeInitOfstSeg0DbW<'_, RegAxPwrTssiTable7Spec> {
        RBmodeInitOfstSeg0DbW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable7Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table7::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table7::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE7 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable7Spec {}
