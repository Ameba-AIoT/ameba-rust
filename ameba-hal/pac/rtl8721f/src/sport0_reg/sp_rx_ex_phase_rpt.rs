#[doc = "Register `SP_RX_EX_PHASE_RPT` reader"]
pub type R = crate::R<SpRxExPhaseRptSpec>;
#[doc = "Field `rx_ex_phase_rpt` reader - Report RX phase, counted by sport_clk"]
pub type RxExPhaseRptR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Report RX phase, counted by sport_clk"]
    #[inline(always)]
    pub fn rx_ex_phase_rpt(&self) -> RxExPhaseRptR {
        RxExPhaseRptR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_rx_ex_phase_rpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpRxExPhaseRptSpec;
impl crate::RegisterSpec for SpRxExPhaseRptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_rx_ex_phase_rpt::R`](R) reader structure"]
impl crate::Readable for SpRxExPhaseRptSpec {}
#[doc = "`reset()` method sets SP_RX_EX_PHASE_RPT to value 0"]
impl crate::Resettable for SpRxExPhaseRptSpec {}
