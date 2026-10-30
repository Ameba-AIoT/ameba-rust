#[doc = "Register `SP_TX_EX_PHASE_RPT` reader"]
pub type R = crate::R<SpTxExPhaseRptSpec>;
#[doc = "Field `tx_ex_phase_rpt` reader - Report TX phase, counted by sport_clk"]
pub type TxExPhaseRptR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Report TX phase, counted by sport_clk"]
    #[inline(always)]
    pub fn tx_ex_phase_rpt(&self) -> TxExPhaseRptR {
        TxExPhaseRptR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_tx_ex_phase_rpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpTxExPhaseRptSpec;
impl crate::RegisterSpec for SpTxExPhaseRptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_tx_ex_phase_rpt::R`](R) reader structure"]
impl crate::Readable for SpTxExPhaseRptSpec {}
#[doc = "`reset()` method sets SP_TX_EX_PHASE_RPT to value 0"]
impl crate::Resettable for SpTxExPhaseRptSpec {}
