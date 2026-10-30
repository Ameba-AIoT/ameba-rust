#[doc = "Register `TXFLR` reader"]
pub type R = crate::R<TxflrSpec>;
#[doc = "Field `TXFLR` reader - Transmit FIFO level. Indicates the FIFO entry level of valid data in normal mode except in receiving data. (or as FIFO_FLR in any mode)"]
pub type TxflrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO level. Indicates the FIFO entry level of valid data in normal mode except in receiving data. (or as FIFO_FLR in any mode)"]
    #[inline(always)]
    pub fn txflr(&self) -> TxflrR {
        TxflrR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Transmit FIFO level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxflrSpec;
impl crate::RegisterSpec for TxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TxflrSpec {}
#[doc = "`reset()` method sets TXFLR to value 0"]
impl crate::Resettable for TxflrSpec {}
