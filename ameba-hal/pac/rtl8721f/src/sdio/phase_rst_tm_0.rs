#[doc = "Register `PHASE_RST_TM_0` reader"]
pub type R = crate::R<PhaseRstTm0Spec>;
#[doc = "Field `phase_rst_time_cfg_7_0` reader - phase_rst_time_cfg low"]
pub type PhaseRstTimeCfg7_0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - phase_rst_time_cfg low"]
    #[inline(always)]
    pub fn phase_rst_time_cfg_7_0(&self) -> PhaseRstTimeCfg7_0R {
        PhaseRstTimeCfg7_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`phase_rst_tm_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhaseRstTm0Spec;
impl crate::RegisterSpec for PhaseRstTm0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phase_rst_tm_0::R`](R) reader structure"]
impl crate::Readable for PhaseRstTm0Spec {}
#[doc = "`reset()` method sets PHASE_RST_TM_0 to value 0"]
impl crate::Resettable for PhaseRstTm0Spec {}
