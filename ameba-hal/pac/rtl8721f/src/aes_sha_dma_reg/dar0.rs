#[doc = "Register `DAR0` reader"]
pub type R = crate::R<Dar0Spec>;
#[doc = "Register `DAR0` writer"]
pub type W = crate::W<Dar0Spec>;
#[doc = "Field `DAR0` reader - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar0R = crate::FieldReader<u32>;
#[doc = "Field `DAR0` writer - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
pub type Dar0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar0(&self) -> Dar0R {
        Dar0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address of DMA transfer.Updated after each destination transfer. The DINC field in the CTLx register determines whether the address increments, or is left unchanged on every destination transfer throughout the block transfer."]
    #[inline(always)]
    pub fn dar0(&mut self) -> Dar0W<'_, Dar0Spec> {
        Dar0W::new(self, 0)
    }
}
#[doc = "Channel 0 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar0Spec;
impl crate::RegisterSpec for Dar0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar0::R`](R) reader structure"]
impl crate::Readable for Dar0Spec {}
#[doc = "`write(|w| ..)` method takes [`dar0::W`](W) writer structure"]
impl crate::Writable for Dar0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAR0 to value 0"]
impl crate::Resettable for Dar0Spec {}
