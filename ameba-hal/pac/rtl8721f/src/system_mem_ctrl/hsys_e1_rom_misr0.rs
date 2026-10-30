#[doc = "Register `HSYS_E1_ROM_MISR0` reader"]
pub type R = crate::R<HsysE1RomMisr0Spec>;
#[doc = "Field `hsys_e1_misr_dout_31_0` reader - "]
pub type HsysE1MisrDout31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hsys_e1_misr_dout_31_0(&self) -> HsysE1MisrDout31_0R {
        HsysE1MisrDout31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hsys_e1_rom_misr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsysE1RomMisr0Spec;
impl crate::RegisterSpec for HsysE1RomMisr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsys_e1_rom_misr0::R`](R) reader structure"]
impl crate::Readable for HsysE1RomMisr0Spec {}
#[doc = "`reset()` method sets HSYS_E1_ROM_MISR0 to value 0"]
impl crate::Resettable for HsysE1RomMisr0Spec {}
