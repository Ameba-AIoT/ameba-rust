#[doc = "Register `MBIST_START_PULSE0` reader"]
pub type R = crate::R<MbistStartPulse0Spec>;
#[doc = "Field `mbist_start_pause_31_0` reader - "]
pub type MbistStartPause31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mbist_start_pause_31_0(&self) -> MbistStartPause31_0R {
        MbistStartPause31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_start_pulse0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistStartPulse0Spec;
impl crate::RegisterSpec for MbistStartPulse0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_start_pulse0::R`](R) reader structure"]
impl crate::Readable for MbistStartPulse0Spec {}
#[doc = "`reset()` method sets MBIST_START_PULSE0 to value 0"]
impl crate::Resettable for MbistStartPulse0Spec {}
