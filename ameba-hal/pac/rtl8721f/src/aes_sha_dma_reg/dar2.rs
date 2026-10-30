#[doc = "Register `DAR2` reader"]
pub type R = crate::R<Dar2Spec>;
#[doc = "Register `DAR2` writer"]
pub type W = crate::W<Dar2Spec>;
#[doc = "Field `DAR2` reader - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar2R = crate::FieldReader<u32>;
#[doc = "Field `DAR2` writer - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar2(&self) -> Dar2R {
        Dar2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar2(&mut self) -> Dar2W<'_, Dar2Spec> {
        Dar2W::new(self, 0)
    }
}
#[doc = "Channel 2 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar2Spec;
impl crate::RegisterSpec for Dar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar2::R`](R) reader structure"]
impl crate::Readable for Dar2Spec {}
#[doc = "`write(|w| ..)` method takes [`dar2::W`](W) writer structure"]
impl crate::Writable for Dar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR2 to value 0"]
impl crate::Resettable for Dar2Spec {}
