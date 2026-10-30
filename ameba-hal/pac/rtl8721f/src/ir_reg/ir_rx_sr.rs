#[doc = "Register `IR_RX_SR` reader"]
pub type R = crate::R<IrRxSrSpec>;
#[doc = "Field `IR_RX_FIFO_FULL_INT_STATUS` reader - Rx FIFO full interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
pub type IrRxFifoFullIntStatusR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_LEVEL_INT_STATUS` reader - Rx FIFO level interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
pub type IrRxFifoLevelIntStatusR = crate::BitReader;
#[doc = "Field `IR_RX_CNT_OF_INT_STATUS` reader - Rx counter overflow interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
pub type IrRxCntOfIntStatusR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_OF_INT_STATUS` reader - Rx FIFO overflow interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
pub type IrRxFifoOfIntStatusR = crate::BitReader;
#[doc = "Field `IR_RX_CNT_THR_INT_STATUS` reader - Rx count threshold interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
pub type IrRxCntThrIntStatusR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_ERROR_INT_STATUS` reader - Rx FIFO error read interrupt status When Rx FIFO is empty, reading the Rx FIFO triggers this interrupt. *0: Interrupt is inactive *1: Interrupt is active"]
pub type IrRxFifoErrorIntStatusR = crate::BitReader;
#[doc = "Field `IR_RX_STATE` reader - *0: Idle *1: Run"]
pub type IrRxStateR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_OFFSET` reader - Rx FIFO offset"]
pub type IrRxFifoOffsetR = crate::FieldReader;
#[doc = "Field `IR_RX_FIFO_FULL` reader - *0: Not full *1: Full"]
pub type IrRxFifoFullR = crate::BitReader;
#[doc = "Field `IR_RX_FIFO_EMPTY` reader - *0: Not empty *1: Empty"]
pub type IrRxFifoEmptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO full interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
    #[inline(always)]
    pub fn ir_rx_fifo_full_int_status(&self) -> IrRxFifoFullIntStatusR {
        IrRxFifoFullIntStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO level interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
    #[inline(always)]
    pub fn ir_rx_fifo_level_int_status(&self) -> IrRxFifoLevelIntStatusR {
        IrRxFifoLevelIntStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx counter overflow interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
    #[inline(always)]
    pub fn ir_rx_cnt_of_int_status(&self) -> IrRxCntOfIntStatusR {
        IrRxCntOfIntStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO overflow interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
    #[inline(always)]
    pub fn ir_rx_fifo_of_int_status(&self) -> IrRxFifoOfIntStatusR {
        IrRxFifoOfIntStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx count threshold interrupt status *0: Interrupt is inactive *1: Interrupt is active"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr_int_status(&self) -> IrRxCntThrIntStatusR {
        IrRxCntThrIntStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO error read interrupt status When Rx FIFO is empty, reading the Rx FIFO triggers this interrupt. *0: Interrupt is inactive *1: Interrupt is active"]
    #[inline(always)]
    pub fn ir_rx_fifo_error_int_status(&self) -> IrRxFifoErrorIntStatusR {
        IrRxFifoErrorIntStatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - *0: Idle *1: Run"]
    #[inline(always)]
    pub fn ir_rx_state(&self) -> IrRxStateR {
        IrRxStateR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Rx FIFO offset"]
    #[inline(always)]
    pub fn ir_rx_fifo_offset(&self) -> IrRxFifoOffsetR {
        IrRxFifoOffsetR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - *0: Not full *1: Full"]
    #[inline(always)]
    pub fn ir_rx_fifo_full(&self) -> IrRxFifoFullR {
        IrRxFifoFullR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - *0: Not empty *1: Empty"]
    #[inline(always)]
    pub fn ir_rx_fifo_empty(&self) -> IrRxFifoEmptyR {
        IrRxFifoEmptyR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "IR Rx FIFO and interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_rx_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrRxSrSpec;
impl crate::RegisterSpec for IrRxSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_rx_sr::R`](R) reader structure"]
impl crate::Readable for IrRxSrSpec {}
#[doc = "`reset()` method sets IR_RX_SR to value 0"]
impl crate::Resettable for IrRxSrSpec {}
