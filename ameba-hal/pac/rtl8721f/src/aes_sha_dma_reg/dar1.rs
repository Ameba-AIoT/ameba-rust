#[doc = "Register `DAR1` reader"]
pub type R = crate::R<Dar1Spec>;
#[doc = "Register `DAR1` writer"]
pub type W = crate::W<Dar1Spec>;
#[doc = "Field `DAR1` reader - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar1R = crate::FieldReader<u32>;
#[doc = "Field `DAR1` writer - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar1(&self) -> Dar1R {
        Dar1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar1(&mut self) -> Dar1W<'_, Dar1Spec> {
        Dar1W::new(self, 0)
    }
}
#[doc = "Channel 1 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar1Spec;
impl crate::RegisterSpec for Dar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar1::R`](R) reader structure"]
impl crate::Readable for Dar1Spec {}
#[doc = "`write(|w| ..)` method takes [`dar1::W`](W) writer structure"]
impl crate::Writable for Dar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR1 to value 0"]
impl crate::Resettable for Dar1Spec {}
