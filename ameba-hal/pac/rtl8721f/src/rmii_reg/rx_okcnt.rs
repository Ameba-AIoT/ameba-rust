#[doc = "Register `RxOKCNT` reader"]
pub type R = crate::R<RxOkcntSpec>;
#[doc = "Field `RxOk` reader - 16-bit counter of Rx Ok packets. Rolls over automatically. Write to clear."]
pub type RxOkR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of Rx Ok packets. Rolls over automatically. Write to clear."]
    #[inline(always)]
    pub fn rx_ok(&self) -> RxOkR {
        RxOkR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_okcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOkcntSpec;
impl crate::RegisterSpec for RxOkcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_okcnt::R`](R) reader structure"]
impl crate::Readable for RxOkcntSpec {}
#[doc = "`reset()` method sets RxOKCNT to value 0"]
impl crate::Resettable for RxOkcntSpec {}
