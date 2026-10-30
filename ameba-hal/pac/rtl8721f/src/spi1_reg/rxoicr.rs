#[doc = "Register `RXOICR` reader"]
pub type R = crate::R<RxoicrSpec>;
#[doc = "Field `RXOICR` reader - Clear Receive FIFO Overflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_rxo_intr interrupt; writing has no effect."]
pub type RxoicrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear Receive FIFO Overflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_rxo_intr interrupt; writing has no effect."]
    #[inline(always)]
    pub fn rxoicr(&self) -> RxoicrR {
        RxoicrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Receive FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoicr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxoicrSpec;
impl crate::RegisterSpec for RxoicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoicr::R`](R) reader structure"]
impl crate::Readable for RxoicrSpec {}
#[doc = "`reset()` method sets RXOICR to value 0"]
impl crate::Resettable for RxoicrSpec {}
