#[doc = "Register `ELR_ADR1_S` reader"]
pub type R = crate::R<ElrAdr1SSpec>;
#[doc = "Field `ELR_ADR1_S` reader - Address\\[63:32\\] of the error request."]
pub type ElrAdr1SR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address\\[63:32\\] of the error request."]
    #[inline(always)]
    pub fn elr_adr1_s(&self) -> ElrAdr1SR {
        ElrAdr1SR::new(self.bits)
    }
}
#[doc = "Error log addrress register1\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_adr1_s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrAdr1SSpec;
impl crate::RegisterSpec for ElrAdr1SSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_adr1_s::R`](R) reader structure"]
impl crate::Readable for ElrAdr1SSpec {}
#[doc = "`reset()` method sets ELR_ADR1_S to value 0"]
impl crate::Resettable for ElrAdr1SSpec {}
