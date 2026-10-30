#[doc = "Register `HSYS_E1_ROM_MISR1` reader"]
pub type R = crate::R<HsysE1RomMisr1Spec>;
#[doc = "Field `hsys_e1_misr_dout_63_32` reader - "]
pub type HsysE1MisrDout63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hsys_e1_misr_dout_63_32(&self) -> HsysE1MisrDout63_32R {
        HsysE1MisrDout63_32R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hsys_e1_rom_misr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsysE1RomMisr1Spec;
impl crate::RegisterSpec for HsysE1RomMisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsys_e1_rom_misr1::R`](R) reader structure"]
impl crate::Readable for HsysE1RomMisr1Spec {}
#[doc = "`reset()` method sets HSYS_E1_ROM_MISR1 to value 0"]
impl crate::Resettable for HsysE1RomMisr1Spec {}
