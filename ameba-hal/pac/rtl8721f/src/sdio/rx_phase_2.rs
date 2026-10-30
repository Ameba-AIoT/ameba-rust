#[doc = "Register `RX_PHASE_2` reader"]
pub type R = crate::R<RxPhase2Spec>;
#[doc = "Field `rx_phase_result_23_16` reader - rx_phase_result 2"]
pub type RxPhaseResult23_16R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - rx_phase_result 2"]
    #[inline(always)]
    pub fn rx_phase_result_23_16(&self) -> RxPhaseResult23_16R {
        RxPhaseResult23_16R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_phase_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxPhase2Spec;
impl crate::RegisterSpec for RxPhase2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_phase_2::R`](R) reader structure"]
impl crate::Readable for RxPhase2Spec {}
#[doc = "`reset()` method sets RX_PHASE_2 to value 0"]
impl crate::Resettable for RxPhase2Spec {}
