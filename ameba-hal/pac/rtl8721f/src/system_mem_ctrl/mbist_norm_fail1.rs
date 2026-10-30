#[doc = "Register `MBIST_NORM_FAIL1` reader"]
pub type R = crate::R<MbistNormFail1Spec>;
#[doc = "Field `mbist_norm_fail_63_32` reader - "]
pub type MbistNormFail63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbist_norm_fail_63_32(&self) -> MbistNormFail63_32R {
        MbistNormFail63_32R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_norm_fail1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistNormFail1Spec;
impl crate::RegisterSpec for MbistNormFail1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_norm_fail1::R`](R) reader structure"]
impl crate::Readable for MbistNormFail1Spec {}
#[doc = "`reset()` method sets MBIST_NORM_FAIL1 to value 0"]
impl crate::Resettable for MbistNormFail1Spec {}
