#[doc = "Register `EALR` reader"]
pub type R = crate::R<EalrSpec>;
#[doc = "Field `error_address` reader - Recorded address when GCM Tag mismatch error or receiving an error response."]
pub type ErrorAddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Recorded address when GCM Tag mismatch error or receiving an error response."]
    #[inline(always)]
    pub fn error_address(&self) -> ErrorAddressR {
        ErrorAddressR::new(self.bits)
    }
}
#[doc = "Error Address Log Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ealr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EalrSpec;
impl crate::RegisterSpec for EalrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ealr::R`](R) reader structure"]
impl crate::Readable for EalrSpec {}
#[doc = "`reset()` method sets EALR to value 0"]
impl crate::Resettable for EalrSpec {}
