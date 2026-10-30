#[doc = "Register `Secure_key1_6` reader"]
pub type R = crate::R<SecureKey1_6Spec>;
#[doc = "Register `Secure_key1_6` writer"]
pub type W = crate::W<SecureKey1_6Spec>;
#[doc = "Field `Secure_Key1_6` reader - Secure key1 register\\[32*6+31:31*6\\]"]
pub type SecureKey1_6R = crate::FieldReader<u32>;
#[doc = "Field `Secure_Key1_6` writer - Secure key1 register\\[32*6+31:31*6\\]"]
pub type SecureKey1_6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure key1 register\\[32*6+31:31*6\\]"]
    #[inline(always)]
    pub fn secure_key1_6(&self) -> SecureKey1_6R {
        SecureKey1_6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure key1 register\\[32*6+31:31*6\\]"]
    #[inline(always)]
    pub fn secure_key1_6(&mut self) -> SecureKey1_6W<'_, SecureKey1_6Spec> {
        SecureKey1_6W::new(self, 0)
    }
}
#[doc = "Secure key1 Register\\[223:192\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_key1_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_key1_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureKey1_6Spec;
impl crate::RegisterSpec for SecureKey1_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_key1_6::R`](R) reader structure"]
impl crate::Readable for SecureKey1_6Spec {}
#[doc = "`write(|w| ..)` method takes [`secure_key1_6::W`](W) writer structure"]
impl crate::Writable for SecureKey1_6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Secure_key1_6 to value 0"]
impl crate::Resettable for SecureKey1_6Spec {}
