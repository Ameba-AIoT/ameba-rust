#[doc = "Register `TX_PHASE_0` reader"]
pub type R = crate::R<TxPhase0Spec>;
#[doc = "Field `tx_phase_result_7_0` reader - tx_phase_result 0"]
pub type TxPhaseResult7_0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - tx_phase_result 0"]
    #[inline(always)]
    pub fn tx_phase_result_7_0(&self) -> TxPhaseResult7_0R {
        TxPhaseResult7_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_phase_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxPhase0Spec;
impl crate::RegisterSpec for TxPhase0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_phase_0::R`](R) reader structure"]
impl crate::Readable for TxPhase0Spec {}
#[doc = "`reset()` method sets TX_PHASE_0 to value 0"]
impl crate::Resettable for TxPhase0Spec {}
