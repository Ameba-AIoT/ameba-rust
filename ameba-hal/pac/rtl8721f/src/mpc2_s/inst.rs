#[doc = "Register `INST` reader"]
pub type R = crate::R<InstSpec>;
#[doc = "Field `INST` reader - The design instance number of RXI-300."]
pub type InstR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The design instance number of RXI-300."]
    #[inline(always)]
    pub fn inst(&self) -> InstR {
        InstR::new(self.bits)
    }
}
#[doc = "RXI300 design instance number register\n\nYou can [`read`](crate::Reg::read) this register and get [`inst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstSpec;
impl crate::RegisterSpec for InstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inst::R`](R) reader structure"]
impl crate::Readable for InstSpec {}
#[doc = "`reset()` method sets INST to value 0"]
impl crate::Resettable for InstSpec {}
