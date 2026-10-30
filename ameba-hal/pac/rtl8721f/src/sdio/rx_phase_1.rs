#[doc = "Register `RX_PHASE_1` reader"]
pub type R = crate::R<RxPhase1Spec>;
#[doc = "Field `rx_phase_result_15_8` reader - rx_phase_result 1"]
pub type RxPhaseResult15_8R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - rx_phase_result 1"]
    #[inline(always)]
    pub fn rx_phase_result_15_8(&self) -> RxPhaseResult15_8R {
        RxPhaseResult15_8R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_phase_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxPhase1Spec;
impl crate::RegisterSpec for RxPhase1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_phase_1::R`](R) reader structure"]
impl crate::Readable for RxPhase1Spec {}
#[doc = "`reset()` method sets RX_PHASE_1 to value 0"]
impl crate::Resettable for RxPhase1Spec {}
