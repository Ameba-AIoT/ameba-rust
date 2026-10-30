#[doc = "Register `REG_AX_PWR_TSSI_TABLE0` reader"]
pub type R = crate::R<RegAxPwrTssiTable0Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE0` writer"]
pub type W = crate::W<RegAxPwrTssiTable0Spec>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw160_0` reader - "]
pub type RBandedgeNonzeroCfgSbw160_0R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw160_0` writer - "]
pub type RBandedgeNonzeroCfgSbw160_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw160_1` reader - "]
pub type RBandedgeNonzeroCfgSbw160_1R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw160_1` writer - "]
pub type RBandedgeNonzeroCfgSbw160_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw160_2` reader - "]
pub type RBandedgeNonzeroCfgSbw160_2R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw160_2` writer - "]
pub type RBandedgeNonzeroCfgSbw160_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw160_3` reader - "]
pub type RBandedgeNonzeroCfgSbw160_3R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw160_3` writer - "]
pub type RBandedgeNonzeroCfgSbw160_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw160_0(&self) -> RBandedgeNonzeroCfgSbw160_0R {
        RBandedgeNonzeroCfgSbw160_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw160_1(&self) -> RBandedgeNonzeroCfgSbw160_1R {
        RBandedgeNonzeroCfgSbw160_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw160_2(&self) -> RBandedgeNonzeroCfgSbw160_2R {
        RBandedgeNonzeroCfgSbw160_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw160_3(&self) -> RBandedgeNonzeroCfgSbw160_3R {
        RBandedgeNonzeroCfgSbw160_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw160_0(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw160_0W<'_, RegAxPwrTssiTable0Spec> {
        RBandedgeNonzeroCfgSbw160_0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw160_1(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw160_1W<'_, RegAxPwrTssiTable0Spec> {
        RBandedgeNonzeroCfgSbw160_1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw160_2(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw160_2W<'_, RegAxPwrTssiTable0Spec> {
        RBandedgeNonzeroCfgSbw160_2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw160_3(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw160_3W<'_, RegAxPwrTssiTable0Spec> {
        RBandedgeNonzeroCfgSbw160_3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable0Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE0 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable0Spec {}
