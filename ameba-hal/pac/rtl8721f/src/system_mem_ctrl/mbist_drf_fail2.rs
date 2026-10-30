#[doc = "Register `MBIST_DRF_FAIL2` reader"]
pub type R = crate::R<MbistDrfFail2Spec>;
#[doc = "Field `mbist_drf_fail_95_64` reader - "]
pub type MbistDrfFail95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbist_drf_fail_95_64(&self) -> MbistDrfFail95_64R {
        MbistDrfFail95_64R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_drf_fail2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistDrfFail2Spec;
impl crate::RegisterSpec for MbistDrfFail2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_drf_fail2::R`](R) reader structure"]
impl crate::Readable for MbistDrfFail2Spec {}
#[doc = "`reset()` method sets MBIST_DRF_FAIL2 to value 0"]
impl crate::Resettable for MbistDrfFail2Spec {}
