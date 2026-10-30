#[doc = "Register `DAR6` reader"]
pub type R = crate::R<Dar6Spec>;
#[doc = "Register `DAR6` writer"]
pub type W = crate::W<Dar6Spec>;
#[doc = "Field `DAR6` reader - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar6R = crate::FieldReader<u32>;
#[doc = "Field `DAR6` writer - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar6(&self) -> Dar6R {
        Dar6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar6(&mut self) -> Dar6W<'_, Dar6Spec> {
        Dar6W::new(self, 0)
    }
}
#[doc = "Channel 6 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar6Spec;
impl crate::RegisterSpec for Dar6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar6::R`](R) reader structure"]
impl crate::Readable for Dar6Spec {}
#[doc = "`write(|w| ..)` method takes [`dar6::W`](W) writer structure"]
impl crate::Writable for Dar6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR6 to value 0"]
impl crate::Resettable for Dar6Spec {}
