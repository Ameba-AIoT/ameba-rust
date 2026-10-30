#[doc = "Register `DAR3` reader"]
pub type R = crate::R<Dar3Spec>;
#[doc = "Register `DAR3` writer"]
pub type W = crate::W<Dar3Spec>;
#[doc = "Field `DAR3` reader - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar3R = crate::FieldReader<u32>;
#[doc = "Field `DAR3` writer - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar3(&self) -> Dar3R {
        Dar3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar3(&mut self) -> Dar3W<'_, Dar3Spec> {
        Dar3W::new(self, 0)
    }
}
#[doc = "Channel 3 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar3Spec;
impl crate::RegisterSpec for Dar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar3::R`](R) reader structure"]
impl crate::Readable for Dar3Spec {}
#[doc = "`write(|w| ..)` method takes [`dar3::W`](W) writer structure"]
impl crate::Writable for Dar3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR3 to value 0"]
impl crate::Resettable for Dar3Spec {}
