#[doc = "Register `REG_OTP_SYSCFG2` reader"]
pub type R = crate::R<RegOtpSyscfg2Spec>;
#[doc = "Register `REG_OTP_SYSCFG2` writer"]
pub type W = crate::W<RegOtpSyscfg2Spec>;
#[doc = "Field `SW_RSVD4` reader - RSVD for SW usage"]
pub type SwRsvd4R = crate::FieldReader<u32>;
#[doc = "Field `SW_RSVD4` writer - RSVD for SW usage"]
pub type SwRsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd4(&self) -> SwRsvd4R {
        SwRsvd4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd4(&mut self) -> SwRsvd4W<'_, RegOtpSyscfg2Spec> {
        SwRsvd4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_otp_syscfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_otp_syscfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOtpSyscfg2Spec;
impl crate::RegisterSpec for RegOtpSyscfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_otp_syscfg2::R`](R) reader structure"]
impl crate::Readable for RegOtpSyscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_otp_syscfg2::W`](W) writer structure"]
impl crate::Writable for RegOtpSyscfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OTP_SYSCFG2 to value 0"]
impl crate::Resettable for RegOtpSyscfg2Spec {}
