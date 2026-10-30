#[doc = "Register `DAR5` reader"]
pub type R = crate::R<Dar5Spec>;
#[doc = "Register `DAR5` writer"]
pub type W = crate::W<Dar5Spec>;
#[doc = "Field `DAR5` reader - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar5R = crate::FieldReader<u32>;
#[doc = "Field `DAR5` writer - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar5(&self) -> Dar5R {
        Dar5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar5(&mut self) -> Dar5W<'_, Dar5Spec> {
        Dar5W::new(self, 0)
    }
}
#[doc = "Channel 5 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar5Spec;
impl crate::RegisterSpec for Dar5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar5::R`](R) reader structure"]
impl crate::Readable for Dar5Spec {}
#[doc = "`write(|w| ..)` method takes [`dar5::W`](W) writer structure"]
impl crate::Writable for Dar5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR5 to value 0"]
impl crate::Resettable for Dar5Spec {}
