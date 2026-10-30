#[doc = "Register `REG_BOOT_VER0` reader"]
pub type R = crate::R<RegBootVer0Spec>;
#[doc = "Field `BOOTLOADER_VERSION0` reader - "]
pub type BootloaderVersion0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bootloader_version0(&self) -> BootloaderVersion0R {
        BootloaderVersion0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_boot_ver0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBootVer0Spec;
impl crate::RegisterSpec for RegBootVer0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_boot_ver0::R`](R) reader structure"]
impl crate::Readable for RegBootVer0Spec {}
#[doc = "`reset()` method sets REG_BOOT_VER0 to value 0xffff_ffff"]
impl crate::Resettable for RegBootVer0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
