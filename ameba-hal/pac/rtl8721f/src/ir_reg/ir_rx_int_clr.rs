#[doc = "Register `IR_RX_INT_CLR` writer"]
pub type W = crate::W<IrRxIntClrSpec>;
#[doc = "Field `IR_RX_FIFO_FULL_INT_CLR` writer - Rx FIFO full interrupt Write 1 to clear"]
pub type IrRxFifoFullIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_LEVEL_INT_CLR` writer - Rx FIFO level interrupt Write 1 to clear"]
pub type IrRxFifoLevelIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_CNT_OF_INT_CLR` writer - Rx counter overflow interrupt Write 1 to clear"]
pub type IrRxCntOfIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_OF_INT_CLR` writer - Rx FIFO overflow interrupt Write 1 to clear"]
pub type IrRxFifoOfIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_CNT_THR_INT_CLR` writer - Rx count threshold interrupt Write 1 to clear"]
pub type IrRxCntThrIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_ERROR_INT_CLR` writer - Rx FIFO error read interrupt Write 1 to clear"]
pub type IrRxFifoErrorIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_RX_FIFO_CLR` writer - Write 1 to clear Rx FIFO"]
pub type IrRxFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Rx FIFO full interrupt Write 1 to clear"]
    #[inline(always)]
    pub fn ir_rx_fifo_full_int_clr(&mut self) -> IrRxFifoFullIntClrW<'_, IrRxIntClrSpec> {
        IrRxFifoFullIntClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO level interrupt Write 1 to clear"]
    #[inline(always)]
    pub fn ir_rx_fifo_level_int_clr(&mut self) -> IrRxFifoLevelIntClrW<'_, IrRxIntClrSpec> {
        IrRxFifoLevelIntClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx counter overflow interrupt Write 1 to clear"]
    #[inline(always)]
    pub fn ir_rx_cnt_of_int_clr(&mut self) -> IrRxCntOfIntClrW<'_, IrRxIntClrSpec> {
        IrRxCntOfIntClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO overflow interrupt Write 1 to clear"]
    #[inline(always)]
    pub fn ir_rx_fifo_of_int_clr(&mut self) -> IrRxFifoOfIntClrW<'_, IrRxIntClrSpec> {
        IrRxFifoOfIntClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx count threshold interrupt Write 1 to clear"]
    #[inline(always)]
    pub fn ir_rx_cnt_thr_int_clr(&mut self) -> IrRxCntThrIntClrW<'_, IrRxIntClrSpec> {
        IrRxCntThrIntClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO error read interrupt Write 1 to clear"]
    #[inline(always)]
    pub fn ir_rx_fifo_error_int_clr(&mut self) -> IrRxFifoErrorIntClrW<'_, IrRxIntClrSpec> {
        IrRxFifoErrorIntClrW::new(self, 5)
    }
    #[doc = "Bit 8 - Write 1 to clear Rx FIFO"]
    #[inline(always)]
    pub fn ir_rx_fifo_clr(&mut self) -> IrRxFifoClrW<'_, IrRxIntClrSpec> {
        IrRxFifoClrW::new(self, 8)
    }
}
#[doc = "IR Rx FIFO and interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_rx_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrRxIntClrSpec;
impl crate::RegisterSpec for IrRxIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ir_rx_int_clr::W`](W) writer structure"]
impl crate::Writable for IrRxIntClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_RX_INT_CLR to value 0"]
impl crate::Resettable for IrRxIntClrSpec {}
