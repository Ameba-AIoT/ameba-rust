#[doc = "Register `IC_RXFLR` reader"]
pub type R = crate::R<IcRxflrSpec>;
#[doc = "Field `RXFLR` reader - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO."]
pub type RxflrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxflr(&self) -> RxflrR {
        RxflrR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "I2C Receive FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_rxflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcRxflrSpec;
impl crate::RegisterSpec for IcRxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_rxflr::R`](R) reader structure"]
impl crate::Readable for IcRxflrSpec {}
#[doc = "`reset()` method sets IC_RXFLR to value 0"]
impl crate::Resettable for IcRxflrSpec {}
