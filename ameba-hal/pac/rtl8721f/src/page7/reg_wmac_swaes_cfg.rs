#[doc = "Register `REG_WMAC_SWAES_CFG` reader"]
pub type R = crate::R<RegWmacSwaesCfgSpec>;
#[doc = "Register `REG_WMAC_SWAES_CFG` writer"]
pub type W = crate::W<RegWmacSwaesCfgSpec>;
#[doc = "Field `BIT_R_WMAC_SWAES_SEL` reader - When R_WMAC_SWAES_SEL =1,the content of register 0750-075F used to be aes engine data in or data out . When R_WMAC_SWAES_SEL =0, the content of register 0754-075F used to be aes engine security key"]
pub type BitRWmacSwaesSelR = crate::BitReader;
#[doc = "Field `BIT_R_WMAC_SWAES_SEL` writer - When R_WMAC_SWAES_SEL =1,the content of register 0750-075F used to be aes engine data in or data out . When R_WMAC_SWAES_SEL =0, the content of register 0754-075F used to be aes engine security key"]
pub type BitRWmacSwaesSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_R_WMAC_SWAES_WE` reader - Write enable of register ro_WMAC_SWAES_RD"]
pub type BitRWmacSwaesWeR = crate::BitReader;
#[doc = "Field `BIT_R_WMAC_SWAES_WE` writer - Write enable of register ro_WMAC_SWAES_RD"]
pub type BitRWmacSwaesWeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SWAES_REQ` reader - software aes engine request polling"]
pub type BitSwaesReqR = crate::BitReader;
#[doc = "Field `BIT_BYPASS_COMBO_ZB_REQ` reader - 1: ignore zb flag from RTK combo IP 0: distinguish zb_request and bt_request from RTK combo IP"]
pub type BitBypassComboZbReqR = crate::BitReader;
#[doc = "Field `BIT_BYPASS_COMBO_ZB_REQ` writer - 1: ignore zb flag from RTK combo IP 0: distinguish zb_request and bt_request from RTK combo IP"]
pub type BitBypassComboZbReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_INNER_COMBO_MODE` reader - 1: RTK combo IP parser zb_request or bt_request,zb_request will use wl-zb coex table co-work with BIT_BYPASS_COMBO_ZB_REQ = 0"]
pub type BitInnerComboModeR = crate::BitReader;
#[doc = "Field `BIT_INNER_COMBO_MODE` writer - 1: RTK combo IP parser zb_request or bt_request,zb_request will use wl-zb coex table co-work with BIT_BYPASS_COMBO_ZB_REQ = 0"]
pub type BitInnerComboModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BT_ARB_TIME` reader - 8 bit timer register : Arbitration window of bt trx request ( in the unit of us)"]
pub type BitBtArbTimeR = crate::FieldReader;
#[doc = "Field `BIT_BT_ARB_TIME` writer - 8 bit timer register : Arbitration window of bt trx request ( in the unit of us)"]
pub type BitBtArbTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BT_ARB_INST_EN` reader - 1: enable GNT_BT_BYPASS_PRIORITY 0 : disable"]
pub type BitBtArbInstEnR = crate::BitReader;
#[doc = "Field `BIT_BT_ARB_INST_EN` writer - 1: enable GNT_BT_BYPASS_PRIORITY 0 : disable"]
pub type BitBtArbInstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GRANTALL_WLMASK` reader - 0: WLAN_ACT to BT is always 0"]
pub type BitGrantallWlmaskR = crate::BitReader;
#[doc = "Field `BIT_GRANTALL_WLMASK` writer - 0: WLAN_ACT to BT is always 0"]
pub type BitGrantallWlmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_BT_TX_SW_EN` reader - SW control GNT_BT_TX enable"]
pub type GntBtTxSwEnR = crate::BitReader;
#[doc = "Field `GNT_BT_TX_SW_EN` writer - SW control GNT_BT_TX enable"]
pub type GntBtTxSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_BT_TX_SW` reader - SW control GNT_BT_TX"]
pub type GntBtTxSwR = crate::BitReader;
#[doc = "Field `GNT_BT_TX_SW` writer - SW control GNT_BT_TX"]
pub type GntBtTxSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GNT_BOTH_POL` reader - grant both choose GNT_BT polarity: 0 : deassert GNT_BT signal as PTA grant both WiFI and BT. 1 : assert GNT_BT signal as PTA grant both WiFi and BT."]
pub type BitGntBothPolR = crate::BitReader;
#[doc = "Field `BIT_GNT_BOTH_POL` writer - grant both choose GNT_BT polarity: 0 : deassert GNT_BT signal as PTA grant both WiFI and BT. 1 : assert GNT_BT signal as PTA grant both WiFi and BT."]
pub type BitGntBothPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_BT_RX_SW_EN` reader - SW control GNT_BT_RX enable"]
pub type GntBtRxSwEnR = crate::BitReader;
#[doc = "Field `GNT_BT_RX_SW_EN` writer - SW control GNT_BT_RX enable"]
pub type GntBtRxSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_BT_RX_SW` reader - SW control GNT_BT_RX"]
pub type GntBtRxSwR = crate::BitReader;
#[doc = "Field `GNT_BT_RX_SW` writer - SW control GNT_BT_RX"]
pub type GntBtRxSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BTCOEX_SW_RST` reader - "]
pub type BitBtcoexSwRstR = crate::BitReader;
#[doc = "Field `BIT_BTCOEX_SW_RST` writer - "]
pub type BitBtcoexSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When R_WMAC_SWAES_SEL =1,the content of register 0750-075F used to be aes engine data in or data out . When R_WMAC_SWAES_SEL =0, the content of register 0754-075F used to be aes engine security key"]
    #[inline(always)]
    pub fn bit_r_wmac_swaes_sel(&self) -> BitRWmacSwaesSelR {
        BitRWmacSwaesSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Write enable of register ro_WMAC_SWAES_RD"]
    #[inline(always)]
    pub fn bit_r_wmac_swaes_we(&self) -> BitRWmacSwaesWeR {
        BitRWmacSwaesWeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - software aes engine request polling"]
    #[inline(always)]
    pub fn bit_swaes_req(&self) -> BitSwaesReqR {
        BitSwaesReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: ignore zb flag from RTK combo IP 0: distinguish zb_request and bt_request from RTK combo IP"]
    #[inline(always)]
    pub fn bit_bypass_combo_zb_req(&self) -> BitBypassComboZbReqR {
        BitBypassComboZbReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: RTK combo IP parser zb_request or bt_request,zb_request will use wl-zb coex table co-work with BIT_BYPASS_COMBO_ZB_REQ = 0"]
    #[inline(always)]
    pub fn bit_inner_combo_mode(&self) -> BitInnerComboModeR {
        BitInnerComboModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 8 bit timer register : Arbitration window of bt trx request ( in the unit of us)"]
    #[inline(always)]
    pub fn bit_bt_arb_time(&self) -> BitBtArbTimeR {
        BitBtArbTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: enable GNT_BT_BYPASS_PRIORITY 0 : disable"]
    #[inline(always)]
    pub fn bit_bt_arb_inst_en(&self) -> BitBtArbInstEnR {
        BitBtArbInstEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: WLAN_ACT to BT is always 0"]
    #[inline(always)]
    pub fn bit_grantall_wlmask(&self) -> BitGrantallWlmaskR {
        BitGrantallWlmaskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SW control GNT_BT_TX enable"]
    #[inline(always)]
    pub fn gnt_bt_tx_sw_en(&self) -> GntBtTxSwEnR {
        GntBtTxSwEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SW control GNT_BT_TX"]
    #[inline(always)]
    pub fn gnt_bt_tx_sw(&self) -> GntBtTxSwR {
        GntBtTxSwR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - grant both choose GNT_BT polarity: 0 : deassert GNT_BT signal as PTA grant both WiFI and BT. 1 : assert GNT_BT signal as PTA grant both WiFi and BT."]
    #[inline(always)]
    pub fn bit_gnt_both_pol(&self) -> BitGntBothPolR {
        BitGntBothPolR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SW control GNT_BT_RX enable"]
    #[inline(always)]
    pub fn gnt_bt_rx_sw_en(&self) -> GntBtRxSwEnR {
        GntBtRxSwEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SW control GNT_BT_RX"]
    #[inline(always)]
    pub fn gnt_bt_rx_sw(&self) -> GntBtRxSwR {
        GntBtRxSwR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn bit_btcoex_sw_rst(&self) -> BitBtcoexSwRstR {
        BitBtcoexSwRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When R_WMAC_SWAES_SEL =1,the content of register 0750-075F used to be aes engine data in or data out . When R_WMAC_SWAES_SEL =0, the content of register 0754-075F used to be aes engine security key"]
    #[inline(always)]
    pub fn bit_r_wmac_swaes_sel(&mut self) -> BitRWmacSwaesSelW<'_, RegWmacSwaesCfgSpec> {
        BitRWmacSwaesSelW::new(self, 0)
    }
    #[doc = "Bit 3 - Write enable of register ro_WMAC_SWAES_RD"]
    #[inline(always)]
    pub fn bit_r_wmac_swaes_we(&mut self) -> BitRWmacSwaesWeW<'_, RegWmacSwaesCfgSpec> {
        BitRWmacSwaesWeW::new(self, 3)
    }
    #[doc = "Bit 8 - 1: ignore zb flag from RTK combo IP 0: distinguish zb_request and bt_request from RTK combo IP"]
    #[inline(always)]
    pub fn bit_bypass_combo_zb_req(&mut self) -> BitBypassComboZbReqW<'_, RegWmacSwaesCfgSpec> {
        BitBypassComboZbReqW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: RTK combo IP parser zb_request or bt_request,zb_request will use wl-zb coex table co-work with BIT_BYPASS_COMBO_ZB_REQ = 0"]
    #[inline(always)]
    pub fn bit_inner_combo_mode(&mut self) -> BitInnerComboModeW<'_, RegWmacSwaesCfgSpec> {
        BitInnerComboModeW::new(self, 9)
    }
    #[doc = "Bits 16:23 - 8 bit timer register : Arbitration window of bt trx request ( in the unit of us)"]
    #[inline(always)]
    pub fn bit_bt_arb_time(&mut self) -> BitBtArbTimeW<'_, RegWmacSwaesCfgSpec> {
        BitBtArbTimeW::new(self, 16)
    }
    #[doc = "Bit 24 - 1: enable GNT_BT_BYPASS_PRIORITY 0 : disable"]
    #[inline(always)]
    pub fn bit_bt_arb_inst_en(&mut self) -> BitBtArbInstEnW<'_, RegWmacSwaesCfgSpec> {
        BitBtArbInstEnW::new(self, 24)
    }
    #[doc = "Bit 25 - 0: WLAN_ACT to BT is always 0"]
    #[inline(always)]
    pub fn bit_grantall_wlmask(&mut self) -> BitGrantallWlmaskW<'_, RegWmacSwaesCfgSpec> {
        BitGrantallWlmaskW::new(self, 25)
    }
    #[doc = "Bit 26 - SW control GNT_BT_TX enable"]
    #[inline(always)]
    pub fn gnt_bt_tx_sw_en(&mut self) -> GntBtTxSwEnW<'_, RegWmacSwaesCfgSpec> {
        GntBtTxSwEnW::new(self, 26)
    }
    #[doc = "Bit 27 - SW control GNT_BT_TX"]
    #[inline(always)]
    pub fn gnt_bt_tx_sw(&mut self) -> GntBtTxSwW<'_, RegWmacSwaesCfgSpec> {
        GntBtTxSwW::new(self, 27)
    }
    #[doc = "Bit 28 - grant both choose GNT_BT polarity: 0 : deassert GNT_BT signal as PTA grant both WiFI and BT. 1 : assert GNT_BT signal as PTA grant both WiFi and BT."]
    #[inline(always)]
    pub fn bit_gnt_both_pol(&mut self) -> BitGntBothPolW<'_, RegWmacSwaesCfgSpec> {
        BitGntBothPolW::new(self, 28)
    }
    #[doc = "Bit 29 - SW control GNT_BT_RX enable"]
    #[inline(always)]
    pub fn gnt_bt_rx_sw_en(&mut self) -> GntBtRxSwEnW<'_, RegWmacSwaesCfgSpec> {
        GntBtRxSwEnW::new(self, 29)
    }
    #[doc = "Bit 30 - SW control GNT_BT_RX"]
    #[inline(always)]
    pub fn gnt_bt_rx_sw(&mut self) -> GntBtRxSwW<'_, RegWmacSwaesCfgSpec> {
        GntBtRxSwW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn bit_btcoex_sw_rst(&mut self) -> BitBtcoexSwRstW<'_, RegWmacSwaesCfgSpec> {
        BitBtcoexSwRstW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_swaes_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_swaes_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacSwaesCfgSpec;
impl crate::RegisterSpec for RegWmacSwaesCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_swaes_cfg::R`](R) reader structure"]
impl crate::Readable for RegWmacSwaesCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_swaes_cfg::W`](W) writer structure"]
impl crate::Writable for RegWmacSwaesCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_SWAES_CFG to value 0"]
impl crate::Resettable for RegWmacSwaesCfgSpec {}
