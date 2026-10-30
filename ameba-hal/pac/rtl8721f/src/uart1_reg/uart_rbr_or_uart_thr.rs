#[doc = "Register `UART_RBR_OR_UART_THR` reader"]
pub type R = crate::R<UartRbrOrUartThrSpec>;
#[doc = "Register `UART_RBR_OR_UART_THR` writer"]
pub type W = crate::W<UartRbrOrUartThrSpec>;
#[doc = "Field `DATABIT` reader - UART_RBR's Rx data bits or UART_THR's Tx data bit\\[7:0\\] Note: Bit\\[0\\] is the least significant bit. It is the first bit serially received/transmitted."]
pub type DatabitR = crate::FieldReader;
#[doc = "Field `DATABIT` writer - UART_RBR's Rx data bits or UART_THR's Tx data bit\\[7:0\\] Note: Bit\\[0\\] is the least significant bit. It is the first bit serially received/transmitted."]
pub type DatabitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - UART_RBR's Rx data bits or UART_THR's Tx data bit\\[7:0\\] Note: Bit\\[0\\] is the least significant bit. It is the first bit serially received/transmitted."]
    #[inline(always)]
    pub fn databit(&self) -> DatabitR {
        DatabitR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART_RBR's Rx data bits or UART_THR's Tx data bit\\[7:0\\] Note: Bit\\[0\\] is the least significant bit. It is the first bit serially received/transmitted."]
    #[inline(always)]
    pub fn databit(&mut self) -> DatabitW<'_, UartRbrOrUartThrSpec> {
        DatabitW::new(self, 0)
    }
}
#[doc = "UART receiver buffer register/UART transmitter holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rbr_or_uart_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_rbr_or_uart_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRbrOrUartThrSpec;
impl crate::RegisterSpec for UartRbrOrUartThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rbr_or_uart_thr::R`](R) reader structure"]
impl crate::Readable for UartRbrOrUartThrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_rbr_or_uart_thr::W`](W) writer structure"]
impl crate::Writable for UartRbrOrUartThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_RBR_OR_UART_THR to value 0"]
impl crate::Resettable for UartRbrOrUartThrSpec {}
