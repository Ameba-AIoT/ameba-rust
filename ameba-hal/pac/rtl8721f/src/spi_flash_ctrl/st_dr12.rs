#[doc = "Register `ST_DR12` reader"]
pub type R = crate::R<StDr12Spec>;
#[doc = "Field `ST_DR12` reader - It is a data buffer with 8-bit width FIFO."]
pub type StDr12R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It is a data buffer with 8-bit width FIFO."]
    #[inline(always)]
    pub fn st_dr12(&self) -> StDr12R {
        StDr12R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Status Data Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`st_dr12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StDr12Spec;
impl crate::RegisterSpec for StDr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st_dr12::R`](R) reader structure"]
impl crate::Readable for StDr12Spec {}
#[doc = "`reset()` method sets ST_DR12 to value 0"]
impl crate::Resettable for StDr12Spec {}
