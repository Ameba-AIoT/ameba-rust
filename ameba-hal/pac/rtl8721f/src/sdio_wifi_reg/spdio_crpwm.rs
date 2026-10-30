#[doc = "Register `SPDIO_CRPWM` reader"]
pub type R = crate::R<SpdioCrpwmSpec>;
#[doc = "Field `SPDIO_HRPWM` reader - hrpwm"]
pub type SpdioHrpwmR = crate::FieldReader;
#[doc = "Field `SPDIO_HRPWM_PS` reader - host write 1 to trigger interrupt to device"]
pub type SpdioHrpwmPsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - hrpwm"]
    #[inline(always)]
    pub fn spdio_hrpwm(&self) -> SpdioHrpwmR {
        SpdioHrpwmR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - host write 1 to trigger interrupt to device"]
    #[inline(always)]
    pub fn spdio_hrpwm_ps(&self) -> SpdioHrpwmPsR {
        SpdioHrpwmPsR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_crpwm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCrpwmSpec;
impl crate::RegisterSpec for SpdioCrpwmSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdio_crpwm::R`](R) reader structure"]
impl crate::Readable for SpdioCrpwmSpec {}
#[doc = "`reset()` method sets SPDIO_CRPWM to value 0"]
impl crate::Resettable for SpdioCrpwmSpec {}
