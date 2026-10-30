#[doc = "Register `ELR_CODE` reader"]
pub type R = crate::R<ElrCodeSpec>;
#[doc = "Field `ELR_CODE` reader - The error code of error request, if it's 0, indicates that there is no error request. 8'h00: No error 8'h01: Address hole error"]
pub type ElrCodeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The error code of error request, if it's 0, indicates that there is no error request. 8'h00: No error 8'h01: Address hole error"]
    #[inline(always)]
    pub fn elr_code(&self) -> ElrCodeR {
        ElrCodeR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_code::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrCodeSpec;
impl crate::RegisterSpec for ElrCodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_code::R`](R) reader structure"]
impl crate::Readable for ElrCodeSpec {}
#[doc = "`reset()` method sets ELR_CODE to value 0"]
impl crate::Resettable for ElrCodeSpec {}
