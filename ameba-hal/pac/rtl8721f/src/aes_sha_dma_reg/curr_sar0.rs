#[doc = "Register `CURR_SAR0` reader"]
pub type R = crate::R<CurrSar0Spec>;
#[doc = "Field `CURR_SAR0` reader - The read back value is source reading address at present"]
pub type CurrSar0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is source reading address at present"]
    #[inline(always)]
    pub fn curr_sar0(&self) -> CurrSar0R {
        CurrSar0R::new(self.bits)
    }
}
#[doc = "The read back value is channel 0 source reading address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_sar0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSar0Spec;
impl crate::RegisterSpec for CurrSar0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_sar0::R`](R) reader structure"]
impl crate::Readable for CurrSar0Spec {}
#[doc = "`reset()` method sets CURR_SAR0 to value 0"]
impl crate::Resettable for CurrSar0Spec {}
