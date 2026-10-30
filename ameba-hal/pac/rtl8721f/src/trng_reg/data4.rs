#[doc = "Register `DATA4` reader"]
pub type R = crate::R<Data4Spec>;
#[doc = "Field `RANDOM_NUMBER` reader - RNG data register 4. Read as 0x0 if RNG valid register is not a ready status. A Read access triggers a buffer pop, so the value is only read once then be discarded."]
pub type RandomNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RNG data register 4. Read as 0x0 if RNG valid register is not a ready status. A Read access triggers a buffer pop, so the value is only read once then be discarded."]
    #[inline(always)]
    pub fn random_number(&self) -> RandomNumberR {
        RandomNumberR::new(self.bits)
    }
}
#[doc = "RNG Data Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data4Spec;
impl crate::RegisterSpec for Data4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data4::R`](R) reader structure"]
impl crate::Readable for Data4Spec {}
#[doc = "`reset()` method sets DATA4 to value 0"]
impl crate::Resettable for Data4Spec {}
