#[doc = "Register `DEVICE_INFO` reader"]
pub type R = crate::R<DeviceInfoSpec>;
#[doc = "Register `DEVICE_INFO` writer"]
pub type W = crate::W<DeviceInfoSpec>;
#[doc = "Field `PAGE_SIZE` reader - The page size of device (byte) = 2^(PAGE_SIZE) The minimum value of page size is 128-byte. Please set PAGE_SIZE >= 7. ex. If the page size of device is 256-byte, PAGE_SIZE should be set to 8. 0x7: 128-byte 0x8: 256-byte 0x9: 512-byte 0xa: 1024-byte 0xb: 2048-byte 0xc: 4096-byte"]
pub type PageSizeR = crate::FieldReader;
#[doc = "Field `PAGE_SIZE` writer - The page size of device (byte) = 2^(PAGE_SIZE) The minimum value of page size is 128-byte. Please set PAGE_SIZE >= 7. ex. If the page size of device is 256-byte, PAGE_SIZE should be set to 8. 0x7: 128-byte 0x8: 256-byte 0x9: 512-byte 0xa: 1024-byte 0xb: 2048-byte 0xc: 4096-byte"]
pub type PageSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WR_PAGE_EN` reader - Enable SPIC to chop the burst write command across page boundaries in auto mode."]
pub type WrPageEnR = crate::BitReader;
#[doc = "Field `WR_PAGE_EN` writer - Enable SPIC to chop the burst write command across page boundaries in auto mode."]
pub type WrPageEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_PAGE_EN` reader - Enable SPIC to chop the burst read command across page boundaries in auto mode."]
pub type RdPageEnR = crate::BitReader;
#[doc = "Field `RD_PAGE_EN` writer - Enable SPIC to chop the burst read command across page boundaries in auto mode."]
pub type RdPageEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOR_FLASH` reader - Set (1) when the SPI device is NOR Flash."]
pub type NorFlashR = crate::BitReader;
#[doc = "Field `NOR_FLASH` writer - Set (1) when the SPI device is NOR Flash."]
pub type NorFlashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND_FLASH` reader - Set (1) when the SPI device is NAND Flash. (not support yet) Available only if configuration of NAND_EN is defined."]
pub type NandFlashR = crate::BitReader;
#[doc = "Field `NAND_FLASH` writer - Set (1) when the SPI device is NAND Flash. (not support yet) Available only if configuration of NAND_EN is defined."]
pub type NandFlashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM` reader - Set (1) when the SPI device is PSRAM. PSRAM will disable WEN and RDSR in auto write. Available only if configuration of PSRAM_EN is defined."]
pub type PsramR = crate::BitReader;
#[doc = "Field `PSRAM` writer - Set (1) when the SPI device is PSRAM. PSRAM will disable WEN and RDSR in auto write. Available only if configuration of PSRAM_EN is defined."]
pub type PsramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEDEC_P2CMF` reader - Set (1) when the SPI cmd is JEDEC Profile 2.0 Command Modifier Formats. (6-byte command and address) Available only if configuration of PSRAM_EN is defined."]
pub type JedecP2cmfR = crate::BitReader;
#[doc = "Field `JEDEC_P2CMF` writer - Set (1) when the SPI cmd is JEDEC Profile 2.0 Command Modifier Formats. (6-byte command and address) Available only if configuration of PSRAM_EN is defined."]
pub type JedecP2cmfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_UNIT_2B` reader - Set (1) when the SPI device stores 2-byte data for each address. Available only if configuration of PSRAM_EN is defined."]
pub type DataUnit2bR = crate::BitReader;
#[doc = "Field `DATA_UNIT_2B` writer - Set (1) when the SPI device stores 2-byte data for each address. Available only if configuration of PSRAM_EN is defined."]
pub type DataUnit2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQ16_DATA_CH` reader - Set (1) when DQ16 PSRAM uses 16-bit data channel. When DQ16_DATA_CH = 1'b1, set CTRLR0\\[19:18\\] DATA_CH = 2'b00."]
pub type Dq16DataChR = crate::BitReader;
#[doc = "Field `DQ16_DATA_CH` writer - Set (1) when DQ16 PSRAM uses 16-bit data channel. When DQ16_DATA_CH = 1'b1, set CTRLR0\\[19:18\\] DATA_CH = 2'b00."]
pub type Dq16DataChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_UNIT_4B` reader - Set (1) when the SPI device stores 4-byte data for each address. Available only if configuration of PSRAM_EN is defined."]
pub type DataUnit4bR = crate::BitReader;
#[doc = "Field `DATA_UNIT_4B` writer - Set (1) when the SPI device stores 4-byte data for each address. Available only if configuration of PSRAM_EN is defined."]
pub type DataUnit4bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The page size of device (byte) = 2^(PAGE_SIZE) The minimum value of page size is 128-byte. Please set PAGE_SIZE >= 7. ex. If the page size of device is 256-byte, PAGE_SIZE should be set to 8. 0x7: 128-byte 0x8: 256-byte 0x9: 512-byte 0xa: 1024-byte 0xb: 2048-byte 0xc: 4096-byte"]
    #[inline(always)]
    pub fn page_size(&self) -> PageSizeR {
        PageSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable SPIC to chop the burst write command across page boundaries in auto mode."]
    #[inline(always)]
    pub fn wr_page_en(&self) -> WrPageEnR {
        WrPageEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable SPIC to chop the burst read command across page boundaries in auto mode."]
    #[inline(always)]
    pub fn rd_page_en(&self) -> RdPageEnR {
        RdPageEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Set (1) when the SPI device is NOR Flash."]
    #[inline(always)]
    pub fn nor_flash(&self) -> NorFlashR {
        NorFlashR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set (1) when the SPI device is NAND Flash. (not support yet) Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn nand_flash(&self) -> NandFlashR {
        NandFlashR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set (1) when the SPI device is PSRAM. PSRAM will disable WEN and RDSR in auto write. Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn psram(&self) -> PsramR {
        PsramR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set (1) when the SPI cmd is JEDEC Profile 2.0 Command Modifier Formats. (6-byte command and address) Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn jedec_p2cmf(&self) -> JedecP2cmfR {
        JedecP2cmfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set (1) when the SPI device stores 2-byte data for each address. Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn data_unit_2b(&self) -> DataUnit2bR {
        DataUnit2bR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set (1) when DQ16 PSRAM uses 16-bit data channel. When DQ16_DATA_CH = 1'b1, set CTRLR0\\[19:18\\] DATA_CH = 2'b00."]
    #[inline(always)]
    pub fn dq16_data_ch(&self) -> Dq16DataChR {
        Dq16DataChR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set (1) when the SPI device stores 4-byte data for each address. Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn data_unit_4b(&self) -> DataUnit4bR {
        DataUnit4bR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The page size of device (byte) = 2^(PAGE_SIZE) The minimum value of page size is 128-byte. Please set PAGE_SIZE >= 7. ex. If the page size of device is 256-byte, PAGE_SIZE should be set to 8. 0x7: 128-byte 0x8: 256-byte 0x9: 512-byte 0xa: 1024-byte 0xb: 2048-byte 0xc: 4096-byte"]
    #[inline(always)]
    pub fn page_size(&mut self) -> PageSizeW<'_, DeviceInfoSpec> {
        PageSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - Enable SPIC to chop the burst write command across page boundaries in auto mode."]
    #[inline(always)]
    pub fn wr_page_en(&mut self) -> WrPageEnW<'_, DeviceInfoSpec> {
        WrPageEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable SPIC to chop the burst read command across page boundaries in auto mode."]
    #[inline(always)]
    pub fn rd_page_en(&mut self) -> RdPageEnW<'_, DeviceInfoSpec> {
        RdPageEnW::new(self, 5)
    }
    #[doc = "Bit 8 - Set (1) when the SPI device is NOR Flash."]
    #[inline(always)]
    pub fn nor_flash(&mut self) -> NorFlashW<'_, DeviceInfoSpec> {
        NorFlashW::new(self, 8)
    }
    #[doc = "Bit 9 - Set (1) when the SPI device is NAND Flash. (not support yet) Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn nand_flash(&mut self) -> NandFlashW<'_, DeviceInfoSpec> {
        NandFlashW::new(self, 9)
    }
    #[doc = "Bit 10 - Set (1) when the SPI device is PSRAM. PSRAM will disable WEN and RDSR in auto write. Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn psram(&mut self) -> PsramW<'_, DeviceInfoSpec> {
        PsramW::new(self, 10)
    }
    #[doc = "Bit 11 - Set (1) when the SPI cmd is JEDEC Profile 2.0 Command Modifier Formats. (6-byte command and address) Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn jedec_p2cmf(&mut self) -> JedecP2cmfW<'_, DeviceInfoSpec> {
        JedecP2cmfW::new(self, 11)
    }
    #[doc = "Bit 12 - Set (1) when the SPI device stores 2-byte data for each address. Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn data_unit_2b(&mut self) -> DataUnit2bW<'_, DeviceInfoSpec> {
        DataUnit2bW::new(self, 12)
    }
    #[doc = "Bit 13 - Set (1) when DQ16 PSRAM uses 16-bit data channel. When DQ16_DATA_CH = 1'b1, set CTRLR0\\[19:18\\] DATA_CH = 2'b00."]
    #[inline(always)]
    pub fn dq16_data_ch(&mut self) -> Dq16DataChW<'_, DeviceInfoSpec> {
        Dq16DataChW::new(self, 13)
    }
    #[doc = "Bit 14 - Set (1) when the SPI device stores 4-byte data for each address. Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn data_unit_4b(&mut self) -> DataUnit4bW<'_, DeviceInfoSpec> {
        DataUnit4bW::new(self, 14)
    }
}
#[doc = "Device info\n\nYou can [`read`](crate::Reg::read) this register and get [`device_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`device_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceInfoSpec;
impl crate::RegisterSpec for DeviceInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_info::R`](R) reader structure"]
impl crate::Readable for DeviceInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`device_info::W`](W) writer structure"]
impl crate::Writable for DeviceInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVICE_INFO to value 0x0118"]
impl crate::Resettable for DeviceInfoSpec {
    const RESET_VALUE: u32 = 0x0118;
}
