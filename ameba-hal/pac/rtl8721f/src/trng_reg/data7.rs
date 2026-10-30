#[doc = "Register `DATA7` reader"]
pub type R = crate::R<Data7Spec>;
#[doc = "Field `RANDOM_NUMBER` reader - RNG data register 7. Read as 0x0 if RNG valid register is not a ready status. A Read access triggers a buffer pop, so the value is only read once then be discarded."]
pub type RandomNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RNG data register 7. Read as 0x0 if RNG valid register is not a ready status. A Read access triggers a buffer pop, so the value is only read once then be discarded."]
    #[inline(always)]
    pub fn random_number(&self) -> RandomNumberR {
        RandomNumberR::new(self.bits)
    }
}
#[doc = "RNG Data Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`data7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data7Spec;
impl crate::RegisterSpec for Data7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data7::R`](R) reader structure"]
impl crate::Readable for Data7Spec {}
#[doc = "`reset()` method sets DATA7 to value 0"]
impl crate::Resettable for Data7Spec {}
