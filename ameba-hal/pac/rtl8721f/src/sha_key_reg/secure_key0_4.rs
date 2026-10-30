#[doc = "Register `Secure_key0_4` reader"]
pub type R = crate::R<SecureKey0_4Spec>;
#[doc = "Register `Secure_key0_4` writer"]
pub type W = crate::W<SecureKey0_4Spec>;
#[doc = "Field `Secure_Key0_4` reader - Secure key0 register\\[32*4+31:31*4\\]"]
pub type SecureKey0_4R = crate::FieldReader<u32>;
#[doc = "Field `Secure_Key0_4` writer - Secure key0 register\\[32*4+31:31*4\\]"]
pub type SecureKey0_4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*4+31:31*4\\]"]
    #[inline(always)]
    pub fn secure_key0_4(&self) -> SecureKey0_4R {
        SecureKey0_4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*4+31:31*4\\]"]
    #[inline(always)]
    pub fn secure_key0_4(&mut self) -> SecureKey0_4W<'_, SecureKey0_4Spec> {
        SecureKey0_4W::new(self, 0)
    }
}
#[doc = "Secure key0 Register\\[159:128\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_key0_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_key0_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureKey0_4Spec;
impl crate::RegisterSpec for SecureKey0_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_key0_4::R`](R) reader structure"]
impl crate::Readable for SecureKey0_4Spec {}
#[doc = "`write(|w| ..)` method takes [`secure_key0_4::W`](W) writer structure"]
impl crate::Writable for SecureKey0_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Secure_key0_4 to value 0"]
impl crate::Resettable for SecureKey0_4Spec {}
