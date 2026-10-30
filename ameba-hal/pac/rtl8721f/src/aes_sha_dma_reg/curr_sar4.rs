#[doc = "Register `CURR_SAR4` reader"]
pub type R = crate::R<CurrSar4Spec>;
#[doc = "Field `CURR_SAR4` reader - The read back value is source reading address at present"]
pub type CurrSar4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The read back value is source reading address at present"]
    #[inline(always)]
    pub fn curr_sar4(&self) -> CurrSar4R {
        CurrSar4R::new(self.bits)
    }
}
#[doc = "The read back value is channel 4 source reading address at present\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_sar4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSar4Spec;
impl crate::RegisterSpec for CurrSar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_sar4::R`](R) reader structure"]
impl crate::Readable for CurrSar4Spec {}
#[doc = "`reset()` method sets CURR_SAR4 to value 0"]
impl crate::Resettable for CurrSar4Spec {}
