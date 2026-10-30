#[doc = "Register `REG_BT_COEX_ENH` reader"]
pub type R = crate::R<RegBtCoexEnhSpec>;
#[doc = "Register `REG_BT_COEX_ENH` writer"]
pub type W = crate::W<RegBtCoexEnhSpec>;
#[doc = "Field `bit_r_bt_cnt_thr` reader - BT count threshold"]
pub type BitRBtCntThrR = crate::FieldReader;
#[doc = "Field `bit_r_bt_cnt_thr` writer - BT count threshold"]
pub type BitRBtCntThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `bit_r_bt_cnt_thren` reader - BT count threshold enable"]
pub type BitRBtCntThrenR = crate::BitReader;
#[doc = "Field `bit_r_bt_cnt_thren` writer - BT count threshold enable"]
pub type BitRBtCntThrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_r_gnt_bt_bb_sw_en` reader - SW control BT baseband enable"]
pub type BitRGntBtBbSwEnR = crate::BitReader;
#[doc = "Field `bit_r_gnt_bt_bb_sw_en` writer - SW control BT baseband enable"]
pub type BitRGntBtBbSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_r_gnt_bt_bb_sw` reader - SW control BT baseband"]
pub type BitRGntBtBbSwR = crate::BitReader;
#[doc = "Field `bit_r_gnt_bt_bb_sw` writer - SW control BT baseband"]
pub type BitRGntBtBbSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_r_gnt_bt_rfc_sw_en` reader - SW control BT RF enable"]
pub type BitRGntBtRfcSwEnR = crate::BitReader;
#[doc = "Field `bit_r_gnt_bt_rfc_sw_en` writer - SW control BT RF enable"]
pub type BitRGntBtRfcSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_r_gnt_bt_rfc_sw` reader - SW control BT RF"]
pub type BitRGntBtRfcSwR = crate::BitReader;
#[doc = "Field `bit_r_gnt_bt_rfc_sw` writer - SW control BT RF"]
pub type BitRGntBtRfcSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BTG_LNA1_GAIN_SEL` reader - control by Driver 0: mask for WL 1: original BTG"]
pub type BitBtgLna1GainSelR = crate::BitReader;
#[doc = "Field `BIT_BTG_LNA1_GAIN_SEL` writer - control by Driver 0: mask for WL 1: original BTG"]
pub type BitBtgLna1GainSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_ZB_DONT_CARE` reader - 1: break table of ZB & WL can be all \"1\" 0: break table of ZB & WL cannot be all \"1\""]
pub type BitDisZbDontCareR = crate::BitReader;
#[doc = "Field `BIT_DIS_ZB_DONT_CARE` writer - 1: break table of ZB & WL can be all \"1\" 0: break table of ZB & WL cannot be all \"1\""]
pub type BitDisZbDontCareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_BT_DONT_CARE` reader - 1: break table of BT & WL can be all \"1\" 0: break table of BT & WL cannot be all \"1\""]
pub type BitDisBtDontCareR = crate::BitReader;
#[doc = "Field `BIT_DIS_BT_DONT_CARE` writer - 1: break table of BT & WL can be all \"1\" 0: break table of BT & WL cannot be all \"1\""]
pub type BitDisBtDontCareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EXT_ZB_REQ_POLAR` reader - Select the polar of ZB_REQUEST, for static mode external ZB 0: low active 1: high active"]
pub type BitExtZbReqPolarR = crate::BitReader;
#[doc = "Field `BIT_EXT_ZB_REQ_POLAR` writer - Select the polar of ZB_REQUEST, for static mode external ZB 0: low active 1: high active"]
pub type BitExtZbReqPolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GNT_ZB_EXT_SW` reader - SW value, external ZigBee"]
pub type BitGntZbExtSwR = crate::BitReader;
#[doc = "Field `BIT_GNT_ZB_EXT_SW` writer - SW value, external ZigBee"]
pub type BitGntZbExtSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GNT_ZB_EXT_SW_EN` reader - Enable SW control gnt_zb_o of external ZigBee"]
pub type BitGntZbExtSwEnR = crate::BitReader;
#[doc = "Field `BIT_GNT_ZB_EXT_SW_EN` writer - Enable SW control gnt_zb_o of external ZigBee"]
pub type BitGntZbExtSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GNT_ZB_RFC_SW` reader - SW value"]
pub type BitGntZbRfcSwR = crate::BitReader;
#[doc = "Field `BIT_GNT_ZB_RFC_SW` writer - SW value"]
pub type BitGntZbRfcSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GNT_ZB_RFC_SW_EN` reader - Enable SW control gnt_zb_rfc"]
pub type BitGntZbRfcSwEnR = crate::BitReader;
#[doc = "Field `BIT_GNT_ZB_RFC_SW_EN` writer - Enable SW control gnt_zb_rfc"]
pub type BitGntZbRfcSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GNT_ZB_BB_SW` reader - SW value"]
pub type BitGntZbBbSwR = crate::BitReader;
#[doc = "Field `BIT_GNT_ZB_BB_SW` writer - SW value"]
pub type BitGntZbBbSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_GNT_ZB_BB_SW_EN` reader - Enable SW control gnt_zb_bb"]
pub type BitGntZbBbSwEnR = crate::BitReader;
#[doc = "Field `BIT_GNT_ZB_BB_SW_EN` writer - Enable SW control gnt_zb_bb"]
pub type BitGntZbBbSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ZB_STATISTIC_EN` reader - enable statistic of ZB"]
pub type BitZbStatisticEnR = crate::BitReader;
#[doc = "Field `BIT_ZB_STATISTIC_EN` writer - enable statistic of ZB"]
pub type BitZbStatisticEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ZB_CNT_THREN` reader - ZB count threshold enable, for statistics co-work with BIT_ZB_CNT_THR"]
pub type BitZbCntThrenR = crate::BitReader;
#[doc = "Field `BIT_ZB_CNT_THREN` writer - ZB count threshold enable, for statistics co-work with BIT_ZB_CNT_THR"]
pub type BitZbCntThrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ZB_ARB_INST_EN` reader - 1: enable the time limit of ZigBee request (zb_pri | zb_rx | zb_tx), PTA is valid during ( <=BIT_ZB_ARB_TIME) 0: disable Co-work with BIT_ZB_ARB_TIME"]
pub type BitZbArbInstEnR = crate::BitReader;
#[doc = "Field `BIT_ZB_ARB_INST_EN` writer - 1: enable the time limit of ZigBee request (zb_pri | zb_rx | zb_tx), PTA is valid during ( <=BIT_ZB_ARB_TIME) 0: disable Co-work with BIT_ZB_ARB_TIME"]
pub type BitZbArbInstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EXT_ZB_GNT_POLAR` reader - Select the polar of ZB_GRANT, for extarnal ZB 0: low active 1: high active"]
pub type BitExtZbGntPolarR = crate::BitReader;
#[doc = "Field `BIT_EXT_ZB_GNT_POLAR` writer - Select the polar of ZB_GRANT, for extarnal ZB 0: low active 1: high active"]
pub type BitExtZbGntPolarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_COEX_TABLE_SEL` reader - Select coexistence table for WLAN & ZB 0: BIT_COEX_TABLE_WL_ZB_1 1: BIT_COEX_TABLE_WL_ZB_2"]
pub type BitCoexTableSelR = crate::BitReader;
#[doc = "Field `BIT_COEX_TABLE_SEL` writer - Select coexistence table for WLAN & ZB 0: BIT_COEX_TABLE_WL_ZB_1 1: BIT_COEX_TABLE_WL_ZB_2"]
pub type BitCoexTableSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EXT_ZIGBEE_MODE` reader - The mode of external ZigBee 1: static PRIORITY 0: directional PRIORITY"]
pub type BitExtZigbeeModeR = crate::BitReader;
#[doc = "Field `BIT_EXT_ZIGBEE_MODE` writer - The mode of external ZigBee 1: static PRIORITY 0: directional PRIORITY"]
pub type BitExtZigbeeModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ZIGBEE_MODE` reader - 0: SOC 1: external"]
pub type BitZigbeeModeR = crate::BitReader;
#[doc = "Field `BIT_ZIGBEE_MODE` writer - 0: SOC 1: external"]
pub type BitZigbeeModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ZIGBEE_EN` reader - Enable ZigBee function"]
pub type BitZigbeeEnR = crate::BitReader;
#[doc = "Field `BIT_ZIGBEE_EN` writer - Enable ZigBee function"]
pub type BitZigbeeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - BT count threshold"]
    #[inline(always)]
    pub fn bit_r_bt_cnt_thr(&self) -> BitRBtCntThrR {
        BitRBtCntThrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - BT count threshold enable"]
    #[inline(always)]
    pub fn bit_r_bt_cnt_thren(&self) -> BitRBtCntThrenR {
        BitRBtCntThrenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SW control BT baseband enable"]
    #[inline(always)]
    pub fn bit_r_gnt_bt_bb_sw_en(&self) -> BitRGntBtBbSwEnR {
        BitRGntBtBbSwEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SW control BT baseband"]
    #[inline(always)]
    pub fn bit_r_gnt_bt_bb_sw(&self) -> BitRGntBtBbSwR {
        BitRGntBtBbSwR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SW control BT RF enable"]
    #[inline(always)]
    pub fn bit_r_gnt_bt_rfc_sw_en(&self) -> BitRGntBtRfcSwEnR {
        BitRGntBtRfcSwEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SW control BT RF"]
    #[inline(always)]
    pub fn bit_r_gnt_bt_rfc_sw(&self) -> BitRGntBtRfcSwR {
        BitRGntBtRfcSwR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - control by Driver 0: mask for WL 1: original BTG"]
    #[inline(always)]
    pub fn bit_btg_lna1_gain_sel(&self) -> BitBtgLna1GainSelR {
        BitBtgLna1GainSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: break table of ZB & WL can be all \"1\" 0: break table of ZB & WL cannot be all \"1\""]
    #[inline(always)]
    pub fn bit_dis_zb_dont_care(&self) -> BitDisZbDontCareR {
        BitDisZbDontCareR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: break table of BT & WL can be all \"1\" 0: break table of BT & WL cannot be all \"1\""]
    #[inline(always)]
    pub fn bit_dis_bt_dont_care(&self) -> BitDisBtDontCareR {
        BitDisBtDontCareR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select the polar of ZB_REQUEST, for static mode external ZB 0: low active 1: high active"]
    #[inline(always)]
    pub fn bit_ext_zb_req_polar(&self) -> BitExtZbReqPolarR {
        BitExtZbReqPolarR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SW value, external ZigBee"]
    #[inline(always)]
    pub fn bit_gnt_zb_ext_sw(&self) -> BitGntZbExtSwR {
        BitGntZbExtSwR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable SW control gnt_zb_o of external ZigBee"]
    #[inline(always)]
    pub fn bit_gnt_zb_ext_sw_en(&self) -> BitGntZbExtSwEnR {
        BitGntZbExtSwEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SW value"]
    #[inline(always)]
    pub fn bit_gnt_zb_rfc_sw(&self) -> BitGntZbRfcSwR {
        BitGntZbRfcSwR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable SW control gnt_zb_rfc"]
    #[inline(always)]
    pub fn bit_gnt_zb_rfc_sw_en(&self) -> BitGntZbRfcSwEnR {
        BitGntZbRfcSwEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SW value"]
    #[inline(always)]
    pub fn bit_gnt_zb_bb_sw(&self) -> BitGntZbBbSwR {
        BitGntZbBbSwR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable SW control gnt_zb_bb"]
    #[inline(always)]
    pub fn bit_gnt_zb_bb_sw_en(&self) -> BitGntZbBbSwEnR {
        BitGntZbBbSwEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - enable statistic of ZB"]
    #[inline(always)]
    pub fn bit_zb_statistic_en(&self) -> BitZbStatisticEnR {
        BitZbStatisticEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ZB count threshold enable, for statistics co-work with BIT_ZB_CNT_THR"]
    #[inline(always)]
    pub fn bit_zb_cnt_thren(&self) -> BitZbCntThrenR {
        BitZbCntThrenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: enable the time limit of ZigBee request (zb_pri | zb_rx | zb_tx), PTA is valid during ( <=BIT_ZB_ARB_TIME) 0: disable Co-work with BIT_ZB_ARB_TIME"]
    #[inline(always)]
    pub fn bit_zb_arb_inst_en(&self) -> BitZbArbInstEnR {
        BitZbArbInstEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Select the polar of ZB_GRANT, for extarnal ZB 0: low active 1: high active"]
    #[inline(always)]
    pub fn bit_ext_zb_gnt_polar(&self) -> BitExtZbGntPolarR {
        BitExtZbGntPolarR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Select coexistence table for WLAN & ZB 0: BIT_COEX_TABLE_WL_ZB_1 1: BIT_COEX_TABLE_WL_ZB_2"]
    #[inline(always)]
    pub fn bit_coex_table_sel(&self) -> BitCoexTableSelR {
        BitCoexTableSelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The mode of external ZigBee 1: static PRIORITY 0: directional PRIORITY"]
    #[inline(always)]
    pub fn bit_ext_zigbee_mode(&self) -> BitExtZigbeeModeR {
        BitExtZigbeeModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: SOC 1: external"]
    #[inline(always)]
    pub fn bit_zigbee_mode(&self) -> BitZigbeeModeR {
        BitZigbeeModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable ZigBee function"]
    #[inline(always)]
    pub fn bit_zigbee_en(&self) -> BitZigbeeEnR {
        BitZigbeeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - BT count threshold"]
    #[inline(always)]
    pub fn bit_r_bt_cnt_thr(&mut self) -> BitRBtCntThrW<'_, RegBtCoexEnhSpec> {
        BitRBtCntThrW::new(self, 0)
    }
    #[doc = "Bit 8 - BT count threshold enable"]
    #[inline(always)]
    pub fn bit_r_bt_cnt_thren(&mut self) -> BitRBtCntThrenW<'_, RegBtCoexEnhSpec> {
        BitRBtCntThrenW::new(self, 8)
    }
    #[doc = "Bit 9 - SW control BT baseband enable"]
    #[inline(always)]
    pub fn bit_r_gnt_bt_bb_sw_en(&mut self) -> BitRGntBtBbSwEnW<'_, RegBtCoexEnhSpec> {
        BitRGntBtBbSwEnW::new(self, 9)
    }
    #[doc = "Bit 10 - SW control BT baseband"]
    #[inline(always)]
    pub fn bit_r_gnt_bt_bb_sw(&mut self) -> BitRGntBtBbSwW<'_, RegBtCoexEnhSpec> {
        BitRGntBtBbSwW::new(self, 10)
    }
    #[doc = "Bit 11 - SW control BT RF enable"]
    #[inline(always)]
    pub fn bit_r_gnt_bt_rfc_sw_en(&mut self) -> BitRGntBtRfcSwEnW<'_, RegBtCoexEnhSpec> {
        BitRGntBtRfcSwEnW::new(self, 11)
    }
    #[doc = "Bit 12 - SW control BT RF"]
    #[inline(always)]
    pub fn bit_r_gnt_bt_rfc_sw(&mut self) -> BitRGntBtRfcSwW<'_, RegBtCoexEnhSpec> {
        BitRGntBtRfcSwW::new(self, 12)
    }
    #[doc = "Bit 14 - control by Driver 0: mask for WL 1: original BTG"]
    #[inline(always)]
    pub fn bit_btg_lna1_gain_sel(&mut self) -> BitBtgLna1GainSelW<'_, RegBtCoexEnhSpec> {
        BitBtgLna1GainSelW::new(self, 14)
    }
    #[doc = "Bit 15 - 1: break table of ZB & WL can be all \"1\" 0: break table of ZB & WL cannot be all \"1\""]
    #[inline(always)]
    pub fn bit_dis_zb_dont_care(&mut self) -> BitDisZbDontCareW<'_, RegBtCoexEnhSpec> {
        BitDisZbDontCareW::new(self, 15)
    }
    #[doc = "Bit 16 - 1: break table of BT & WL can be all \"1\" 0: break table of BT & WL cannot be all \"1\""]
    #[inline(always)]
    pub fn bit_dis_bt_dont_care(&mut self) -> BitDisBtDontCareW<'_, RegBtCoexEnhSpec> {
        BitDisBtDontCareW::new(self, 16)
    }
    #[doc = "Bit 17 - Select the polar of ZB_REQUEST, for static mode external ZB 0: low active 1: high active"]
    #[inline(always)]
    pub fn bit_ext_zb_req_polar(&mut self) -> BitExtZbReqPolarW<'_, RegBtCoexEnhSpec> {
        BitExtZbReqPolarW::new(self, 17)
    }
    #[doc = "Bit 18 - SW value, external ZigBee"]
    #[inline(always)]
    pub fn bit_gnt_zb_ext_sw(&mut self) -> BitGntZbExtSwW<'_, RegBtCoexEnhSpec> {
        BitGntZbExtSwW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable SW control gnt_zb_o of external ZigBee"]
    #[inline(always)]
    pub fn bit_gnt_zb_ext_sw_en(&mut self) -> BitGntZbExtSwEnW<'_, RegBtCoexEnhSpec> {
        BitGntZbExtSwEnW::new(self, 19)
    }
    #[doc = "Bit 20 - SW value"]
    #[inline(always)]
    pub fn bit_gnt_zb_rfc_sw(&mut self) -> BitGntZbRfcSwW<'_, RegBtCoexEnhSpec> {
        BitGntZbRfcSwW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable SW control gnt_zb_rfc"]
    #[inline(always)]
    pub fn bit_gnt_zb_rfc_sw_en(&mut self) -> BitGntZbRfcSwEnW<'_, RegBtCoexEnhSpec> {
        BitGntZbRfcSwEnW::new(self, 21)
    }
    #[doc = "Bit 22 - SW value"]
    #[inline(always)]
    pub fn bit_gnt_zb_bb_sw(&mut self) -> BitGntZbBbSwW<'_, RegBtCoexEnhSpec> {
        BitGntZbBbSwW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable SW control gnt_zb_bb"]
    #[inline(always)]
    pub fn bit_gnt_zb_bb_sw_en(&mut self) -> BitGntZbBbSwEnW<'_, RegBtCoexEnhSpec> {
        BitGntZbBbSwEnW::new(self, 23)
    }
    #[doc = "Bit 24 - enable statistic of ZB"]
    #[inline(always)]
    pub fn bit_zb_statistic_en(&mut self) -> BitZbStatisticEnW<'_, RegBtCoexEnhSpec> {
        BitZbStatisticEnW::new(self, 24)
    }
    #[doc = "Bit 25 - ZB count threshold enable, for statistics co-work with BIT_ZB_CNT_THR"]
    #[inline(always)]
    pub fn bit_zb_cnt_thren(&mut self) -> BitZbCntThrenW<'_, RegBtCoexEnhSpec> {
        BitZbCntThrenW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: enable the time limit of ZigBee request (zb_pri | zb_rx | zb_tx), PTA is valid during ( <=BIT_ZB_ARB_TIME) 0: disable Co-work with BIT_ZB_ARB_TIME"]
    #[inline(always)]
    pub fn bit_zb_arb_inst_en(&mut self) -> BitZbArbInstEnW<'_, RegBtCoexEnhSpec> {
        BitZbArbInstEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Select the polar of ZB_GRANT, for extarnal ZB 0: low active 1: high active"]
    #[inline(always)]
    pub fn bit_ext_zb_gnt_polar(&mut self) -> BitExtZbGntPolarW<'_, RegBtCoexEnhSpec> {
        BitExtZbGntPolarW::new(self, 27)
    }
    #[doc = "Bit 28 - Select coexistence table for WLAN & ZB 0: BIT_COEX_TABLE_WL_ZB_1 1: BIT_COEX_TABLE_WL_ZB_2"]
    #[inline(always)]
    pub fn bit_coex_table_sel(&mut self) -> BitCoexTableSelW<'_, RegBtCoexEnhSpec> {
        BitCoexTableSelW::new(self, 28)
    }
    #[doc = "Bit 29 - The mode of external ZigBee 1: static PRIORITY 0: directional PRIORITY"]
    #[inline(always)]
    pub fn bit_ext_zigbee_mode(&mut self) -> BitExtZigbeeModeW<'_, RegBtCoexEnhSpec> {
        BitExtZigbeeModeW::new(self, 29)
    }
    #[doc = "Bit 30 - 0: SOC 1: external"]
    #[inline(always)]
    pub fn bit_zigbee_mode(&mut self) -> BitZigbeeModeW<'_, RegBtCoexEnhSpec> {
        BitZigbeeModeW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable ZigBee function"]
    #[inline(always)]
    pub fn bit_zigbee_en(&mut self) -> BitZigbeeEnW<'_, RegBtCoexEnhSpec> {
        BitZigbeeEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_coex_enh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_coex_enh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtCoexEnhSpec;
impl crate::RegisterSpec for RegBtCoexEnhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_coex_enh::R`](R) reader structure"]
impl crate::Readable for RegBtCoexEnhSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_coex_enh::W`](W) writer structure"]
impl crate::Writable for RegBtCoexEnhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_COEX_ENH to value 0x1802_1e00"]
impl crate::Resettable for RegBtCoexEnhSpec {
    const RESET_VALUE: u32 = 0x1802_1e00;
}
