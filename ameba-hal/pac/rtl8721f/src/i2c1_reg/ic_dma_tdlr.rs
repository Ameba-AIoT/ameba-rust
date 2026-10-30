#[doc = "Register `IC_DMA_TDLR` reader"]
pub type R = crate::R<IcDmaTdlrSpec>;
#[doc = "Register `IC_DMA_TDLR` writer"]
pub type W = crate::W<IcDmaTdlrSpec>;
#[doc = "Field `DMATDL` reader - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE=1."]
pub type DmatdlR = crate::FieldReader;
#[doc = "Field `DMATDL` writer - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE=1."]
pub type DmatdlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE=1."]
    #[inline(always)]
    pub fn dmatdl(&self) -> DmatdlR {
        DmatdlR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE=1."]
    #[inline(always)]
    pub fn dmatdl(&mut self) -> DmatdlW<'_, IcDmaTdlrSpec> {
        DmatdlW::new(self, 0)
    }
}
#[doc = "DMA Transmit Data Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_dma_tdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_tdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDmaTdlrSpec;
impl crate::RegisterSpec for IcDmaTdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dma_tdlr::R`](R) reader structure"]
impl crate::Readable for IcDmaTdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_dma_tdlr::W`](W) writer structure"]
impl crate::Writable for IcDmaTdlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_DMA_TDLR to value 0"]
impl crate::Resettable for IcDmaTdlrSpec {}
