#[doc = "Register `GRP_PWR_MSK1` reader"]
pub type R = crate::R<GrpPwrMsk1Spec>;
#[doc = "Register `GRP_PWR_MSK1` writer"]
pub type W = crate::W<GrpPwrMsk1Spec>;
#[doc = "Field `grp_pmsk_wpoff_e1` reader - "]
pub type GrpPmskWpoffE1R = crate::FieldReader<u16>;
#[doc = "Field `grp_pmsk_wpoff_e1` writer - "]
pub type GrpPmskWpoffE1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `grp_pmsk_wpoff_e2` reader - "]
pub type GrpPmskWpoffE2R = crate::FieldReader;
#[doc = "Field `grp_pmsk_wpoff_e2` writer - "]
pub type GrpPmskWpoffE2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_wpoff_e3` reader - "]
pub type GrpPmskWpoffE3R = crate::BitReader;
#[doc = "Field `grp_pmsk_wpoff_e3` writer - "]
pub type GrpPmskWpoffE3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_wpoff_e4` reader - "]
pub type GrpPmskWpoffE4R = crate::BitReader;
#[doc = "Field `grp_pmsk_wpoff_e4` writer - "]
pub type GrpPmskWpoffE4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_wpoff_e5` reader - "]
pub type GrpPmskWpoffE5R = crate::FieldReader;
#[doc = "Field `grp_pmsk_wpoff_e5` writer - "]
pub type GrpPmskWpoffE5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_wpon_e0` reader - "]
pub type GrpPmskWponE0R = crate::BitReader;
#[doc = "Field `grp_pmsk_wpon_e0` writer - "]
pub type GrpPmskWponE0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_bt_e0` reader - "]
pub type GrpPmskBtE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_bt_e0` writer - "]
pub type GrpPmskBtE0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `grp_pmsk_bt_e1` reader - "]
pub type GrpPmskBtE1R = crate::BitReader;
#[doc = "Field `grp_pmsk_bt_e1` writer - "]
pub type GrpPmskBtE1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pmsk_km4tz_cache_e0` reader - "]
pub type GrpPmskKm4tzCacheE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_km4tz_cache_e0` writer - "]
pub type GrpPmskKm4tzCacheE0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `grp_pmsk_km4tz_itcm0_e0` reader - "]
pub type GrpPmskKm4tzItcm0E0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_km4tz_itcm0_e0` writer - "]
pub type GrpPmskKm4tzItcm0E0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `grp_pmsk_km4ns_cache_e0` reader - "]
pub type GrpPmskKm4nsCacheE0R = crate::FieldReader;
#[doc = "Field `grp_pmsk_km4ns_cache_e0` writer - "]
pub type GrpPmskKm4nsCacheE0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e1(&self) -> GrpPmskWpoffE1R {
        GrpPmskWpoffE1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e2(&self) -> GrpPmskWpoffE2R {
        GrpPmskWpoffE2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e3(&self) -> GrpPmskWpoffE3R {
        GrpPmskWpoffE3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e4(&self) -> GrpPmskWpoffE4R {
        GrpPmskWpoffE4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e5(&self) -> GrpPmskWpoffE5R {
        GrpPmskWpoffE5R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn grp_pmsk_wpon_e0(&self) -> GrpPmskWponE0R {
        GrpPmskWponE0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn grp_pmsk_bt_e0(&self) -> GrpPmskBtE0R {
        GrpPmskBtE0R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn grp_pmsk_bt_e1(&self) -> GrpPmskBtE1R {
        GrpPmskBtE1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn grp_pmsk_km4tz_cache_e0(&self) -> GrpPmskKm4tzCacheE0R {
        GrpPmskKm4tzCacheE0R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn grp_pmsk_km4tz_itcm0_e0(&self) -> GrpPmskKm4tzItcm0E0R {
        GrpPmskKm4tzItcm0E0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn grp_pmsk_km4ns_cache_e0(&self) -> GrpPmskKm4nsCacheE0R {
        GrpPmskKm4nsCacheE0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e1(&mut self) -> GrpPmskWpoffE1W<'_, GrpPwrMsk1Spec> {
        GrpPmskWpoffE1W::new(self, 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e2(&mut self) -> GrpPmskWpoffE2W<'_, GrpPwrMsk1Spec> {
        GrpPmskWpoffE2W::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e3(&mut self) -> GrpPmskWpoffE3W<'_, GrpPwrMsk1Spec> {
        GrpPmskWpoffE3W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e4(&mut self) -> GrpPmskWpoffE4W<'_, GrpPwrMsk1Spec> {
        GrpPmskWpoffE4W::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn grp_pmsk_wpoff_e5(&mut self) -> GrpPmskWpoffE5W<'_, GrpPwrMsk1Spec> {
        GrpPmskWpoffE5W::new(self, 14)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn grp_pmsk_wpon_e0(&mut self) -> GrpPmskWponE0W<'_, GrpPwrMsk1Spec> {
        GrpPmskWponE0W::new(self, 16)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn grp_pmsk_bt_e0(&mut self) -> GrpPmskBtE0W<'_, GrpPwrMsk1Spec> {
        GrpPmskBtE0W::new(self, 17)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn grp_pmsk_bt_e1(&mut self) -> GrpPmskBtE1W<'_, GrpPwrMsk1Spec> {
        GrpPmskBtE1W::new(self, 19)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn grp_pmsk_km4tz_cache_e0(&mut self) -> GrpPmskKm4tzCacheE0W<'_, GrpPwrMsk1Spec> {
        GrpPmskKm4tzCacheE0W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn grp_pmsk_km4tz_itcm0_e0(&mut self) -> GrpPmskKm4tzItcm0E0W<'_, GrpPwrMsk1Spec> {
        GrpPmskKm4tzItcm0E0W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn grp_pmsk_km4ns_cache_e0(&mut self) -> GrpPmskKm4nsCacheE0W<'_, GrpPwrMsk1Spec> {
        GrpPmskKm4nsCacheE0W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_pwr_msk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_pwr_msk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpPwrMsk1Spec;
impl crate::RegisterSpec for GrpPwrMsk1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_pwr_msk1::R`](R) reader structure"]
impl crate::Readable for GrpPwrMsk1Spec {}
#[doc = "`write(|w| ..)` method takes [`grp_pwr_msk1::W`](W) writer structure"]
impl crate::Writable for GrpPwrMsk1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_PWR_MSK1 to value 0xffff_ffff"]
impl crate::Resettable for GrpPwrMsk1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
