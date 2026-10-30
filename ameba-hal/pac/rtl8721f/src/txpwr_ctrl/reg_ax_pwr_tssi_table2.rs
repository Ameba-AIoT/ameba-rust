#[doc = "Register `REG_AX_PWR_TSSI_TABLE2` reader"]
pub type R = crate::R<RegAxPwrTssiTable2Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE2` writer"]
pub type W = crate::W<RegAxPwrTssiTable2Spec>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw20` reader - "]
pub type RBandedgeNonzeroCfgSbw20R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw20` writer - "]
pub type RBandedgeNonzeroCfgSbw20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw40_0` reader - "]
pub type RBandedgeNonzeroCfgSbw40_0R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw40_0` writer - "]
pub type RBandedgeNonzeroCfgSbw40_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw40_1` reader - "]
pub type RBandedgeNonzeroCfgSbw40_1R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw40_1` writer - "]
pub type RBandedgeNonzeroCfgSbw40_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw80_0` reader - "]
pub type RBandedgeNonzeroCfgSbw80_0R = crate::FieldReader;
#[doc = "Field `r_bandedge_nonzero_cfg_sbw80_0` writer - "]
pub type RBandedgeNonzeroCfgSbw80_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw20(&self) -> RBandedgeNonzeroCfgSbw20R {
        RBandedgeNonzeroCfgSbw20R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw40_0(&self) -> RBandedgeNonzeroCfgSbw40_0R {
        RBandedgeNonzeroCfgSbw40_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw40_1(&self) -> RBandedgeNonzeroCfgSbw40_1R {
        RBandedgeNonzeroCfgSbw40_1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw80_0(&self) -> RBandedgeNonzeroCfgSbw80_0R {
        RBandedgeNonzeroCfgSbw80_0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw20(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw20W<'_, RegAxPwrTssiTable2Spec> {
        RBandedgeNonzeroCfgSbw20W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw40_0(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw40_0W<'_, RegAxPwrTssiTable2Spec> {
        RBandedgeNonzeroCfgSbw40_0W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw40_1(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw40_1W<'_, RegAxPwrTssiTable2Spec> {
        RBandedgeNonzeroCfgSbw40_1W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_nonzero_cfg_sbw80_0(
        &mut self,
    ) -> RBandedgeNonzeroCfgSbw80_0W<'_, RegAxPwrTssiTable2Spec> {
        RBandedgeNonzeroCfgSbw80_0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable2Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE2 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable2Spec {}
