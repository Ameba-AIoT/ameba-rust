#[doc = "Register `DMATDLR` reader"]
pub type R = crate::R<DmatdlrSpec>;
#[doc = "Register `DMATDLR` writer"]
pub type W = crate::W<DmatdlrSpec>;
#[doc = "Field `DMATDL` reader - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1. For DMATDL decode, refer to the following description: 000000: dma_tx_req is asserted when 0 data entry is present in transmit FIFO. 000001: dma_tx_req is asserted when 1 data entry is present in transmit FIFO. 000010: dma_tx_req is asserted when 2 data entries are present in transmit FIFO. 000011: dma_tx_req is asserted when 3 data entries are present in transmit FIFO. ... 111100: dma_tx_req is asserted when 60 data entries are present in transmit FIFO. 111101: dma_tx_req is asserted when 61 data entries are present in transmit FIFO. 111110: dma_tx_req is asserted when 62 data entries are present in transmit FIFO. 111111: dma_tx_req is asserted when 63 data entries are present in transmit FIFO."]
pub type DmatdlR = crate::FieldReader;
#[doc = "Field `DMATDL` writer - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1. For DMATDL decode, refer to the following description: 000000: dma_tx_req is asserted when 0 data entry is present in transmit FIFO. 000001: dma_tx_req is asserted when 1 data entry is present in transmit FIFO. 000010: dma_tx_req is asserted when 2 data entries are present in transmit FIFO. 000011: dma_tx_req is asserted when 3 data entries are present in transmit FIFO. ... 111100: dma_tx_req is asserted when 60 data entries are present in transmit FIFO. 111101: dma_tx_req is asserted when 61 data entries are present in transmit FIFO. 111110: dma_tx_req is asserted when 62 data entries are present in transmit FIFO. 111111: dma_tx_req is asserted when 63 data entries are present in transmit FIFO."]
pub type DmatdlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1. For DMATDL decode, refer to the following description: 000000: dma_tx_req is asserted when 0 data entry is present in transmit FIFO. 000001: dma_tx_req is asserted when 1 data entry is present in transmit FIFO. 000010: dma_tx_req is asserted when 2 data entries are present in transmit FIFO. 000011: dma_tx_req is asserted when 3 data entries are present in transmit FIFO. ... 111100: dma_tx_req is asserted when 60 data entries are present in transmit FIFO. 111101: dma_tx_req is asserted when 61 data entries are present in transmit FIFO. 111110: dma_tx_req is asserted when 62 data entries are present in transmit FIFO. 111111: dma_tx_req is asserted when 63 data entries are present in transmit FIFO."]
    #[inline(always)]
    pub fn dmatdl(&self) -> DmatdlR {
        DmatdlR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1. For DMATDL decode, refer to the following description: 000000: dma_tx_req is asserted when 0 data entry is present in transmit FIFO. 000001: dma_tx_req is asserted when 1 data entry is present in transmit FIFO. 000010: dma_tx_req is asserted when 2 data entries are present in transmit FIFO. 000011: dma_tx_req is asserted when 3 data entries are present in transmit FIFO. ... 111100: dma_tx_req is asserted when 60 data entries are present in transmit FIFO. 111101: dma_tx_req is asserted when 61 data entries are present in transmit FIFO. 111110: dma_tx_req is asserted when 62 data entries are present in transmit FIFO. 111111: dma_tx_req is asserted when 63 data entries are present in transmit FIFO."]
    #[inline(always)]
    pub fn dmatdl(&mut self) -> DmatdlW<'_, DmatdlrSpec> {
        DmatdlW::new(self, 0)
    }
}
#[doc = "DMA Transmit Data Level\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatdlrSpec;
impl crate::RegisterSpec for DmatdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdlr::R`](R) reader structure"]
impl crate::Readable for DmatdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatdlr::W`](W) writer structure"]
impl crate::Writable for DmatdlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATDLR to value 0"]
impl crate::Resettable for DmatdlrSpec {}
