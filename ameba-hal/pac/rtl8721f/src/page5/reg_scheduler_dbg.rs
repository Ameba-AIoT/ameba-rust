#[doc = "Register `REG_SCHEDULER_DBG` reader"]
pub type R = crate::R<RegSchedulerDbgSpec>;
#[doc = "Field `BIT_DBG_SCHEDULER` reader - "]
pub type BitDbgSchedulerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bit_dbg_scheduler(&self) -> BitDbgSchedulerR {
        BitDbgSchedulerR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_scheduler_dbg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSchedulerDbgSpec;
impl crate::RegisterSpec for RegSchedulerDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_scheduler_dbg::R`](R) reader structure"]
impl crate::Readable for RegSchedulerDbgSpec {}
#[doc = "`reset()` method sets REG_SCHEDULER_DBG to value 0"]
impl crate::Resettable for RegSchedulerDbgSpec {}
