#[doc = "Register `ST_DR1` reader"]
pub type R = crate::R<StDr1Spec>;
#[doc = "Field `ST_DR1` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr1(&self) -> StDr1R {
        StDr1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr1Spec;
impl crate::RegisterSpec for StDr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr1::R`](R) reader structure"]
impl crate::Readable for StDr1Spec {}
#[doc = "`reset()` method sets ST_DR1 to value 0"]
impl crate::Resettable for StDr1Spec {}
