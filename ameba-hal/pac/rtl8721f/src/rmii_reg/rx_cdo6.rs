#[doc = "Register `RxCDO6` reader"]
pub type R = crate::R<RxCdo6Spec>;
#[doc = "Field `RxCDO6` reader - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
pub type RxCdo6R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
    #[inline(always)]
    pub fn rx_cdo6(&self) -> RxCdo6R {
        RxCdo6R::new(self.bits & 0x0fff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cdo6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCdo6Spec;
impl crate::RegisterSpec for RxCdo6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_cdo6::R`](R) reader structure"]
impl crate::Readable for RxCdo6Spec {}
#[doc = "`reset()` method sets RxCDO6 to value 0"]
impl crate::Resettable for RxCdo6Spec {}
