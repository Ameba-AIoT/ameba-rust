#[doc = "Register `UART_DBGR` reader"]
pub type R = crate::R<UartDbgrSpec>;
#[doc = "Field `DBG_UART` reader - The output value of debug port, depending on dbg_sel value in SCR\\[2:0\\]"]
pub type DbgUartR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The output value of debug port, depending on dbg_sel value in SCR\\[2:0\\]"]
    #[inline(always)]
    pub fn dbg_uart(&self) -> DbgUartR {
        DbgUartR::new(self.bits)
    }
}
#[doc = "UART debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_dbgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartDbgrSpec;
impl crate::RegisterSpec for UartDbgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_dbgr::R`](R) reader structure"]
impl crate::Readable for UartDbgrSpec {}
#[doc = "`reset()` method sets UART_DBGR to value 0"]
impl crate::Resettable for UartDbgrSpec {}
