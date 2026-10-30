#[doc = "Register `UART_RBR` reader"]
pub type R = crate::R<UartRbrSpec>;
#[doc = "Field `rxdatabit` reader - Tx data bit 0~7 Note: Bit\\[0\\] is the least significant bit. It is the first bit serially received/transmitted."]
pub type RxdatabitR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Tx data bit 0~7 Note: Bit\\[0\\] is the least significant bit. It is the first bit serially received/transmitted."]
    #[inline(always)]
    pub fn rxdatabit(&self) -> RxdatabitR {
        RxdatabitR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART receiver buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRbrSpec;
impl crate::RegisterSpec for UartRbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rbr::R`](R) reader structure"]
impl crate::Readable for UartRbrSpec {}
#[doc = "`reset()` method sets UART_RBR to value 0"]
impl crate::Resettable for UartRbrSpec {}
