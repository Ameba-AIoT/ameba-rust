#[doc = "Register `Shared_key1_5` reader"]
pub type R = crate::R<SharedKey1_5Spec>;
#[doc = "Register `Shared_key1_5` writer"]
pub type W = crate::W<SharedKey1_5Spec>;
#[doc = "Field `Shared_key1_5` reader - Shared key1 register\\[32*5+31:32*5\\]"]
pub type SharedKey1_5R = crate::FieldReader<u32>;
#[doc = "Field `Shared_key1_5` writer - Shared key1 register\\[32*5+31:32*5\\]"]
pub type SharedKey1_5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shared key1 register\\[32*5+31:32*5\\]"]
    #[inline(always)]
    pub fn shared_key1_5(&self) -> SharedKey1_5R {
        SharedKey1_5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shared key1 register\\[32*5+31:32*5\\]"]
    #[inline(always)]
    pub fn shared_key1_5(&mut self) -> SharedKey1_5W<'_, SharedKey1_5Spec> {
        SharedKey1_5W::new(self, 0)
    }
}
#[doc = "Shared key1 Register\\[191:160\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_key1_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_key1_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedKey1_5Spec;
impl crate::RegisterSpec for SharedKey1_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shared_key1_5::R`](R) reader structure"]
impl crate::Readable for SharedKey1_5Spec {}
#[doc = "`write(|w| ..)` method takes [`shared_key1_5::W`](W) writer structure"]
impl crate::Writable for SharedKey1_5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Shared_key1_5 to value 0"]
impl crate::Resettable for SharedKey1_5Spec {}
