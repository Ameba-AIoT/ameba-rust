#[doc = "Register `OTP_DUMMY` reader"]
pub type R = crate::R<OtpDummySpec>;
#[doc = "Register `OTP_DUMMY` writer"]
pub type W = crate::W<OtpDummySpec>;
#[doc = "Field `dummy` reader - dummy reg for hw"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - dummy reg for hw"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - dummy reg for hw"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - dummy reg for hw"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, OtpDummySpec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpDummySpec;
impl crate::RegisterSpec for OtpDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_dummy::R`](R) reader structure"]
impl crate::Readable for OtpDummySpec {}
#[doc = "`write(|w| ..)` method takes [`otp_dummy::W`](W) writer structure"]
impl crate::Writable for OtpDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_DUMMY to value 0"]
impl crate::Resettable for OtpDummySpec {}
