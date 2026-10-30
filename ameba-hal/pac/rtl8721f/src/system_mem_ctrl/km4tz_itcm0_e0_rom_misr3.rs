#[doc = "Register `KM4TZ_ITCM0_E0_ROM_MISR3` reader"]
pub type R = crate::R<Km4tzItcm0E0RomMisr3Spec>;
#[doc = "Field `km4tz_itcm0_e0_misr_dout_31_0` reader - "]
pub type Km4tzItcm0E0MisrDout31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn km4tz_itcm0_e0_misr_dout_31_0(&self) -> Km4tzItcm0E0MisrDout31_0R {
        Km4tzItcm0E0MisrDout31_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`km4tz_itcm0_e0_rom_misr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Km4tzItcm0E0RomMisr3Spec;
impl crate::RegisterSpec for Km4tzItcm0E0RomMisr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`km4tz_itcm0_e0_rom_misr3::R`](R) reader structure"]
impl crate::Readable for Km4tzItcm0E0RomMisr3Spec {}
#[doc = "`reset()` method sets KM4TZ_ITCM0_E0_ROM_MISR3 to value 0"]
impl crate::Resettable for Km4tzItcm0E0RomMisr3Spec {}
