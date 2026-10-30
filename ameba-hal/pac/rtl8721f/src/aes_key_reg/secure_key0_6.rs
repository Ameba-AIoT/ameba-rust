#[doc = "Register `Secure_key0_6` reader"]
pub type R = crate::R<SecureKey0_6Spec>;
#[doc = "Register `Secure_key0_6` writer"]
pub type W = crate::W<SecureKey0_6Spec>;
#[doc = "Field `Secure_Key0_6` reader - Secure key0 register\\[32*6+31:31*6\\]"]
pub type SecureKey0_6R = crate::FieldReader<u32>;
#[doc = "Field `Secure_Key0_6` writer - Secure key0 register\\[32*6+31:31*6\\]"]
pub type SecureKey0_6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*6+31:31*6\\]"]
    #[inline(always)]
    pub fn secure_key0_6(&self) -> SecureKey0_6R {
        SecureKey0_6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*6+31:31*6\\]"]
    #[inline(always)]
    pub fn secure_key0_6(&mut self) -> SecureKey0_6W<'_, SecureKey0_6Spec> {
        SecureKey0_6W::new(self, 0)
    }
}
#[doc = "Secure key0 Register\\[223:192\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_key0_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_key0_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureKey0_6Spec;
impl crate::RegisterSpec for SecureKey0_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_key0_6::R`](R) reader structure"]
impl crate::Readable for SecureKey0_6Spec {}
#[doc = "`write(|w| ..)` method takes [`secure_key0_6::W`](W) writer structure"]
impl crate::Writable for SecureKey0_6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Secure_key0_6 to value 0"]
impl crate::Resettable for SecureKey0_6Spec {}
