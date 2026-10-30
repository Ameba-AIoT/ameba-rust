#[doc = "Register `RXFLR` reader"]
pub type R = crate::R<RxflrSpec>;
#[doc = "Field `RXTFL` reader - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO."]
pub type RxtflR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Receive FIFO Level. Contains the number of valid data entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxtfl(&self) -> RxtflR {
        RxtflR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Receive FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxflrSpec;
impl crate::RegisterSpec for RxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RxflrSpec {}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RxflrSpec {}
