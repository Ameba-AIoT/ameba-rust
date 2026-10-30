#[doc = "Register `ST_DR13` reader"]
pub type R = crate::R<StDr13Spec>;
#[doc = "Field `ST_DR13` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr13R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr13(&self) -> StDr13R {
        StDr13R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr13Spec;
impl crate::RegisterSpec for StDr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr13::R`](R) reader structure"]
impl crate::Readable for StDr13Spec {}
#[doc = "`reset()` method sets ST_DR13 to value 0"]
impl crate::Resettable for StDr13Spec {}
