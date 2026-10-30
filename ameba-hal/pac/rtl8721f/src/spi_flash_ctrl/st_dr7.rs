#[doc = "Register `ST_DR7` reader"]
pub type R = crate::R<StDr7Spec>;
#[doc = "Field `ST_DR7` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr7R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr7(&self) -> StDr7R {
        StDr7R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr7Spec;
impl crate::RegisterSpec for StDr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr7::R`](R) reader structure"]
impl crate::Readable for StDr7Spec {}
#[doc = "`reset()` method sets ST_DR7 to value 0"]
impl crate::Resettable for StDr7Spec {}
