#[doc = "Register `RxCDO2` reader"]
pub type R = crate::R<RxCdo2Spec>;
#[doc = "Field `RxCDO2` reader - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
pub type RxCdo2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
    #[inline(always)]
    pub fn rx_cdo2(&self) -> RxCdo2R {
        RxCdo2R::new(self.bits & 0x0fff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cdo2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCdo2Spec;
impl crate::RegisterSpec for RxCdo2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_cdo2::R`](R) reader structure"]
impl crate::Readable for RxCdo2Spec {}
#[doc = "`reset()` method sets RxCDO2 to value 0"]
impl crate::Resettable for RxCdo2Spec {}
