#[doc = "Register `DAR4` reader"]
pub type R = crate::R<Dar4Spec>;
#[doc = "Register `DAR4` writer"]
pub type W = crate::W<Dar4Spec>;
#[doc = "Field `DAR4` reader - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar4R = crate::FieldReader<u32>;
#[doc = "Field `DAR4` writer - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar4(&self) -> Dar4R {
        Dar4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar4(&mut self) -> Dar4W<'_, Dar4Spec> {
        Dar4W::new(self, 0)
    }
}
#[doc = "Channel 4 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar4Spec;
impl crate::RegisterSpec for Dar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar4::R`](R) reader structure"]
impl crate::Readable for Dar4Spec {}
#[doc = "`write(|w| ..)` method takes [`dar4::W`](W) writer structure"]
impl crate::Writable for Dar4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR4 to value 0"]
impl crate::Resettable for Dar4Spec {}
