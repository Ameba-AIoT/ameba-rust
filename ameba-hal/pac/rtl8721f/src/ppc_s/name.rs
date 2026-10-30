#[doc = "Register `NAME` reader"]
pub type R = crate::R<NameSpec>;
#[doc = "Field `NAME` reader - The design name of RXI-300."]
pub type NameR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The design name of RXI-300."]
    #[inline(always)]
    pub fn name(&self) -> NameR {
        NameR::new(self.bits)
    }
}
#[doc = "RXI300 design name register\n\nYou can [`read`](crate::Reg::read) this register and get [`name::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NameSpec;
impl crate::RegisterSpec for NameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`name::R`](R) reader structure"]
impl crate::Readable for NameSpec {}
#[doc = "`reset()` method sets NAME to value 0x0005_2300"]
impl crate::Resettable for NameSpec {
    const RESET_VALUE: u32 = 0x0005_2300;
}
