#[doc = "Register `MBIST_GRP_EN_CTRL2` reader"]
pub type R = crate::R<MbistGrpEnCtrl2Spec>;
#[doc = "Register `MBIST_GRP_EN_CTRL2` writer"]
pub type W = crate::W<MbistGrpEnCtrl2Spec>;
#[doc = "Field `grp_en_mjpeg_e2` reader - "]
pub type GrpEnMjpegE2R = crate::FieldReader;
#[doc = "Field `grp_en_mjpeg_e2` writer - "]
pub type GrpEnMjpegE2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `grp_en_can_e0` reader - "]
pub type GrpEnCanE0R = crate::FieldReader;
#[doc = "Field `grp_en_can_e0` writer - "]
pub type GrpEnCanE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_en_ppe_e0` reader - "]
pub type GrpEnPpeE0R = crate::FieldReader;
#[doc = "Field `grp_en_ppe_e0` writer - "]
pub type GrpEnPpeE0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `grp_en_ppe_e1` reader - "]
pub type GrpEnPpeE1R = crate::FieldReader;
#[doc = "Field `grp_en_ppe_e1` writer - "]
pub type GrpEnPpeE1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn grp_en_mjpeg_e2(&self) -> GrpEnMjpegE2R {
        GrpEnMjpegE2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn grp_en_can_e0(&self) -> GrpEnCanE0R {
        GrpEnCanE0R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn grp_en_ppe_e0(&self) -> GrpEnPpeE0R {
        GrpEnPpeE0R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:24"]
    #[inline(always)]
    pub fn grp_en_ppe_e1(&self) -> GrpEnPpeE1R {
        GrpEnPpeE1R::new(((self.bits >> 17) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn grp_en_mjpeg_e2(&mut self) -> GrpEnMjpegE2W<'_, MbistGrpEnCtrl2Spec> {
        GrpEnMjpegE2W::new(self, 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn grp_en_can_e0(&mut self) -> GrpEnCanE0W<'_, MbistGrpEnCtrl2Spec> {
        GrpEnCanE0W::new(self, 7)
    }
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn grp_en_ppe_e0(&mut self) -> GrpEnPpeE0W<'_, MbistGrpEnCtrl2Spec> {
        GrpEnPpeE0W::new(self, 9)
    }
    #[doc = "Bits 17:24"]
    #[inline(always)]
    pub fn grp_en_ppe_e1(&mut self) -> GrpEnPpeE1W<'_, MbistGrpEnCtrl2Spec> {
        GrpEnPpeE1W::new(self, 17)
    }
}
#[doc = "Used to on-off mbist of memory block in a group.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_grp_en_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_grp_en_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistGrpEnCtrl2Spec;
impl crate::RegisterSpec for MbistGrpEnCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_grp_en_ctrl2::R`](R) reader structure"]
impl crate::Readable for MbistGrpEnCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mbist_grp_en_ctrl2::W`](W) writer structure"]
impl crate::Writable for MbistGrpEnCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBIST_GRP_EN_CTRL2 to value 0x01ff_ffff"]
impl crate::Resettable for MbistGrpEnCtrl2Spec {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
