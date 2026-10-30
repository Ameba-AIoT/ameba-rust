#[doc = "Register `UART_REG_DBG` reader"]
pub type R = crate::R<UartRegDbgSpec>;
#[doc = "Field `dbg_uart` reader - The output value of debug port, depend on dbg_sel value in SCR\\[2:0\\]"]
pub type DbgUartR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The output value of debug port, depend on dbg_sel value in SCR\\[2:0\\]"]
    #[inline(always)]
    pub fn dbg_uart(&self) -> DbgUartR {
        DbgUartR::new(self.bits)
    }
}
#[doc = "UART debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_reg_dbg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRegDbgSpec;
impl crate::RegisterSpec for UartRegDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_reg_dbg::R`](R) reader structure"]
impl crate::Readable for UartRegDbgSpec {}
#[doc = "`reset()` method sets UART_REG_DBG to value 0"]
impl crate::Resettable for UartRegDbgSpec {}
