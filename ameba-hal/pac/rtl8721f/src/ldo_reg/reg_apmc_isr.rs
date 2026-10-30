#[doc = "Register `REG_APMC_ISR` reader"]
pub type R = crate::R<RegApmcIsrSpec>;
#[doc = "Field `cur_isr` reader - regu pmc current processing isr, one hot"]
pub type CurIsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - regu pmc current processing isr, one hot"]
    #[inline(always)]
    pub fn cur_isr(&self) -> CurIsrR {
        CurIsrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_apmc_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegApmcIsrSpec;
impl crate::RegisterSpec for RegApmcIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_apmc_isr::R`](R) reader structure"]
impl crate::Readable for RegApmcIsrSpec {}
#[doc = "`reset()` method sets REG_APMC_ISR to value 0"]
impl crate::Resettable for RegApmcIsrSpec {}
