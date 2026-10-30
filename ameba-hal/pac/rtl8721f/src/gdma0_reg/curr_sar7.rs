#[doc = "Register `CURR_SAR7` reader"]
pub type R = crate::R<CurrSar7Spec>;
#[doc = "Field `CURR_SAR7` reader - The read back value is source reading address at present"]
pub type CurrSar7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is source reading address at present"]
    #[inline(always)]
    pub fn curr_sar7(&self) -> CurrSar7R {
        CurrSar7R::new(self.bits)
    }
}
#[doc = "The read back value is channel 7 source reading address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_sar7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSar7Spec;
impl crate::RegisterSpec for CurrSar7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_sar7::R`](R) reader structure"]
impl crate::Readable for CurrSar7Spec {}
#[doc = "`reset()` method sets CURR_SAR7 to value 0"]
impl crate::Resettable for CurrSar7Spec {}
