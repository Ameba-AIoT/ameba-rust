#[doc = "Register `IC_DMA_CR` reader"]
pub type R = crate::R<IcDmaCrSpec>;
#[doc = "Register `IC_DMA_CR` writer"]
pub type W = crate::W<IcDmaCrSpec>;
#[doc = "Field `RDMAE` reader - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. * 0: Receive DMA disabled * 1: Receive DMA enabled"]
pub type RdmaeR = crate::BitReader;
#[doc = "Field `RDMAE` writer - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. * 0: Receive DMA disabled * 1: Receive DMA enabled"]
pub type RdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDMAE` reader - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. * 0: Transmit DMA disabled * 1: Transmit DMA enabled"]
pub type TdmaeR = crate::BitReader;
#[doc = "Field `TDMAE` writer - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. * 0: Transmit DMA disabled * 1: Transmit DMA enabled"]
pub type TdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. * 0: Receive DMA disabled * 1: Receive DMA enabled"]
    #[inline(always)]
    pub fn rdmae(&self) -> RdmaeR {
        RdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. * 0: Transmit DMA disabled * 1: Transmit DMA enabled"]
    #[inline(always)]
    pub fn tdmae(&self) -> TdmaeR {
        TdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. * 0: Receive DMA disabled * 1: Receive DMA enabled"]
    #[inline(always)]
    pub fn rdmae(&mut self) -> RdmaeW<'_, IcDmaCrSpec> {
        RdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. * 0: Transmit DMA disabled * 1: Transmit DMA enabled"]
    #[inline(always)]
    pub fn tdmae(&mut self) -> TdmaeW<'_, IcDmaCrSpec> {
        TdmaeW::new(self, 1)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_dma_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDmaCrSpec;
impl crate::RegisterSpec for IcDmaCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dma_cr::R`](R) reader structure"]
impl crate::Readable for IcDmaCrSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_dma_cr::W`](W) writer structure"]
impl crate::Writable for IcDmaCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_DMA_CR to value 0"]
impl crate::Resettable for IcDmaCrSpec {}
