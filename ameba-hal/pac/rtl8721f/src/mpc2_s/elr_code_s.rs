#[doc = "Register `ELR_CODE_S` reader"]
pub type R = crate::R<ElrCodeSSpec>;
#[doc = "Field `ELR_CODE_S` reader - The error code of error request, if it's 0, indicates that there is no error request. 8'h00: No error 8'h02: Secure fault error"]
pub type ElrCodeSR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The error code of error request, if it's 0, indicates that there is no error request. 8'h00: No error 8'h02: Secure fault error"]
    #[inline(always)]
    pub fn elr_code_s(&self) -> ElrCodeSR {
        ElrCodeSR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_code_s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrCodeSSpec;
impl crate::RegisterSpec for ElrCodeSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_code_s::R`](R) reader structure"]
impl crate::Readable for ElrCodeSSpec {}
#[doc = "`reset()` method sets ELR_CODE_S to value 0"]
impl crate::Resettable for ElrCodeSSpec {}
