#[doc = "Register `Shared_key1_7` reader"]
pub type R = crate::R<SharedKey1_7Spec>;
#[doc = "Register `Shared_key1_7` writer"]
pub type W = crate::W<SharedKey1_7Spec>;
#[doc = "Field `Shared_key1_7` reader - Shared key1 register\\[32*7+31:32*7\\]"]
pub type SharedKey1_7R = crate::FieldReader<u32>;
#[doc = "Field `Shared_key1_7` writer - Shared key1 register\\[32*7+31:32*7\\]"]
pub type SharedKey1_7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shared key1 register\\[32*7+31:32*7\\]"]
    #[inline(always)]
    pub fn shared_key1_7(&self) -> SharedKey1_7R {
        SharedKey1_7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shared key1 register\\[32*7+31:32*7\\]"]
    #[inline(always)]
    pub fn shared_key1_7(&mut self) -> SharedKey1_7W<'_, SharedKey1_7Spec> {
        SharedKey1_7W::new(self, 0)
    }
}
#[doc = "Shared key1 Register\\[255:224\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_key1_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_key1_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedKey1_7Spec;
impl crate::RegisterSpec for SharedKey1_7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shared_key1_7::R`](R) reader structure"]
impl crate::Readable for SharedKey1_7Spec {}
#[doc = "`write(|w| ..)` method takes [`shared_key1_7::W`](W) writer structure"]
impl crate::Writable for SharedKey1_7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Shared_key1_7 to value 0"]
impl crate::Resettable for SharedKey1_7Spec {}
