#[doc = "Register `EEEP_CR6` reader"]
pub type R = crate::R<EeepCr6Spec>;
#[doc = "Register `EEEP_CR6` writer"]
pub type W = crate::W<EeepCr6Spec>;
#[doc = "Field `eeep_abig_off` reader - "]
pub type EeepAbigOffR = crate::FieldReader<u16>;
#[doc = "Field `eeep_abig_off` writer - "]
pub type EeepAbigOffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `eeep_abig_on` reader - "]
pub type EeepAbigOnR = crate::FieldReader<u16>;
#[doc = "Field `eeep_abig_on` writer - "]
pub type EeepAbigOnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn eeep_abig_off(&self) -> EeepAbigOffR {
        EeepAbigOffR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn eeep_abig_on(&self) -> EeepAbigOnR {
        EeepAbigOnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn eeep_abig_off(&mut self) -> EeepAbigOffW<'_, EeepCr6Spec> {
        EeepAbigOffW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn eeep_abig_on(&mut self) -> EeepAbigOnW<'_, EeepCr6Spec> {
        EeepAbigOnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`eeep_cr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeep_cr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeepCr6Spec;
impl crate::RegisterSpec for EeepCr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeep_cr6::R`](R) reader structure"]
impl crate::Readable for EeepCr6Spec {}
#[doc = "`write(|w| ..)` method takes [`eeep_cr6::W`](W) writer structure"]
impl crate::Writable for EeepCr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEEP_CR6 to value 0xb80a_800a"]
impl crate::Resettable for EeepCr6Spec {
    const RESET_VALUE: u32 = 0xb80a_800a;
}
