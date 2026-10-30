#[doc = "Register `SAR7` reader"]
pub type R = crate::R<Sar7Spec>;
#[doc = "Register `SAR7` writer"]
pub type W = crate::W<Sar7Spec>;
#[doc = "Field `SAR7` reader - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar7R = crate::FieldReader<u32>;
#[doc = "Field `SAR7` writer - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar7(&self) -> Sar7R {
        Sar7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar7(&mut self) -> Sar7W<'_, Sar7Spec> {
        Sar7W::new(self, 0)
    }
}
#[doc = "Channel 7 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar7Spec;
impl crate::RegisterSpec for Sar7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar7::R`](R) reader structure"]
impl crate::Readable for Sar7Spec {}
#[doc = "`write(|w| ..)` method takes [`sar7::W`](W) writer structure"]
impl crate::Writable for Sar7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR7 to value 0"]
impl crate::Resettable for Sar7Spec {}
