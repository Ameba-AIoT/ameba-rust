#[doc = "Register `REG_TDE_DEBUG` reader"]
pub type R = crate::R<RegTdeDebugSpec>;
#[doc = "Field `BIT_TDE_DEBUG` reader - tde debug port register."]
pub type BitTdeDebugR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - tde debug port register."]
    #[inline(always)]
    pub fn bit_tde_debug(&self) -> BitTdeDebugR {
        BitTdeDebugR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tde_debug::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTdeDebugSpec;
impl crate::RegisterSpec for RegTdeDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tde_debug::R`](R) reader structure"]
impl crate::Readable for RegTdeDebugSpec {}
#[doc = "`reset()` method sets REG_TDE_DEBUG to value 0"]
impl crate::Resettable for RegTdeDebugSpec {}
