#[doc = "Register `UART_THR3` writer"]
pub type W = crate::W<UartThr3Spec>;
#[doc = "Field `tx3databit` writer - Tx data bit 0~7 Note: Bit 0 is the least significant bit. It is the first bit serially received/transmitted"]
pub type Tx3databitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Tx data bit 0~7 Note: Bit 0 is the least significant bit. It is the first bit serially received/transmitted"]
    #[inline(always)]
    pub fn tx3databit(&mut self) -> Tx3databitW<'_, UartThr3Spec> {
        Tx3databitW::new(self, 0)
    }
}
#[doc = "UART transmitter holding register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_thr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartThr3Spec;
impl crate::RegisterSpec for UartThr3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_thr3::W`](W) writer structure"]
impl crate::Writable for UartThr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_THR3 to value 0"]
impl crate::Resettable for UartThr3Spec {}
