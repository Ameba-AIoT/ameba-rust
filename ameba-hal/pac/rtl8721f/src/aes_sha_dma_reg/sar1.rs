#[doc = "Register `SAR1` reader"]
pub type R = crate::R<Sar1Spec>;
#[doc = "Register `SAR1` writer"]
pub type W = crate::W<Sar1Spec>;
#[doc = "Field `SAR1` reader - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar1R = crate::FieldReader<u32>;
#[doc = "Field `SAR1` writer - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar1(&self) -> Sar1R {
        Sar1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar1(&mut self) -> Sar1W<'_, Sar1Spec> {
        Sar1W::new(self, 0)
    }
}
#[doc = "Channel 1 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar1Spec;
impl crate::RegisterSpec for Sar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1::R`](R) reader structure"]
impl crate::Readable for Sar1Spec {}
#[doc = "`write(|w| ..)` method takes [`sar1::W`](W) writer structure"]
impl crate::Writable for Sar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR1 to value 0"]
impl crate::Resettable for Sar1Spec {}
