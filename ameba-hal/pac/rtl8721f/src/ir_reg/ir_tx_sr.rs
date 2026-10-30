#[doc = "Register `IR_TX_SR` reader"]
pub type R = crate::R<IrTxSrSpec>;
#[doc = "Field `IR_TX_FIFO_EMPTY_INT_STATUS` reader - Tx FIFO empty interrupt *0: Interrupt inactive *1: Interrupt active"]
pub type IrTxFifoEmptyIntStatusR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_LEVEL_INT_STATUS` reader - When Tx FIFO offset <= threshold value, interrupt is triggered. *0: Interrupt inactive *1: Interrupt active"]
pub type IrTxFifoLevelIntStatusR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_OVER_INT_STATUS` reader - Tx FIFO overflow interrupt *0: Interrupt inactive *1: Interrupt active"]
pub type IrTxFifoOverIntStatusR = crate::BitReader;
#[doc = "Field `IR_TX_STATUS` reader - *0: Idle *1: Run"]
pub type IrTxStatusR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_OFFSET` reader - Tx FIFO offset is from 0 to 32. Note: After Tx last packet, hardware can't clear Tx FIFO offset."]
pub type IrTxFifoOffsetR = crate::FieldReader;
#[doc = "Field `IR_TX_FIFO_FULL` reader - *0: Not full *1: Full"]
pub type IrTxFifoFullR = crate::BitReader;
#[doc = "Field `IR_TX_FIFO_EMPTY` reader - *0: Not empty *1: Empty"]
pub type IrTxFifoEmptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Tx FIFO empty interrupt *0: Interrupt inactive *1: Interrupt active"]
    #[inline(always)]
    pub fn ir_tx_fifo_empty_int_status(&self) -> IrTxFifoEmptyIntStatusR {
        IrTxFifoEmptyIntStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When Tx FIFO offset <= threshold value, interrupt is triggered. *0: Interrupt inactive *1: Interrupt active"]
    #[inline(always)]
    pub fn ir_tx_fifo_level_int_status(&self) -> IrTxFifoLevelIntStatusR {
        IrTxFifoLevelIntStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx FIFO overflow interrupt *0: Interrupt inactive *1: Interrupt active"]
    #[inline(always)]
    pub fn ir_tx_fifo_over_int_status(&self) -> IrTxFifoOverIntStatusR {
        IrTxFifoOverIntStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - *0: Idle *1: Run"]
    #[inline(always)]
    pub fn ir_tx_status(&self) -> IrTxStatusR {
        IrTxStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Tx FIFO offset is from 0 to 32. Note: After Tx last packet, hardware can't clear Tx FIFO offset."]
    #[inline(always)]
    pub fn ir_tx_fifo_offset(&self) -> IrTxFifoOffsetR {
        IrTxFifoOffsetR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - *0: Not full *1: Full"]
    #[inline(always)]
    pub fn ir_tx_fifo_full(&self) -> IrTxFifoFullR {
        IrTxFifoFullR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - *0: Not empty *1: Empty"]
    #[inline(always)]
    pub fn ir_tx_fifo_empty(&self) -> IrTxFifoEmptyR {
        IrTxFifoEmptyR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "IR Tx FIFO and interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_tx_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrTxSrSpec;
impl crate::RegisterSpec for IrTxSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir_tx_sr::R`](R) reader structure"]
impl crate::Readable for IrTxSrSpec {}
#[doc = "`reset()` method sets IR_TX_SR to value 0"]
impl crate::Resettable for IrTxSrSpec {}
