#[doc = "Register `REG_AX_PWR_TSSI_TABLE6` reader"]
pub type R = crate::R<RegAxPwrTssiTable6Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE6` writer"]
pub type W = crate::W<RegAxPwrTssiTable6Spec>;
#[doc = "Field `r_bandedge_zero_cfg_sbw40_0` reader - "]
pub type RBandedgeZeroCfgSbw40_0R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw40_0` writer - "]
pub type RBandedgeZeroCfgSbw40_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_zero_cfg_sbw40_1` reader - "]
pub type RBandedgeZeroCfgSbw40_1R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw40_1` writer - "]
pub type RBandedgeZeroCfgSbw40_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_zero_cfg_sbw80_0` reader - "]
pub type RBandedgeZeroCfgSbw80_0R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw80_0` writer - "]
pub type RBandedgeZeroCfgSbw80_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_zero_cfg_sbw80_1` reader - "]
pub type RBandedgeZeroCfgSbw80_1R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw80_1` writer - "]
pub type RBandedgeZeroCfgSbw80_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw40_0(&self) -> RBandedgeZeroCfgSbw40_0R {
        RBandedgeZeroCfgSbw40_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw40_1(&self) -> RBandedgeZeroCfgSbw40_1R {
        RBandedgeZeroCfgSbw40_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw80_0(&self) -> RBandedgeZeroCfgSbw80_0R {
        RBandedgeZeroCfgSbw80_0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw80_1(&self) -> RBandedgeZeroCfgSbw80_1R {
        RBandedgeZeroCfgSbw80_1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw40_0(
        &mut self,
    ) -> RBandedgeZeroCfgSbw40_0W<'_, RegAxPwrTssiTable6Spec> {
        RBandedgeZeroCfgSbw40_0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw40_1(
        &mut self,
    ) -> RBandedgeZeroCfgSbw40_1W<'_, RegAxPwrTssiTable6Spec> {
        RBandedgeZeroCfgSbw40_1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw80_0(
        &mut self,
    ) -> RBandedgeZeroCfgSbw80_0W<'_, RegAxPwrTssiTable6Spec> {
        RBandedgeZeroCfgSbw80_0W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw80_1(
        &mut self,
    ) -> RBandedgeZeroCfgSbw80_1W<'_, RegAxPwrTssiTable6Spec> {
        RBandedgeZeroCfgSbw80_1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable6Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table6::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table6::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE6 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable6Spec {}
