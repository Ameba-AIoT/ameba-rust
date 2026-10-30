#[doc = "Register `IR_RX_FIFO` reader"]
pub type R = crate::R<IrRxFifoSpec>;
#[doc = "Field `IR_RX_CNT` reader - 31-bits cycle duration"]
pub type IrRxCntR = crate::FieldReader<u32>;
#[doc = "Field `IR_RX_LEVEL` reader - Rx Level *1: High level *0: Low level"]
pub type IrRxLevelR = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - 31-bits cycle duration"]
    #[inline(always)]
    pub fn ir_rx_cnt(&self) -> IrRxCntR {
        IrRxCntR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Rx Level *1: High level *0: Low level"]
    #[inline(always)]
    pub fn ir_rx_level(&self) -> IrRxLevelR {
        IrRxLevelR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "IR Rx FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_rx_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrRxFifoSpec;
impl crate::RegisterSpec for IrRxFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_rx_fifo::R`](R) reader structure"]
impl crate::Readable for IrRxFifoSpec {}
#[doc = "`reset()` method sets IR_RX_FIFO to value 0"]
impl crate::Resettable for IrRxFifoSpec {}
