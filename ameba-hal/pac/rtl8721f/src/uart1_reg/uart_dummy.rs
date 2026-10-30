#[doc = "Register `UART_DUMMY` reader"]
pub type R = crate::R<UartDummySpec>;
#[doc = "Field `dummy` reader - rsvd for hw"]
pub type DummyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - rsvd for hw"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "UART Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_dummy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartDummySpec;
impl crate::RegisterSpec for UartDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_dummy::R`](R) reader structure"]
impl crate::Readable for UartDummySpec {}
#[doc = "`reset()` method sets UART_DUMMY to value 0"]
impl crate::Resettable for UartDummySpec {}
