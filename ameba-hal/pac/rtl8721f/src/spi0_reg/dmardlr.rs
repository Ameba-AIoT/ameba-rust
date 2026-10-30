#[doc = "Register `DMARDLR` reader"]
pub type R = crate::R<DmardlrSpec>;
#[doc = "Register `DMARDLR` writer"]
pub type W = crate::W<DmardlrSpec>;
#[doc = "Field `DMARDL` reader - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1. For DMARDL decode, refer to the following description: 000000: dma_rx_req is asserted when 1 or more data entries are present in transmit FIFO. 000001: dma_rx_req is asserted when 2 or more data entries are present in transmit FIFO. 000010: dma_rx_req is asserted when 3 or more data entries are present in transmit FIFO. 000011: dma_rx_req is asserted when 4 or more data entries are present in transmit FIFO. ... 111100: dma_rx_req is asserted when 61 or more data entries are present in transmit FIFO. 111101: dma_rx_req is asserted when 62 or more data entries are present in transmit FIFO. 111110: dma_rx_req is asserted when 63 or more data entries are present in transmit FIFO. 111111: dma_rx_req is asserted when 64 data entries are present in transmit FIFO."]
pub type DmardlR = crate::FieldReader;
#[doc = "Field `DMARDL` writer - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1. For DMARDL decode, refer to the following description: 000000: dma_rx_req is asserted when 1 or more data entries are present in transmit FIFO. 000001: dma_rx_req is asserted when 2 or more data entries are present in transmit FIFO. 000010: dma_rx_req is asserted when 3 or more data entries are present in transmit FIFO. 000011: dma_rx_req is asserted when 4 or more data entries are present in transmit FIFO. ... 111100: dma_rx_req is asserted when 61 or more data entries are present in transmit FIFO. 111101: dma_rx_req is asserted when 62 or more data entries are present in transmit FIFO. 111110: dma_rx_req is asserted when 63 or more data entries are present in transmit FIFO. 111111: dma_rx_req is asserted when 64 data entries are present in transmit FIFO."]
pub type DmardlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1. For DMARDL decode, refer to the following description: 000000: dma_rx_req is asserted when 1 or more data entries are present in transmit FIFO. 000001: dma_rx_req is asserted when 2 or more data entries are present in transmit FIFO. 000010: dma_rx_req is asserted when 3 or more data entries are present in transmit FIFO. 000011: dma_rx_req is asserted when 4 or more data entries are present in transmit FIFO. ... 111100: dma_rx_req is asserted when 61 or more data entries are present in transmit FIFO. 111101: dma_rx_req is asserted when 62 or more data entries are present in transmit FIFO. 111110: dma_rx_req is asserted when 63 or more data entries are present in transmit FIFO. 111111: dma_rx_req is asserted when 64 data entries are present in transmit FIFO."]
    #[inline(always)]
    pub fn dmardl(&self) -> DmardlR {
        DmardlR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1. For DMARDL decode, refer to the following description: 000000: dma_rx_req is asserted when 1 or more data entries are present in transmit FIFO. 000001: dma_rx_req is asserted when 2 or more data entries are present in transmit FIFO. 000010: dma_rx_req is asserted when 3 or more data entries are present in transmit FIFO. 000011: dma_rx_req is asserted when 4 or more data entries are present in transmit FIFO. ... 111100: dma_rx_req is asserted when 61 or more data entries are present in transmit FIFO. 111101: dma_rx_req is asserted when 62 or more data entries are present in transmit FIFO. 111110: dma_rx_req is asserted when 63 or more data entries are present in transmit FIFO. 111111: dma_rx_req is asserted when 64 data entries are present in transmit FIFO."]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DmardlW<'_, DmardlrSpec> {
        DmardlW::new(self, 0)
    }
}
#[doc = "DMA Receive Data Level\n\nYou can [`read`](crate::Reg::read) this register and get [`dmardlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmardlrSpec;
impl crate::RegisterSpec for DmardlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmardlr::R`](R) reader structure"]
impl crate::Readable for DmardlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmardlr::W`](W) writer structure"]
impl crate::Writable for DmardlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMARDLR to value 0"]
impl crate::Resettable for DmardlrSpec {}
