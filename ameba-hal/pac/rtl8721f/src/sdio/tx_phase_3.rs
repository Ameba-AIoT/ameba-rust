#[doc = "Register `TX_PHASE_3` reader"]
pub type R = crate::R<TxPhase3Spec>;
#[doc = "Field `tx_phase_result_31_24` reader - tx_phase_result 3"]
pub type TxPhaseResult31_24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - tx_phase_result 3"]
    #[inline(always)]
    pub fn tx_phase_result_31_24(&self) -> TxPhaseResult31_24R {
        TxPhaseResult31_24R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_phase_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxPhase3Spec;
impl crate::RegisterSpec for TxPhase3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_phase_3::R`](R) reader structure"]
impl crate::Readable for TxPhase3Spec {}
#[doc = "`reset()` method sets TX_PHASE_3 to value 0"]
impl crate::Resettable for TxPhase3Spec {}
