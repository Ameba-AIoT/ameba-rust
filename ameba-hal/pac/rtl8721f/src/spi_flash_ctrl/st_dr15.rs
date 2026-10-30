#[doc = "Register `ST_DR15` reader"]
pub type R = crate::R<StDr15Spec>;
#[doc = "Field `ST_DR15` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr15R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr15(&self) -> StDr15R {
        StDr15R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr15Spec;
impl crate::RegisterSpec for StDr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr15::R`](R) reader structure"]
impl crate::Readable for StDr15Spec {}
#[doc = "`reset()` method sets ST_DR15 to value 0"]
impl crate::Resettable for StDr15Spec {}
