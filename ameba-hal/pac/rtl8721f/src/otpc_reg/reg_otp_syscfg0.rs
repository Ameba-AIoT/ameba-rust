#[doc = "Register `REG_OTP_SYSCFG0` reader"]
pub type R = crate::R<RegOtpSyscfg0Spec>;
#[doc = "Register `REG_OTP_SYSCFG0` writer"]
pub type W = crate::W<RegOtpSyscfg0Spec>;
#[doc = "Field `sw_rsvd_00` reader - "]
pub type SwRsvd00R = crate::BitReader;
#[doc = "Field `sw_rsvd_00` writer - "]
pub type SwRsvd00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_SRC_SEL` reader - Choose Boot Src 0/3: boot from flash 1: boot from USB 2: boot from SDIO"]
pub type BootSrcSelR = crate::FieldReader;
#[doc = "Field `BOOT_SRC_SEL` writer - Choose Boot Src 0/3: boot from flash 1: boot from USB 2: boot from SDIO"]
pub type BootSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BT_Function_EN` reader - 0:disable 1:enable"]
pub type BtFunctionEnR = crate::BitReader;
#[doc = "Field `BT_Function_EN` writer - 0:disable 1:enable"]
pub type BtFunctionEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PINMUX` reader - 00:PINMUX Index 0 01:PINMUX Index 1 10:PINMUX Index 2 11:PINMUX Index 3"]
pub type SdioPinmuxR = crate::FieldReader;
#[doc = "Field `SDIO_PINMUX` writer - 00:PINMUX Index 0 01:PINMUX Index 1 10:PINMUX Index 2 11:PINMUX Index 3"]
pub type SdioPinmuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BOOT_INT_FLASH` reader - Boot from internal flash or external flash. 0/3: determined by power-on latch 1: internal flash 2: external flash"]
pub type BootIntFlashR = crate::FieldReader;
#[doc = "Field `BOOT_INT_FLASH` writer - Boot from internal flash or external flash. 0/3: determined by power-on latch 1: internal flash 2: external flash"]
pub type BootIntFlashW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sw_rsvd_01` reader - RSVD5 for SW usage"]
pub type SwRsvd01R = crate::FieldReader;
#[doc = "Field `sw_rsvd_01` writer - RSVD5 for SW usage"]
pub type SwRsvd01W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLASH_DEEP_SLEEP_EN` reader - Flash deepsleep enable"]
pub type FlashDeepSleepEnR = crate::BitReader;
#[doc = "Field `FLASH_DEEP_SLEEP_EN` writer - Flash deepsleep enable"]
pub type FlashDeepSleepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIC_ADDR_4BYTE_EN` reader - SPIC address 4-byte enable"]
pub type SpicAddr4byteEnR = crate::BitReader;
#[doc = "Field `SPIC_ADDR_4BYTE_EN` writer - SPIC address 4-byte enable"]
pub type SpicAddr4byteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIC_BOOT_SPEEDUP_DIS` reader - SPIC boot speed up disable"]
pub type SpicBootSpeedupDisR = crate::BitReader;
#[doc = "Field `SPIC_BOOT_SPEEDUP_DIS` writer - SPIC boot speed up disable"]
pub type SpicBootSpeedupDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_FLASH_BAUD_SEL` reader - Boot Flash clock selection. Baudrate = 40/(2*(1+x)) 0: 20M 1: 10M"]
pub type BootFlashBaudSelR = crate::BitReader;
#[doc = "Field `BOOT_FLASH_BAUD_SEL` writer - Boot Flash clock selection. Baudrate = 40/(2*(1+x)) 0: 20M 1: 10M"]
pub type BootFlashBaudSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND_PAGE_SIZE` reader - Nand FLASH Page size 0: 2KB 1: 4KB"]
pub type NandPageSizeR = crate::BitReader;
#[doc = "Field `NAND_PAGE_SIZE` writer - Nand FLASH Page size 0: 2KB 1: 4KB"]
pub type NandPageSizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_CNT_LMT_EN` reader - Enable boot count limit 0: disable 1: enable"]
pub type BootCntLmtEnR = crate::BitReader;
#[doc = "Field `BOOT_CNT_LMT_EN` writer - Enable boot count limit 0: disable 1: enable"]
pub type BootCntLmtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_NOR_FLASH` reader - Boot from Nor flash, or Nand flash, or try. 0/3: determined by power-on latch 1: nor flash 2: nand flash"]
pub type BootNorFlashR = crate::FieldReader;
#[doc = "Field `BOOT_NOR_FLASH` writer - Boot from Nor flash, or Nand flash, or try. 0/3: determined by power-on latch 1: nor flash 2: nand flash"]
pub type BootNorFlashW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIS_BOOT_LOG_EN` reader - Boot log disable"]
pub type DisBootLogEnR = crate::BitReader;
#[doc = "Field `DIS_BOOT_LOG_EN` writer - Boot log disable"]
pub type DisBootLogEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOW_BAUD_LOG_EN` reader - Loguart Baudrate selection 1: 115200bps 0: 1.5Mbps"]
pub type LowBaudLogEnR = crate::BitReader;
#[doc = "Field `LOW_BAUD_LOG_EN` writer - Loguart Baudrate selection 1: 115200bps 0: 1.5Mbps"]
pub type LowBaudLogEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOGIC_SECURE_BOOT_EN` reader - Secure boot enable"]
pub type LogicSecureBootEnR = crate::BitReader;
#[doc = "Field `LOGIC_SECURE_BOOT_EN` writer - Secure boot enable"]
pub type LogicSecureBootEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOGIC_RSIP_EN` reader - RSIP enable"]
pub type LogicRsipEnR = crate::BitReader;
#[doc = "Field `LOGIC_RSIP_EN` writer - RSIP enable"]
pub type LogicRsipEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOGIC_RDP_EN` reader - RDP enable"]
pub type LogicRdpEnR = crate::BitReader;
#[doc = "Field `LOGIC_RDP_EN` writer - RDP enable"]
pub type LogicRdpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_rsvd_02` reader - "]
pub type SwRsvd02R = crate::BitReader;
#[doc = "Field `sw_rsvd_02` writer - "]
pub type SwRsvd02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_CLK_CAL_EN` reader - SoC clock calibration for USB 0: disable 1: enable"]
pub type UsbClkCalEnR = crate::BitReader;
#[doc = "Field `USB_CLK_CAL_EN` writer - SoC clock calibration for USB 0: disable 1: enable"]
pub type UsbClkCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DOWNLOAD_EN` reader - USB download enable 0: disable 1: enable"]
pub type UsbDownloadEnR = crate::BitReader;
#[doc = "Field `USB_DOWNLOAD_EN` writer - USB download enable 0: disable 1: enable"]
pub type UsbDownloadEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_rsvd_00(&self) -> SwRsvd00R {
        SwRsvd00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Choose Boot Src 0/3: boot from flash 1: boot from USB 2: boot from SDIO"]
    #[inline(always)]
    pub fn boot_src_sel(&self) -> BootSrcSelR {
        BootSrcSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 0:disable 1:enable"]
    #[inline(always)]
    pub fn bt_function_en(&self) -> BtFunctionEnR {
        BtFunctionEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 00:PINMUX Index 0 01:PINMUX Index 1 10:PINMUX Index 2 11:PINMUX Index 3"]
    #[inline(always)]
    pub fn sdio_pinmux(&self) -> SdioPinmuxR {
        SdioPinmuxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Boot from internal flash or external flash. 0/3: determined by power-on latch 1: internal flash 2: external flash"]
    #[inline(always)]
    pub fn boot_int_flash(&self) -> BootIntFlashR {
        BootIntFlashR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - RSVD5 for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd_01(&self) -> SwRsvd01R {
        SwRsvd01R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Flash deepsleep enable"]
    #[inline(always)]
    pub fn flash_deep_sleep_en(&self) -> FlashDeepSleepEnR {
        FlashDeepSleepEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPIC address 4-byte enable"]
    #[inline(always)]
    pub fn spic_addr_4byte_en(&self) -> SpicAddr4byteEnR {
        SpicAddr4byteEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SPIC boot speed up disable"]
    #[inline(always)]
    pub fn spic_boot_speedup_dis(&self) -> SpicBootSpeedupDisR {
        SpicBootSpeedupDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Boot Flash clock selection. Baudrate = 40/(2*(1+x)) 0: 20M 1: 10M"]
    #[inline(always)]
    pub fn boot_flash_baud_sel(&self) -> BootFlashBaudSelR {
        BootFlashBaudSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Nand FLASH Page size 0: 2KB 1: 4KB"]
    #[inline(always)]
    pub fn nand_page_size(&self) -> NandPageSizeR {
        NandPageSizeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable boot count limit 0: disable 1: enable"]
    #[inline(always)]
    pub fn boot_cnt_lmt_en(&self) -> BootCntLmtEnR {
        BootCntLmtEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Boot from Nor flash, or Nand flash, or try. 0/3: determined by power-on latch 1: nor flash 2: nand flash"]
    #[inline(always)]
    pub fn boot_nor_flash(&self) -> BootNorFlashR {
        BootNorFlashR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Boot log disable"]
    #[inline(always)]
    pub fn dis_boot_log_en(&self) -> DisBootLogEnR {
        DisBootLogEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Loguart Baudrate selection 1: 115200bps 0: 1.5Mbps"]
    #[inline(always)]
    pub fn low_baud_log_en(&self) -> LowBaudLogEnR {
        LowBaudLogEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Secure boot enable"]
    #[inline(always)]
    pub fn logic_secure_boot_en(&self) -> LogicSecureBootEnR {
        LogicSecureBootEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RSIP enable"]
    #[inline(always)]
    pub fn logic_rsip_en(&self) -> LogicRsipEnR {
        LogicRsipEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RDP enable"]
    #[inline(always)]
    pub fn logic_rdp_en(&self) -> LogicRdpEnR {
        LogicRdpEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sw_rsvd_02(&self) -> SwRsvd02R {
        SwRsvd02R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SoC clock calibration for USB 0: disable 1: enable"]
    #[inline(always)]
    pub fn usb_clk_cal_en(&self) -> UsbClkCalEnR {
        UsbClkCalEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB download enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn usb_download_en(&self) -> UsbDownloadEnR {
        UsbDownloadEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_rsvd_00(&mut self) -> SwRsvd00W<'_, RegOtpSyscfg0Spec> {
        SwRsvd00W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Choose Boot Src 0/3: boot from flash 1: boot from USB 2: boot from SDIO"]
    #[inline(always)]
    pub fn boot_src_sel(&mut self) -> BootSrcSelW<'_, RegOtpSyscfg0Spec> {
        BootSrcSelW::new(self, 1)
    }
    #[doc = "Bit 3 - 0:disable 1:enable"]
    #[inline(always)]
    pub fn bt_function_en(&mut self) -> BtFunctionEnW<'_, RegOtpSyscfg0Spec> {
        BtFunctionEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 00:PINMUX Index 0 01:PINMUX Index 1 10:PINMUX Index 2 11:PINMUX Index 3"]
    #[inline(always)]
    pub fn sdio_pinmux(&mut self) -> SdioPinmuxW<'_, RegOtpSyscfg0Spec> {
        SdioPinmuxW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Boot from internal flash or external flash. 0/3: determined by power-on latch 1: internal flash 2: external flash"]
    #[inline(always)]
    pub fn boot_int_flash(&mut self) -> BootIntFlashW<'_, RegOtpSyscfg0Spec> {
        BootIntFlashW::new(self, 6)
    }
    #[doc = "Bits 8:15 - RSVD5 for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd_01(&mut self) -> SwRsvd01W<'_, RegOtpSyscfg0Spec> {
        SwRsvd01W::new(self, 8)
    }
    #[doc = "Bit 16 - Flash deepsleep enable"]
    #[inline(always)]
    pub fn flash_deep_sleep_en(&mut self) -> FlashDeepSleepEnW<'_, RegOtpSyscfg0Spec> {
        FlashDeepSleepEnW::new(self, 16)
    }
    #[doc = "Bit 17 - SPIC address 4-byte enable"]
    #[inline(always)]
    pub fn spic_addr_4byte_en(&mut self) -> SpicAddr4byteEnW<'_, RegOtpSyscfg0Spec> {
        SpicAddr4byteEnW::new(self, 17)
    }
    #[doc = "Bit 18 - SPIC boot speed up disable"]
    #[inline(always)]
    pub fn spic_boot_speedup_dis(&mut self) -> SpicBootSpeedupDisW<'_, RegOtpSyscfg0Spec> {
        SpicBootSpeedupDisW::new(self, 18)
    }
    #[doc = "Bit 19 - Boot Flash clock selection. Baudrate = 40/(2*(1+x)) 0: 20M 1: 10M"]
    #[inline(always)]
    pub fn boot_flash_baud_sel(&mut self) -> BootFlashBaudSelW<'_, RegOtpSyscfg0Spec> {
        BootFlashBaudSelW::new(self, 19)
    }
    #[doc = "Bit 20 - Nand FLASH Page size 0: 2KB 1: 4KB"]
    #[inline(always)]
    pub fn nand_page_size(&mut self) -> NandPageSizeW<'_, RegOtpSyscfg0Spec> {
        NandPageSizeW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable boot count limit 0: disable 1: enable"]
    #[inline(always)]
    pub fn boot_cnt_lmt_en(&mut self) -> BootCntLmtEnW<'_, RegOtpSyscfg0Spec> {
        BootCntLmtEnW::new(self, 21)
    }
    #[doc = "Bits 22:23 - Boot from Nor flash, or Nand flash, or try. 0/3: determined by power-on latch 1: nor flash 2: nand flash"]
    #[inline(always)]
    pub fn boot_nor_flash(&mut self) -> BootNorFlashW<'_, RegOtpSyscfg0Spec> {
        BootNorFlashW::new(self, 22)
    }
    #[doc = "Bit 24 - Boot log disable"]
    #[inline(always)]
    pub fn dis_boot_log_en(&mut self) -> DisBootLogEnW<'_, RegOtpSyscfg0Spec> {
        DisBootLogEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Loguart Baudrate selection 1: 115200bps 0: 1.5Mbps"]
    #[inline(always)]
    pub fn low_baud_log_en(&mut self) -> LowBaudLogEnW<'_, RegOtpSyscfg0Spec> {
        LowBaudLogEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Secure boot enable"]
    #[inline(always)]
    pub fn logic_secure_boot_en(&mut self) -> LogicSecureBootEnW<'_, RegOtpSyscfg0Spec> {
        LogicSecureBootEnW::new(self, 26)
    }
    #[doc = "Bit 27 - RSIP enable"]
    #[inline(always)]
    pub fn logic_rsip_en(&mut self) -> LogicRsipEnW<'_, RegOtpSyscfg0Spec> {
        LogicRsipEnW::new(self, 27)
    }
    #[doc = "Bit 28 - RDP enable"]
    #[inline(always)]
    pub fn logic_rdp_en(&mut self) -> LogicRdpEnW<'_, RegOtpSyscfg0Spec> {
        LogicRdpEnW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sw_rsvd_02(&mut self) -> SwRsvd02W<'_, RegOtpSyscfg0Spec> {
        SwRsvd02W::new(self, 29)
    }
    #[doc = "Bit 30 - SoC clock calibration for USB 0: disable 1: enable"]
    #[inline(always)]
    pub fn usb_clk_cal_en(&mut self) -> UsbClkCalEnW<'_, RegOtpSyscfg0Spec> {
        UsbClkCalEnW::new(self, 30)
    }
    #[doc = "Bit 31 - USB download enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn usb_download_en(&mut self) -> UsbDownloadEnW<'_, RegOtpSyscfg0Spec> {
        UsbDownloadEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_otp_syscfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_otp_syscfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOtpSyscfg0Spec;
impl crate::RegisterSpec for RegOtpSyscfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_otp_syscfg0::R`](R) reader structure"]
impl crate::Readable for RegOtpSyscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_otp_syscfg0::W`](W) writer structure"]
impl crate::Writable for RegOtpSyscfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OTP_SYSCFG0 to value 0"]
impl crate::Resettable for RegOtpSyscfg0Spec {}
