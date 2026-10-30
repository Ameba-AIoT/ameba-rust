#[doc = "Register `DAR7` reader"]
pub type R = crate::R<Dar7Spec>;
#[doc = "Register `DAR7` writer"]
pub type W = crate::W<Dar7Spec>;
#[doc = "Field `DAR7` reader - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar7R = crate::FieldReader<u32>;
#[doc = "Field `DAR7` writer - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar7(&self) -> Dar7R {
        Dar7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar7(&mut self) -> Dar7W<'_, Dar7Spec> {
        Dar7W::new(self, 0)
    }
}
#[doc = "Channel 7 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar7Spec;
impl crate::RegisterSpec for Dar7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar7::R`](R) reader structure"]
impl crate::Readable for Dar7Spec {}
#[doc = "`write(|w| ..)` method takes [`dar7::W`](W) writer structure"]
impl crate::Writable for Dar7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR7 to value 0"]
impl crate::Resettable for Dar7Spec {}
