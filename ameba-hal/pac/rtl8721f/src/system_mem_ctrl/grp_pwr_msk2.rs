#[doc = "Register `GRP_PWR_MSK2` reader"]
pub type R = crate::R<GrpPwrMsk2Spec>;
#[doc = "Register `GRP_PWR_MSK2` writer"]
pub type W = crate::W<GrpPwrMsk2Spec>;
#[doc = "Field `grp_pmsk_lcdc_e0` reader - "]
pub type GrpPmskLcdcE0R = crate::BitReader;
#[doc = "Field `grp_pmsk_lcdc_e0` writer - "]
pub type GrpPmskLcdcE0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_mjpeg_e0` reader - "]
pub type GrpPmskMjpegE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_mjpeg_e0` writer - "]
pub type GrpPmskMjpegE0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `grp_pmsk_mjpeg_e1` reader - "]
pub type GrpPmskMjpegE1R = crate::BitReader;
#[doc = "Field `grp_pmsk_mjpeg_e1` writer - "]
pub type GrpPmskMjpegE1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_mjpeg_e2` reader - "]
pub type GrpPmskMjpegE2R = crate::FieldReader;
#[doc = "Field `grp_pmsk_mjpeg_e2` writer - "]
pub type GrpPmskMjpegE2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `grp_pmsk_gmac_e0` reader - "]
pub type GrpPmskGmacE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_gmac_e0` writer - "]
pub type GrpPmskGmacE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_can_e0` reader - "]
pub type GrpPmskCanE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_can_e0` writer - "]
pub type GrpPmskCanE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_pke_e0` reader - "]
pub type GrpPmskPkeE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_pke_e0` writer - "]
pub type GrpPmskPkeE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_pke_e1` reader - "]
pub type GrpPmskPkeE1R = crate::BitReader;
#[doc = "Field `grp_pmsk_pke_e1` writer - "]
pub type GrpPmskPkeE1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_ppe_e0` reader - "]
pub type GrpPmskPpeE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_ppe_e0` writer - "]
pub type GrpPmskPpeE0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn grp_pmsk_lcdc_e0(&self) -> GrpPmskLcdcE0R {
        GrpPmskLcdcE0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn grp_pmsk_mjpeg_e0(&self) -> GrpPmskMjpegE0R {
        GrpPmskMjpegE0R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn grp_pmsk_mjpeg_e1(&self) -> GrpPmskMjpegE1R {
        GrpPmskMjpegE1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn grp_pmsk_mjpeg_e2(&self) -> GrpPmskMjpegE2R {
        GrpPmskMjpegE2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn grp_pmsk_gmac_e0(&self) -> GrpPmskGmacE0R {
        GrpPmskGmacE0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn grp_pmsk_can_e0(&self) -> GrpPmskCanE0R {
        GrpPmskCanE0R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn grp_pmsk_pke_e0(&self) -> GrpPmskPkeE0R {
        GrpPmskPkeE0R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn grp_pmsk_pke_e1(&self) -> GrpPmskPkeE1R {
        GrpPmskPkeE1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:30"]
    #[inline(always)]
    pub fn grp_pmsk_ppe_e0(&self) -> GrpPmskPpeE0R {
        GrpPmskPpeE0R::new(((self.bits >> 23) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn grp_pmsk_lcdc_e0(&mut self) -> GrpPmskLcdcE0W<'_, GrpPwrMsk2Spec> {
        GrpPmskLcdcE0W::new(self, 0)
    }
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn grp_pmsk_mjpeg_e0(&mut self) -> GrpPmskMjpegE0W<'_, GrpPwrMsk2Spec> {
        GrpPmskMjpegE0W::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn grp_pmsk_mjpeg_e1(&mut self) -> GrpPmskMjpegE1W<'_, GrpPwrMsk2Spec> {
        GrpPmskMjpegE1W::new(self, 8)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn grp_pmsk_mjpeg_e2(&mut self) -> GrpPmskMjpegE2W<'_, GrpPwrMsk2Spec> {
        GrpPmskMjpegE2W::new(self, 9)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn grp_pmsk_gmac_e0(&mut self) -> GrpPmskGmacE0W<'_, GrpPwrMsk2Spec> {
        GrpPmskGmacE0W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn grp_pmsk_can_e0(&mut self) -> GrpPmskCanE0W<'_, GrpPwrMsk2Spec> {
        GrpPmskCanE0W::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn grp_pmsk_pke_e0(&mut self) -> GrpPmskPkeE0W<'_, GrpPwrMsk2Spec> {
        GrpPmskPkeE0W::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn grp_pmsk_pke_e1(&mut self) -> GrpPmskPkeE1W<'_, GrpPwrMsk2Spec> {
        GrpPmskPkeE1W::new(self, 22)
    }
    #[doc = "Bits 23:30"]
    #[inline(always)]
    pub fn grp_pmsk_ppe_e0(&mut self) -> GrpPmskPpeE0W<'_, GrpPwrMsk2Spec> {
        GrpPmskPpeE0W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_pwr_msk2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_pwr_msk2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpPwrMsk2Spec;
impl crate::RegisterSpec for GrpPwrMsk2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_pwr_msk2::R`](R) reader structure"]
impl crate::Readable for GrpPwrMsk2Spec {}
#[doc = "`write(|w| ..)` method takes [`grp_pwr_msk2::W`](W) writer structure"]
impl crate::Writable for GrpPwrMsk2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_PWR_MSK2 to value 0x7fff_ffff"]
impl crate::Resettable for GrpPwrMsk2Spec {
    const RESET_VALUE: u32 = 0x7fff_ffff;
}
