#[doc = "Register `REG_DRVIER_TO_FIRMWARE_INFO` reader"]
pub type R = crate::R<RegDrvierToFirmwareInfoSpec>;
#[doc = "Register `REG_DRVIER_TO_FIRMWARE_INFO` writer"]
pub type W = crate::W<RegDrvierToFirmwareInfoSpec>;
#[doc = "Field `BIT_DRV2FW_AHB` reader - DRIVER TO FIRMWARE MESSAGE"]
pub type BitDrv2fwAhbR = crate::FieldReader<u32>;
#[doc = "Field `BIT_DRV2FW_AHB` writer - DRIVER TO FIRMWARE MESSAGE"]
pub type BitDrv2fwAhbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DRIVER TO FIRMWARE MESSAGE"]
    #[inline(always)]
    pub fn bit_drv2fw_ahb(&self) -> BitDrv2fwAhbR {
        BitDrv2fwAhbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DRIVER TO FIRMWARE MESSAGE"]
    #[inline(always)]
    pub fn bit_drv2fw_ahb(&mut self) -> BitDrv2fwAhbW<'_, RegDrvierToFirmwareInfoSpec> {
        BitDrv2fwAhbW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_drvier_to_firmware_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_drvier_to_firmware_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDrvierToFirmwareInfoSpec;
impl crate::RegisterSpec for RegDrvierToFirmwareInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_drvier_to_firmware_info::R`](R) reader structure"]
impl crate::Readable for RegDrvierToFirmwareInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_drvier_to_firmware_info::W`](W) writer structure"]
impl crate::Writable for RegDrvierToFirmwareInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DRVIER_TO_FIRMWARE_INFO to value 0"]
impl crate::Resettable for RegDrvierToFirmwareInfoSpec {}
