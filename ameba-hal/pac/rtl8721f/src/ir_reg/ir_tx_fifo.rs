#[doc = "Register `IR_TX_FIFO` writer"]
pub type W = crate::W<IrTxFifoSpec>;
#[doc = "Field `IR_TX_DATA_TIME` writer - Real active time = (IR_TX_DATA_TIME + 1) * IR_TX_CLK_Period"]
pub type IrTxDataTimeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `IR_TX_COMPENSATION` writer - *0x0: IR_TX_CLK_Period = Tsys_clk * IR_CLK_DIV *0x1: IR_TX_CLK_Period = (1 + 1/2) Tsys_clk * IR_CLK_DIV *0x2: IR_TX_CLK_Period = (1 + 1/4) Tsys_clk * IR_CLK_DIV *0x3: IR_TX_CLK_Period = Tsys_clk * (IR_TX_COMPE_DIV + 1)"]
pub type IrTxCompensationW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IR_TX_DATA_END_FLAG` writer - *0: Normal packet *1: Last packet"]
pub type IrTxDataEndFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_TX_DATA_TYPE` writer - Data type *0: Inactive carrier (no carrier) *1: Active carrier (carrier)"]
pub type IrTxDataTypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:27 - Real active time = (IR_TX_DATA_TIME + 1) * IR_TX_CLK_Period"]
    #[inline(always)]
    pub fn ir_tx_data_time(&mut self) -> IrTxDataTimeW<'_, IrTxFifoSpec> {
        IrTxDataTimeW::new(self, 0)
    }
    #[doc = "Bits 28:29 - *0x0: IR_TX_CLK_Period = Tsys_clk * IR_CLK_DIV *0x1: IR_TX_CLK_Period = (1 + 1/2) Tsys_clk * IR_CLK_DIV *0x2: IR_TX_CLK_Period = (1 + 1/4) Tsys_clk * IR_CLK_DIV *0x3: IR_TX_CLK_Period = Tsys_clk * (IR_TX_COMPE_DIV + 1)"]
    #[inline(always)]
    pub fn ir_tx_compensation(&mut self) -> IrTxCompensationW<'_, IrTxFifoSpec> {
        IrTxCompensationW::new(self, 28)
    }
    #[doc = "Bit 30 - *0: Normal packet *1: Last packet"]
    #[inline(always)]
    pub fn ir_tx_data_end_flag(&mut self) -> IrTxDataEndFlagW<'_, IrTxFifoSpec> {
        IrTxDataEndFlagW::new(self, 30)
    }
    #[doc = "Bit 31 - Data type *0: Inactive carrier (no carrier) *1: Active carrier (carrier)"]
    #[inline(always)]
    pub fn ir_tx_data_type(&mut self) -> IrTxDataTypeW<'_, IrTxFifoSpec> {
        IrTxDataTypeW::new(self, 31)
    }
}
#[doc = "IR Tx FIFO register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_tx_fifo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrTxFifoSpec;
impl crate::RegisterSpec for IrTxFifoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ir_tx_fifo::W`](W) writer structure"]
impl crate::Writable for IrTxFifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR_TX_FIFO to value 0"]
impl crate::Resettable for IrTxFifoSpec {}
