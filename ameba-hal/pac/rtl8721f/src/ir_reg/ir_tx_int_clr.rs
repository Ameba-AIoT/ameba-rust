#[doc = "Register `IR_TX_INT_CLR` writer"]
pub type W = crate::W<IrTxIntClrSpec>;
#[doc = "Field `IR_TX_FIFO_CLR` writer - Write 1 to clear Tx FIFO"]
pub type IrTxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_EMPTY_INT_CLR` writer - Tx FIFO empty interrupt Write 1 to clear"]
pub type IrTxFifoEmptyIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_LEVEL_INT_CLR` writer - When Tx FIFO offset <= threshold value, interrupt is triggered. Write 1 to clear"]
pub type IrTxFifoLevelIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_FIFO_OVER_INT_CLR` writer - Tx FIFO overflow interrupt Write 1 to clear"]
pub type IrTxFifoOverIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to clear Tx FIFO"]
    #[inline(always)]
    pub fn ir_tx_fifo_clr(&mut self) -> IrTxFifoClrW<'_, IrTxIntClrSpec> {
        IrTxFifoClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx FIFO empty interrupt Write 1 to clear"]
    #[inline(always)]
    pub fn ir_tx_fifo_empty_int_clr(&mut self) -> IrTxFifoEmptyIntClrW<'_, IrTxIntClrSpec> {
        IrTxFifoEmptyIntClrW::new(self, 1)
    }
    #[doc = "Bit 2 - When Tx FIFO offset <= threshold value, interrupt is triggered. Write 1 to clear"]
    #[inline(always)]
    pub fn ir_tx_fifo_level_int_clr(&mut self) -> IrTxFifoLevelIntClrW<'_, IrTxIntClrSpec> {
        IrTxFifoLevelIntClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Tx FIFO overflow interrupt Write 1 to clear"]
    #[inline(always)]
    pub fn ir_tx_fifo_over_int_clr(&mut self) -> IrTxFifoOverIntClrW<'_, IrTxIntClrSpec> {
        IrTxFifoOverIntClrW::new(self, 3)
    }
}
#[doc = "IR Tx FIFO and interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_tx_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrTxIntClrSpec;
impl crate::RegisterSpec for IrTxIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ir_tx_int_clr::W`](W) writer structure"]
impl crate::Writable for IrTxIntClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_TX_INT_CLR to value 0"]
impl crate::Resettable for IrTxIntClrSpec {}
