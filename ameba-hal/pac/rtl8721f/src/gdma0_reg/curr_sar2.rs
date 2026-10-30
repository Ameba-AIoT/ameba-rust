#[doc = "Register `CURR_SAR2` reader"]
pub type R = crate::R<CurrSar2Spec>;
#[doc = "Field `CURR_SAR2` reader - The read back value is source reading address at present"]
pub type CurrSar2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is source reading address at present"]
    #[inline(always)]
    pub fn curr_sar2(&self) -> CurrSar2R {
        CurrSar2R::new(self.bits)
    }
}
#[doc = "The read back value is channel 2 source reading address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_sar2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSar2Spec;
impl crate::RegisterSpec for CurrSar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_sar2::R`](R) reader structure"]
impl crate::Readable for CurrSar2Spec {}
#[doc = "`reset()` method sets CURR_SAR2 to value 0"]
impl crate::Resettable for CurrSar2Spec {}
