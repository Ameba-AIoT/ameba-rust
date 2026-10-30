#[doc = "Register `REG_OTHER_CFG` reader"]
pub type R = crate::R<RegOtherCfgSpec>;
#[doc = "Register `REG_OTHER_CFG` writer"]
pub type W = crate::W<RegOtherCfgSpec>;
#[doc = "Field `BIT_EN_SR_CTS2S` reader - enable send cts2s frame during SR"]
pub type BitEnSrCts2sR = crate::BitReader;
#[doc = "Field `BIT_EN_SR_CTS2S` writer - enable send cts2s frame during SR"]
pub type BitEnSrCts2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BT_POLLUTE_RX` reader - enable write BT Polluted field during RX RESP"]
pub type BitEnBtPolluteRxR = crate::BitReader;
#[doc = "Field `BIT_EN_BT_POLLUTE_RX` writer - enable write BT Polluted field during RX RESP"]
pub type BitEnBtPolluteRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TXOP_BASE_RTS` reader - enable TXOP duration-based RTS/CTS"]
pub type BitEnTxopBaseRtsR = crate::BitReader;
#[doc = "Field `BIT_EN_TXOP_BASE_RTS` writer - enable TXOP duration-based RTS/CTS"]
pub type BitEnTxopBaseRtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RTS_TXOP_THRS` reader - unit:32us, valid when BIT_EN_TXOP_BASE_RTS = 1"]
pub type BitRtsTxopThrsR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RTS_TXOP_THRS` writer - unit:32us, valid when BIT_EN_TXOP_BASE_RTS = 1"]
pub type BitRtsTxopThrsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `dummy` reader - RSVD"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - RSVD"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EN_A_CTRL_BREAK` reader - 1: break the AGGR process when the a_ctrl field in TXDESC is different"]
pub type BitEnACtrlBreakR = crate::BitReader;
#[doc = "Field `BIT_EN_A_CTRL_BREAK` writer - 1: break the AGGR process when the a_ctrl field in TXDESC is different"]
pub type BitEnACtrlBreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ndpa_rate` reader - NDPA tx rate setting, from CCK 1M to VHT NSS2 MCS9"]
pub type BitNdpaRateR = crate::FieldReader<u16>;
#[doc = "Field `BIT_ndpa_rate` writer - NDPA tx rate setting, from CCK 1M to VHT NSS2 MCS9"]
pub type BitNdpaRateW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_gi_ltf_fb_sel` reader - 0: no GI_LTF fallback 1: GI_LTF fallback option 1 enabled (downgrade GI_LTF first from init. rate) Others RSVD"]
pub type BitGiLtfFbSelR = crate::BitReader;
#[doc = "Field `BIT_gi_ltf_fb_sel` writer - 0: no GI_LTF fallback 1: GI_LTF fallback option 1 enabled (downgrade GI_LTF first from init. rate) Others RSVD"]
pub type BitGiLtfFbSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable send cts2s frame during SR"]
    #[inline(always)]
    pub fn bit_en_sr_cts2s(&self) -> BitEnSrCts2sR {
        BitEnSrCts2sR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable write BT Polluted field during RX RESP"]
    #[inline(always)]
    pub fn bit_en_bt_pollute_rx(&self) -> BitEnBtPolluteRxR {
        BitEnBtPolluteRxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable TXOP duration-based RTS/CTS"]
    #[inline(always)]
    pub fn bit_en_txop_base_rts(&self) -> BitEnTxopBaseRtsR {
        BitEnTxopBaseRtsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:12 - unit:32us, valid when BIT_EN_TXOP_BASE_RTS = 1"]
    #[inline(always)]
    pub fn bit_rts_txop_thrs(&self) -> BitRtsTxopThrsR {
        BitRtsTxopThrsR::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 13:20 - RSVD"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bit 21 - 1: break the AGGR process when the a_ctrl field in TXDESC is different"]
    #[inline(always)]
    pub fn bit_en_a_ctrl_break(&self) -> BitEnACtrlBreakR {
        BitEnACtrlBreakR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:30 - NDPA tx rate setting, from CCK 1M to VHT NSS2 MCS9"]
    #[inline(always)]
    pub fn bit_ndpa_rate(&self) -> BitNdpaRateR {
        BitNdpaRateR::new(((self.bits >> 22) & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - 0: no GI_LTF fallback 1: GI_LTF fallback option 1 enabled (downgrade GI_LTF first from init. rate) Others RSVD"]
    #[inline(always)]
    pub fn bit_gi_ltf_fb_sel(&self) -> BitGiLtfFbSelR {
        BitGiLtfFbSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable send cts2s frame during SR"]
    #[inline(always)]
    pub fn bit_en_sr_cts2s(&mut self) -> BitEnSrCts2sW<'_, RegOtherCfgSpec> {
        BitEnSrCts2sW::new(self, 0)
    }
    #[doc = "Bit 1 - enable write BT Polluted field during RX RESP"]
    #[inline(always)]
    pub fn bit_en_bt_pollute_rx(&mut self) -> BitEnBtPolluteRxW<'_, RegOtherCfgSpec> {
        BitEnBtPolluteRxW::new(self, 1)
    }
    #[doc = "Bit 2 - enable TXOP duration-based RTS/CTS"]
    #[inline(always)]
    pub fn bit_en_txop_base_rts(&mut self) -> BitEnTxopBaseRtsW<'_, RegOtherCfgSpec> {
        BitEnTxopBaseRtsW::new(self, 2)
    }
    #[doc = "Bits 3:12 - unit:32us, valid when BIT_EN_TXOP_BASE_RTS = 1"]
    #[inline(always)]
    pub fn bit_rts_txop_thrs(&mut self) -> BitRtsTxopThrsW<'_, RegOtherCfgSpec> {
        BitRtsTxopThrsW::new(self, 3)
    }
    #[doc = "Bits 13:20 - RSVD"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegOtherCfgSpec> {
        DummyW::new(self, 13)
    }
    #[doc = "Bit 21 - 1: break the AGGR process when the a_ctrl field in TXDESC is different"]
    #[inline(always)]
    pub fn bit_en_a_ctrl_break(&mut self) -> BitEnACtrlBreakW<'_, RegOtherCfgSpec> {
        BitEnACtrlBreakW::new(self, 21)
    }
    #[doc = "Bits 22:30 - NDPA tx rate setting, from CCK 1M to VHT NSS2 MCS9"]
    #[inline(always)]
    pub fn bit_ndpa_rate(&mut self) -> BitNdpaRateW<'_, RegOtherCfgSpec> {
        BitNdpaRateW::new(self, 22)
    }
    #[doc = "Bit 31 - 0: no GI_LTF fallback 1: GI_LTF fallback option 1 enabled (downgrade GI_LTF first from init. rate) Others RSVD"]
    #[inline(always)]
    pub fn bit_gi_ltf_fb_sel(&mut self) -> BitGiLtfFbSelW<'_, RegOtherCfgSpec> {
        BitGiLtfFbSelW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_other_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_other_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOtherCfgSpec;
impl crate::RegisterSpec for RegOtherCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_other_cfg::R`](R) reader structure"]
impl crate::Readable for RegOtherCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_other_cfg::W`](W) writer structure"]
impl crate::Writable for RegOtherCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OTHER_CFG to value 0x0120_0002"]
impl crate::Resettable for RegOtherCfgSpec {
    const RESET_VALUE: u32 = 0x0120_0002;
}
