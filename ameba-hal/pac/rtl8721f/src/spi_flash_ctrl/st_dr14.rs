#[doc = "Register `ST_DR14` reader"]
pub type R = crate::R<StDr14Spec>;
#[doc = "Field `ST_DR14` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr14R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr14(&self) -> StDr14R {
        StDr14R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr14Spec;
impl crate::RegisterSpec for StDr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr14::R`](R) reader structure"]
impl crate::Readable for StDr14Spec {}
#[doc = "`reset()` method sets ST_DR14 to value 0"]
impl crate::Resettable for StDr14Spec {}
