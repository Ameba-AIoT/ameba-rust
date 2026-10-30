#[doc = "Register `UART_RXDBCR` reader"]
pub type R = crate::R<UartRxdbcrSpec>;
#[doc = "Register `UART_RXDBCR` writer"]
pub type W = crate::W<UartRxdbcrSpec>;
#[doc = "Field `DBNC_FEN` reader - UART RX debounce enable"]
pub type DbncFenR = crate::BitReader;
#[doc = "Field `DBNC_FEN` writer - UART RX debounce enable"]
pub type DbncFenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNC_CYC` reader - Debounce cycles in unit of sclk"]
pub type DbncCycR = crate::FieldReader<u16>;
#[doc = "Field `DBNC_CYC` writer - Debounce cycles in unit of sclk"]
pub type DbncCycW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - UART RX debounce enable"]
    #[inline(always)]
    pub fn dbnc_fen(&self) -> DbncFenR {
        DbncFenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - Debounce cycles in unit of sclk"]
    #[inline(always)]
    pub fn dbnc_cyc(&self) -> DbncCycR {
        DbncCycR::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - UART RX debounce enable"]
    #[inline(always)]
    pub fn dbnc_fen(&mut self) -> DbncFenW<'_, UartRxdbcrSpec> {
        DbncFenW::new(self, 0)
    }
    #[doc = "Bits 1:15 - Debounce cycles in unit of sclk"]
    #[inline(always)]
    pub fn dbnc_cyc(&mut self) -> DbncCycW<'_, UartRxdbcrSpec> {
        DbncCycW::new(self, 1)
    }
}
#[doc = "UART RX debounce control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rxdbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_rxdbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRxdbcrSpec;
impl crate::RegisterSpec for UartRxdbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rxdbcr::R`](R) reader structure"]
impl crate::Readable for UartRxdbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_rxdbcr::W`](W) writer structure"]
impl crate::Writable for UartRxdbcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_RXDBCR to value 0x1e"]
impl crate::Resettable for UartRxdbcrSpec {
    const RESET_VALUE: u32 = 0x1e;
}
