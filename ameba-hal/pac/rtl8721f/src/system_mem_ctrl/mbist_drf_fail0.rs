#[doc = "Register `MBIST_DRF_FAIL0` reader"]
pub type R = crate::R<MbistDrfFail0Spec>;
#[doc = "Field `mbist_drf_fail_31_0` reader - "]
pub type MbistDrfFail31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbist_drf_fail_31_0(&self) -> MbistDrfFail31_0R {
        MbistDrfFail31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_drf_fail0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistDrfFail0Spec;
impl crate::RegisterSpec for MbistDrfFail0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_drf_fail0::R`](R) reader structure"]
impl crate::Readable for MbistDrfFail0Spec {}
#[doc = "`reset()` method sets MBIST_DRF_FAIL0 to value 0"]
impl crate::Resettable for MbistDrfFail0Spec {}
