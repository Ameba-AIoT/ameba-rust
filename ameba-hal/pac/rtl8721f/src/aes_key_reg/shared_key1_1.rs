#[doc = "Register `Shared_key1_1` reader"]
pub type R = crate::R<SharedKey1_1Spec>;
#[doc = "Register `Shared_key1_1` writer"]
pub type W = crate::W<SharedKey1_1Spec>;
#[doc = "Field `Shared_key1_1` reader - Shared key1 register\\[32*1+31:32*1\\]"]
pub type SharedKey1_1R = crate::FieldReader<u32>;
#[doc = "Field `Shared_key1_1` writer - Shared key1 register\\[32*1+31:32*1\\]"]
pub type SharedKey1_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shared key1 register\\[32*1+31:32*1\\]"]
    #[inline(always)]
    pub fn shared_key1_1(&self) -> SharedKey1_1R {
        SharedKey1_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shared key1 register\\[32*1+31:32*1\\]"]
    #[inline(always)]
    pub fn shared_key1_1(&mut self) -> SharedKey1_1W<'_, SharedKey1_1Spec> {
        SharedKey1_1W::new(self, 0)
    }
}
#[doc = "Shared key1 Register\\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_key1_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_key1_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedKey1_1Spec;
impl crate::RegisterSpec for SharedKey1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shared_key1_1::R`](R) reader structure"]
impl crate::Readable for SharedKey1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`shared_key1_1::W`](W) writer structure"]
impl crate::Writable for SharedKey1_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Shared_key1_1 to value 0"]
impl crate::Resettable for SharedKey1_1Spec {}
