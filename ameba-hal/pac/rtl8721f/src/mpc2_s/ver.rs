#[doc = "Register `VER` reader"]
pub type R = crate::R<VerSpec>;
#[doc = "Field `VER` reader - The design version of RXI-300."]
pub type VerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The design version of RXI-300."]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(self.bits)
    }
}
#[doc = "RXI300 design version register\n\nYou can [`read`](crate::Reg::read) this register and get [`ver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VerSpec;
impl crate::RegisterSpec for VerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver::R`](R) reader structure"]
impl crate::Readable for VerSpec {}
#[doc = "`reset()` method sets VER to value 0x0300"]
impl crate::Resettable for VerSpec {
    const RESET_VALUE: u32 = 0x0300;
}
