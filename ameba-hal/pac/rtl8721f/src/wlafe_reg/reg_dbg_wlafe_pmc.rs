#[doc = "Register `REG_DBG_WLAFE_PMC` reader"]
pub type R = crate::R<RegDbgWlafePmcSpec>;
#[doc = "Field `dbg_wlafe_pmc` reader - debug signal of wlafe_pmc"]
pub type DbgWlafePmcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - debug signal of wlafe_pmc"]
    #[inline(always)]
    pub fn dbg_wlafe_pmc(&self) -> DbgWlafePmcR {
        DbgWlafePmcR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dbg_wlafe_pmc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDbgWlafePmcSpec;
impl crate::RegisterSpec for RegDbgWlafePmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dbg_wlafe_pmc::R`](R) reader structure"]
impl crate::Readable for RegDbgWlafePmcSpec {}
#[doc = "`reset()` method sets REG_DBG_WLAFE_PMC to value 0"]
impl crate::Resettable for RegDbgWlafePmcSpec {}
