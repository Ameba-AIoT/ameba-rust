#[doc = "Register `IC_DMA_MODE` reader"]
pub type R = crate::R<IcDmaModeSpec>;
#[doc = "Register `IC_DMA_MODE` writer"]
pub type W = crate::W<IcDmaModeSpec>;
#[doc = "Field `DMA_MODE` reader - * 0: DWC DMA legacy mode * 1: DMA with control register * 2: DMA with transfer descriptor"]
pub type DmaModeR = crate::FieldReader;
#[doc = "Field `DMA_MODE` writer - * 0: DWC DMA legacy mode * 1: DMA with control register * 2: DMA with transfer descriptor"]
pub type DmaModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - * 0: DWC DMA legacy mode * 1: DMA with control register * 2: DMA with transfer descriptor"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DmaModeR {
        DmaModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - * 0: DWC DMA legacy mode * 1: DMA with control register * 2: DMA with transfer descriptor"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DmaModeW<'_, IcDmaModeSpec> {
        DmaModeW::new(self, 0)
    }
}
#[doc = "I2C DMA Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_dma_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDmaModeSpec;
impl crate::RegisterSpec for IcDmaModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dma_mode::R`](R) reader structure"]
impl crate::Readable for IcDmaModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_dma_mode::W`](W) writer structure"]
impl crate::Writable for IcDmaModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_DMA_MODE to value 0"]
impl crate::Resettable for IcDmaModeSpec {}
