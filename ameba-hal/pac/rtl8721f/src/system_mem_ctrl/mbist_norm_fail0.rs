#[doc = "Register `MBIST_NORM_FAIL0` reader"]
pub type R = crate::R<MbistNormFail0Spec>;
#[doc = "Field `mbist_norm_fail_31_0` reader - "]
pub type MbistNormFail31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbist_norm_fail_31_0(&self) -> MbistNormFail31_0R {
        MbistNormFail31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_norm_fail0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistNormFail0Spec;
impl crate::RegisterSpec for MbistNormFail0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_norm_fail0::R`](R) reader structure"]
impl crate::Readable for MbistNormFail0Spec {}
#[doc = "`reset()` method sets MBIST_NORM_FAIL0 to value 0"]
impl crate::Resettable for MbistNormFail0Spec {}
