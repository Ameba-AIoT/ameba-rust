#[doc = "Register `IC_TXFLR` reader"]
pub type R = crate::R<IcTxflrSpec>;
#[doc = "Field `TXFLR` reader - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO."]
pub type TxflrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn txflr(&self) -> TxflrR {
        TxflrR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "I2C Transmit FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_txflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcTxflrSpec;
impl crate::RegisterSpec for IcTxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_txflr::R`](R) reader structure"]
impl crate::Readable for IcTxflrSpec {}
#[doc = "`reset()` method sets IC_TXFLR to value 0"]
impl crate::Resettable for IcTxflrSpec {}
