#[doc = "Register `REG_BEAMFORMING_MIMO_STATUS` reader"]
pub type R = crate::R<RegBeamformingMimoStatusSpec>;
#[doc = "Field `ro_wmac_he_csi_mimo_h` reader - msb 24bit of he csi mimo field"]
pub type RoWmacHeCsiMimoHR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - msb 24bit of he csi mimo field"]
    #[inline(always)]
    pub fn ro_wmac_he_csi_mimo_h(&self) -> RoWmacHeCsiMimoHR {
        RoWmacHeCsiMimoHR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_beamforming_mimo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBeamformingMimoStatusSpec;
impl crate::RegisterSpec for RegBeamformingMimoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_beamforming_mimo_status::R`](R) reader structure"]
impl crate::Readable for RegBeamformingMimoStatusSpec {}
#[doc = "`reset()` method sets REG_BEAMFORMING_MIMO_STATUS to value 0"]
impl crate::Resettable for RegBeamformingMimoStatusSpec {}
