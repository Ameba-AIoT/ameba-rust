#[doc = "Register `SAR0` reader"]
pub type R = crate::R<Sar0Spec>;
#[doc = "Register `SAR0` writer"]
pub type W = crate::W<Sar0Spec>;
#[doc = "Field `SAR0` reader - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar0R = crate::FieldReader<u32>;
#[doc = "Field `SAR0` writer - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
pub type Sar0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar0(&self) -> Sar0R {
        Sar0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address of DMA transfer.Updated after each source transfer. The SINC field in the CTLx register determines whether the address increments, or is left unchanged on every source transfer throughout the block transfer."]
    #[inline(always)]
    pub fn sar0(&mut self) -> Sar0W<'_, Sar0Spec> {
        Sar0W::new(self, 0)
    }
}
#[doc = "Channel 0 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar0Spec;
impl crate::RegisterSpec for Sar0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar0::R`](R) reader structure"]
impl crate::Readable for Sar0Spec {}
#[doc = "`write(|w| ..)` method takes [`sar0::W`](W) writer structure"]
impl crate::Writable for Sar0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR0 to value 0"]
impl crate::Resettable for Sar0Spec {}
