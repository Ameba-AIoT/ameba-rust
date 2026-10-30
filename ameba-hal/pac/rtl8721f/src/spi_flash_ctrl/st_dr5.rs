#[doc = "Register `ST_DR5` reader"]
pub type R = crate::R<StDr5Spec>;
#[doc = "Field `ST_DR5` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr5R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr5(&self) -> StDr5R {
        StDr5R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr5Spec;
impl crate::RegisterSpec for StDr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr5::R`](R) reader structure"]
impl crate::Readable for StDr5Spec {}
#[doc = "`reset()` method sets ST_DR5 to value 0"]
impl crate::Resettable for StDr5Spec {}
