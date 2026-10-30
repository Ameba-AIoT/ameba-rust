#[doc = "Register `REG_OTP_SYSCFG3` reader"]
pub type R = crate::R<RegOtpSyscfg3Spec>;
#[doc = "Register `REG_OTP_SYSCFG3` writer"]
pub type W = crate::W<RegOtpSyscfg3Spec>;
#[doc = "Field `SW_RSVD5` reader - RSVD for SW usage"]
pub type SwRsvd5R = crate::FieldReader<u32>;
#[doc = "Field `SW_RSVD5` writer - RSVD for SW usage"]
pub type SwRsvd5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd5(&self) -> SwRsvd5R {
        SwRsvd5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd5(&mut self) -> SwRsvd5W<'_, RegOtpSyscfg3Spec> {
        SwRsvd5W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_otp_syscfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_otp_syscfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOtpSyscfg3Spec;
impl crate::RegisterSpec for RegOtpSyscfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_otp_syscfg3::R`](R) reader structure"]
impl crate::Readable for RegOtpSyscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_otp_syscfg3::W`](W) writer structure"]
impl crate::Writable for RegOtpSyscfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OTP_SYSCFG3 to value 0"]
impl crate::Resettable for RegOtpSyscfg3Spec {}
