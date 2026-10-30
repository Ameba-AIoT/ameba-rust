#[doc = "Register `CAN_FD_SSP_CAL` reader"]
pub type R = crate::R<CanFdSspCalSpec>;
#[doc = "Register `CAN_FD_SSP_CAL` writer"]
pub type W = crate::W<CanFdSspCalSpec>;
#[doc = "Field `can_fd_ssp` reader - ssp position, delay from start of one bit, if automatic calculation is enabled, the field will update after calculation"]
pub type CanFdSspR = crate::FieldReader;
#[doc = "Field `can_fd_ssp` writer - ssp position, delay from start of one bit, if automatic calculation is enabled, the field will update after calculation"]
pub type CanFdSspW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_fd_ssp_min` reader - minimum of ssp from start of one bit, only functional when can_fd_ssp_auto set"]
pub type CanFdSspMinR = crate::FieldReader;
#[doc = "Field `can_fd_ssp_min` writer - minimum of ssp from start of one bit, only functional when can_fd_ssp_auto set"]
pub type CanFdSspMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_fd_ssp_dco` reader - delay compensation offset"]
pub type CanFdSspDcoR = crate::FieldReader;
#[doc = "Field `can_fd_ssp_dco` writer - delay compensation offset"]
pub type CanFdSspDcoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_fd_ssp_dco2` reader - delay compensation offset2"]
pub type CanFdSspDco2R = crate::FieldReader;
#[doc = "Field `can_fd_ssp_dco2` writer - delay compensation offset2"]
pub type CanFdSspDco2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ssp position, delay from start of one bit, if automatic calculation is enabled, the field will update after calculation"]
    #[inline(always)]
    pub fn can_fd_ssp(&self) -> CanFdSspR {
        CanFdSspR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - minimum of ssp from start of one bit, only functional when can_fd_ssp_auto set"]
    #[inline(always)]
    pub fn can_fd_ssp_min(&self) -> CanFdSspMinR {
        CanFdSspMinR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - delay compensation offset"]
    #[inline(always)]
    pub fn can_fd_ssp_dco(&self) -> CanFdSspDcoR {
        CanFdSspDcoR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - delay compensation offset2"]
    #[inline(always)]
    pub fn can_fd_ssp_dco2(&self) -> CanFdSspDco2R {
        CanFdSspDco2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ssp position, delay from start of one bit, if automatic calculation is enabled, the field will update after calculation"]
    #[inline(always)]
    pub fn can_fd_ssp(&mut self) -> CanFdSspW<'_, CanFdSspCalSpec> {
        CanFdSspW::new(self, 0)
    }
    #[doc = "Bits 8:15 - minimum of ssp from start of one bit, only functional when can_fd_ssp_auto set"]
    #[inline(always)]
    pub fn can_fd_ssp_min(&mut self) -> CanFdSspMinW<'_, CanFdSspCalSpec> {
        CanFdSspMinW::new(self, 8)
    }
    #[doc = "Bits 16:23 - delay compensation offset"]
    #[inline(always)]
    pub fn can_fd_ssp_dco(&mut self) -> CanFdSspDcoW<'_, CanFdSspCalSpec> {
        CanFdSspDcoW::new(self, 16)
    }
    #[doc = "Bits 24:31 - delay compensation offset2"]
    #[inline(always)]
    pub fn can_fd_ssp_dco2(&mut self) -> CanFdSspDco2W<'_, CanFdSspCalSpec> {
        CanFdSspDco2W::new(self, 24)
    }
}
#[doc = "CAN FD SSP configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fd_ssp_cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_fd_ssp_cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanFdSspCalSpec;
impl crate::RegisterSpec for CanFdSspCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fd_ssp_cal::R`](R) reader structure"]
impl crate::Readable for CanFdSspCalSpec {}
#[doc = "`write(|w| ..)` method takes [`can_fd_ssp_cal::W`](W) writer structure"]
impl crate::Writable for CanFdSspCalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_FD_SSP_CAL to value 0"]
impl crate::Resettable for CanFdSspCalSpec {}
