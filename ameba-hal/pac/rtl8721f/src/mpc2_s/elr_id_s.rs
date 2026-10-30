#[doc = "Register `ELR_ID_S` reader"]
pub type R = crate::R<ElrIdSSpec>;
#[doc = "Field `ELR_ID_S` reader - Tagid of the error request. 9'b00000????: M0 access error 9'b00001????: M1 access error 9'b0100?????: M2 access error 9'b000110???: M3 access error 9'b0001010??: M4 access error 9'b000100110: M5-0 access error 9'b000100111: M5-1 access error 9'b0101?????: M6 access error 9'b1????????: M7 access error 9'b00010000?: M8 access error 9'b00010010: M9 access error 9'b00010001: M10 access error 9'b011000???: M11 access error"]
pub type ElrIdSR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Tagid of the error request. 9'b00000????: M0 access error 9'b00001????: M1 access error 9'b0100?????: M2 access error 9'b000110???: M3 access error 9'b0001010??: M4 access error 9'b000100110: M5-0 access error 9'b000100111: M5-1 access error 9'b0101?????: M6 access error 9'b1????????: M7 access error 9'b00010000?: M8 access error 9'b00010010: M9 access error 9'b00010001: M10 access error 9'b011000???: M11 access error"]
    #[inline(always)]
    pub fn elr_id_s(&self) -> ElrIdSR {
        ElrIdSR::new(self.bits)
    }
}
#[doc = "Error log ID information register\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_id_s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrIdSSpec;
impl crate::RegisterSpec for ElrIdSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_id_s::R`](R) reader structure"]
impl crate::Readable for ElrIdSSpec {}
#[doc = "`reset()` method sets ELR_ID_S to value 0"]
impl crate::Resettable for ElrIdSSpec {}
