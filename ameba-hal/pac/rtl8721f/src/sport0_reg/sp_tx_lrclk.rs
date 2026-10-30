#[doc = "Register `SP_TX_LRCLK` reader"]
pub type R = crate::R<SpTxLrclkSpec>;
#[doc = "Register `SP_TX_LRCLK` writer"]
pub type W = crate::W<SpTxLrclkSpec>;
#[doc = "Field `txdma_burstsize` reader - TX DMA burst size"]
pub type TxdmaBurstsizeR = crate::FieldReader;
#[doc = "Field `txdma_burstsize` writer - TX DMA burst size"]
pub type TxdmaBurstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DUMMY2` reader - -"]
pub type Dummy2R = crate::FieldReader;
#[doc = "Field `DUMMY2` writer - -"]
pub type Dummy2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rxdma_busrtsize` reader - RX DMA burst size"]
pub type RxdmaBusrtsizeR = crate::FieldReader;
#[doc = "Field `rxdma_busrtsize` writer - RX DMA burst size"]
pub type RxdmaBusrtsizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DUMMY1` reader - -"]
pub type Dummy1R = crate::BitReader;
#[doc = "Field `DUMMY1` writer - -"]
pub type Dummy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifi_trig_audio_en` reader - enable wifi trigger to start audio stream"]
pub type WifiTrigAudioEnR = crate::BitReader;
#[doc = "Field `wifi_trig_audio_en` writer - enable wifi trigger to start audio stream"]
pub type WifiTrigAudioEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_bclk_div_ratio` reader - TX bclk even-bit integer divider. Used in \"mode_40mhz\" set as 1'b1. (tx_bclk_div_ratio + 1) is the number of \"sck_out\" cycles within a \"ws_out_tx\" cycle (1/fs). Default of (tx_bclk_div_ratio + 1) is 64. Set as 64 - 1 = 63. Only odd number supported. Maximum is 255."]
pub type TxBclkDivRatioR = crate::FieldReader;
#[doc = "Field `tx_bclk_div_ratio` writer - TX bclk even-bit integer divider. Used in \"mode_40mhz\" set as 1'b1. (tx_bclk_div_ratio + 1) is the number of \"sck_out\" cycles within a \"ws_out_tx\" cycle (1/fs). Default of (tx_bclk_div_ratio + 1) is 64. Set as 64 - 1 = 63. Only odd number supported. Maximum is 255."]
pub type TxBclkDivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rx_bclk_div_ratio` reader - RX bclk even-bit integer divider. Used in \"mode_40mhz\" set as 1'b1. (rx_bclk_div_ratio + 1) is the number of \"sck_out\" cycles within a \"ws_out_rx\" cycle (1/fs). Default of (rx_bclk_div_ratio + 1) is 64. Set as 64 - 1 = 63. Only odd number supported. Maximum is 255."]
pub type RxBclkDivRatioR = crate::FieldReader;
#[doc = "Field `rx_bclk_div_ratio` writer - RX bclk even-bit integer divider. Used in \"mode_40mhz\" set as 1'b1. (rx_bclk_div_ratio + 1) is the number of \"sck_out\" cycles within a \"ws_out_rx\" cycle (1/fs). Default of (rx_bclk_div_ratio + 1) is 64. Set as 64 - 1 = 63. Only odd number supported. Maximum is 255."]
pub type RxBclkDivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - TX DMA burst size"]
    #[inline(always)]
    pub fn txdma_burstsize(&self) -> TxdmaBurstsizeR {
        TxdmaBurstsizeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - -"]
    #[inline(always)]
    pub fn dummy2(&self) -> Dummy2R {
        Dummy2R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - RX DMA burst size"]
    #[inline(always)]
    pub fn rxdma_busrtsize(&self) -> RxdmaBusrtsizeR {
        RxdmaBusrtsizeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - -"]
    #[inline(always)]
    pub fn dummy1(&self) -> Dummy1R {
        Dummy1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - enable wifi trigger to start audio stream"]
    #[inline(always)]
    pub fn wifi_trig_audio_en(&self) -> WifiTrigAudioEnR {
        WifiTrigAudioEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - TX bclk even-bit integer divider. Used in \"mode_40mhz\" set as 1'b1. (tx_bclk_div_ratio + 1) is the number of \"sck_out\" cycles within a \"ws_out_tx\" cycle (1/fs). Default of (tx_bclk_div_ratio + 1) is 64. Set as 64 - 1 = 63. Only odd number supported. Maximum is 255."]
    #[inline(always)]
    pub fn tx_bclk_div_ratio(&self) -> TxBclkDivRatioR {
        TxBclkDivRatioR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX bclk even-bit integer divider. Used in \"mode_40mhz\" set as 1'b1. (rx_bclk_div_ratio + 1) is the number of \"sck_out\" cycles within a \"ws_out_rx\" cycle (1/fs). Default of (rx_bclk_div_ratio + 1) is 64. Set as 64 - 1 = 63. Only odd number supported. Maximum is 255."]
    #[inline(always)]
    pub fn rx_bclk_div_ratio(&self) -> RxBclkDivRatioR {
        RxBclkDivRatioR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TX DMA burst size"]
    #[inline(always)]
    pub fn txdma_burstsize(&mut self) -> TxdmaBurstsizeW<'_, SpTxLrclkSpec> {
        TxdmaBurstsizeW::new(self, 0)
    }
    #[doc = "Bits 6:7 - -"]
    #[inline(always)]
    pub fn dummy2(&mut self) -> Dummy2W<'_, SpTxLrclkSpec> {
        Dummy2W::new(self, 6)
    }
    #[doc = "Bits 8:13 - RX DMA burst size"]
    #[inline(always)]
    pub fn rxdma_busrtsize(&mut self) -> RxdmaBusrtsizeW<'_, SpTxLrclkSpec> {
        RxdmaBusrtsizeW::new(self, 8)
    }
    #[doc = "Bit 14 - -"]
    #[inline(always)]
    pub fn dummy1(&mut self) -> Dummy1W<'_, SpTxLrclkSpec> {
        Dummy1W::new(self, 14)
    }
    #[doc = "Bit 15 - enable wifi trigger to start audio stream"]
    #[inline(always)]
    pub fn wifi_trig_audio_en(&mut self) -> WifiTrigAudioEnW<'_, SpTxLrclkSpec> {
        WifiTrigAudioEnW::new(self, 15)
    }
    #[doc = "Bits 16:23 - TX bclk even-bit integer divider. Used in \"mode_40mhz\" set as 1'b1. (tx_bclk_div_ratio + 1) is the number of \"sck_out\" cycles within a \"ws_out_tx\" cycle (1/fs). Default of (tx_bclk_div_ratio + 1) is 64. Set as 64 - 1 = 63. Only odd number supported. Maximum is 255."]
    #[inline(always)]
    pub fn tx_bclk_div_ratio(&mut self) -> TxBclkDivRatioW<'_, SpTxLrclkSpec> {
        TxBclkDivRatioW::new(self, 16)
    }
    #[doc = "Bits 24:31 - RX bclk even-bit integer divider. Used in \"mode_40mhz\" set as 1'b1. (rx_bclk_div_ratio + 1) is the number of \"sck_out\" cycles within a \"ws_out_rx\" cycle (1/fs). Default of (rx_bclk_div_ratio + 1) is 64. Set as 64 - 1 = 63. Only odd number supported. Maximum is 255."]
    #[inline(always)]
    pub fn rx_bclk_div_ratio(&mut self) -> RxBclkDivRatioW<'_, SpTxLrclkSpec> {
        RxBclkDivRatioW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_tx_lrclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_tx_lrclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpTxLrclkSpec;
impl crate::RegisterSpec for SpTxLrclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_tx_lrclk::R`](R) reader structure"]
impl crate::Readable for SpTxLrclkSpec {}
#[doc = "`write(|w| ..)` method takes [`sp_tx_lrclk::W`](W) writer structure"]
impl crate::Writable for SpTxLrclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_TX_LRCLK to value 0x3f3f_1010"]
impl crate::Resettable for SpTxLrclkSpec {
    const RESET_VALUE: u32 = 0x3f3f_1010;
}
