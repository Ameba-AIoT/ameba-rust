#[doc = "Register `IC_DMA_RDLR` reader"]
pub type R = crate::R<IcDmaRdlrSpec>;
#[doc = "Register `IC_DMA_RDLR` writer"]
pub type W = crate::W<IcDmaRdlrSpec>;
#[doc = "Field `DMARDL` reader - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level=DMARDL+1; that is, the dma_tx_req signal is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value+1, and RDMAE=1. For instance, when DMARDL is 0, the dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
pub type DmardlR = crate::FieldReader;
#[doc = "Field `DMARDL` writer - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level=DMARDL+1; that is, the dma_tx_req signal is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value+1, and RDMAE=1. For instance, when DMARDL is 0, the dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
pub type DmardlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level=DMARDL+1; that is, the dma_tx_req signal is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value+1, and RDMAE=1. For instance, when DMARDL is 0, the dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
    #[inline(always)]
    pub fn dmardl(&self) -> DmardlR {
        DmardlR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level=DMARDL+1; that is, the dma_tx_req signal is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value+1, and RDMAE=1. For instance, when DMARDL is 0, the dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DmardlW<'_, IcDmaRdlrSpec> {
        DmardlW::new(self, 0)
    }
}
#[doc = "I2C Reveive Data Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_dma_rdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_rdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcDmaRdlrSpec;
impl crate::RegisterSpec for IcDmaRdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_dma_rdlr::R`](R) reader structure"]
impl crate::Readable for IcDmaRdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_dma_rdlr::W`](W) writer structure"]
impl crate::Writable for IcDmaRdlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_DMA_RDLR to value 0"]
impl crate::Resettable for IcDmaRdlrSpec {}
