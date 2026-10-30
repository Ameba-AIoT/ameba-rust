#[doc = "Register `MBIST_GRP_EN_CTRL1` reader"]
pub type R = crate::R<MbistGrpEnCtrl1Spec>;
#[doc = "Register `MBIST_GRP_EN_CTRL1` writer"]
pub type W = crate::W<MbistGrpEnCtrl1Spec>;
#[doc = "Field `grp_en_wpoff_e1` reader - "]
pub type GrpEnWpoffE1R = crate::FieldReader<u16>;
#[doc = "Field `grp_en_wpoff_e1` writer - "]
pub type GrpEnWpoffE1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `grp_en_bt_e0` reader - "]
pub type GrpEnBtE0R = crate::FieldReader;
#[doc = "Field `grp_en_bt_e0` writer - "]
pub type GrpEnBtE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_en_km4tz_cache_e0` reader - "]
pub type GrpEnKm4tzCacheE0R = crate::FieldReader;
#[doc = "Field `grp_en_km4tz_cache_e0` writer - "]
pub type GrpEnKm4tzCacheE0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `grp_en_km4tz_itcm0_e0` reader - "]
pub type GrpEnKm4tzItcm0E0R = crate::FieldReader;
#[doc = "Field `grp_en_km4tz_itcm0_e0` writer - "]
pub type GrpEnKm4tzItcm0E0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `grp_en_km4ns_cache_e0` reader - "]
pub type GrpEnKm4nsCacheE0R = crate::FieldReader;
#[doc = "Field `grp_en_km4ns_cache_e0` writer - "]
pub type GrpEnKm4nsCacheE0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `grp_en_mjpeg_e0` reader - "]
pub type GrpEnMjpegE0R = crate::FieldReader;
#[doc = "Field `grp_en_mjpeg_e0` writer - "]
pub type GrpEnMjpegE0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn grp_en_wpoff_e1(&self) -> GrpEnWpoffE1R {
        GrpEnWpoffE1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn grp_en_bt_e0(&self) -> GrpEnBtE0R {
        GrpEnBtE0R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn grp_en_km4tz_cache_e0(&self) -> GrpEnKm4tzCacheE0R {
        GrpEnKm4tzCacheE0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn grp_en_km4tz_itcm0_e0(&self) -> GrpEnKm4tzItcm0E0R {
        GrpEnKm4tzItcm0E0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn grp_en_km4ns_cache_e0(&self) -> GrpEnKm4nsCacheE0R {
        GrpEnKm4nsCacheE0R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn grp_en_mjpeg_e0(&self) -> GrpEnMjpegE0R {
        GrpEnMjpegE0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn grp_en_wpoff_e1(&mut self) -> GrpEnWpoffE1W<'_, MbistGrpEnCtrl1Spec> {
        GrpEnWpoffE1W::new(self, 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn grp_en_bt_e0(&mut self) -> GrpEnBtE0W<'_, MbistGrpEnCtrl1Spec> {
        GrpEnBtE0W::new(self, 10)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn grp_en_km4tz_cache_e0(&mut self) -> GrpEnKm4tzCacheE0W<'_, MbistGrpEnCtrl1Spec> {
        GrpEnKm4tzCacheE0W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn grp_en_km4tz_itcm0_e0(&mut self) -> GrpEnKm4tzItcm0E0W<'_, MbistGrpEnCtrl1Spec> {
        GrpEnKm4tzItcm0E0W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn grp_en_km4ns_cache_e0(&mut self) -> GrpEnKm4nsCacheE0W<'_, MbistGrpEnCtrl1Spec> {
        GrpEnKm4nsCacheE0W::new(self, 20)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn grp_en_mjpeg_e0(&mut self) -> GrpEnMjpegE0W<'_, MbistGrpEnCtrl1Spec> {
        GrpEnMjpegE0W::new(self, 24)
    }
}
#[doc = "Used to on-off mbist of memory block in a group.\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_grp_en_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_grp_en_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistGrpEnCtrl1Spec;
impl crate::RegisterSpec for MbistGrpEnCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_grp_en_ctrl1::R`](R) reader structure"]
impl crate::Readable for MbistGrpEnCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mbist_grp_en_ctrl1::W`](W) writer structure"]
impl crate::Writable for MbistGrpEnCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBIST_GRP_EN_CTRL1 to value 0x7fff_ffff"]
impl crate::Resettable for MbistGrpEnCtrl1Spec {
    const RESET_VALUE: u32 = 0x7fff_ffff;
}
