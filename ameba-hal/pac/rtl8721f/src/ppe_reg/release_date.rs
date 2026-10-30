#[doc = "Register `RELEASE_DATE` reader"]
pub type R = crate::R<ReleaseDateSpec>;
#[doc = "Field `RELEASE_DATE` reader - Release Date"]
pub type ReleaseDateR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Release Date"]
    #[inline(always)]
    pub fn release_date(&self) -> ReleaseDateR {
        ReleaseDateR::new(self.bits)
    }
}
#[doc = "Release Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`release_date::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReleaseDateSpec;
impl crate::RegisterSpec for ReleaseDateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`release_date::R`](R) reader structure"]
impl crate::Readable for ReleaseDateSpec {}
#[doc = "`reset()` method sets RELEASE_DATE to value 0"]
impl crate::Resettable for ReleaseDateSpec {}
