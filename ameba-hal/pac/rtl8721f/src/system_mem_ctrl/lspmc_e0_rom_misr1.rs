#[doc = "Register `LSPMC_E0_ROM_MISR1` reader"]
pub type R = crate::R<LspmcE0RomMisr1Spec>;
#[doc = "Field `lspmc_e0_misr_dout_31_0` reader - "]
pub type LspmcE0MisrDout31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lspmc_e0_misr_dout_31_0(&self) -> LspmcE0MisrDout31_0R {
        LspmcE0MisrDout31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lspmc_e0_rom_misr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LspmcE0RomMisr1Spec;
impl crate::RegisterSpec for LspmcE0RomMisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lspmc_e0_rom_misr1::R`](R) reader structure"]
impl crate::Readable for LspmcE0RomMisr1Spec {}
#[doc = "`reset()` method sets LSPMC_E0_ROM_MISR1 to value 0"]
impl crate::Resettable for LspmcE0RomMisr1Spec {}
