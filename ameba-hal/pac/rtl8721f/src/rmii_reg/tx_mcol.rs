#[doc = "Register `TxMCol` reader"]
pub type R = crate::R<TxMcolSpec>;
#[doc = "Field `TxMCol` reader - 16-bit counter of TxCol packets. Rolls over automatically. Write to clear. This keeps track of those packets with less than 16 collisions (or the configured retry count) before Tx Ok"]
pub type TxMcolR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of TxCol packets. Rolls over automatically. Write to clear. This keeps track of those packets with less than 16 collisions (or the configured retry count) before Tx Ok"]
    #[inline(always)]
    pub fn tx_mcol(&self) -> TxMcolR {
        TxMcolR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_mcol::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxMcolSpec;
impl crate::RegisterSpec for TxMcolSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tx_mcol::R`](R) reader structure"]
impl crate::Readable for TxMcolSpec {}
#[doc = "`reset()` method sets TxMCol to value 0"]
impl crate::Resettable for TxMcolSpec {}
