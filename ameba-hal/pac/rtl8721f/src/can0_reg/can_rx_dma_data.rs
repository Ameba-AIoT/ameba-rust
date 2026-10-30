#[doc = "Register `CAN_RX_DMA_DATA` reader"]
pub type R = crate::R<CanRxDmaDataSpec>;
#[doc = "Field `can_rx_dma_data` reader - For DMA read received data"]
pub type CanRxDmaDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - For DMA read received data"]
    #[inline(always)]
    pub fn can_rx_dma_data(&self) -> CanRxDmaDataR {
        CanRxDmaDataR::new(self.bits)
    }
}
#[doc = "Register for dma access message buffe\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rx_dma_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRxDmaDataSpec;
impl crate::RegisterSpec for CanRxDmaDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rx_dma_data::R`](R) reader structure"]
impl crate::Readable for CanRxDmaDataSpec {}
#[doc = "`reset()` method sets CAN_RX_DMA_DATA to value 0"]
impl crate::Resettable for CanRxDmaDataSpec {}
