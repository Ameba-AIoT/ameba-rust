#[doc = "Register `REG_AX_PWR_TSSI_TABLE3` reader"]
pub type R = crate::R<RegAxPwrTssiTable3Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE3` writer"]
pub type W = crate::W<RegAxPwrTssiTable3Spec>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw80_1` reader - "]
pub type RBandedgeNonzeroCfgSbw80_1R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw80_1` writer - "]
pub type RBandedgeNonzeroCfgSbw80_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw80_2` reader - "]
pub type RBandedgeNonzeroCfgSbw80_2R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw80_2` writer - "]
pub type RBandedgeNonzeroCfgSbw80_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw80_3` reader - "]
pub type RBandedgeNonzeroCfgSbw80_3R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw80_3` writer - "]
pub type RBandedgeNonzeroCfgSbw80_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_zero_cfg_sbw160_0` reader - "]
pub type RBandedgeZeroCfgSbw160_0R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw160_0` writer - "]
pub type RBandedgeZeroCfgSbw160_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw80_1(&self) -> RBandedgeNonzeroCfgSbw80_1R {
        RBandedgeNonzeroCfgSbw80_1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw80_2(&self) -> RBandedgeNonzeroCfgSbw80_2R {
        RBandedgeNonzeroCfgSbw80_2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw80_3(&self) -> RBandedgeNonzeroCfgSbw80_3R {
        RBandedgeNonzeroCfgSbw80_3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw160_0(&self) -> RBandedgeZeroCfgSbw160_0R {
        RBandedgeZeroCfgSbw160_0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw80_1(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw80_1W<'_, RegAxPwrTssiTable3Spec> {
        RBandedgeNonzeroCfgSbw80_1W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw80_2(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw80_2W<'_, RegAxPwrTssiTable3Spec> {
        RBandedgeNonzeroCfgSbw80_2W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw80_3(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw80_3W<'_, RegAxPwrTssiTable3Spec> {
        RBandedgeNonzeroCfgSbw80_3W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw160_0(
        &mut self,
    ) -> RBandedgeZeroCfgSbw160_0W<'_, RegAxPwrTssiTable3Spec> {
        RBandedgeZeroCfgSbw160_0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable3Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table3::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table3::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE3 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable3Spec {}
