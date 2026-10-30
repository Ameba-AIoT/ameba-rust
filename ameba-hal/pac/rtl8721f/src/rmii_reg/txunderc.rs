#[doc = "Register `TXUNDERC` reader"]
pub type R = crate::R<TxundercSpec>;
#[doc = "Field `TXUNDERC` reader - 16-bit counter of Tx Underrun packets. Rolls over automatically. Write to clear. gmac only."]
pub type TxundercR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of Tx Underrun packets. Rolls over automatically. Write to clear. gmac only."]
    #[inline(always)]
    pub fn txunderc(&self) -> TxundercR {
        TxundercR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`txunderc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxundercSpec;
impl crate::RegisterSpec for TxundercSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`txunderc::R`](R) reader structure"]
impl crate::Readable for TxundercSpec {}
#[doc = "`reset()` method sets TXUNDERC to value 0"]
impl crate::Resettable for TxundercSpec {}
