#[doc = "Register `SEC_IDR` reader"]
pub type R = crate::R<SecIdrSpec>;
#[doc = "Field `SEC_IDR` reader - SEC ID register"]
pub type SecIdrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SEC ID register"]
    #[inline(always)]
    pub fn sec_idr(&self) -> SecIdrR {
        SecIdrR::new(self.bits)
    }
}
#[doc = "SEC ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecIdrSpec;
impl crate::RegisterSpec for SecIdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_idr::R`](R) reader structure"]
impl crate::Readable for SecIdrSpec {}
#[doc = "`reset()` method sets SEC_IDR to value 0"]
impl crate::Resettable for SecIdrSpec {}
