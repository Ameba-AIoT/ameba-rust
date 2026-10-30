#[doc = "Register `CURR_DAR7` reader"]
pub type R = crate::R<CurrDar7Spec>;
#[doc = "Field `CURR_DAR7` reader - The read back value is destination writing address at present"]
pub type CurrDar7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is destination writing address at present"]
    #[inline(always)]
    pub fn curr_dar7(&self) -> CurrDar7R {
        CurrDar7R::new(self.bits)
    }
}
#[doc = "The read back value is channel 7 destination writing address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_dar7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrDar7Spec;
impl crate::RegisterSpec for CurrDar7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_dar7::R`](R) reader structure"]
impl crate::Readable for CurrDar7Spec {}
#[doc = "`reset()` method sets CURR_DAR7 to value 0"]
impl crate::Resettable for CurrDar7Spec {}
