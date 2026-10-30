#[doc = "Register `REG_SEC_BPC_CTRL` reader"]
pub type R = crate::R<RegSecBpcCtrlSpec>;
#[doc = "Register `REG_SEC_BPC_CTRL` writer"]
pub type W = crate::W<RegSecBpcCtrlSpec>;
#[doc = "Field `bpc_usb` reader - Select IP controller source : usb 1:control by nonsec zone 0:control by secure zone"]
pub type BpcUsbR = crate::BitReader;
#[doc = "Field `bpc_usb` writer - Select IP controller source : usb 1:control by nonsec zone 0:control by secure zone"]
pub type BpcUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_sdio` reader - Select IP controller source : sdio 1:control by nonsec zone 0:control by secure zone"]
pub type BpcSdioR = crate::BitReader;
#[doc = "Field `bpc_sdio` writer - Select IP controller source : sdio 1:control by nonsec zone 0:control by secure zone"]
pub type BpcSdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_dmac` reader - Select IP controller source : dmac 1:control by nonsec zone 0:control by secure zone"]
pub type BpcDmacR = crate::BitReader;
#[doc = "Field `bpc_dmac` writer - Select IP controller source : dmac 1:control by nonsec zone 0:control by secure zone"]
pub type BpcDmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_ltim` reader - Select IP controller source : ltim 1:control by nonsec zone 0:control by secure zone"]
pub type BpcLtimR = crate::BitReader;
#[doc = "Field `bpc_ltim` writer - Select IP controller source : ltim 1:control by nonsec zone 0:control by secure zone"]
pub type BpcLtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_pwm` reader - Select IP controller source : pwm 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPwmR = crate::BitReader;
#[doc = "Field `bpc_pwm` writer - Select IP controller source : pwm 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPwmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_crypto` reader - Select IP controller source : crypto 1:control by nonsec zone 0:control by secure zone"]
pub type BpcCryptoR = crate::BitReader;
#[doc = "Field `bpc_crypto` writer - Select IP controller source : crypto 1:control by nonsec zone 0:control by secure zone"]
pub type BpcCryptoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_trng` reader - Select IP controller source : trng 1:control by nonsec zone 0:control by secure zone"]
pub type BpcTrngR = crate::BitReader;
#[doc = "Field `bpc_trng` writer - Select IP controller source : trng 1:control by nonsec zone 0:control by secure zone"]
pub type BpcTrngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_pke` reader - Select IP controller source : pke 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPkeR = crate::BitReader;
#[doc = "Field `bpc_pke` writer - Select IP controller source : pke 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_lcdc` reader - Select IP controller source : lcdc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcLcdcR = crate::BitReader;
#[doc = "Field `bpc_lcdc` writer - Select IP controller source : lcdc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcLcdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_mjpeg` reader - Select IP controller source : mjpeg 1:control by nonsec zone 0:control by secure zone"]
pub type BpcMjpegR = crate::BitReader;
#[doc = "Field `bpc_mjpeg` writer - Select IP controller source : mjpeg 1:control by nonsec zone 0:control by secure zone"]
pub type BpcMjpegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_ppe` reader - Select IP controller source : ppe 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPpeR = crate::BitReader;
#[doc = "Field `bpc_ppe` writer - Select IP controller source : ppe 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_otpc` reader - Select IP controller source : otpc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcOtpcR = crate::BitReader;
#[doc = "Field `bpc_otpc` writer - Select IP controller source : otpc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcOtpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_irda` reader - Select IP controller source : irda 1:control by nonsec zone 0:control by secure zone"]
pub type BpcIrdaR = crate::BitReader;
#[doc = "Field `bpc_irda` writer - Select IP controller source : irda 1:control by nonsec zone 0:control by secure zone"]
pub type BpcIrdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_spi` reader - Select IP controller source : spi 1:control by nonsec zone 0:control by secure zone"]
pub type BpcSpiR = crate::BitReader;
#[doc = "Field `bpc_spi` writer - Select IP controller source : spi 1:control by nonsec zone 0:control by secure zone"]
pub type BpcSpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_i2c` reader - Select IP controller source : i2c 1:control by nonsec zone 0:control by secure zone"]
pub type BpcI2cR = crate::BitReader;
#[doc = "Field `bpc_i2c` writer - Select IP controller source : i2c 1:control by nonsec zone 0:control by secure zone"]
pub type BpcI2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_ptim` reader - Select IP controller source : ptim 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPtimR = crate::BitReader;
#[doc = "Field `bpc_ptim` writer - Select IP controller source : ptim 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_gpio` reader - Select IP controller source : gpio 1:control by nonsec zone 0:control by secure zone"]
pub type BpcGpioR = crate::BitReader;
#[doc = "Field `bpc_gpio` writer - Select IP controller source : gpio 1:control by nonsec zone 0:control by secure zone"]
pub type BpcGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_wdg` reader - Select IP controller source : wdg 1:control by nonsec zone 0:control by secure zone"]
pub type BpcWdgR = crate::BitReader;
#[doc = "Field `bpc_wdg` writer - Select IP controller source : wdg 1:control by nonsec zone 0:control by secure zone"]
pub type BpcWdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_ipc` reader - Select IP controller source : ipc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcIpcR = crate::BitReader;
#[doc = "Field `bpc_ipc` writer - Select IP controller source : ipc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcIpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_dtim` reader - Select IP controller source : dtim 1:control by nonsec zone 0:control by secure zone"]
pub type BpcDtimR = crate::BitReader;
#[doc = "Field `bpc_dtim` writer - Select IP controller source : dtim 1:control by nonsec zone 0:control by secure zone"]
pub type BpcDtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_adc` reader - Select IP controller source : adc4m 1:control by nonsec zone 0:control by secure zone"]
pub type BpcAdcR = crate::BitReader;
#[doc = "Field `bpc_adc` writer - Select IP controller source : adc4m 1:control by nonsec zone 0:control by secure zone"]
pub type BpcAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_uart` reader - Select IP controller source : uart 1:control by nonsec zone 0:control by secure zone"]
pub type BpcUartR = crate::BitReader;
#[doc = "Field `bpc_uart` writer - Select IP controller source : uart 1:control by nonsec zone 0:control by secure zone"]
pub type BpcUartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_loguart` reader - Select IP controller source : loguart 1:control by nonsec zone 0:control by secure zone"]
pub type BpcLoguartR = crate::BitReader;
#[doc = "Field `bpc_loguart` writer - Select IP controller source : loguart 1:control by nonsec zone 0:control by secure zone"]
pub type BpcLoguartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_gmac` reader - Select IP controller source : gmac 1:control by nonsec zone 0:control by secure zone"]
pub type BpcGmacR = crate::BitReader;
#[doc = "Field `bpc_gmac` writer - Select IP controller source : gmac 1:control by nonsec zone 0:control by secure zone"]
pub type BpcGmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_can` reader - Select IP controller source : can 1:control by nonsec zone 0:control by secure zone"]
pub type BpcCanR = crate::BitReader;
#[doc = "Field `bpc_can` writer - Select IP controller source : can 1:control by nonsec zone 0:control by secure zone"]
pub type BpcCanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_audio` reader - Select IP controller source : audio 1:control by nonsec zone 0:control by secure zone"]
pub type BpcAudioR = crate::BitReader;
#[doc = "Field `bpc_audio` writer - Select IP controller source : audio 1:control by nonsec zone 0:control by secure zone"]
pub type BpcAudioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_thm` reader - Select IP controller source : thm 1:control by nonsec zone 0:control by secure zone"]
pub type BpcThmR = crate::BitReader;
#[doc = "Field `bpc_thm` writer - Select IP controller source : thm 1:control by nonsec zone 0:control by secure zone"]
pub type BpcThmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_ledc` reader - Select IP controller source : ledc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcLedcR = crate::BitReader;
#[doc = "Field `bpc_ledc` writer - Select IP controller source : ledc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcLedcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_atim` reader - Select IP controller source : atim 1:control by nonsec zone 0:control by secure zone"]
pub type BpcAtimR = crate::BitReader;
#[doc = "Field `bpc_atim` writer - Select IP controller source : atim 1:control by nonsec zone 0:control by secure zone"]
pub type BpcAtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_rtc` reader - Select IP controller source : rtc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcRtcR = crate::BitReader;
#[doc = "Field `bpc_rtc` writer - Select IP controller source : rtc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_chipen` reader - Select IP controller source : chipen 1:control by nonsec zone 0:control by secure zone"]
pub type BpcChipenR = crate::BitReader;
#[doc = "Field `bpc_chipen` writer - Select IP controller source : chipen 1:control by nonsec zone 0:control by secure zone"]
pub type BpcChipenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_bor` reader - Select IP controller source : bor 1:control by nonsec zone 0:control by secure zone"]
pub type BpcBorR = crate::BitReader;
#[doc = "Field `bpc_bor` writer - Select IP controller source : bor 1:control by nonsec zone 0:control by secure zone"]
pub type BpcBorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select IP controller source : usb 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_usb(&self) -> BpcUsbR {
        BpcUsbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select IP controller source : sdio 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_sdio(&self) -> BpcSdioR {
        BpcSdioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select IP controller source : dmac 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_dmac(&self) -> BpcDmacR {
        BpcDmacR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select IP controller source : ltim 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ltim(&self) -> BpcLtimR {
        BpcLtimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select IP controller source : pwm 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_pwm(&self) -> BpcPwmR {
        BpcPwmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select IP controller source : crypto 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_crypto(&self) -> BpcCryptoR {
        BpcCryptoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select IP controller source : trng 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_trng(&self) -> BpcTrngR {
        BpcTrngR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select IP controller source : pke 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_pke(&self) -> BpcPkeR {
        BpcPkeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select IP controller source : lcdc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_lcdc(&self) -> BpcLcdcR {
        BpcLcdcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select IP controller source : mjpeg 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_mjpeg(&self) -> BpcMjpegR {
        BpcMjpegR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select IP controller source : ppe 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ppe(&self) -> BpcPpeR {
        BpcPpeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select IP controller source : otpc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_otpc(&self) -> BpcOtpcR {
        BpcOtpcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select IP controller source : irda 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_irda(&self) -> BpcIrdaR {
        BpcIrdaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select IP controller source : spi 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_spi(&self) -> BpcSpiR {
        BpcSpiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select IP controller source : i2c 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_i2c(&self) -> BpcI2cR {
        BpcI2cR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select IP controller source : ptim 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ptim(&self) -> BpcPtimR {
        BpcPtimR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select IP controller source : gpio 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_gpio(&self) -> BpcGpioR {
        BpcGpioR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select IP controller source : wdg 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_wdg(&self) -> BpcWdgR {
        BpcWdgR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select IP controller source : ipc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ipc(&self) -> BpcIpcR {
        BpcIpcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select IP controller source : dtim 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_dtim(&self) -> BpcDtimR {
        BpcDtimR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select IP controller source : adc4m 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_adc(&self) -> BpcAdcR {
        BpcAdcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select IP controller source : uart 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_uart(&self) -> BpcUartR {
        BpcUartR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Select IP controller source : loguart 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_loguart(&self) -> BpcLoguartR {
        BpcLoguartR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Select IP controller source : gmac 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_gmac(&self) -> BpcGmacR {
        BpcGmacR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select IP controller source : can 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_can(&self) -> BpcCanR {
        BpcCanR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select IP controller source : audio 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_audio(&self) -> BpcAudioR {
        BpcAudioR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Select IP controller source : thm 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_thm(&self) -> BpcThmR {
        BpcThmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Select IP controller source : ledc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ledc(&self) -> BpcLedcR {
        BpcLedcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Select IP controller source : atim 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_atim(&self) -> BpcAtimR {
        BpcAtimR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select IP controller source : rtc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_rtc(&self) -> BpcRtcR {
        BpcRtcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Select IP controller source : chipen 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_chipen(&self) -> BpcChipenR {
        BpcChipenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Select IP controller source : bor 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_bor(&self) -> BpcBorR {
        BpcBorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select IP controller source : usb 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_usb(&mut self) -> BpcUsbW<'_, RegSecBpcCtrlSpec> {
        BpcUsbW::new(self, 0)
    }
    #[doc = "Bit 1 - Select IP controller source : sdio 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_sdio(&mut self) -> BpcSdioW<'_, RegSecBpcCtrlSpec> {
        BpcSdioW::new(self, 1)
    }
    #[doc = "Bit 2 - Select IP controller source : dmac 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_dmac(&mut self) -> BpcDmacW<'_, RegSecBpcCtrlSpec> {
        BpcDmacW::new(self, 2)
    }
    #[doc = "Bit 3 - Select IP controller source : ltim 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ltim(&mut self) -> BpcLtimW<'_, RegSecBpcCtrlSpec> {
        BpcLtimW::new(self, 3)
    }
    #[doc = "Bit 4 - Select IP controller source : pwm 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_pwm(&mut self) -> BpcPwmW<'_, RegSecBpcCtrlSpec> {
        BpcPwmW::new(self, 4)
    }
    #[doc = "Bit 5 - Select IP controller source : crypto 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_crypto(&mut self) -> BpcCryptoW<'_, RegSecBpcCtrlSpec> {
        BpcCryptoW::new(self, 5)
    }
    #[doc = "Bit 6 - Select IP controller source : trng 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_trng(&mut self) -> BpcTrngW<'_, RegSecBpcCtrlSpec> {
        BpcTrngW::new(self, 6)
    }
    #[doc = "Bit 7 - Select IP controller source : pke 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_pke(&mut self) -> BpcPkeW<'_, RegSecBpcCtrlSpec> {
        BpcPkeW::new(self, 7)
    }
    #[doc = "Bit 8 - Select IP controller source : lcdc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_lcdc(&mut self) -> BpcLcdcW<'_, RegSecBpcCtrlSpec> {
        BpcLcdcW::new(self, 8)
    }
    #[doc = "Bit 9 - Select IP controller source : mjpeg 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_mjpeg(&mut self) -> BpcMjpegW<'_, RegSecBpcCtrlSpec> {
        BpcMjpegW::new(self, 9)
    }
    #[doc = "Bit 10 - Select IP controller source : ppe 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ppe(&mut self) -> BpcPpeW<'_, RegSecBpcCtrlSpec> {
        BpcPpeW::new(self, 10)
    }
    #[doc = "Bit 11 - Select IP controller source : otpc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_otpc(&mut self) -> BpcOtpcW<'_, RegSecBpcCtrlSpec> {
        BpcOtpcW::new(self, 11)
    }
    #[doc = "Bit 12 - Select IP controller source : irda 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_irda(&mut self) -> BpcIrdaW<'_, RegSecBpcCtrlSpec> {
        BpcIrdaW::new(self, 12)
    }
    #[doc = "Bit 13 - Select IP controller source : spi 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_spi(&mut self) -> BpcSpiW<'_, RegSecBpcCtrlSpec> {
        BpcSpiW::new(self, 13)
    }
    #[doc = "Bit 14 - Select IP controller source : i2c 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_i2c(&mut self) -> BpcI2cW<'_, RegSecBpcCtrlSpec> {
        BpcI2cW::new(self, 14)
    }
    #[doc = "Bit 15 - Select IP controller source : ptim 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ptim(&mut self) -> BpcPtimW<'_, RegSecBpcCtrlSpec> {
        BpcPtimW::new(self, 15)
    }
    #[doc = "Bit 16 - Select IP controller source : gpio 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_gpio(&mut self) -> BpcGpioW<'_, RegSecBpcCtrlSpec> {
        BpcGpioW::new(self, 16)
    }
    #[doc = "Bit 17 - Select IP controller source : wdg 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_wdg(&mut self) -> BpcWdgW<'_, RegSecBpcCtrlSpec> {
        BpcWdgW::new(self, 17)
    }
    #[doc = "Bit 18 - Select IP controller source : ipc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ipc(&mut self) -> BpcIpcW<'_, RegSecBpcCtrlSpec> {
        BpcIpcW::new(self, 18)
    }
    #[doc = "Bit 19 - Select IP controller source : dtim 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_dtim(&mut self) -> BpcDtimW<'_, RegSecBpcCtrlSpec> {
        BpcDtimW::new(self, 19)
    }
    #[doc = "Bit 20 - Select IP controller source : adc4m 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_adc(&mut self) -> BpcAdcW<'_, RegSecBpcCtrlSpec> {
        BpcAdcW::new(self, 20)
    }
    #[doc = "Bit 21 - Select IP controller source : uart 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_uart(&mut self) -> BpcUartW<'_, RegSecBpcCtrlSpec> {
        BpcUartW::new(self, 21)
    }
    #[doc = "Bit 22 - Select IP controller source : loguart 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_loguart(&mut self) -> BpcLoguartW<'_, RegSecBpcCtrlSpec> {
        BpcLoguartW::new(self, 22)
    }
    #[doc = "Bit 23 - Select IP controller source : gmac 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_gmac(&mut self) -> BpcGmacW<'_, RegSecBpcCtrlSpec> {
        BpcGmacW::new(self, 23)
    }
    #[doc = "Bit 24 - Select IP controller source : can 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_can(&mut self) -> BpcCanW<'_, RegSecBpcCtrlSpec> {
        BpcCanW::new(self, 24)
    }
    #[doc = "Bit 25 - Select IP controller source : audio 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_audio(&mut self) -> BpcAudioW<'_, RegSecBpcCtrlSpec> {
        BpcAudioW::new(self, 25)
    }
    #[doc = "Bit 26 - Select IP controller source : thm 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_thm(&mut self) -> BpcThmW<'_, RegSecBpcCtrlSpec> {
        BpcThmW::new(self, 26)
    }
    #[doc = "Bit 27 - Select IP controller source : ledc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_ledc(&mut self) -> BpcLedcW<'_, RegSecBpcCtrlSpec> {
        BpcLedcW::new(self, 27)
    }
    #[doc = "Bit 28 - Select IP controller source : atim 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_atim(&mut self) -> BpcAtimW<'_, RegSecBpcCtrlSpec> {
        BpcAtimW::new(self, 28)
    }
    #[doc = "Bit 29 - Select IP controller source : rtc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_rtc(&mut self) -> BpcRtcW<'_, RegSecBpcCtrlSpec> {
        BpcRtcW::new(self, 29)
    }
    #[doc = "Bit 30 - Select IP controller source : chipen 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_chipen(&mut self) -> BpcChipenW<'_, RegSecBpcCtrlSpec> {
        BpcChipenW::new(self, 30)
    }
    #[doc = "Bit 31 - Select IP controller source : bor 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_bor(&mut self) -> BpcBorW<'_, RegSecBpcCtrlSpec> {
        BpcBorW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sec_bpc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sec_bpc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSecBpcCtrlSpec;
impl crate::RegisterSpec for RegSecBpcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sec_bpc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSecBpcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sec_bpc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSecBpcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SEC_BPC_CTRL to value 0xffff_ffff"]
impl crate::Resettable for RegSecBpcCtrlSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
