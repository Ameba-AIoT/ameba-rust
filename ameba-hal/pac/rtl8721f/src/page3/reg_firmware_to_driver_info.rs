#[doc = "Register `REG_FIRMWARE_TO_DRIVER_INFO` reader"]
pub type R = crate::R<RegFirmwareToDriverInfoSpec>;
#[doc = "Field `BIT_FW2DRV_AHB` reader - FIRMWARE TO DRIVER MESSAGE"]
pub type BitFw2drvAhbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIRMWARE TO DRIVER MESSAGE"]
    #[inline(always)]
    pub fn bit_fw2drv_ahb(&self) -> BitFw2drvAhbR {
        BitFw2drvAhbR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_firmware_to_driver_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFirmwareToDriverInfoSpec;
impl crate::RegisterSpec for RegFirmwareToDriverInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_firmware_to_driver_info::R`](R) reader structure"]
impl crate::Readable for RegFirmwareToDriverInfoSpec {}
#[doc = "`reset()` method sets REG_FIRMWARE_TO_DRIVER_INFO to value 0"]
impl crate::Resettable for RegFirmwareToDriverInfoSpec {}
