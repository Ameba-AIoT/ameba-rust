#[doc = "Register `REG_DBG_INFO` reader"]
pub type R = crate::R<RegDbgInfoSpec>;
#[doc = "Field `PSRAM_PHY_DBG` reader - psram phy debug signals."]
pub type PsramPhyDbgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - psram phy debug signals."]
    #[inline(always)]
    pub fn psram_phy_dbg(&self) -> PsramPhyDbgR {
        PsramPhyDbgR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dbg_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDbgInfoSpec;
impl crate::RegisterSpec for RegDbgInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dbg_info::R`](R) reader structure"]
impl crate::Readable for RegDbgInfoSpec {}
#[doc = "`reset()` method sets REG_DBG_INFO to value 0"]
impl crate::Resettable for RegDbgInfoSpec {}
