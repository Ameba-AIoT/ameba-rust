#[doc = "Register `CURR_DAR1` reader"]
pub type R = crate::R<CurrDar1Spec>;
#[doc = "Field `CURR_DAR1` reader - The read back value is destination writing address at present"]
pub type CurrDar1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is destination writing address at present"]
    #[inline(always)]
    pub fn curr_dar1(&self) -> CurrDar1R {
        CurrDar1R::new(self.bits)
    }
}
#[doc = "The read back value is channel 1 destination writing address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_dar1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrDar1Spec;
impl crate::RegisterSpec for CurrDar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_dar1::R`](R) reader structure"]
impl crate::Readable for CurrDar1Spec {}
#[doc = "`reset()` method sets CURR_DAR1 to value 0"]
impl crate::Resettable for CurrDar1Spec {}
