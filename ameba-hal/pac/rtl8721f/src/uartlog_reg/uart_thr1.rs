#[doc = "Register `UART_THR1` writer"]
pub type W = crate::W<UartThr1Spec>;
#[doc = "Field `tx1databit` writer - Tx data bit 0~7 Note: Bit 0 is the least significant bit. It is the first bit serially received/transmitted"]
pub type Tx1databitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Tx data bit 0~7 Note: Bit 0 is the least significant bit. It is the first bit serially received/transmitted"]
    #[inline(always)]
    pub fn tx1databit(&mut self) -> Tx1databitW<'_, UartThr1Spec> {
        Tx1databitW::new(self, 0)
    }
}
#[doc = "UART transmitter holding register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_thr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartThr1Spec;
impl crate::RegisterSpec for UartThr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_thr1::W`](W) writer structure"]
impl crate::Writable for UartThr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_THR1 to value 0"]
impl crate::Resettable for UartThr1Spec {}
