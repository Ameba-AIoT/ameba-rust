#[doc = "Register `ELR_ADR0` reader"]
pub type R = crate::R<ElrAdr0Spec>;
#[doc = "Field `ELR_ADR0` reader - Address\\[31:0\\] of the error request."]
pub type ElrAdr0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address\\[31:0\\] of the error request."]
    #[inline(always)]
    pub fn elr_adr0(&self) -> ElrAdr0R {
        ElrAdr0R::new(self.bits)
    }
}
#[doc = "Error log addrress register0\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_adr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrAdr0Spec;
impl crate::RegisterSpec for ElrAdr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_adr0::R`](R) reader structure"]
impl crate::Readable for ElrAdr0Spec {}
#[doc = "`reset()` method sets ELR_ADR0 to value 0"]
impl crate::Resettable for ElrAdr0Spec {}
