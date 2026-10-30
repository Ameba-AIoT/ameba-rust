#[doc = "Register `Secure_key0_7` reader"]
pub type R = crate::R<SecureKey0_7Spec>;
#[doc = "Register `Secure_key0_7` writer"]
pub type W = crate::W<SecureKey0_7Spec>;
#[doc = "Field `Secure_Key0_7` reader - Secure key0 register\\[32*7+31:31*7\\]"]
pub type SecureKey0_7R = crate::FieldReader<u32>;
#[doc = "Field `Secure_Key0_7` writer - Secure key0 register\\[32*7+31:31*7\\]"]
pub type SecureKey0_7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*7+31:31*7\\]"]
    #[inline(always)]
    pub fn secure_key0_7(&self) -> SecureKey0_7R {
        SecureKey0_7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*7+31:31*7\\]"]
    #[inline(always)]
    pub fn secure_key0_7(&mut self) -> SecureKey0_7W<'_, SecureKey0_7Spec> {
        SecureKey0_7W::new(self, 0)
    }
}
#[doc = "Secure key0 Register\\[255:224\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_key0_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_key0_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureKey0_7Spec;
impl crate::RegisterSpec for SecureKey0_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_key0_7::R`](R) reader structure"]
impl crate::Readable for SecureKey0_7Spec {}
#[doc = "`write(|w| ..)` method takes [`secure_key0_7::W`](W) writer structure"]
impl crate::Writable for SecureKey0_7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Secure_key0_7 to value 0"]
impl crate::Resettable for SecureKey0_7Spec {}
