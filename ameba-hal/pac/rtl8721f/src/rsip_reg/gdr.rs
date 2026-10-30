#[doc = "Register `GDR` reader"]
pub type R = crate::R<GdrSpec>;
#[doc = "Field `date` reader - Date of code generation"]
pub type DateR = crate::FieldReader<u16>;
#[doc = "Field `year` reader - Year of code generation"]
pub type YearR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Date of code generation"]
    #[inline(always)]
    pub fn date(&self) -> DateR {
        DateR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Year of code generation"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Gen Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdrSpec;
impl crate::RegisterSpec for GdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdr::R`](R) reader structure"]
impl crate::Readable for GdrSpec {}
#[doc = "`reset()` method sets GDR to value 0"]
impl crate::Resettable for GdrSpec {}
