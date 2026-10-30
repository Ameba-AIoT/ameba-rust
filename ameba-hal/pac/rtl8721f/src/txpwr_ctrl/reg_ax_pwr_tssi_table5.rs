#[doc = "Register `REG_AX_PWR_TSSI_TABLE5` reader"]
pub type R = crate::R<RegAxPwrTssiTable5Spec>;
#[doc = "Register `REG_AX_PWR_TSSI_TABLE5` writer"]
pub type W = crate::W<RegAxPwrTssiTable5Spec>;
#[doc = "Field `r_bandedge_zero_cfg_sbw160_5` reader - "]
pub type RBandedgeZeroCfgSbw160_5R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw160_5` writer - "]
pub type RBandedgeZeroCfgSbw160_5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_zero_cfg_sbw160_6` reader - "]
pub type RBandedgeZeroCfgSbw160_6R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw160_6` writer - "]
pub type RBandedgeZeroCfgSbw160_6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_zero_cfg_sbw160_7` reader - "]
pub type RBandedgeZeroCfgSbw160_7R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw160_7` writer - "]
pub type RBandedgeZeroCfgSbw160_7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_bandedge_zero_cfg_sbw20` reader - "]
pub type RBandedgeZeroCfgSbw20R = crate::FieldReader;
#[doc = "Field `r_bandedge_zero_cfg_sbw20` writer - "]
pub type RBandedgeZeroCfgSbw20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw160_5(&self) -> RBandedgeZeroCfgSbw160_5R {
        RBandedgeZeroCfgSbw160_5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw160_6(&self) -> RBandedgeZeroCfgSbw160_6R {
        RBandedgeZeroCfgSbw160_6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw160_7(&self) -> RBandedgeZeroCfgSbw160_7R {
        RBandedgeZeroCfgSbw160_7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw20(&self) -> RBandedgeZeroCfgSbw20R {
        RBandedgeZeroCfgSbw20R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw160_5(
        &mut self,
    ) -> RBandedgeZeroCfgSbw160_5W<'_, RegAxPwrTssiTable5Spec> {
        RBandedgeZeroCfgSbw160_5W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw160_6(
        &mut self,
    ) -> RBandedgeZeroCfgSbw160_6W<'_, RegAxPwrTssiTable5Spec> {
        RBandedgeZeroCfgSbw160_6W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw160_7(
        &mut self,
    ) -> RBandedgeZeroCfgSbw160_7W<'_, RegAxPwrTssiTable5Spec> {
        RBandedgeZeroCfgSbw160_7W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn r_bandedge_zero_cfg_sbw20(
        &mut self,
    ) -> RBandedgeZeroCfgSbw20W<'_, RegAxPwrTssiTable5Spec> {
        RBandedgeZeroCfgSbw20W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_tssi_table5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_tssi_table5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrTssiTable5Spec;
impl crate::RegisterSpec for RegAxPwrTssiTable5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_tssi_table5::R`](R) reader structure"]
impl crate::Readable for RegAxPwrTssiTable5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_tssi_table5::W`](W) writer structure"]
impl crate::Writable for RegAxPwrTssiTable5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_TSSI_TABLE5 to value 0"]
impl crate::Resettable for RegAxPwrTssiTable5Spec {}
