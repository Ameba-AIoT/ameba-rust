#[doc = "Register `MBIST_GRP_EN_CTRL0` reader"]
pub type R = crate::R<MbistGrpEnCtrl0Spec>;
#[doc = "Register `MBIST_GRP_EN_CTRL0` writer"]
pub type W = crate::W<MbistGrpEnCtrl0Spec>;
#[doc = "Field `grp_en_hsys_e0` reader - "]
pub type GrpEnHsysE0R = crate::FieldReader;
#[doc = "Field `grp_en_hsys_e0` writer - "]
pub type GrpEnHsysE0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `grp_en_hsys_e1` reader - "]
pub type GrpEnHsysE1R = crate::FieldReader<u16>;
#[doc = "Field `grp_en_hsys_e1` writer - "]
pub type GrpEnHsysE1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `grp_en_aonpmc_e0` reader - "]
pub type GrpEnAonpmcE0R = crate::FieldReader;
#[doc = "Field `grp_en_aonpmc_e0` writer - "]
pub type GrpEnAonpmcE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_en_sdio_e0` reader - "]
pub type GrpEnSdioE0R = crate::FieldReader;
#[doc = "Field `grp_en_sdio_e0` writer - "]
pub type GrpEnSdioE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_en_wlk4_e0` reader - "]
pub type GrpEnWlk4E0R = crate::FieldReader;
#[doc = "Field `grp_en_wlk4_e0` writer - "]
pub type GrpEnWlk4E0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn grp_en_hsys_e0(&self) -> GrpEnHsysE0R {
        GrpEnHsysE0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16"]
    #[inline(always)]
    pub fn grp_en_hsys_e1(&self) -> GrpEnHsysE1R {
        GrpEnHsysE1R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn grp_en_aonpmc_e0(&self) -> GrpEnAonpmcE0R {
        GrpEnAonpmcE0R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn grp_en_sdio_e0(&self) -> GrpEnSdioE0R {
        GrpEnSdioE0R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn grp_en_wlk4_e0(&self) -> GrpEnWlk4E0R {
        GrpEnWlk4E0R::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn grp_en_hsys_e0(&mut self) -> GrpEnHsysE0W<'_, MbistGrpEnCtrl0Spec> {
        GrpEnHsysE0W::new(self, 0)
    }
    #[doc = "Bits 8:16"]
    #[inline(always)]
    pub fn grp_en_hsys_e1(&mut self) -> GrpEnHsysE1W<'_, MbistGrpEnCtrl0Spec> {
        GrpEnHsysE1W::new(self, 8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn grp_en_aonpmc_e0(&mut self) -> GrpEnAonpmcE0W<'_, MbistGrpEnCtrl0Spec> {
        GrpEnAonpmcE0W::new(self, 17)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn grp_en_sdio_e0(&mut self) -> GrpEnSdioE0W<'_, MbistGrpEnCtrl0Spec> {
        GrpEnSdioE0W::new(self, 19)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn grp_en_wlk4_e0(&mut self) -> GrpEnWlk4E0W<'_, MbistGrpEnCtrl0Spec> {
        GrpEnWlk4E0W::new(self, 21)
    }
}
#[doc = "Used to on-off mbist of memory block in a group.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_grp_en_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_grp_en_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistGrpEnCtrl0Spec;
impl crate::RegisterSpec for MbistGrpEnCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_grp_en_ctrl0::R`](R) reader structure"]
impl crate::Readable for MbistGrpEnCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mbist_grp_en_ctrl0::W`](W) writer structure"]
impl crate::Writable for MbistGrpEnCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBIST_GRP_EN_CTRL0 to value 0x007f_ffff"]
impl crate::Resettable for MbistGrpEnCtrl0Spec {
    const RESET_VALUE: u32 = 0x007f_ffff;
}
