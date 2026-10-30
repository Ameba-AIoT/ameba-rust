#[doc = "Register `CURR_DAR0` reader"]
pub type R = crate::R<CurrDar0Spec>;
#[doc = "Field `CURR_DAR0` reader - The read back value is destination writing address at present"]
pub type CurrDar0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is destination writing address at present"]
    #[inline(always)]
    pub fn curr_dar0(&self) -> CurrDar0R {
        CurrDar0R::new(self.bits)
    }
}
#[doc = "The read back value is channel 0 destination writing address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_dar0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrDar0Spec;
impl crate::RegisterSpec for CurrDar0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_dar0::R`](R) reader structure"]
impl crate::Readable for CurrDar0Spec {}
#[doc = "`reset()` method sets CURR_DAR0 to value 0"]
impl crate::Resettable for CurrDar0Spec {}
