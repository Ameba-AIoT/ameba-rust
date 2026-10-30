#[doc = "Register `REG_BOOT_VER1` reader"]
pub type R = crate::R<RegBootVer1Spec>;
#[doc = "Field `BOOTLOADER_VERSION1` reader - "]
pub type BootloaderVersion1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bootloader_version1(&self) -> BootloaderVersion1R {
        BootloaderVersion1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_boot_ver1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBootVer1Spec;
impl crate::RegisterSpec for RegBootVer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_boot_ver1::R`](R) reader structure"]
impl crate::Readable for RegBootVer1Spec {}
#[doc = "`reset()` method sets REG_BOOT_VER1 to value 0xffff_ffff"]
impl crate::Resettable for RegBootVer1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
