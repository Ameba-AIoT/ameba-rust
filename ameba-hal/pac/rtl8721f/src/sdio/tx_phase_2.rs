#[doc = "Register `TX_PHASE_2` reader"]
pub type R = crate::R<TxPhase2Spec>;
#[doc = "Field `tx_phase_result_23_16` reader - tx_phase_result 2"]
pub type TxPhaseResult23_16R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - tx_phase_result 2"]
    #[inline(always)]
    pub fn tx_phase_result_23_16(&self) -> TxPhaseResult23_16R {
        TxPhaseResult23_16R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_phase_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxPhase2Spec;
impl crate::RegisterSpec for TxPhase2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_phase_2::R`](R) reader structure"]
impl crate::Readable for TxPhase2Spec {}
#[doc = "`reset()` method sets TX_PHASE_2 to value 0"]
impl crate::Resettable for TxPhase2Spec {}
