#[doc = "Register `ST_DR3` reader"]
pub type R = crate::R<StDr3Spec>;
#[doc = "Field `ST_DR3` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr3(&self) -> StDr3R {
        StDr3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr3Spec;
impl crate::RegisterSpec for StDr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr3::R`](R) reader structure"]
impl crate::Readable for StDr3Spec {}
#[doc = "`reset()` method sets ST_DR3 to value 0"]
impl crate::Resettable for StDr3Spec {}
