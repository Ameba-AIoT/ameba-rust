#[doc = "Register `SAR5` reader"]
pub type R = crate::R<Sar5Spec>;
#[doc = "Register `SAR5` writer"]
pub type W = crate::W<Sar5Spec>;
#[doc = "Field `SAR5` reader - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar5R = crate::FieldReader<u32>;
#[doc = "Field `SAR5` writer - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar5(&self) -> Sar5R {
        Sar5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar5(&mut self) -> Sar5W<'_, Sar5Spec> {
        Sar5W::new(self, 0)
    }
}
#[doc = "Channel 5 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar5Spec;
impl crate::RegisterSpec for Sar5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar5::R`](R) reader structure"]
impl crate::Readable for Sar5Spec {}
#[doc = "`write(|w| ..)` method takes [`sar5::W`](W) writer structure"]
impl crate::Writable for Sar5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR5 to value 0"]
impl crate::Resettable for Sar5Spec {}
