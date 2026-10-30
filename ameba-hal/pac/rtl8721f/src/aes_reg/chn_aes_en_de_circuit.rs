#[doc = "Register `chn_aes_en_de_circuit` reader"]
pub type R = crate::R<ChnAesEnDeCircuitSpec>;
#[doc = "Field `de_circuit` reader - Indicate which AES engine has the decryption circuit."]
pub type DeCircuitR = crate::FieldReader<u16>;
#[doc = "Field `en_circuit` reader - Indicate which AES engine has the encryption circuit."]
pub type EnCircuitR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Indicate which AES engine has the decryption circuit."]
    #[inline(always)]
    pub fn de_circuit(&self) -> DeCircuitR {
        DeCircuitR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Indicate which AES engine has the encryption circuit."]
    #[inline(always)]
    pub fn en_circuit(&self) -> EnCircuitR {
        EnCircuitR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Enable circuit status register\n\nYou can [`read`](crate::Reg::read) this register and get [`chn_aes_en_de_circuit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnAesEnDeCircuitSpec;
impl crate::RegisterSpec for ChnAesEnDeCircuitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn_aes_en_de_circuit::R`](R) reader structure"]
impl crate::Readable for ChnAesEnDeCircuitSpec {}
#[doc = "`reset()` method sets chn_aes_en_de_circuit to value 0"]
impl crate::Resettable for ChnAesEnDeCircuitSpec {}
