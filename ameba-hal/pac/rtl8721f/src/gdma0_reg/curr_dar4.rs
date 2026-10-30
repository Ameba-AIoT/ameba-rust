#[doc = "Register `CURR_DAR4` reader"]
pub type R = crate::R<CurrDar4Spec>;
#[doc = "Field `CURR_DAR4` reader - The read back value is destination writing address at present"]
pub type CurrDar4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is destination writing address at present"]
    #[inline(always)]
    pub fn curr_dar4(&self) -> CurrDar4R {
        CurrDar4R::new(self.bits)
    }
}
#[doc = "The read back value is channel 4 destination writing address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_dar4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrDar4Spec;
impl crate::RegisterSpec for CurrDar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_dar4::R`](R) reader structure"]
impl crate::Readable for CurrDar4Spec {}
#[doc = "`reset()` method sets CURR_DAR4 to value 0"]
impl crate::Resettable for CurrDar4Spec {}
