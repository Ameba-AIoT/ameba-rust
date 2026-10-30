#[doc = "Register `Date` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Field `Date` reader - "]
pub type DateR = crate::FieldReader<u16>;
#[doc = "Field `Year` reader - "]
pub type YearR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn date(&self) -> DateR {
        DateR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Generated date register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`reset()` method sets Date to value 0"]
impl crate::Resettable for DateSpec {}
