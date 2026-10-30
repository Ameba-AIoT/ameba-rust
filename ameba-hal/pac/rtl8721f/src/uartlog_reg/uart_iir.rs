#[doc = "Register `UART_IIR` reader"]
pub type R = crate::R<UartIirSpec>;
#[doc = "Field `int_pend` reader - Indicate if the interrupt is pending. *0: Pending, and the IIR contents may be used as a pointer to the appropriate interrupt service routine. *1: No interrupt is pending."]
pub type IntPendR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicate if the interrupt is pending. *0: Pending, and the IIR contents may be used as a pointer to the appropriate interrupt service routine. *1: No interrupt is pending."]
    #[inline(always)]
    pub fn int_pend(&self) -> IntPendR {
        IntPendR::new((self.bits & 1) != 0)
    }
}
#[doc = "UART interrupt identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIirSpec;
impl crate::RegisterSpec for UartIirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_iir::R`](R) reader structure"]
impl crate::Readable for UartIirSpec {}
#[doc = "`reset()` method sets UART_IIR to value 0x01"]
impl crate::Resettable for UartIirSpec {
    const RESET_VALUE: u32 = 0x01;
}
