#[doc = "Register `CURR_DAR3` reader"]
pub type R = crate::R<CurrDar3Spec>;
#[doc = "Field `CURR_DAR3` reader - The read back value is destination writing address at present"]
pub type CurrDar3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is destination writing address at present"]
    #[inline(always)]
    pub fn curr_dar3(&self) -> CurrDar3R {
        CurrDar3R::new(self.bits)
    }
}
#[doc = "The read back value is channel 3 destination writing address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_dar3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrDar3Spec;
impl crate::RegisterSpec for CurrDar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_dar3::R`](R) reader structure"]
impl crate::Readable for CurrDar3Spec {}
#[doc = "`reset()` method sets CURR_DAR3 to value 0"]
impl crate::Resettable for CurrDar3Spec {}
