#[doc = "Register `UART_RX_BYTE_CNT` reader"]
pub type R = crate::R<UartRxByteCntSpec>;
#[doc = "Register `UART_RX_BYTE_CNT` writer"]
pub type W = crate::W<UartRxByteCntSpec>;
#[doc = "Field `RO_RX_BYTE_CNT` reader - Count the byte number of data read from rxfifo"]
pub type RoRxByteCntR = crate::FieldReader<u16>;
#[doc = "Field `CLR_RX_BYTE_CNT` writer - Write 1 to clear ro_rx_byte_cnt"]
pub type ClrRxByteCntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Count the byte number of data read from rxfifo"]
    #[inline(always)]
    pub fn ro_rx_byte_cnt(&self) -> RoRxByteCntR {
        RoRxByteCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - Write 1 to clear ro_rx_byte_cnt"]
    #[inline(always)]
    pub fn clr_rx_byte_cnt(&mut self) -> ClrRxByteCntW<'_, UartRxByteCntSpec> {
        ClrRxByteCntW::new(self, 16)
    }
}
#[doc = "UART Rx data byte count register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rx_byte_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_rx_byte_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRxByteCntSpec;
impl crate::RegisterSpec for UartRxByteCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rx_byte_cnt::R`](R) reader structure"]
impl crate::Readable for UartRxByteCntSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_rx_byte_cnt::W`](W) writer structure"]
impl crate::Writable for UartRxByteCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_RX_BYTE_CNT to value 0"]
impl crate::Resettable for UartRxByteCntSpec {}
