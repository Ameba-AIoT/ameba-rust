#[doc = "Register `RXFLR` reader"]
pub type R = crate::R<RxflrSpec>;
#[doc = "Field `RXFLR` reader - Receive FIFO level. Indicates the FIFO entries of valid data in receiving data."]
pub type RxflrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Receive FIFO level. Indicates the FIFO entries of valid data in receiving data."]
    #[inline(always)]
    pub fn rxflr(&self) -> RxflrR {
        RxflrR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Receive FIFO level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxflrSpec;
impl crate::RegisterSpec for RxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RxflrSpec {}
#[doc = "`reset()` method sets RXFLR to value 0"]
impl crate::Resettable for RxflrSpec {}
