#[doc = "Register `ST_DR6` reader"]
pub type R = crate::R<StDr6Spec>;
#[doc = "Field `ST_DR6` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr6R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr6(&self) -> StDr6R {
        StDr6R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr6Spec;
impl crate::RegisterSpec for StDr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr6::R`](R) reader structure"]
impl crate::Readable for StDr6Spec {}
#[doc = "`reset()` method sets ST_DR6 to value 0"]
impl crate::Resettable for StDr6Spec {}
