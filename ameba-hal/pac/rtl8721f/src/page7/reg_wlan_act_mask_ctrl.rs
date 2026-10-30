#[doc = "Register `REG_WLAN_ACT_MASK_CTRL` reader"]
pub type R = crate::R<RegWlanActMaskCtrlSpec>;
#[doc = "Register `REG_WLAN_ACT_MASK_CTRL` writer"]
pub type W = crate::W<RegWlanActMaskCtrlSpec>;
#[doc = "Field `BIT_WACTMSK_ON_DUR` reader - WLAN can block BT during MASK On duration. Unit 0.625ms"]
pub type BitWactmskOnDurR = crate::FieldReader;
#[doc = "Field `BIT_WACTMSK_ON_DUR` writer - WLAN can block BT during MASK On duration. Unit 0.625ms"]
pub type BitWactmskOnDurW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WACTMSK_OFF_DUR` reader - WLAN will reserve channel for BT during MASK OFF duration. Unit 0.625ms"]
pub type BitWactmskOffDurR = crate::FieldReader;
#[doc = "Field `BIT_WACTMSK_OFF_DUR` writer - WLAN will reserve channel for BT during MASK OFF duration. Unit 0.625ms"]
pub type BitWactmskOffDurW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GNT_WL_SW_EN` reader - SW control GNT_WL enable"]
pub type GntWlSwEnR = crate::BitReader;
#[doc = "Field `GNT_WL_SW_EN` writer - SW control GNT_WL enable"]
pub type GntWlSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_WL_SW` reader - SW control GNT_WL"]
pub type GntWlSwR = crate::BitReader;
#[doc = "Field `GNT_WL_SW` writer - SW control GNT_WL"]
pub type GntWlSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_WL_TX_SW_EN` reader - SW control GNT_WL_TX enable"]
pub type GntWlTxSwEnR = crate::BitReader;
#[doc = "Field `GNT_WL_TX_SW_EN` writer - SW control GNT_WL_TX enable"]
pub type GntWlTxSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_WL_TX_SW` reader - SW control GNT_WL_TX"]
pub type GntWlTxSwR = crate::BitReader;
#[doc = "Field `GNT_WL_TX_SW` writer - SW control GNT_WL_TX"]
pub type GntWlTxSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_WL_RX_SW_EN` reader - SW control GNT_WL_RX enable"]
pub type GntWlRxSwEnR = crate::BitReader;
#[doc = "Field `GNT_WL_RX_SW_EN` writer - SW control GNT_WL_RX enable"]
pub type GntWlRxSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNT_WL_RX_SW` reader - SW control GNT_WL_RX"]
pub type GntWlRxSwR = crate::BitReader;
#[doc = "Field `GNT_WL_RX_SW` writer - SW control GNT_WL_RX"]
pub type GntWlRxSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTA_GNT_INFO` reader - 0\\]: gnt_wl \\[1\\]: gnt_bt_bb \\[2\\]: gnt_bt_rfc \\[3\\]: gnt_zb_bb \\[4\\]: gnt_zb_rfc \\[5\\]: gnt_wl_tx \\[6\\]: gnt_wl_rx \\[7\\]: gnt_bt_tx \\[8\\]: gnt_bt_rx"]
pub type PtaGntInfoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - WLAN can block BT during MASK On duration. Unit 0.625ms"]
    #[inline(always)]
    pub fn bit_wactmsk_on_dur(&self) -> BitWactmskOnDurR {
        BitWactmskOnDurR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - WLAN will reserve channel for BT during MASK OFF duration. Unit 0.625ms"]
    #[inline(always)]
    pub fn bit_wactmsk_off_dur(&self) -> BitWactmskOffDurR {
        BitWactmskOffDurR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - SW control GNT_WL enable"]
    #[inline(always)]
    pub fn gnt_wl_sw_en(&self) -> GntWlSwEnR {
        GntWlSwEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SW control GNT_WL"]
    #[inline(always)]
    pub fn gnt_wl_sw(&self) -> GntWlSwR {
        GntWlSwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SW control GNT_WL_TX enable"]
    #[inline(always)]
    pub fn gnt_wl_tx_sw_en(&self) -> GntWlTxSwEnR {
        GntWlTxSwEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SW control GNT_WL_TX"]
    #[inline(always)]
    pub fn gnt_wl_tx_sw(&self) -> GntWlTxSwR {
        GntWlTxSwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SW control GNT_WL_RX enable"]
    #[inline(always)]
    pub fn gnt_wl_rx_sw_en(&self) -> GntWlRxSwEnR {
        GntWlRxSwEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SW control GNT_WL_RX"]
    #[inline(always)]
    pub fn gnt_wl_rx_sw(&self) -> GntWlRxSwR {
        GntWlRxSwR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 0\\]: gnt_wl \\[1\\]: gnt_bt_bb \\[2\\]: gnt_bt_rfc \\[3\\]: gnt_zb_bb \\[4\\]: gnt_zb_rfc \\[5\\]: gnt_wl_tx \\[6\\]: gnt_wl_rx \\[7\\]: gnt_bt_tx \\[8\\]: gnt_bt_rx"]
    #[inline(always)]
    pub fn pta_gnt_info(&self) -> PtaGntInfoR {
        PtaGntInfoR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - WLAN can block BT during MASK On duration. Unit 0.625ms"]
    #[inline(always)]
    pub fn bit_wactmsk_on_dur(&mut self) -> BitWactmskOnDurW<'_, RegWlanActMaskCtrlSpec> {
        BitWactmskOnDurW::new(self, 0)
    }
    #[doc = "Bits 8:15 - WLAN will reserve channel for BT during MASK OFF duration. Unit 0.625ms"]
    #[inline(always)]
    pub fn bit_wactmsk_off_dur(&mut self) -> BitWactmskOffDurW<'_, RegWlanActMaskCtrlSpec> {
        BitWactmskOffDurW::new(self, 8)
    }
    #[doc = "Bit 16 - SW control GNT_WL enable"]
    #[inline(always)]
    pub fn gnt_wl_sw_en(&mut self) -> GntWlSwEnW<'_, RegWlanActMaskCtrlSpec> {
        GntWlSwEnW::new(self, 16)
    }
    #[doc = "Bit 17 - SW control GNT_WL"]
    #[inline(always)]
    pub fn gnt_wl_sw(&mut self) -> GntWlSwW<'_, RegWlanActMaskCtrlSpec> {
        GntWlSwW::new(self, 17)
    }
    #[doc = "Bit 18 - SW control GNT_WL_TX enable"]
    #[inline(always)]
    pub fn gnt_wl_tx_sw_en(&mut self) -> GntWlTxSwEnW<'_, RegWlanActMaskCtrlSpec> {
        GntWlTxSwEnW::new(self, 18)
    }
    #[doc = "Bit 19 - SW control GNT_WL_TX"]
    #[inline(always)]
    pub fn gnt_wl_tx_sw(&mut self) -> GntWlTxSwW<'_, RegWlanActMaskCtrlSpec> {
        GntWlTxSwW::new(self, 19)
    }
    #[doc = "Bit 20 - SW control GNT_WL_RX enable"]
    #[inline(always)]
    pub fn gnt_wl_rx_sw_en(&mut self) -> GntWlRxSwEnW<'_, RegWlanActMaskCtrlSpec> {
        GntWlRxSwEnW::new(self, 20)
    }
    #[doc = "Bit 21 - SW control GNT_WL_RX"]
    #[inline(always)]
    pub fn gnt_wl_rx_sw(&mut self) -> GntWlRxSwW<'_, RegWlanActMaskCtrlSpec> {
        GntWlRxSwW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wlan_act_mask_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wlan_act_mask_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlanActMaskCtrlSpec;
impl crate::RegisterSpec for RegWlanActMaskCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wlan_act_mask_ctrl::R`](R) reader structure"]
impl crate::Readable for RegWlanActMaskCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wlan_act_mask_ctrl::W`](W) writer structure"]
impl crate::Writable for RegWlanActMaskCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WLAN_ACT_MASK_CTRL to value 0"]
impl crate::Resettable for RegWlanActMaskCtrlSpec {}
