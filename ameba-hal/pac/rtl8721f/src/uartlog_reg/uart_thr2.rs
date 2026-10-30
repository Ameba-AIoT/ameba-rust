#[doc = "Register `UART_THR2` writer"]
pub type W = crate::W<UartThr2Spec>;
#[doc = "Field `tx2databit` writer - Tx data bit 0~7 Note: Bit 0 is the least significant bit. It is the first bit serially received/transmitted"]
pub type Tx2databitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Tx data bit 0~7 Note: Bit 0 is the least significant bit. It is the first bit serially received/transmitted"]
    #[inline(always)]
    pub fn tx2databit(&mut self) -> Tx2databitW<'_, UartThr2Spec> {
        Tx2databitW::new(self, 0)
    }
}
#[doc = "UART transmitter holding register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_thr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartThr2Spec;
impl crate::RegisterSpec for UartThr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_thr2::W`](W) writer structure"]
impl crate::Writable for UartThr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_THR2 to value 0"]
impl crate::Resettable for UartThr2Spec {}
