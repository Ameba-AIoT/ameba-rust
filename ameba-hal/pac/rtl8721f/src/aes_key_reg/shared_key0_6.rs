#[doc = "Register `Shared_key0_6` reader"]
pub type R = crate::R<SharedKey0_6Spec>;
#[doc = "Register `Shared_key0_6` writer"]
pub type W = crate::W<SharedKey0_6Spec>;
#[doc = "Field `Shared_Key0_6` reader - Shared key0 register\\[32*6+31:32*6\\]"]
pub type SharedKey0_6R = crate::FieldReader<u32>;
#[doc = "Field `Shared_Key0_6` writer - Shared key0 register\\[32*6+31:32*6\\]"]
pub type SharedKey0_6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shared key0 register\\[32*6+31:32*6\\]"]
    #[inline(always)]
    pub fn shared_key0_6(&self) -> SharedKey0_6R {
        SharedKey0_6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shared key0 register\\[32*6+31:32*6\\]"]
    #[inline(always)]
    pub fn shared_key0_6(&mut self) -> SharedKey0_6W<'_, SharedKey0_6Spec> {
        SharedKey0_6W::new(self, 0)
    }
}
#[doc = "Shared key0 Register\\[223:192\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_key0_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_key0_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedKey0_6Spec;
impl crate::RegisterSpec for SharedKey0_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shared_key0_6::R`](R) reader structure"]
impl crate::Readable for SharedKey0_6Spec {}
#[doc = "`write(|w| ..)` method takes [`shared_key0_6::W`](W) writer structure"]
impl crate::Writable for SharedKey0_6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Shared_key0_6 to value 0"]
impl crate::Resettable for SharedKey0_6Spec {}
