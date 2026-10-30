#[doc = "Register `TXFLR` reader"]
pub type R = crate::R<TxflrSpec>;
#[doc = "Field `TXTFL` reader - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO."]
pub type TxtflR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn txtfl(&self) -> TxtflR {
        TxtflR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Transmit FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxflrSpec;
impl crate::RegisterSpec for TxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TxflrSpec {}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TxflrSpec {}
