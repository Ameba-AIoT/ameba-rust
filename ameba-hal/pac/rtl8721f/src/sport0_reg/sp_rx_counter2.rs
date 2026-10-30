#[doc = "Register `SP_RX_COUNTER2` reader"]
pub type R = crate::R<SpRxCounter2Spec>;
#[doc = "Field `rx_fs_phase_rpt` reader - Report RX phase"]
pub type RxFsPhaseRptR = crate::FieldReader;
#[doc = "Field `rx_sport_counter` reader - For DSP read instant rx sport counter value, counter down"]
pub type RxSportCounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - Report RX phase"]
    #[inline(always)]
    pub fn rx_fs_phase_rpt(&self) -> RxFsPhaseRptR {
        RxFsPhaseRptR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - For DSP read instant rx sport counter value, counter down"]
    #[inline(always)]
    pub fn rx_sport_counter(&self) -> RxSportCounterR {
        RxSportCounterR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_rx_counter2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpRxCounter2Spec;
impl crate::RegisterSpec for SpRxCounter2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_rx_counter2::R`](R) reader structure"]
impl crate::Readable for SpRxCounter2Spec {}
#[doc = "`reset()` method sets SP_RX_COUNTER2 to value 0"]
impl crate::Resettable for SpRxCounter2Spec {}
