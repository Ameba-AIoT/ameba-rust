#[doc = "Register `ST_DR2` reader"]
pub type R = crate::R<StDr2Spec>;
#[doc = "Field `ST_DR2` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr2(&self) -> StDr2R {
        StDr2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr2Spec;
impl crate::RegisterSpec for StDr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr2::R`](R) reader structure"]
impl crate::Readable for StDr2Spec {}
#[doc = "`reset()` method sets ST_DR2 to value 0"]
impl crate::Resettable for StDr2Spec {}
