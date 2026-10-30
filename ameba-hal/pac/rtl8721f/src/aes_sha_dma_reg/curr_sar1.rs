#[doc = "Register `CURR_SAR1` reader"]
pub type R = crate::R<CurrSar1Spec>;
#[doc = "Field `CURR_SAR1` reader - The read back value is source reading address at present"]
pub type CurrSar1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is source reading address at present"]
    #[inline(always)]
    pub fn curr_sar1(&self) -> CurrSar1R {
        CurrSar1R::new(self.bits)
    }
}
#[doc = "The read back value is channel 1 source reading address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_sar1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSar1Spec;
impl crate::RegisterSpec for CurrSar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_sar1::R`](R) reader structure"]
impl crate::Readable for CurrSar1Spec {}
#[doc = "`reset()` method sets CURR_SAR1 to value 0"]
impl crate::Resettable for CurrSar1Spec {}
