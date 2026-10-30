#[doc = "Register `KM4TZ_ITCM0_E0_ROM_MISR4` reader"]
pub type R = crate::R<Km4tzItcm0E0RomMisr4Spec>;
#[doc = "Field `km4tz_itcm0_e0_misr_dout_63_32` reader - "]
pub type Km4tzItcm0E0MisrDout63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn km4tz_itcm0_e0_misr_dout_63_32(&self) -> Km4tzItcm0E0MisrDout63_32R {
        Km4tzItcm0E0MisrDout63_32R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`km4tz_itcm0_e0_rom_misr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Km4tzItcm0E0RomMisr4Spec;
impl crate::RegisterSpec for Km4tzItcm0E0RomMisr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`km4tz_itcm0_e0_rom_misr4::R`](R) reader structure"]
impl crate::Readable for Km4tzItcm0E0RomMisr4Spec {}
#[doc = "`reset()` method sets KM4TZ_ITCM0_E0_ROM_MISR4 to value 0"]
impl crate::Resettable for Km4tzItcm0E0RomMisr4Spec {}
