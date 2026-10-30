#[doc = "Register `OTP_CTRL` reader"]
pub type R = crate::R<OtpCtrlSpec>;
#[doc = "Register `OTP_CTRL` writer"]
pub type W = crate::W<OtpCtrlSpec>;
#[doc = "Field `EF_DSLP_TIM` reader - auto return to deep-sleep if idle"]
pub type EfDslpTimR = crate::FieldReader<u32>;
#[doc = "Field `EF_DSLP_TIM` writer - auto return to deep-sleep if idle"]
pub type EfDslpTimW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `EF_PG_PWD` reader - program password"]
pub type EfPgPwdR = crate::FieldReader;
#[doc = "Field `EF_PG_PWD` writer - program password"]
pub type EfPgPwdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:18 - auto return to deep-sleep if idle"]
    #[inline(always)]
    pub fn ef_dslp_tim(&self) -> EfDslpTimR {
        EfDslpTimR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 24:31 - program password"]
    #[inline(always)]
    pub fn ef_pg_pwd(&self) -> EfPgPwdR {
        EfPgPwdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - auto return to deep-sleep if idle"]
    #[inline(always)]
    pub fn ef_dslp_tim(&mut self) -> EfDslpTimW<'_, OtpCtrlSpec> {
        EfDslpTimW::new(self, 0)
    }
    #[doc = "Bits 24:31 - program password"]
    #[inline(always)]
    pub fn ef_pg_pwd(&mut self) -> EfPgPwdW<'_, OtpCtrlSpec> {
        EfPgPwdW::new(self, 24)
    }
}
#[doc = "OTP control register (program password / deep-sleep timer)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpCtrlSpec;
impl crate::RegisterSpec for OtpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_ctrl::R`](R) reader structure"]
impl crate::Readable for OtpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_ctrl::W`](W) writer structure"]
impl crate::Writable for OtpCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_CTRL to value 0"]
impl crate::Resettable for OtpCtrlSpec {}
