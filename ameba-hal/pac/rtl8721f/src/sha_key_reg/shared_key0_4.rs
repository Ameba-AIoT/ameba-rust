#[doc = "Register `Shared_key0_4` reader"]
pub type R = crate::R<SharedKey0_4Spec>;
#[doc = "Register `Shared_key0_4` writer"]
pub type W = crate::W<SharedKey0_4Spec>;
#[doc = "Field `Shared_Key0_4` reader - Shared key0 register\\[32*4+31:32*4\\]"]
pub type SharedKey0_4R = crate::FieldReader<u32>;
#[doc = "Field `Shared_Key0_4` writer - Shared key0 register\\[32*4+31:32*4\\]"]
pub type SharedKey0_4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shared key0 register\\[32*4+31:32*4\\]"]
    #[inline(always)]
    pub fn shared_key0_4(&self) -> SharedKey0_4R {
        SharedKey0_4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shared key0 register\\[32*4+31:32*4\\]"]
    #[inline(always)]
    pub fn shared_key0_4(&mut self) -> SharedKey0_4W<'_, SharedKey0_4Spec> {
        SharedKey0_4W::new(self, 0)
    }
}
#[doc = "Shared key0 Register\\[159:128\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_key0_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_key0_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedKey0_4Spec;
impl crate::RegisterSpec for SharedKey0_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shared_key0_4::R`](R) reader structure"]
impl crate::Readable for SharedKey0_4Spec {}
#[doc = "`write(|w| ..)` method takes [`shared_key0_4::W`](W) writer structure"]
impl crate::Writable for SharedKey0_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Shared_key0_4 to value 0"]
impl crate::Resettable for SharedKey0_4Spec {}
