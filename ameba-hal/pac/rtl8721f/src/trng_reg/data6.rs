#[doc = "Register `DATA6` reader"]
pub type R = crate::R<Data6Spec>;
#[doc = "Field `RANDOM_NUMBER` reader - RNG data register 6. Read as 0x0 if RNG valid register is not a ready status. A Read access triggers a buffer pop, so the value is only read once then be discarded."]
pub type RandomNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RNG data register 6. Read as 0x0 if RNG valid register is not a ready status. A Read access triggers a buffer pop, so the value is only read once then be discarded."]
    #[inline(always)]
    pub fn random_number(&self) -> RandomNumberR {
        RandomNumberR::new(self.bits)
    }
}
#[doc = "RNG Data Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`data6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data6Spec;
impl crate::RegisterSpec for Data6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data6::R`](R) reader structure"]
impl crate::Readable for Data6Spec {}
#[doc = "`reset()` method sets DATA6 to value 0"]
impl crate::Resettable for Data6Spec {}
