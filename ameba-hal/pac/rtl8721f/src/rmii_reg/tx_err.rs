#[doc = "Register `TxErr` reader"]
pub type R = crate::R<TxErrSpec>;
#[doc = "Field `TxErr` reader - 16-bit counter of TxErr packets. Rolls over automatically. Write to clear. The source of TxErr are txunder, txabt(in half-duplex mode) or txlinkerr."]
pub type TxErrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of TxErr packets. Rolls over automatically. Write to clear. The source of TxErr are txunder, txabt(in half-duplex mode) or txlinkerr."]
    #[inline(always)]
    pub fn tx_err(&self) -> TxErrR {
        TxErrR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxErrSpec;
impl crate::RegisterSpec for TxErrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tx_err::R`](R) reader structure"]
impl crate::Readable for TxErrSpec {}
#[doc = "`reset()` method sets TxErr to value 0"]
impl crate::Resettable for TxErrSpec {}
