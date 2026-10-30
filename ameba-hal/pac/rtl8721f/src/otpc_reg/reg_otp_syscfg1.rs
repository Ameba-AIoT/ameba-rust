#[doc = "Register `REG_OTP_SYSCFG1` reader"]
pub type R = crate::R<RegOtpSyscfg1Spec>;
#[doc = "Register `REG_OTP_SYSCFG1` writer"]
pub type W = crate::W<RegOtpSyscfg1Spec>;
#[doc = "Field `SW_RSVD0` reader - RSVD for SW usage"]
pub type SwRsvd0R = crate::FieldReader;
#[doc = "Field `SW_RSVD0` writer - RSVD for SW usage"]
pub type SwRsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_RSVD1` reader - RSVD for SW usage"]
pub type SwRsvd1R = crate::FieldReader;
#[doc = "Field `SW_RSVD1` writer - RSVD for SW usage"]
pub type SwRsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_RSVD2` reader - RSVD for SW usage"]
pub type SwRsvd2R = crate::FieldReader;
#[doc = "Field `SW_RSVD2` writer - RSVD for SW usage"]
pub type SwRsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_RSVD3` reader - RSVD for SW usage"]
pub type SwRsvd3R = crate::FieldReader;
#[doc = "Field `SW_RSVD3` writer - RSVD for SW usage"]
pub type SwRsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd0(&self) -> SwRsvd0R {
        SwRsvd0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd1(&self) -> SwRsvd1R {
        SwRsvd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd2(&self) -> SwRsvd2R {
        SwRsvd2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd3(&self) -> SwRsvd3R {
        SwRsvd3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd0(&mut self) -> SwRsvd0W<'_, RegOtpSyscfg1Spec> {
        SwRsvd0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd1(&mut self) -> SwRsvd1W<'_, RegOtpSyscfg1Spec> {
        SwRsvd1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd2(&mut self) -> SwRsvd2W<'_, RegOtpSyscfg1Spec> {
        SwRsvd2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd3(&mut self) -> SwRsvd3W<'_, RegOtpSyscfg1Spec> {
        SwRsvd3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_otp_syscfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_otp_syscfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOtpSyscfg1Spec;
impl crate::RegisterSpec for RegOtpSyscfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_otp_syscfg1::R`](R) reader structure"]
impl crate::Readable for RegOtpSyscfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_otp_syscfg1::W`](W) writer structure"]
impl crate::Writable for RegOtpSyscfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OTP_SYSCFG1 to value 0"]
impl crate::Resettable for RegOtpSyscfg1Spec {}
