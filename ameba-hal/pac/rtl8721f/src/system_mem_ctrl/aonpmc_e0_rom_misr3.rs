#[doc = "Register `AONPMC_E0_ROM_MISR3` reader"]
pub type R = crate::R<AonpmcE0RomMisr3Spec>;
#[doc = "Field `aonpmc_e0_misr_dout_35_32` reader - "]
pub type AonpmcE0MisrDout35_32R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn aonpmc_e0_misr_dout_35_32(&self) -> AonpmcE0MisrDout35_32R {
        AonpmcE0MisrDout35_32R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`aonpmc_e0_rom_misr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonpmcE0RomMisr3Spec;
impl crate::RegisterSpec for AonpmcE0RomMisr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aonpmc_e0_rom_misr3::R`](R) reader structure"]
impl crate::Readable for AonpmcE0RomMisr3Spec {}
#[doc = "`reset()` method sets AONPMC_E0_ROM_MISR3 to value 0"]
impl crate::Resettable for AonpmcE0RomMisr3Spec {}
