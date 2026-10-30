#[doc = "Register `ST_DR0` reader"]
pub type R = crate::R<StDr0Spec>;
#[doc = "Field `ST_DR0` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr0(&self) -> StDr0R {
        StDr0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr0Spec;
impl crate::RegisterSpec for StDr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr0::R`](R) reader structure"]
impl crate::Readable for StDr0Spec {}
#[doc = "`reset()` method sets ST_DR0 to value 0"]
impl crate::Resettable for StDr0Spec {}
