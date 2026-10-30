#[doc = "Register `chn_aes_gcm_xts_circuit` reader"]
pub type R = crate::R<ChnAesGcmXtsCircuitSpec>;
#[doc = "Field `gcm_circuit` reader - Indicate which AES engine has the GCM and GMAC circuit."]
pub type GcmCircuitR = crate::FieldReader<u16>;
#[doc = "Field `xts_circuit` reader - Indicate which AES engine has the XTS circuit."]
pub type XtsCircuitR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Indicate which AES engine has the GCM and GMAC circuit."]
    #[inline(always)]
    pub fn gcm_circuit(&self) -> GcmCircuitR {
        GcmCircuitR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Indicate which AES engine has the XTS circuit."]
    #[inline(always)]
    pub fn xts_circuit(&self) -> XtsCircuitR {
        XtsCircuitR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Enable GCM and XTS status register\n\nYou can [`read`](crate::Reg::read) this register and get [`chn_aes_gcm_xts_circuit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnAesGcmXtsCircuitSpec;
impl crate::RegisterSpec for ChnAesGcmXtsCircuitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn_aes_gcm_xts_circuit::R`](R) reader structure"]
impl crate::Readable for ChnAesGcmXtsCircuitSpec {}
#[doc = "`reset()` method sets chn_aes_gcm_xts_circuit to value 0"]
impl crate::Resettable for ChnAesGcmXtsCircuitSpec {}
