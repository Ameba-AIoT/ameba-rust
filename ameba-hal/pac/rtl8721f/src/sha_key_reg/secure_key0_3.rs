#[doc = "Register `Secure_key0_3` reader"]
pub type R = crate::R<SecureKey0_3Spec>;
#[doc = "Register `Secure_key0_3` writer"]
pub type W = crate::W<SecureKey0_3Spec>;
#[doc = "Field `Secure_Key0_3` reader - Secure key0 register\\[32*3+31:31*3\\]"]
pub type SecureKey0_3R = crate::FieldReader<u32>;
#[doc = "Field `Secure_Key0_3` writer - Secure key0 register\\[32*3+31:31*3\\]"]
pub type SecureKey0_3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*3+31:31*3\\]"]
    #[inline(always)]
    pub fn secure_key0_3(&self) -> SecureKey0_3R {
        SecureKey0_3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*3+31:31*3\\]"]
    #[inline(always)]
    pub fn secure_key0_3(&mut self) -> SecureKey0_3W<'_, SecureKey0_3Spec> {
        SecureKey0_3W::new(self, 0)
    }
}
#[doc = "Secure key0 Register\\[127:96\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_key0_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_key0_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureKey0_3Spec;
impl crate::RegisterSpec for SecureKey0_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_key0_3::R`](R) reader structure"]
impl crate::Readable for SecureKey0_3Spec {}
#[doc = "`write(|w| ..)` method takes [`secure_key0_3::W`](W) writer structure"]
impl crate::Writable for SecureKey0_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Secure_key0_3 to value 0"]
impl crate::Resettable for SecureKey0_3Spec {}
