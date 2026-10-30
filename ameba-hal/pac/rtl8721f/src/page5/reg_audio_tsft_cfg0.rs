#[doc = "Register `REG_AUDIO_TSFT_CFG0` reader"]
pub type R = crate::R<RegAudioTsftCfg0Spec>;
#[doc = "Register `REG_AUDIO_TSFT_CFG0` writer"]
pub type W = crate::W<RegAudioTsftCfg0Spec>;
#[doc = "Field `BIT_MAC_CLK_PRD` reader - set the period of MAC CLK, default 40M-25ns"]
pub type BitMacClkPrdR = crate::FieldReader;
#[doc = "Field `BIT_MAC_CLK_PRD` writer - set the period of MAC CLK, default 40M-25ns"]
pub type BitMacClkPrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_NEW_RX_NSCNT_EN` reader - 1: enable the new ns counter of rx 0: follow old ns counter as tx"]
pub type BitNewRxNscntEnR = crate::BitReader;
#[doc = "Field `BIT_NEW_RX_NSCNT_EN` writer - 1: enable the new ns counter of rx 0: follow old ns counter as tx"]
pub type BitNewRxNscntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PATH_DELAY_NS_OFDM` reader - the NS part of RX OFDM6M PATH DELAY"]
pub type BitPathDelayNsOfdmR = crate::FieldReader<u16>;
#[doc = "Field `BIT_PATH_DELAY_NS_OFDM` writer - the NS part of RX OFDM6M PATH DELAY"]
pub type BitPathDelayNsOfdmW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BIT_PATH_DELAY_NS_CCK` reader - the NS part of RX CCK1M PATH DELAY"]
pub type BitPathDelayNsCckR = crate::FieldReader<u16>;
#[doc = "Field `BIT_PATH_DELAY_NS_CCK` writer - the NS part of RX CCK1M PATH DELAY"]
pub type BitPathDelayNsCckW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - set the period of MAC CLK, default 40M-25ns"]
    #[inline(always)]
    pub fn bit_mac_clk_prd(&self) -> BitMacClkPrdR {
        BitMacClkPrdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: enable the new ns counter of rx 0: follow old ns counter as tx"]
    #[inline(always)]
    pub fn bit_new_rx_nscnt_en(&self) -> BitNewRxNscntEnR {
        BitNewRxNscntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:21 - the NS part of RX OFDM6M PATH DELAY"]
    #[inline(always)]
    pub fn bit_path_delay_ns_ofdm(&self) -> BitPathDelayNsOfdmR {
        BitPathDelayNsOfdmR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - the NS part of RX CCK1M PATH DELAY"]
    #[inline(always)]
    pub fn bit_path_delay_ns_cck(&self) -> BitPathDelayNsCckR {
        BitPathDelayNsCckR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - set the period of MAC CLK, default 40M-25ns"]
    #[inline(always)]
    pub fn bit_mac_clk_prd(&mut self) -> BitMacClkPrdW<'_, RegAudioTsftCfg0Spec> {
        BitMacClkPrdW::new(self, 0)
    }
    #[doc = "Bit 8 - 1: enable the new ns counter of rx 0: follow old ns counter as tx"]
    #[inline(always)]
    pub fn bit_new_rx_nscnt_en(&mut self) -> BitNewRxNscntEnW<'_, RegAudioTsftCfg0Spec> {
        BitNewRxNscntEnW::new(self, 8)
    }
    #[doc = "Bits 12:21 - the NS part of RX OFDM6M PATH DELAY"]
    #[inline(always)]
    pub fn bit_path_delay_ns_ofdm(&mut self) -> BitPathDelayNsOfdmW<'_, RegAudioTsftCfg0Spec> {
        BitPathDelayNsOfdmW::new(self, 12)
    }
    #[doc = "Bits 22:31 - the NS part of RX CCK1M PATH DELAY"]
    #[inline(always)]
    pub fn bit_path_delay_ns_cck(&mut self) -> BitPathDelayNsCckW<'_, RegAudioTsftCfg0Spec> {
        BitPathDelayNsCckW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_audio_tsft_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_audio_tsft_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAudioTsftCfg0Spec;
impl crate::RegisterSpec for RegAudioTsftCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_audio_tsft_cfg0::R`](R) reader structure"]
impl crate::Readable for RegAudioTsftCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_audio_tsft_cfg0::W`](W) writer structure"]
impl crate::Writable for RegAudioTsftCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AUDIO_TSFT_CFG0 to value 0x19"]
impl crate::Resettable for RegAudioTsftCfg0Spec {
    const RESET_VALUE: u32 = 0x19;
}
