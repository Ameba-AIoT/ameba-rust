#[doc = "Register `DMARDLR` reader"]
pub type R = crate::R<DmardlrSpec>;
#[doc = "Register `DMARDLR` writer"]
pub type W = crate::W<DmardlrSpec>;
#[doc = "Field `DMARDL` reader - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1."]
pub type DmardlR = crate::FieldReader;
#[doc = "Field `DMARDL` writer - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1."]
pub type DmardlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1."]
    #[inline(always)]
    pub fn dmardl(&self) -> DmardlR {
        DmardlR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1."]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DmardlW<'_, DmardlrSpec> {
        DmardlW::new(self, 0)
    }
}
#[doc = "DMA Receive Data Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmardlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
