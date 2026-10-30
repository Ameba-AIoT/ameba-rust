#[doc = "Register `REG_NDP_RATE` reader"]
pub type R = crate::R<RegNdpRateSpec>;
#[doc = "Register `REG_NDP_RATE` writer"]
pub type W = crate::W<RegNdpRateSpec>;
#[doc = "Field `BIT_NDP_RATE` reader - NDPA tx rate setting, from CCK 1M to VHT NSS2 MCS9"]
pub type BitNdpRateR = crate::FieldReader<u16>;
#[doc = "Field `BIT_NDP_RATE` writer - NDPA tx rate setting, from CCK 1M to VHT NSS2 MCS9"]
pub type BitNdpRateW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_USE_NDP_PARAMETER` reader - 1: select NDP rate from CR 0X149C\\[8:0\\]"]
pub type BitUseNdpParameterR = crate::BitReader;
#[doc = "Field `BIT_USE_NDP_PARAMETER` writer - 1: select NDP rate from CR 0X149C\\[8:0\\]"]
pub type BitUseNdpParameterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TX_CNT_CLR` reader - Write 1 to clear the counters."]
pub type BitTxCntClrR = crate::BitReader;
#[doc = "Field `BIT_TX_CNT_CLR` writer - Write 1 to clear the counters."]
pub type BitTxCntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_REF_MACID` reader - if BIT_REF_MACID = 0xF, count all eligible frames and ignore MACID; else, only count the frames whose MACID = BIT_REF_MACID BMC packet ignore MACID."]
pub type BitRefMacidR = crate::FieldReader;
#[doc = "Field `BIT_REF_MACID` writer - if BIT_REF_MACID = 0xF, count all eligible frames and ignore MACID; else, only count the frames whose MACID = BIT_REF_MACID BMC packet ignore MACID."]
pub type BitRefMacidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_TXPKT_CNT_SEL` reader - Select the counter to show in 0X14A0 BIT_TXPKT_CNT_RPT. 0: r_TX_OK_RTY0 1: r_TX_OK_RTY1 2: r_TX_OK_RTY2 3: r_TX_OK_RTY3 4: r_TX_OK_RTY4 5: r_RTY_DROP 6: r_LT_DROP 7: r_BMC_TXOK 8: r_TOTAL_PKT_CNT 9: r_RTS_DROP 10: r_RTS_OK"]
pub type BitTxpktCntSelR = crate::FieldReader;
#[doc = "Field `BIT_TXPKT_CNT_SEL` writer - Select the counter to show in 0X14A0 BIT_TXPKT_CNT_RPT. 0: r_TX_OK_RTY0 1: r_TX_OK_RTY1 2: r_TX_OK_RTY2 3: r_TX_OK_RTY3 4: r_TX_OK_RTY4 5: r_RTY_DROP 6: r_LT_DROP 7: r_BMC_TXOK 8: r_TOTAL_PKT_CNT 9: r_RTS_DROP 10: r_RTS_OK"]
pub type BitTxpktCntSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:8 - NDPA tx rate setting, from CCK 1M to VHT NSS2 MCS9"]
    #[inline(always)]
    pub fn bit_ndp_rate(&self) -> BitNdpRateR {
        BitNdpRateR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - 1: select NDP rate from CR 0X149C\\[8:0\\]"]
    #[inline(always)]
    pub fn bit_use_ndp_parameter(&self) -> BitUseNdpParameterR {
        BitUseNdpParameterR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23 - Write 1 to clear the counters."]
    #[inline(always)]
    pub fn bit_tx_cnt_clr(&self) -> BitTxCntClrR {
        BitTxCntClrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - if BIT_REF_MACID = 0xF, count all eligible frames and ignore MACID; else, only count the frames whose MACID = BIT_REF_MACID BMC packet ignore MACID."]
    #[inline(always)]
    pub fn bit_ref_macid(&self) -> BitRefMacidR {
        BitRefMacidR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Select the counter to show in 0X14A0 BIT_TXPKT_CNT_RPT. 0: r_TX_OK_RTY0 1: r_TX_OK_RTY1 2: r_TX_OK_RTY2 3: r_TX_OK_RTY3 4: r_TX_OK_RTY4 5: r_RTY_DROP 6: r_LT_DROP 7: r_BMC_TXOK 8: r_TOTAL_PKT_CNT 9: r_RTS_DROP 10: r_RTS_OK"]
    #[inline(always)]
    pub fn bit_txpkt_cnt_sel(&self) -> BitTxpktCntSelR {
        BitTxpktCntSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - NDPA tx rate setting, from CCK 1M to VHT NSS2 MCS9"]
    #[inline(always)]
    pub fn bit_ndp_rate(&mut self) -> BitNdpRateW<'_, RegNdpRateSpec> {
        BitNdpRateW::new(self, 0)
    }
    #[doc = "Bit 9 - 1: select NDP rate from CR 0X149C\\[8:0\\]"]
    #[inline(always)]
    pub fn bit_use_ndp_parameter(&mut self) -> BitUseNdpParameterW<'_, RegNdpRateSpec> {
        BitUseNdpParameterW::new(self, 9)
    }
    #[doc = "Bit 23 - Write 1 to clear the counters."]
    #[inline(always)]
    pub fn bit_tx_cnt_clr(&mut self) -> BitTxCntClrW<'_, RegNdpRateSpec> {
        BitTxCntClrW::new(self, 23)
    }
    #[doc = "Bits 24:27 - if BIT_REF_MACID = 0xF, count all eligible frames and ignore MACID; else, only count the frames whose MACID = BIT_REF_MACID BMC packet ignore MACID."]
    #[inline(always)]
    pub fn bit_ref_macid(&mut self) -> BitRefMacidW<'_, RegNdpRateSpec> {
        BitRefMacidW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Select the counter to show in 0X14A0 BIT_TXPKT_CNT_RPT. 0: r_TX_OK_RTY0 1: r_TX_OK_RTY1 2: r_TX_OK_RTY2 3: r_TX_OK_RTY3 4: r_TX_OK_RTY4 5: r_RTY_DROP 6: r_LT_DROP 7: r_BMC_TXOK 8: r_TOTAL_PKT_CNT 9: r_RTS_DROP 10: r_RTS_OK"]
    #[inline(always)]
    pub fn bit_txpkt_cnt_sel(&mut self) -> BitTxpktCntSelW<'_, RegNdpRateSpec> {
        BitTxpktCntSelW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ndp_rate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ndp_rate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNdpRateSpec;
impl crate::RegisterSpec for RegNdpRateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ndp_rate::R`](R) reader structure"]
impl crate::Readable for RegNdpRateSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ndp_rate::W`](W) writer structure"]
impl crate::Writable for RegNdpRateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NDP_RATE to value 0x0f00_0204"]
impl crate::Resettable for RegNdpRateSpec {
    const RESET_VALUE: u32 = 0x0f00_0204;
}
