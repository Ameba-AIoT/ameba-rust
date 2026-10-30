#[doc = "Register `SPDIO_CRPWM2` reader"]
pub type R = crate::R<SpdioCrpwm2Spec>;
#[doc = "Field `SPDIO_HRPWM2` reader - NO USED"]
pub type SpdioHrpwm2R = crate::FieldReader<u16>;
#[doc = "Field `SPDIO_HRPWM2_PS` reader - host write 1 to trigger interrupt to device"]
pub type SpdioHrpwm2PsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:14 - NO USED"]
    #[inline(always)]
    pub fn spdio_hrpwm2(&self) -> SpdioHrpwm2R {
        SpdioHrpwm2R::new(self.bits & 0x7fff)
    }
    #[doc = "Bit 15 - host write 1 to trigger interrupt to device"]
    #[inline(always)]
    pub fn spdio_hrpwm2_ps(&self) -> SpdioHrpwm2PsR {
        SpdioHrpwm2PsR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_crpwm2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioCrpwm2Spec;
impl crate::RegisterSpec for SpdioCrpwm2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdio_crpwm2::R`](R) reader structure"]
impl crate::Readable for SpdioCrpwm2Spec {}
#[doc = "`reset()` method sets SPDIO_CRPWM2 to value 0"]
impl crate::Resettable for SpdioCrpwm2Spec {}
