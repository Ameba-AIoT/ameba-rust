#[doc = "Register `AONPMC_E0_ROM_MISR2` reader"]
pub type R = crate::R<AonpmcE0RomMisr2Spec>;
#[doc = "Field `aonpmc_e0_misr_dout_31_0` reader - "]
pub type AonpmcE0MisrDout31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aonpmc_e0_misr_dout_31_0(&self) -> AonpmcE0MisrDout31_0R {
        AonpmcE0MisrDout31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`aonpmc_e0_rom_misr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonpmcE0RomMisr2Spec;
impl crate::RegisterSpec for AonpmcE0RomMisr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aonpmc_e0_rom_misr2::R`](R) reader structure"]
impl crate::Readable for AonpmcE0RomMisr2Spec {}
#[doc = "`reset()` method sets AONPMC_E0_ROM_MISR2 to value 0"]
impl crate::Resettable for AonpmcE0RomMisr2Spec {}
