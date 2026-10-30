#[doc = "Register `REG_OTA_ADDR` reader"]
pub type R = crate::R<RegOtaAddrSpec>;
#[doc = "Field `BOOTLOADER_addr` reader - "]
pub type BootloaderAddrR = crate::FieldReader<u16>;
#[doc = "Field `rsvd2_sw` reader - "]
pub type Rsvd2SwR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootloader_addr(&self) -> BootloaderAddrR {
        BootloaderAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rsvd2_sw(&self) -> Rsvd2SwR {
        Rsvd2SwR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ota_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOtaAddrSpec;
impl crate::RegisterSpec for RegOtaAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ota_addr::R`](R) reader structure"]
impl crate::Readable for RegOtaAddrSpec {}
#[doc = "`reset()` method sets REG_OTA_ADDR to value 0xffff_ffff"]
impl crate::Resettable for RegOtaAddrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
