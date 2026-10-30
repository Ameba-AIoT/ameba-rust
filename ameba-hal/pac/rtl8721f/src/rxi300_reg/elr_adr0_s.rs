#[doc = "Register `ELR_ADR0_S` reader"]
pub type R = crate::R<ElrAdr0SSpec>;
#[doc = "Field `ELR_ADR0_S` reader - Address\\[31:0\\] of the error request."]
pub type ElrAdr0SR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address\\[31:0\\] of the error request."]
    #[inline(always)]
    pub fn elr_adr0_s(&self) -> ElrAdr0SR {
        ElrAdr0SR::new(self.bits)
    }
}
#[doc = "Error log addrress register0\n\nYou can [`read`](crate::Reg::read) this register and get [`elr_adr0_s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ElrAdr0SSpec;
impl crate::RegisterSpec for ElrAdr0SSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`elr_adr0_s::R`](R) reader structure"]
impl crate::Readable for ElrAdr0SSpec {}
#[doc = "`reset()` method sets ELR_ADR0_S to value 0"]
impl crate::Resettable for ElrAdr0SSpec {}
