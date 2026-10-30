#[doc = "Register `REG_DBG_PORT_REG` reader"]
pub type R = crate::R<RegDbgPortRegSpec>;
#[doc = "Field `BIT_DEBUG_ST` reader - Debug signals, sel by 70\\[31:24\\]"]
pub type BitDebugStR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Debug signals, sel by 70\\[31:24\\]"]
    #[inline(always)]
    pub fn bit_debug_st(&self) -> BitDebugStR {
        BitDebugStR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dbg_port_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDbgPortRegSpec;
impl crate::RegisterSpec for RegDbgPortRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dbg_port_reg::R`](R) reader structure"]
impl crate::Readable for RegDbgPortRegSpec {}
#[doc = "`reset()` method sets REG_DBG_PORT_REG to value 0"]
impl crate::Resettable for RegDbgPortRegSpec {}
