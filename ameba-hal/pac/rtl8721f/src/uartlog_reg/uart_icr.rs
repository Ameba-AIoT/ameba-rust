#[doc = "Register `UART_ICR` writer"]
pub type W = crate::W<UartIcrSpec>;
#[doc = "Field `RLSICF` writer - Receiver Line Statue Interrupt Clear Flag Writing 1 to this bit clears PARITY_ERR, OVERRUN_ERR, FRAMING_ERR, BREAK_ERR_INT and RXFIFO_ERR in the USART_LSR register."]
pub type RlsicfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOICF` writer - Timeout Interrupt Clear Flag"]
pub type ToicfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDICF` writer - Monitor done clear flag"]
pub type MdicfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPRLSICF` writer - Relay Path Receiver Line Statue Interrupt Clear Flag Writing 1 to this bit clears relay path's PARITY_ERR, OVERRUN_ERR, FRAMING_ERR and RXFIFO_ERR in the USART_LSR register."]
pub type RprlsicfW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Monitor done clear flag"]
    #[inline(always)]
    pub fn mdicf(&mut self) -> MdicfW<'_, UartIcrSpec> {
        MdicfW::new(self, 2)
    }
    #[doc = "Bit 3 - Relay Path Receiver Line Statue Interrupt Clear Flag Writing 1 to this bit clears relay path's PARITY_ERR, OVERRUN_ERR, FRAMING_ERR and RXFIFO_ERR in the USART_LSR register."]
    #[inline(always)]
    pub fn rprlsicf(&mut self) -> RprlsicfW<'_, UartIcrSpec> {
        RprlsicfW::new(self, 3)
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
