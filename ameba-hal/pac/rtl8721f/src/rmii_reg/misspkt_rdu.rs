#[doc = "Register `Misspkt_rdu` reader"]
pub type R = crate::R<MisspktRduSpec>;
#[doc = "Field `RDU_MISSPKTC` reader - 16-bit counter of miss packets due to rx descriptor is unavailable. Rolls over automatically. Write to clear. gmac only."]
pub type RduMisspktcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit counter of miss packets due to rx descriptor is unavailable. Rolls over automatically. Write to clear. gmac only."]
    #[inline(always)]
    pub fn rdu_misspktc(&self) -> RduMisspktcR {
        RduMisspktcR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`misspkt_rdu::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisspktRduSpec;
impl crate::RegisterSpec for MisspktRduSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`misspkt_rdu::R`](R) reader structure"]
impl crate::Readable for MisspktRduSpec {}
#[doc = "`reset()` method sets Misspkt_rdu to value 0"]
impl crate::Resettable for MisspktRduSpec {}
