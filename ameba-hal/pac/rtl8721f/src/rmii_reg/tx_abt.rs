#[doc = "Register `TxAbt` reader"]
pub type R = crate::R<TxAbtSpec>;
#[doc = "Field `TxAbt` reader - 16-bit counter of Tx aborted packets. Rolls over automatically. Write to clear. This accounts for over collision, underrun, LNK failure conditions."]
pub type TxAbtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of Tx aborted packets. Rolls over automatically. Write to clear. This accounts for over collision, underrun, LNK failure conditions."]
    #[inline(always)]
    pub fn tx_abt(&self) -> TxAbtR {
        TxAbtR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_abt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxAbtSpec;
impl crate::RegisterSpec for TxAbtSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tx_abt::R`](R) reader structure"]
impl crate::Readable for TxAbtSpec {}
#[doc = "`reset()` method sets TxAbt to value 0"]
impl crate::Resettable for TxAbtSpec {}
