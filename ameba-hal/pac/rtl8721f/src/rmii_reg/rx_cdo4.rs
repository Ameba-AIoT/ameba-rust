#[doc = "Register `RxCDO4` reader"]
pub type R = crate::R<RxCdo4Spec>;
#[doc = "Field `RxCDO4` reader - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
pub type RxCdo4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Rx Current Descriptor Offset of queue: RxFDP+RxCDO = current descriptor pointer. CDO increments by 16 each time (each increment is one byte)"]
    #[inline(always)]
    pub fn rx_cdo4(&self) -> RxCdo4R {
        RxCdo4R::new(self.bits & 0x0fff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cdo4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCdo4Spec;
impl crate::RegisterSpec for RxCdo4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rx_cdo4::R`](R) reader structure"]
impl crate::Readable for RxCdo4Spec {}
#[doc = "`reset()` method sets RxCDO4 to value 0"]
impl crate::Resettable for RxCdo4Spec {}
