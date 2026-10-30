#[doc = "Register `MBIST_DRF_FAIL1` reader"]
pub type R = crate::R<MbistDrfFail1Spec>;
#[doc = "Field `mbist_drf_fail_63_32` reader - "]
pub type MbistDrfFail63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbist_drf_fail_63_32(&self) -> MbistDrfFail63_32R {
        MbistDrfFail63_32R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_drf_fail1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistDrfFail1Spec;
impl crate::RegisterSpec for MbistDrfFail1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_drf_fail1::R`](R) reader structure"]
impl crate::Readable for MbistDrfFail1Spec {}
#[doc = "`reset()` method sets MBIST_DRF_FAIL1 to value 0"]
impl crate::Resettable for MbistDrfFail1Spec {}
