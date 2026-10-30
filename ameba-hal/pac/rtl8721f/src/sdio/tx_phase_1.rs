#[doc = "Register `TX_PHASE_1` reader"]
pub type R = crate::R<TxPhase1Spec>;
#[doc = "Field `tx_phase_result_15_8` reader - tx_phase_result 1"]
pub type TxPhaseResult15_8R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - tx_phase_result 1"]
    #[inline(always)]
    pub fn tx_phase_result_15_8(&self) -> TxPhaseResult15_8R {
        TxPhaseResult15_8R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_phase_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxPhase1Spec;
impl crate::RegisterSpec for TxPhase1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_phase_1::R`](R) reader structure"]
impl crate::Readable for TxPhase1Spec {}
#[doc = "`reset()` method sets TX_PHASE_1 to value 0"]
impl crate::Resettable for TxPhase1Spec {}
