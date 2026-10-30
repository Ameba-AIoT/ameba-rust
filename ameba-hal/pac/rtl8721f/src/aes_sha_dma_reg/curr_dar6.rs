#[doc = "Register `CURR_DAR6` reader"]
pub type R = crate::R<CurrDar6Spec>;
#[doc = "Field `CURR_DAR6` reader - The read back value is destination writing address at present"]
pub type CurrDar6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is destination writing address at present"]
    #[inline(always)]
    pub fn curr_dar6(&self) -> CurrDar6R {
        CurrDar6R::new(self.bits)
    }
}
#[doc = "The read back value is channel 6 destination writing address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_dar6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrDar6Spec;
impl crate::RegisterSpec for CurrDar6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_dar6::R`](R) reader structure"]
impl crate::Readable for CurrDar6Spec {}
#[doc = "`reset()` method sets CURR_DAR6 to value 0"]
impl crate::Resettable for CurrDar6Spec {}
