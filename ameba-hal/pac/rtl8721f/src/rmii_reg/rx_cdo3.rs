#[doc = "Register `RxCDO3` reader"]
pub type R = crate::R<RxCdo3Spec>;
#[doc = "Field `RxCDO3` reader - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
pub type RxCdo3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
    #[inline(always)]
    pub fn rx_cdo3(&self) -> RxCdo3R {
        RxCdo3R::new(self.bits & 0x0fff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cdo3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCdo3Spec;
impl crate::RegisterSpec for RxCdo3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_cdo3::R`](R) reader structure"]
impl crate::Readable for RxCdo3Spec {}
#[doc = "`reset()` method sets RxCDO3 to value 0"]
impl crate::Resettable for RxCdo3Spec {}
