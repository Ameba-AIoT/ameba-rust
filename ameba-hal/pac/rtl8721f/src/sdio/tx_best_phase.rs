#[doc = "Register `TX_BEST_PHASE` reader"]
pub type R = crate::R<TxBestPhaseSpec>;
#[doc = "Field `tx_best_phase` reader - rx_best_phase"]
pub type TxBestPhaseR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - rx_best_phase"]
    #[inline(always)]
    pub fn tx_best_phase(&self) -> TxBestPhaseR {
        TxBestPhaseR::new(self.bits & 0x1f)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_best_phase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxBestPhaseSpec;
impl crate::RegisterSpec for TxBestPhaseSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tx_best_phase::R`](R) reader structure"]
impl crate::Readable for TxBestPhaseSpec {}
#[doc = "`reset()` method sets TX_BEST_PHASE to value 0"]
impl crate::Resettable for TxBestPhaseSpec {}
