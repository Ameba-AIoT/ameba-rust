#[doc = "Register `MBIST_NORM_FAIL3` reader"]
pub type R = crate::R<MbistNormFail3Spec>;
#[doc = "Field `mbist_norm_fail_99_96` reader - "]
pub type MbistNormFail99_96R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mbist_norm_fail_99_96(&self) -> MbistNormFail99_96R {
        MbistNormFail99_96R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_norm_fail3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistNormFail3Spec;
impl crate::RegisterSpec for MbistNormFail3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_norm_fail3::R`](R) reader structure"]
impl crate::Readable for MbistNormFail3Spec {}
#[doc = "`reset()` method sets MBIST_NORM_FAIL3 to value 0"]
impl crate::Resettable for MbistNormFail3Spec {}
