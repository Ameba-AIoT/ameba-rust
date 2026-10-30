#[doc = "Register `PKE_E1_ROM_MISR5` reader"]
pub type R = crate::R<PkeE1RomMisr5Spec>;
#[doc = "Field `pke_e1_misr_dout_63_32` reader - "]
pub type PkeE1MisrDout63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pke_e1_misr_dout_63_32(&self) -> PkeE1MisrDout63_32R {
        PkeE1MisrDout63_32R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_e1_rom_misr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeE1RomMisr5Spec;
impl crate::RegisterSpec for PkeE1RomMisr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_e1_rom_misr5::R`](R) reader structure"]
impl crate::Readable for PkeE1RomMisr5Spec {}
#[doc = "`reset()` method sets PKE_E1_ROM_MISR5 to value 0"]
impl crate::Resettable for PkeE1RomMisr5Spec {}
