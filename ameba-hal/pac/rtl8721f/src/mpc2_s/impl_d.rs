#[doc = "Register `IMPL_D` reader"]
pub type R = crate::R<ImplDSpec>;
#[doc = "Field `IMPL_D` reader - The implementatin date of RXI-300. Reset value depends on implementation date."]
pub type ImplDR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The implementatin date of RXI-300. Reset value depends on implementation date."]
    #[inline(always)]
    pub fn impl_d(&self) -> ImplDR {
        ImplDR::new(self.bits)
    }
}
#[doc = "RXI300 implementation date register\n\nYou can [`read`](crate::Reg::read) this register and get [`impl_d::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImplDSpec;
impl crate::RegisterSpec for ImplDSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`impl_d::R`](R) reader structure"]
impl crate::Readable for ImplDSpec {}
#[doc = "`reset()` method sets IMPL_D to value 0x0510_1058"]
impl crate::Resettable for ImplDSpec {
    const RESET_VALUE: u32 = 0x0510_1058;
}
