#[doc = "Register `RX_PHASE_3` reader"]
pub type R = crate::R<RxPhase3Spec>;
#[doc = "Field `rx_phase_result_31_24` reader - rx_phase_result 3"]
pub type RxPhaseResult31_24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - rx_phase_result 3"]
    #[inline(always)]
    pub fn rx_phase_result_31_24(&self) -> RxPhaseResult31_24R {
        RxPhaseResult31_24R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_phase_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxPhase3Spec;
impl crate::RegisterSpec for RxPhase3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_phase_3::R`](R) reader structure"]
impl crate::Readable for RxPhase3Spec {}
#[doc = "`reset()` method sets RX_PHASE_3 to value 0"]
impl crate::Resettable for RxPhase3Spec {}
