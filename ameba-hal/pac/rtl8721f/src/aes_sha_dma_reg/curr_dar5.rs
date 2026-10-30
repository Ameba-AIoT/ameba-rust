#[doc = "Register `CURR_DAR5` reader"]
pub type R = crate::R<CurrDar5Spec>;
#[doc = "Field `CURR_DAR5` reader - The read back value is destination writing address at present"]
pub type CurrDar5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is destination writing address at present"]
    #[inline(always)]
    pub fn curr_dar5(&self) -> CurrDar5R {
        CurrDar5R::new(self.bits)
    }
}
#[doc = "The read back value is channel 5 destination writing address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_dar5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrDar5Spec;
impl crate::RegisterSpec for CurrDar5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_dar5::R`](R) reader structure"]
impl crate::Readable for CurrDar5Spec {}
#[doc = "`reset()` method sets CURR_DAR5 to value 0"]
impl crate::Resettable for CurrDar5Spec {}
