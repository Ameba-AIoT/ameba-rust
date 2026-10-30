#[doc = "Register `CURR_SAR3` reader"]
pub type R = crate::R<CurrSar3Spec>;
#[doc = "Field `CURR_SAR3` reader - The read back value is source reading address at present"]
pub type CurrSar3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is source reading address at present"]
    #[inline(always)]
    pub fn curr_sar3(&self) -> CurrSar3R {
        CurrSar3R::new(self.bits)
    }
}
#[doc = "The read back value is channel 3 source reading address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_sar3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSar3Spec;
impl crate::RegisterSpec for CurrSar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_sar3::R`](R) reader structure"]
impl crate::Readable for CurrSar3Spec {}
#[doc = "`reset()` method sets CURR_SAR3 to value 0"]
impl crate::Resettable for CurrSar3Spec {}
