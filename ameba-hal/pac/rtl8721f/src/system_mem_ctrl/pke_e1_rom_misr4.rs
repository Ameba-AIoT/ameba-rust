#[doc = "Register `PKE_E1_ROM_MISR4` reader"]
pub type R = crate::R<PkeE1RomMisr4Spec>;
#[doc = "Field `pke_e1_misr_dout_31_0` reader - "]
pub type PkeE1MisrDout31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pke_e1_misr_dout_31_0(&self) -> PkeE1MisrDout31_0R {
        PkeE1MisrDout31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_e1_rom_misr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeE1RomMisr4Spec;
impl crate::RegisterSpec for PkeE1RomMisr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_e1_rom_misr4::R`](R) reader structure"]
impl crate::Readable for PkeE1RomMisr4Spec {}
#[doc = "`reset()` method sets PKE_E1_ROM_MISR4 to value 0"]
impl crate::Resettable for PkeE1RomMisr4Spec {}
