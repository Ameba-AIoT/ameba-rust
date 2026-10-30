#[doc = "Register `REG_AX_PWR_UL_CTRL2` reader"]
pub type R = crate::R<RegAxPwrUlCtrl2Spec>;
#[doc = "Register `REG_AX_PWR_UL_CTRL2` writer"]
pub type W = crate::W<RegAxPwrUlCtrl2Spec>;
#[doc = "Field `r_cfo_comp0` reader - STA_UL"]
pub type RCfoComp0R = crate::FieldReader;
#[doc = "Field `r_cfo_comp0` writer - STA_UL"]
pub type RCfoComp0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_cfo_comp1` reader - STA_UL"]
pub type RCfoComp1R = crate::FieldReader;
#[doc = "Field `r_cfo_comp1` writer - STA_UL"]
pub type RCfoComp1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_cfo_comp2` reader - STA_UL"]
pub type RCfoComp2R = crate::FieldReader;
#[doc = "Field `r_cfo_comp2` writer - STA_UL"]
pub type RCfoComp2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_cfo_comp3` reader - STA_UL"]
pub type RCfoComp3R = crate::FieldReader;
#[doc = "Field `r_cfo_comp3` writer - STA_UL"]
pub type RCfoComp3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_cfo_comp4` reader - (RTL_only) STA_UL Default 6 no need to set"]
pub type RCfoComp4R = crate::FieldReader;
#[doc = "Field `r_cfo_comp4` writer - (RTL_only) STA_UL Default 6 no need to set"]
pub type RCfoComp4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_cfo_comp_norm_resp` reader - (RTL_only) STA_UL no need to set"]
pub type RCfoCompNormRespR = crate::FieldReader;
#[doc = "Field `r_cfo_comp_norm_resp` writer - (RTL_only) STA_UL no need to set"]
pub type RCfoCompNormRespW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_cfo_comp_sr` reader - (RTL_only) STA_UL no need to set"]
pub type RCfoCompSrR = crate::FieldReader;
#[doc = "Field `r_cfo_comp_sr` writer - (RTL_only) STA_UL no need to set"]
pub type RCfoCompSrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - STA_UL"]
    #[inline(always)]
    pub fn r_cfo_comp0(&self) -> RCfoComp0R {
        RCfoComp0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - STA_UL"]
    #[inline(always)]
    pub fn r_cfo_comp1(&self) -> RCfoComp1R {
        RCfoComp1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - STA_UL"]
    #[inline(always)]
    pub fn r_cfo_comp2(&self) -> RCfoComp2R {
        RCfoComp2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - STA_UL"]
    #[inline(always)]
    pub fn r_cfo_comp3(&self) -> RCfoComp3R {
        RCfoComp3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - (RTL_only) STA_UL Default 6 no need to set"]
    #[inline(always)]
    pub fn r_cfo_comp4(&self) -> RCfoComp4R {
        RCfoComp4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - (RTL_only) STA_UL no need to set"]
    #[inline(always)]
    pub fn r_cfo_comp_norm_resp(&self) -> RCfoCompNormRespR {
        RCfoCompNormRespR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - (RTL_only) STA_UL no need to set"]
    #[inline(always)]
    pub fn r_cfo_comp_sr(&self) -> RCfoCompSrR {
        RCfoCompSrR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - STA_UL"]
    #[inline(always)]
    pub fn r_cfo_comp0(&mut self) -> RCfoComp0W<'_, RegAxPwrUlCtrl2Spec> {
        RCfoComp0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - STA_UL"]
    #[inline(always)]
    pub fn r_cfo_comp1(&mut self) -> RCfoComp1W<'_, RegAxPwrUlCtrl2Spec> {
        RCfoComp1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - STA_UL"]
    #[inline(always)]
    pub fn r_cfo_comp2(&mut self) -> RCfoComp2W<'_, RegAxPwrUlCtrl2Spec> {
        RCfoComp2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - STA_UL"]
    #[inline(always)]
    pub fn r_cfo_comp3(&mut self) -> RCfoComp3W<'_, RegAxPwrUlCtrl2Spec> {
        RCfoComp3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - (RTL_only) STA_UL Default 6 no need to set"]
    #[inline(always)]
    pub fn r_cfo_comp4(&mut self) -> RCfoComp4W<'_, RegAxPwrUlCtrl2Spec> {
        RCfoComp4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - (RTL_only) STA_UL no need to set"]
    #[inline(always)]
    pub fn r_cfo_comp_norm_resp(&mut self) -> RCfoCompNormRespW<'_, RegAxPwrUlCtrl2Spec> {
        RCfoCompNormRespW::new(self, 20)
    }
    #[doc = "Bits 24:26 - (RTL_only) STA_UL no need to set"]
    #[inline(always)]
    pub fn r_cfo_comp_sr(&mut self) -> RCfoCompSrW<'_, RegAxPwrUlCtrl2Spec> {
        RCfoCompSrW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlCtrl2Spec;
impl crate::RegisterSpec for RegAxPwrUlCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_ctrl2::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_ctrl2::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_CTRL2 to value 0x0776_7777"]
impl crate::Resettable for RegAxPwrUlCtrl2Spec {
    const RESET_VALUE: u32 = 0x0776_7777;
}
