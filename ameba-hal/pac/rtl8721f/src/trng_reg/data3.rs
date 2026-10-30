#[doc = "Register `DATA3` reader"]
pub type R = crate::R<Data3Spec>;
#[doc = "Field `RANDOM_NUMBER` reader - RNG data register 3. Read as 0x0 if RNG valid register is not a ready status. A Read access triggers a buffer pop, so the value is only read once then be discarded."]
pub type RandomNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RNG data register 3. Read as 0x0 if RNG valid register is not a ready status. A Read access triggers a buffer pop, so the value is only read once then be discarded."]
    #[inline(always)]
    pub fn random_number(&self) -> RandomNumberR {
        RandomNumberR::new(self.bits)
    }
}
#[doc = "RNG Data Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data3Spec;
impl crate::RegisterSpec for Data3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data3::R`](R) reader structure"]
impl crate::Readable for Data3Spec {}
#[doc = "`reset()` method sets DATA3 to value 0"]
impl crate::Resettable for Data3Spec {}
