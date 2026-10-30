#[doc = "Register `Secure_key0_5` reader"]
pub type R = crate::R<SecureKey0_5Spec>;
#[doc = "Register `Secure_key0_5` writer"]
pub type W = crate::W<SecureKey0_5Spec>;
#[doc = "Field `Secure_Key0_5` reader - Secure key0 register\\[32*5+31:31*5\\]"]
pub type SecureKey0_5R = crate::FieldReader<u32>;
#[doc = "Field `Secure_Key0_5` writer - Secure key0 register\\[32*5+31:31*5\\]"]
pub type SecureKey0_5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*5+31:31*5\\]"]
    #[inline(always)]
    pub fn secure_key0_5(&self) -> SecureKey0_5R {
        SecureKey0_5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure key0 register\\[32*5+31:31*5\\]"]
    #[inline(always)]
    pub fn secure_key0_5(&mut self) -> SecureKey0_5W<'_, SecureKey0_5Spec> {
        SecureKey0_5W::new(self, 0)
    }
}
#[doc = "Secure key0 Register\\[191:160\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_key0_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_key0_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureKey0_5Spec;
impl crate::RegisterSpec for SecureKey0_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_key0_5::R`](R) reader structure"]
impl crate::Readable for SecureKey0_5Spec {}
#[doc = "`write(|w| ..)` method takes [`secure_key0_5::W`](W) writer structure"]
impl crate::Writable for SecureKey0_5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Secure_key0_5 to value 0"]
impl crate::Resettable for SecureKey0_5Spec {}
