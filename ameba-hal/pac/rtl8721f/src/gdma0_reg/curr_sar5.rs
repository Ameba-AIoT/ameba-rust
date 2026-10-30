#[doc = "Register `CURR_SAR5` reader"]
pub type R = crate::R<CurrSar5Spec>;
#[doc = "Field `CURR_SAR5` reader - The read back value is source reading address at present"]
pub type CurrSar5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is source reading address at present"]
    #[inline(always)]
    pub fn curr_sar5(&self) -> CurrSar5R {
        CurrSar5R::new(self.bits)
    }
}
#[doc = "The read back value is channel 5 source reading address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_sar5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSar5Spec;
impl crate::RegisterSpec for CurrSar5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_sar5::R`](R) reader structure"]
impl crate::Readable for CurrSar5Spec {}
#[doc = "`reset()` method sets CURR_SAR5 to value 0"]
impl crate::Resettable for CurrSar5Spec {}
