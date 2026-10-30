#[doc = "Register `REG_DEBUG` reader"]
pub type R = crate::R<RegDebugSpec>;
#[doc = "Field `adbg_regu` reader - REGU interface debug"]
pub type AdbgReguR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REGU interface debug"]
    #[inline(always)]
    pub fn adbg_regu(&self) -> AdbgReguR {
        AdbgReguR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_debug::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDebugSpec;
impl crate::RegisterSpec for RegDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_debug::R`](R) reader structure"]
impl crate::Readable for RegDebugSpec {}
#[doc = "`reset()` method sets REG_DEBUG to value 0"]
impl crate::Resettable for RegDebugSpec {}
