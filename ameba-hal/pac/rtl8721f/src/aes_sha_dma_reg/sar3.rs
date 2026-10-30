#[doc = "Register `SAR3` reader"]
pub type R = crate::R<Sar3Spec>;
#[doc = "Register `SAR3` writer"]
pub type W = crate::W<Sar3Spec>;
#[doc = "Field `SAR3` reader - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar3R = crate::FieldReader<u32>;
#[doc = "Field `SAR3` writer - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar3(&self) -> Sar3R {
        Sar3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar3(&mut self) -> Sar3W<'_, Sar3Spec> {
        Sar3W::new(self, 0)
    }
}
#[doc = "Channel 3 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar3Spec;
impl crate::RegisterSpec for Sar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar3::R`](R) reader structure"]
impl crate::Readable for Sar3Spec {}
#[doc = "`write(|w| ..)` method takes [`sar3::W`](W) writer structure"]
impl crate::Writable for Sar3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR3 to value 0"]
impl crate::Resettable for Sar3Spec {}
