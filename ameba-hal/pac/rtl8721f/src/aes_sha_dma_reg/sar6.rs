#[doc = "Register `SAR6` reader"]
pub type R = crate::R<Sar6Spec>;
#[doc = "Register `SAR6` writer"]
pub type W = crate::W<Sar6Spec>;
#[doc = "Field `SAR6` reader - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar6R = crate::FieldReader<u32>;
#[doc = "Field `SAR6` writer - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar6(&self) -> Sar6R {
        Sar6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar6(&mut self) -> Sar6W<'_, Sar6Spec> {
        Sar6W::new(self, 0)
    }
}
#[doc = "Channel 6 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar6Spec;
impl crate::RegisterSpec for Sar6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar6::R`](R) reader structure"]
impl crate::Readable for Sar6Spec {}
#[doc = "`write(|w| ..)` method takes [`sar6::W`](W) writer structure"]
impl crate::Writable for Sar6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR6 to value 0"]
impl crate::Resettable for Sar6Spec {}
