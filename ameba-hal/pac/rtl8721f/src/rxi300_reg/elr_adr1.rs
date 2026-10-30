#[doc = "Register `ELR_ADR1` reader"]
pub type R = crate::R<ElrAdr1Spec>;
#[doc = "Field `ELR_ADR1` reader - Address\\[63:32\\] of the error request."]
pub type ElrAdr1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address\\[63:32\\] of the error request."]
    #[inline(always)]
    pub fn elr_adr1(&self) -> ElrAdr1R {
        ElrAdr1R::new(self.bits)
    }
}
#[doc = "Error log addrress register1\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_adr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrAdr1Spec;
impl crate::RegisterSpec for ElrAdr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_adr1::R`](R) reader structure"]
impl crate::Readable for ElrAdr1Spec {}
#[doc = "`reset()` method sets ELR_ADR1 to value 0"]
impl crate::Resettable for ElrAdr1Spec {}
