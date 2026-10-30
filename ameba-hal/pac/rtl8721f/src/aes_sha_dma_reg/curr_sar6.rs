#[doc = "Register `CURR_SAR6` reader"]
pub type R = crate::R<CurrSar6Spec>;
#[doc = "Field `CURR_SAR6` reader - The read back value is source reading address at present"]
pub type CurrSar6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is source reading address at present"]
    #[inline(always)]
    pub fn curr_sar6(&self) -> CurrSar6R {
        CurrSar6R::new(self.bits)
    }
}
#[doc = "The read back value is channel 6 source reading address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_sar6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSar6Spec;
impl crate::RegisterSpec for CurrSar6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_sar6::R`](R) reader structure"]
impl crate::Readable for CurrSar6Spec {}
#[doc = "`reset()` method sets CURR_SAR6 to value 0"]
impl crate::Resettable for CurrSar6Spec {}
