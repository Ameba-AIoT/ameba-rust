#[doc = "Register `UART_THR0` writer"]
pub type W = crate::W<UartThr0Spec>;
#[doc = "Field `tx0databit` writer - Tx data bit 0~7 Note: Bit 0 is the least significant bit. It is the first bit serially received/transmitted"]
pub type Tx0databitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Tx data bit 0~7 Note: Bit 0 is the least significant bit. It is the first bit serially received/transmitted"]
    #[inline(always)]
    pub fn tx0databit(&mut self) -> Tx0databitW<'_, UartThr0Spec> {
        Tx0databitW::new(self, 0)
    }
}
#[doc = "UART transmitter holding register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_thr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartThr0Spec;
impl crate::RegisterSpec for UartThr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_thr0::W`](W) writer structure"]
impl crate::Writable for UartThr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_THR0 to value 0"]
impl crate::Resettable for UartThr0Spec {}
