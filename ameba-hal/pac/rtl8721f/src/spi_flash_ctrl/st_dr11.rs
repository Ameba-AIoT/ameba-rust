#[doc = "Register `ST_DR11` reader"]
pub type R = crate::R<StDr11Spec>;
#[doc = "Field `ST_DR11` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr11R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr11(&self) -> StDr11R {
        StDr11R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr11Spec;
impl crate::RegisterSpec for StDr11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr11::R`](R) reader structure"]
impl crate::Readable for StDr11Spec {}
#[doc = "`reset()` method sets ST_DR11 to value 0"]
impl crate::Resettable for StDr11Spec {}
