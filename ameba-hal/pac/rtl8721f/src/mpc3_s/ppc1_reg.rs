#[doc = "Register `PPC1_REG` reader"]
pub type R = crate::R<Ppc1RegSpec>;
#[doc = "Register `PPC1_REG` writer"]
pub type W = crate::W<Ppc1RegSpec>;
#[doc = "Field `WIFI_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type WifiCfgR = crate::BitReader;
#[doc = "Field `WIFI_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type WifiCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type UsbCfgR = crate::BitReader;
#[doc = "Field `USB_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type UsbCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKE_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type PkeCfgR = crate::BitReader;
#[doc = "Field `PKE_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type PkeCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDC_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type LcdcCfgR = crate::BitReader;
#[doc = "Field `LCDC_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type LcdcCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MJPEG_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type MjpegCfgR = crate::BitReader;
#[doc = "Field `MJPEG_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type MjpegCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_DEVICE_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type SdioDeviceCfgR = crate::BitReader;
#[doc = "Field `SDIO_DEVICE_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type SdioDeviceCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HOST_DFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type SdioHostDfgR = crate::BitReader;
#[doc = "Field `SDIO_HOST_DFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type SdioHostDfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPORT0_I2S` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Sport0I2sR = crate::BitReader;
#[doc = "Field `SPORT0_I2S` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Sport0I2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMII_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type RmiiCfgR = crate::BitReader;
#[doc = "Field `RMII_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type RmiiCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type EccCfgR = crate::BitReader;
#[doc = "Field `ECC_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type EccCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIC_USER_MODE` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type SpicUserModeR = crate::BitReader;
#[doc = "Field `SPIC_USER_MODE` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type SpicUserModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_SPIC_USERMODE` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type PsramSpicUsermodeR = crate::BitReader;
#[doc = "Field `PSRAM_SPIC_USERMODE` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type PsramSpicUsermodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_PHY` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type PsramPhyR = crate::BitReader;
#[doc = "Field `PSRAM_PHY` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type PsramPhyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA0_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Gdma0CfgR = crate::BitReader;
#[doc = "Field `GDMA0_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Gdma0CfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Spi0R = crate::BitReader;
#[doc = "Field `SPI0` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type AesCfgR = crate::BitReader;
#[doc = "Field `AES_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type AesCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type ShaCfgR = crate::BitReader;
#[doc = "Field `SHA_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type ShaCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_SHA_DMA` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type AesShaDmaR = crate::BitReader;
#[doc = "Field `AES_SHA_DMA` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type AesShaDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPE_CFG` reader - 1: Non-secure attribution 0: Secure attribution"]
pub type PpeCfgR = crate::BitReader;
#[doc = "Field `PPE_CFG` writer - 1: Non-secure attribution 0: Secure attribution"]
pub type PpeCfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn wifi_cfg(&self) -> WifiCfgR {
        WifiCfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn usb_cfg(&self) -> UsbCfgR {
        UsbCfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn pke_cfg(&self) -> PkeCfgR {
        PkeCfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn lcdc_cfg(&self) -> LcdcCfgR {
        LcdcCfgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn mjpeg_cfg(&self) -> MjpegCfgR {
        MjpegCfgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sdio_device_cfg(&self) -> SdioDeviceCfgR {
        SdioDeviceCfgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sdio_host_dfg(&self) -> SdioHostDfgR {
        SdioHostDfgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sport0_i2s(&self) -> Sport0I2sR {
        Sport0I2sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn rmii_cfg(&self) -> RmiiCfgR {
        RmiiCfgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn ecc_cfg(&self) -> EccCfgR {
        EccCfgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn spic_user_mode(&self) -> SpicUserModeR {
        SpicUserModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn psram_spic_usermode(&self) -> PsramSpicUsermodeR {
        PsramSpicUsermodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn psram_phy(&self) -> PsramPhyR {
        PsramPhyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn gdma0_cfg(&self) -> Gdma0CfgR {
        Gdma0CfgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn spi0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn aes_cfg(&self) -> AesCfgR {
        AesCfgR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sha_cfg(&self) -> ShaCfgR {
        ShaCfgR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn aes_sha_dma(&self) -> AesShaDmaR {
        AesShaDmaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn ppe_cfg(&self) -> PpeCfgR {
        PpeCfgR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn wifi_cfg(&mut self) -> WifiCfgW<'_, Ppc1RegSpec> {
        WifiCfgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn usb_cfg(&mut self) -> UsbCfgW<'_, Ppc1RegSpec> {
        UsbCfgW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn pke_cfg(&mut self) -> PkeCfgW<'_, Ppc1RegSpec> {
        PkeCfgW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn lcdc_cfg(&mut self) -> LcdcCfgW<'_, Ppc1RegSpec> {
        LcdcCfgW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn mjpeg_cfg(&mut self) -> MjpegCfgW<'_, Ppc1RegSpec> {
        MjpegCfgW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sdio_device_cfg(&mut self) -> SdioDeviceCfgW<'_, Ppc1RegSpec> {
        SdioDeviceCfgW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sdio_host_dfg(&mut self) -> SdioHostDfgW<'_, Ppc1RegSpec> {
        SdioHostDfgW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sport0_i2s(&mut self) -> Sport0I2sW<'_, Ppc1RegSpec> {
        Sport0I2sW::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn rmii_cfg(&mut self) -> RmiiCfgW<'_, Ppc1RegSpec> {
        RmiiCfgW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn ecc_cfg(&mut self) -> EccCfgW<'_, Ppc1RegSpec> {
        EccCfgW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn spic_user_mode(&mut self) -> SpicUserModeW<'_, Ppc1RegSpec> {
        SpicUserModeW::new(self, 10)
    }
    #[doc = "Bit 11 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn psram_spic_usermode(&mut self) -> PsramSpicUsermodeW<'_, Ppc1RegSpec> {
        PsramSpicUsermodeW::new(self, 11)
    }
    #[doc = "Bit 12 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn psram_phy(&mut self) -> PsramPhyW<'_, Ppc1RegSpec> {
        PsramPhyW::new(self, 12)
    }
    #[doc = "Bit 13 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn gdma0_cfg(&mut self) -> Gdma0CfgW<'_, Ppc1RegSpec> {
        Gdma0CfgW::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn spi0(&mut self) -> Spi0W<'_, Ppc1RegSpec> {
        Spi0W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Ppc1RegSpec> {
        Spi1W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn aes_cfg(&mut self) -> AesCfgW<'_, Ppc1RegSpec> {
        AesCfgW::new(self, 16)
    }
    #[doc = "Bit 17 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn sha_cfg(&mut self) -> ShaCfgW<'_, Ppc1RegSpec> {
        ShaCfgW::new(self, 17)
    }
    #[doc = "Bit 18 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn aes_sha_dma(&mut self) -> AesShaDmaW<'_, Ppc1RegSpec> {
        AesShaDmaW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Non-secure attribution 0: Secure attribution"]
    #[inline(always)]
    pub fn ppe_cfg(&mut self) -> PpeCfgW<'_, Ppc1RegSpec> {
        PpeCfgW::new(self, 19)
    }
}
#[doc = "PPC1 configure register0\n\nYou can [`read`](crate::Reg::read) this register and get [`ppc1_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppc1_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppc1RegSpec;
impl crate::RegisterSpec for Ppc1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppc1_reg::R`](R) reader structure"]
impl crate::Readable for Ppc1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`ppc1_reg::W`](W) writer structure"]
impl crate::Writable for Ppc1RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPC1_REG to value 0"]
impl crate::Resettable for Ppc1RegSpec {}
