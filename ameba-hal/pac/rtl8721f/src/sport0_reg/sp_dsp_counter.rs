#[doc = "Register `SP_DSP_COUNTER` reader"]
pub type R = crate::R<SpDspCounterSpec>;
#[doc = "Field `tx_fs_phase_rpt` reader - Report tx phase"]
pub type TxFsPhaseRptR = crate::FieldReader;
#[doc = "Field `tx_sport_counter` reader - For DSP read instant tx sport counter value, counter down"]
pub type TxSportCounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - Report tx phase"]
    #[inline(always)]
    pub fn tx_fs_phase_rpt(&self) -> TxFsPhaseRptR {
        TxFsPhaseRptR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - For DSP read instant tx sport counter value, counter down"]
    #[inline(always)]
    pub fn tx_sport_counter(&self) -> TxSportCounterR {
        TxSportCounterR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_dsp_counter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpDspCounterSpec;
impl crate::RegisterSpec for SpDspCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_dsp_counter::R`](R) reader structure"]
impl crate::Readable for SpDspCounterSpec {}
#[doc = "`reset()` method sets SP_DSP_COUNTER to value 0"]
impl crate::Resettable for SpDspCounterSpec {}
