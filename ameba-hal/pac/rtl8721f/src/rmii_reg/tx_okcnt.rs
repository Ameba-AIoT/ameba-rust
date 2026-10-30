#[doc = "Register `TxOKCNT` reader"]
pub type R = crate::R<TxOkcntSpec>;
#[doc = "Field `TxOK` reader - 16-bit counter of Tx Ok packets. Rolls over automatically. Write to clear."]
pub type TxOkR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of Tx Ok packets. Rolls over automatically. Write to clear."]
    #[inline(always)]
    pub fn tx_ok(&self) -> TxOkR {
        TxOkR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_okcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxOkcntSpec;
impl crate::RegisterSpec for TxOkcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tx_okcnt::R`](R) reader structure"]
impl crate::Readable for TxOkcntSpec {}
#[doc = "`reset()` method sets TxOKCNT to value 0"]
impl crate::Resettable for TxOkcntSpec {}
