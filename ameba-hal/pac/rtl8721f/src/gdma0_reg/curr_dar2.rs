#[doc = "Register `CURR_DAR2` reader"]
pub type R = crate::R<CurrDar2Spec>;
#[doc = "Field `CURR_DAR2` reader - The read back value is destination writing address at present"]
pub type CurrDar2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is destination writing address at present"]
    #[inline(always)]
    pub fn curr_dar2(&self) -> CurrDar2R {
        CurrDar2R::new(self.bits)
    }
}
#[doc = "The read back value is channel 2 destination writing address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_dar2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrDar2Spec;
impl crate::RegisterSpec for CurrDar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_dar2::R`](R) reader structure"]
impl crate::Readable for CurrDar2Spec {}
#[doc = "`reset()` method sets CURR_DAR2 to value 0"]
impl crate::Resettable for CurrDar2Spec {}
