#[doc = "Register `ST_DR8` reader"]
pub type R = crate::R<StDr8Spec>;
#[doc = "Field `ST_DR8` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr8R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr8(&self) -> StDr8R {
        StDr8R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr8Spec;
impl crate::RegisterSpec for StDr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr8::R`](R) reader structure"]
impl crate::Readable for StDr8Spec {}
#[doc = "`reset()` method sets ST_DR8 to value 0"]
impl crate::Resettable for StDr8Spec {}
