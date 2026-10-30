#[doc = "Register `ST_DR4` reader"]
pub type R = crate::R<StDr4Spec>;
#[doc = "Field `ST_DR4` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr4(&self) -> StDr4R {
        StDr4R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr4Spec;
impl crate::RegisterSpec for StDr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr4::R`](R) reader structure"]
impl crate::Readable for StDr4Spec {}
#[doc = "`reset()` method sets ST_DR4 to value 0"]
impl crate::Resettable for StDr4Spec {}
