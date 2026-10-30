#[doc = "Register `ELR_ID` reader"]
pub type R = crate::R<ElrIdSpec>;
#[doc = "Field `ELR_ID` reader - Tagid of the error request. 9'b00001????: M1 access error 9'b000100110: M5-0 access error 9'b000100111: M5-1 access error"]
pub type ElrIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Tagid of the error request. 9'b00001????: M1 access error 9'b000100110: M5-0 access error 9'b000100111: M5-1 access error"]
    #[inline(always)]
    pub fn elr_id(&self) -> ElrIdR {
        ElrIdR::new(self.bits)
    }
}
#[doc = "Error log ID information register\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrIdSpec;
impl crate::RegisterSpec for ElrIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_id::R`](R) reader structure"]
impl crate::Readable for ElrIdSpec {}
#[doc = "`reset()` method sets ELR_ID to value 0"]
impl crate::Resettable for ElrIdSpec {}
