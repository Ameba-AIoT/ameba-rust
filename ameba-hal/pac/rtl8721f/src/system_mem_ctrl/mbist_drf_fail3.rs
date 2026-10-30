#[doc = "Register `MBIST_DRF_FAIL3` reader"]
pub type R = crate::R<MbistDrfFail3Spec>;
#[doc = "Field `mbist_drf_fail_99_96` reader - "]
pub type MbistDrfFail99_96R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mbist_drf_fail_99_96(&self) -> MbistDrfFail99_96R {
        MbistDrfFail99_96R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_drf_fail3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistDrfFail3Spec;
impl crate::RegisterSpec for MbistDrfFail3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_drf_fail3::R`](R) reader structure"]
impl crate::Readable for MbistDrfFail3Spec {}
#[doc = "`reset()` method sets MBIST_DRF_FAIL3 to value 0"]
impl crate::Resettable for MbistDrfFail3Spec {}
