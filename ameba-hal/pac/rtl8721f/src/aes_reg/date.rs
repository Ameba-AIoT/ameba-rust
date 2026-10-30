#[doc = "Register `date` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Field `date` reader - The date that design was generated."]
pub type DateR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The date that design was generated."]
    #[inline(always)]
    pub fn date(&self) -> DateR {
        DateR::new(self.bits)
    }
}
#[doc = "Date register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`reset()` method sets date to value 0"]
impl crate::Resettable for DateSpec {}
