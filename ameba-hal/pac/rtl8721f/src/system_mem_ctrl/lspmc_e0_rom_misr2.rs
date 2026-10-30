#[doc = "Register `LSPMC_E0_ROM_MISR2` reader"]
pub type R = crate::R<LspmcE0RomMisr2Spec>;
#[doc = "Field `lspmc_e0_misr_dout_35_32` reader - "]
pub type LspmcE0MisrDout35_32R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn lspmc_e0_misr_dout_35_32(&self) -> LspmcE0MisrDout35_32R {
        LspmcE0MisrDout35_32R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lspmc_e0_rom_misr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LspmcE0RomMisr2Spec;
impl crate::RegisterSpec for LspmcE0RomMisr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lspmc_e0_rom_misr2::R`](R) reader structure"]
impl crate::Readable for LspmcE0RomMisr2Spec {}
#[doc = "`reset()` method sets LSPMC_E0_ROM_MISR2 to value 0"]
impl crate::Resettable for LspmcE0RomMisr2Spec {}
