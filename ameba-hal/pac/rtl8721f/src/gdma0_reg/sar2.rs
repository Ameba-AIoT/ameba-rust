#[doc = "Register `SAR2` reader"]
pub type R = crate::R<Sar2Spec>;
#[doc = "Register `SAR2` writer"]
pub type W = crate::W<Sar2Spec>;
#[doc = "Field `SAR2` reader - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar2R = crate::FieldReader<u32>;
#[doc = "Field `SAR2` writer - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar2(&self) -> Sar2R {
        Sar2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar2(&mut self) -> Sar2W<'_, Sar2Spec> {
        Sar2W::new(self, 0)
    }
}
#[doc = "Channel 2 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar2Spec;
impl crate::RegisterSpec for Sar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2::R`](R) reader structure"]
impl crate::Readable for Sar2Spec {}
#[doc = "`write(|w| ..)` method takes [`sar2::W`](W) writer structure"]
impl crate::Writable for Sar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR2 to value 0"]
impl crate::Resettable for Sar2Spec {}
