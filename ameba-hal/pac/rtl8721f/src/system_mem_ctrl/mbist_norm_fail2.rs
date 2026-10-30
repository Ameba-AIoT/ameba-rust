#[doc = "Register `MBIST_NORM_FAIL2` reader"]
pub type R = crate::R<MbistNormFail2Spec>;
#[doc = "Field `mbist_norm_fail_95_64` reader - "]
pub type MbistNormFail95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbist_norm_fail_95_64(&self) -> MbistNormFail95_64R {
        MbistNormFail95_64R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_norm_fail2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistNormFail2Spec;
impl crate::RegisterSpec for MbistNormFail2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_norm_fail2::R`](R) reader structure"]
impl crate::Readable for MbistNormFail2Spec {}
#[doc = "`reset()` method sets MBIST_NORM_FAIL2 to value 0"]
impl crate::Resettable for MbistNormFail2Spec {}
