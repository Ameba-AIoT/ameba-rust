#[doc = "Register `RXUICR` reader"]
pub type R = crate::R<RxuicrSpec>;
#[doc = "Field `RXUICR` reader - Clear Receive FIFO Underflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_rxu_intr interrupt; writing has no effect."]
pub type RxuicrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear Receive FIFO Underflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_rxu_intr interrupt; writing has no effect."]
    #[inline(always)]
    pub fn rxuicr(&self) -> RxuicrR {
        RxuicrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Receive FIFO Underflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxuicr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxuicrSpec;
impl crate::RegisterSpec for RxuicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxuicr::R`](R) reader structure"]
impl crate::Readable for RxuicrSpec {}
#[doc = "`reset()` method sets RXUICR to value 0"]
impl crate::Resettable for RxuicrSpec {}
