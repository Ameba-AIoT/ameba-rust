#[doc = "Register `ST_DR10` reader"]
pub type R = crate::R<StDr10Spec>;
#[doc = "Field `ST_DR10` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr10R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr10(&self) -> StDr10R {
        StDr10R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr10Spec;
impl crate::RegisterSpec for StDr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr10::R`](R) reader structure"]
impl crate::Readable for StDr10Spec {}
#[doc = "`reset()` method sets ST_DR10 to value 0"]
impl crate::Resettable for StDr10Spec {}
