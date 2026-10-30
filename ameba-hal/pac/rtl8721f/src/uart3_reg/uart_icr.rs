#[doc = "Register `UART_ICR` writer"]
pub type W = crate::W<UartIcrSpec>;
#[doc = "Field `RLSICF` writer - Receiver Line Statue Interrupt Clear Flag Writing 1 to this bit clears PARITY_ERR, OVERRUN_ERR, FRAMING_ERR, BREAK_ERR_INT and RXFIFO_ERR in the USART_LSR register."]
pub type RlsicfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOICF` writer - Timeout Interrupt Clear Flag"]
pub type ToicfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MICF` writer - Modem Interrupt Clear Flag"]
pub type MicfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDICF` writer - Monitor Done Interrupt Flag"]
pub type MdicfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNDICF` writer - Rx noise Detected Interrupt Flag"]
pub type RxndicfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETICF` writer - Rxfifo empty timeout Interrupt Clear Flag"]
pub type ReticfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receiver Line Statue Interrupt Clear Flag Writing 1 to this bit clears PARITY_ERR, OVERRUN_ERR, FRAMING_ERR, BREAK_ERR_INT and RXFIFO_ERR in the USART_LSR register."]
    #[inline(always)]
    pub fn rlsicf(&mut self) -> RlsicfW<'_, UartIcrSpec> {
        RlsicfW::new(self, 0)
    }
    #[doc = "Bit 1 - Timeout Interrupt Clear Flag"]
    #[inline(always)]
    pub fn toicf(&mut self) -> ToicfW<'_, UartIcrSpec> {
        ToicfW::new(self, 1)
    }
    #[doc = "Bit 2 - Modem Interrupt Clear Flag"]
    #[inline(always)]
    pub fn micf(&mut self) -> MicfW<'_, UartIcrSpec> {
        MicfW::new(self, 2)
    }
    #[doc = "Bit 3 - Monitor Done Interrupt Flag"]
    #[inline(always)]
    pub fn mdicf(&mut self) -> MdicfW<'_, UartIcrSpec> {
        MdicfW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx noise Detected Interrupt Flag"]
    #[inline(always)]
    pub fn rxndicf(&mut self) -> RxndicfW<'_, UartIcrSpec> {
        RxndicfW::new(self, 4)
    }
    #[doc = "Bit 5 - Rxfifo empty timeout Interrupt Clear Flag"]
    #[inline(always)]
    pub fn reticf(&mut self) -> ReticfW<'_, UartIcrSpec> {
        ReticfW::new(self, 5)
    }
}
#[doc = "UART interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIcrSpec;
impl crate::RegisterSpec for UartIcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_icr::W`](W) writer structure"]
impl crate::Writable for UartIcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_ICR to value 0"]
impl crate::Resettable for UartIcrSpec {}
