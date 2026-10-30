#[doc = "Register `IMPL_Y` reader"]
pub type R = crate::R<ImplYSpec>;
#[doc = "Field `IMPL_Y` reader - The implementatin year of RXI-300. Reset value depends on implementation date."]
pub type ImplYR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The implementatin year of RXI-300. Reset value depends on implementation date."]
    #[inline(always)]
    pub fn impl_y(&self) -> ImplYR {
        ImplYR::new(self.bits)
    }
}
#[doc = "RXI300 implementation year register\n\nYou can [`read`](crate::Reg::read) this register and get [`impl_y::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImplYSpec;
impl crate::RegisterSpec for ImplYSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`impl_y::R`](R) reader structure"]
impl crate::Readable for ImplYSpec {}
#[doc = "`reset()` method sets IMPL_Y to value 0x2024"]
impl crate::Resettable for ImplYSpec {
    const RESET_VALUE: u32 = 0x2024;
}
