#[doc = "Register `Rx_CDO1` reader"]
pub type R = crate::R<RxCdo1Spec>;
#[doc = "Field `RxCDO1` reader - Rx Current Descriptor Offset of 1st queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)."]
pub type RxCdo1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Rx Current Descriptor Offset of 1st queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)."]
    #[inline(always)]
    pub fn rx_cdo1(&self) -> RxCdo1R {
        RxCdo1R::new(self.bits & 0x0fff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cdo1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCdo1Spec;
impl crate::RegisterSpec for RxCdo1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_cdo1::R`](R) reader structure"]
impl crate::Readable for RxCdo1Spec {}
#[doc = "`reset()` method sets Rx_CDO1 to value 0"]
impl crate::Resettable for RxCdo1Spec {}
