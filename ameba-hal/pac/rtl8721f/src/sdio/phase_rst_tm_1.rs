#[doc = "Register `PHASE_RST_TM_1` reader"]
pub type R = crate::R<PhaseRstTm1Spec>;
#[doc = "Field `phase_rst_time_cfg_11_8` reader - phase_rst_time_cfg high"]
pub type PhaseRstTimeCfg11_8R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - phase_rst_time_cfg high"]
    #[inline(always)]
    pub fn phase_rst_time_cfg_11_8(&self) -> PhaseRstTimeCfg11_8R {
        PhaseRstTimeCfg11_8R::new(self.bits & 0x0f)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`phase_rst_tm_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhaseRstTm1Spec;
impl crate::RegisterSpec for PhaseRstTm1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phase_rst_tm_1::R`](R) reader structure"]
impl crate::Readable for PhaseRstTm1Spec {}
#[doc = "`reset()` method sets PHASE_RST_TM_1 to value 0x01"]
impl crate::Resettable for PhaseRstTm1Spec {
    const RESET_VALUE: u8 = 0x01;
}
