#[doc = "Register `EEEP_CR7` reader"]
pub type R = crate::R<EeepCr7Spec>;
#[doc = "Register `EEEP_CR7` writer"]
pub type W = crate::W<EeepCr7Spec>;
#[doc = "Field `eeep_ldvbias_off` reader - "]
pub type EeepLdvbiasOffR = crate::FieldReader<u16>;
#[doc = "Field `eeep_ldvbias_off` writer - "]
pub type EeepLdvbiasOffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `eeep_ldvbias_on` reader - "]
pub type EeepLdvbiasOnR = crate::FieldReader<u16>;
#[doc = "Field `eeep_ldvbias_on` writer - "]
pub type EeepLdvbiasOnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn eeep_ldvbias_off(&self) -> EeepLdvbiasOffR {
        EeepLdvbiasOffR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn eeep_ldvbias_on(&self) -> EeepLdvbiasOnR {
        EeepLdvbiasOnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn eeep_ldvbias_off(&mut self) -> EeepLdvbiasOffW<'_, EeepCr7Spec> {
        EeepLdvbiasOffW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn eeep_ldvbias_on(&mut self) -> EeepLdvbiasOnW<'_, EeepCr7Spec> {
        EeepLdvbiasOnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eeep_cr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeep_cr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeepCr7Spec;
impl crate::RegisterSpec for EeepCr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeep_cr7::R`](R) reader structure"]
impl crate::Readable for EeepCr7Spec {}
#[doc = "`write(|w| ..)` method takes [`eeep_cr7::W`](W) writer structure"]
impl crate::Writable for EeepCr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEEP_CR7 to value 0x7550_0550"]
impl crate::Resettable for EeepCr7Spec {
    const RESET_VALUE: u32 = 0x7550_0550;
}
