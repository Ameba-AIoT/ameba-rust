#[doc = "Register `MissPkt` reader"]
pub type R = crate::R<MissPktSpec>;
#[doc = "Field `MissPkt` reader - 16-bit counter of Missed packets. Rolls over automatically. Write to clear."]
pub type MissPktR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of Missed packets. Rolls over automatically. Write to clear."]
    #[inline(always)]
    pub fn miss_pkt(&self) -> MissPktR {
        MissPktR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`miss_pkt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MissPktSpec;
impl crate::RegisterSpec for MissPktSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`miss_pkt::R`](R) reader structure"]
impl crate::Readable for MissPktSpec {}
#[doc = "`reset()` method sets MissPkt to value 0"]
impl crate::Resettable for MissPktSpec {}
