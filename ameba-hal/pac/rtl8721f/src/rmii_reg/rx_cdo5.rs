#[doc = "Register `RxCDO5` reader"]
pub type R = crate::R<RxCdo5Spec>;
#[doc = "Field `RxCDO5` reader - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
pub type RxCdo5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
    #[inline(always)]
    pub fn rx_cdo5(&self) -> RxCdo5R {
        RxCdo5R::new(self.bits & 0x0fff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cdo5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCdo5Spec;
impl crate::RegisterSpec for RxCdo5Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_cdo5::R`](R) reader structure"]
impl crate::Readable for RxCdo5Spec {}
#[doc = "`reset()` method sets RxCDO5 to value 0"]
impl crate::Resettable for RxCdo5Spec {}
