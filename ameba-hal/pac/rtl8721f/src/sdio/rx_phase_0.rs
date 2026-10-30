#[doc = "Register `RX_PHASE_0` reader"]
pub type R = crate::R<RxPhase0Spec>;
#[doc = "Field `rx_phase_result_7_0` reader - rx_phase_result 0"]
pub type RxPhaseResult7_0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - rx_phase_result 0"]
    #[inline(always)]
    pub fn rx_phase_result_7_0(&self) -> RxPhaseResult7_0R {
        RxPhaseResult7_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_phase_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxPhase0Spec;
impl crate::RegisterSpec for RxPhase0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_phase_0::R`](R) reader structure"]
impl crate::Readable for RxPhase0Spec {}
#[doc = "`reset()` method sets RX_PHASE_0 to value 0"]
impl crate::Resettable for RxPhase0Spec {}
