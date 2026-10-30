#[doc = "Register `TXOICR` reader"]
pub type R = crate::R<TxoicrSpec>;
#[doc = "Field `TXOICR` reader - Clear Transmit FIFO Overflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_txo_intr interrupt; writing has no effect."]
pub type TxoicrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear Transmit FIFO Overflow Interrupt. This register reflects the status of the interrupt. A read from this register clears the ssi_txo_intr interrupt; writing has no effect."]
    #[inline(always)]
    pub fn txoicr(&self) -> TxoicrR {
        TxoicrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Transmit FIFO Overflow Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txoicr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxoicrSpec;
impl crate::RegisterSpec for TxoicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoicr::R`](R) reader structure"]
impl crate::Readable for TxoicrSpec {}
#[doc = "`reset()` method sets TXOICR to value 0"]
impl crate::Resettable for TxoicrSpec {}
