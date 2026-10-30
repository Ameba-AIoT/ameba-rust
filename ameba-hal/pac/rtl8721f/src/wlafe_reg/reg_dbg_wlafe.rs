#[doc = "Register `REG_DBG_WLAFE` reader"]
pub type R = crate::R<RegDbgWlafeSpec>;
#[doc = "Field `dbg_wlafe_reg` reader - debug signal of wlafe_reg"]
pub type DbgWlafeRegR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - debug signal of wlafe_reg"]
    #[inline(always)]
    pub fn dbg_wlafe_reg(&self) -> DbgWlafeRegR {
        DbgWlafeRegR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dbg_wlafe::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDbgWlafeSpec;
impl crate::RegisterSpec for RegDbgWlafeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dbg_wlafe::R`](R) reader structure"]
impl crate::Readable for RegDbgWlafeSpec {}
#[doc = "`reset()` method sets REG_DBG_WLAFE to value 0"]
impl crate::Resettable for RegDbgWlafeSpec {}
