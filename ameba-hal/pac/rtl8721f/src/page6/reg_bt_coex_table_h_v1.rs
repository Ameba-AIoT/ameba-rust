#[doc = "Register `REG_BT_COEX_TABLE_H_V1` reader"]
pub type R = crate::R<RegBtCoexTableHV1Spec>;
#[doc = "Register `REG_BT_COEX_TABLE_H_V1` writer"]
pub type W = crate::W<RegBtCoexTableHV1Spec>;
#[doc = "Field `BIT_ANT_SEL_V1` reader - single or dual antenna selection (0: single; 1 : dual)"]
pub type BitAntSelV1R = crate::BitReader;
#[doc = "Field `BIT_ANT_SEL_V1` writer - single or dual antenna selection (0: single; 1 : dual)"]
pub type BitAntSelV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_OOB_V1` reader - out of band indication (0: in band ; 1 : out of band)"]
pub type BitOobV1R = crate::BitReader;
#[doc = "Field `BIT_OOB_V1` writer - out of band indication (0: in band ; 1 : out of band)"]
pub type BitOobV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_RXRESP` reader - Priority Mask for Rx response"]
pub type BitPriMaskRxrespR = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_RXRESP` writer - Priority Mask for Rx response"]
pub type BitPriMaskRxrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_TXRESP` reader - Priority Mask for Tx response"]
pub type BitPriMaskTxrespR = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_TXRESP` writer - Priority Mask for Tx response"]
pub type BitPriMaskTxrespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_TBTT_P0` reader - Priority Mask for P0 Beacon"]
pub type BitPriMaskTbttP0R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_TBTT_P0` writer - Priority Mask for P0 Beacon"]
pub type BitPriMaskTbttP0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_TBTT_P1` reader - Priority Mask for P1 Beacon"]
pub type BitPriMaskTbttP1R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_TBTT_P1` writer - Priority Mask for P1 Beacon"]
pub type BitPriMaskTbttP1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_NUM` reader - Priority Mask for Tx packet number"]
pub type BitPriMaskNumR = crate::FieldReader;
#[doc = "Field `BIT_PRI_MASK_NUM` writer - Priority Mask for Tx packet number"]
pub type BitPriMaskNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_PRI_MASK_RTY_V1` reader - Priority Mask for Tx Retry Packet"]
pub type BitPriMaskRtyV1R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_RTY_V1` writer - Priority Mask for Tx Retry Packet"]
pub type BitPriMaskRtyV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_OFDM_V1` reader - Priority Mask for Tx OFDM"]
pub type BitPriMaskOfdmV1R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_OFDM_V1` writer - Priority Mask for Tx OFDM"]
pub type BitPriMaskOfdmV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_CCK_V1` reader - Priority Mask for Tx CCK"]
pub type BitPriMaskCckV1R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_CCK_V1` writer - Priority Mask for Tx CCK"]
pub type BitPriMaskCckV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_NAV` reader - Priority Mask for Tx NAV"]
pub type BitPriMaskNavR = crate::FieldReader;
#[doc = "Field `BIT_PRI_MASK_NAV` writer - Priority Mask for Tx NAV"]
pub type BitPriMaskNavW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PRI_MASK_TXAC` reader - Priority Mask for Tx Queue"]
pub type BitPriMaskTxacR = crate::FieldReader;
#[doc = "Field `BIT_PRI_MASK_TXAC` writer - Priority Mask for Tx Queue"]
pub type BitPriMaskTxacW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_PRI_MASK_RXCCK_V1` reader - Priority Mask for RX CCK"]
pub type BitPriMaskRxcckV1R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_RXCCK_V1` writer - Priority Mask for RX CCK"]
pub type BitPriMaskRxcckV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_RXOFDM_V1` reader - Priority Mask for RX OFDM"]
pub type BitPriMaskRxofdmV1R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_RXOFDM_V1` writer - Priority Mask for RX OFDM"]
pub type BitPriMaskRxofdmV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_RX_RESP_V1` reader - Priority Mask for RX Response Packet"]
pub type BitPriMaskRxRespV1R = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_RX_RESP_V1` writer - Priority Mask for RX Response Packet"]
pub type BitPriMaskRxRespV1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PRI_MASK_DW_NAN` reader - Priority Mask for P2 NAN DW"]
pub type BitPriMaskDwNanR = crate::BitReader;
#[doc = "Field `BIT_PRI_MASK_DW_NAN` writer - Priority Mask for P2 NAN DW"]
pub type BitPriMaskDwNanW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - single or dual antenna selection (0: single; 1 : dual)"]
    #[inline(always)]
    pub fn bit_ant_sel_v1(&self) -> BitAntSelV1R {
        BitAntSelV1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - out of band indication (0: in band ; 1 : out of band)"]
    #[inline(always)]
    pub fn bit_oob_v1(&self) -> BitOobV1R {
        BitOobV1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Priority Mask for Rx response"]
    #[inline(always)]
    pub fn bit_pri_mask_rxresp(&self) -> BitPriMaskRxrespR {
        BitPriMaskRxrespR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Priority Mask for Tx response"]
    #[inline(always)]
    pub fn bit_pri_mask_txresp(&self) -> BitPriMaskTxrespR {
        BitPriMaskTxrespR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Priority Mask for P0 Beacon"]
    #[inline(always)]
    pub fn bit_pri_mask_tbtt_p0(&self) -> BitPriMaskTbttP0R {
        BitPriMaskTbttP0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Priority Mask for P1 Beacon"]
    #[inline(always)]
    pub fn bit_pri_mask_tbtt_p1(&self) -> BitPriMaskTbttP1R {
        BitPriMaskTbttP1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Priority Mask for Tx packet number"]
    #[inline(always)]
    pub fn bit_pri_mask_num(&self) -> BitPriMaskNumR {
        BitPriMaskNumR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Priority Mask for Tx Retry Packet"]
    #[inline(always)]
    pub fn bit_pri_mask_rty_v1(&self) -> BitPriMaskRtyV1R {
        BitPriMaskRtyV1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Priority Mask for Tx OFDM"]
    #[inline(always)]
    pub fn bit_pri_mask_ofdm_v1(&self) -> BitPriMaskOfdmV1R {
        BitPriMaskOfdmV1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Priority Mask for Tx CCK"]
    #[inline(always)]
    pub fn bit_pri_mask_cck_v1(&self) -> BitPriMaskCckV1R {
        BitPriMaskCckV1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:20 - Priority Mask for Tx NAV"]
    #[inline(always)]
    pub fn bit_pri_mask_nav(&self) -> BitPriMaskNavR {
        BitPriMaskNavR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:27 - Priority Mask for Tx Queue"]
    #[inline(always)]
    pub fn bit_pri_mask_txac(&self) -> BitPriMaskTxacR {
        BitPriMaskTxacR::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Priority Mask for RX CCK"]
    #[inline(always)]
    pub fn bit_pri_mask_rxcck_v1(&self) -> BitPriMaskRxcckV1R {
        BitPriMaskRxcckV1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Priority Mask for RX OFDM"]
    #[inline(always)]
    pub fn bit_pri_mask_rxofdm_v1(&self) -> BitPriMaskRxofdmV1R {
        BitPriMaskRxofdmV1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Priority Mask for RX Response Packet"]
    #[inline(always)]
    pub fn bit_pri_mask_rx_resp_v1(&self) -> BitPriMaskRxRespV1R {
        BitPriMaskRxRespV1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Priority Mask for P2 NAN DW"]
    #[inline(always)]
    pub fn bit_pri_mask_dw_nan(&self) -> BitPriMaskDwNanR {
        BitPriMaskDwNanR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - single or dual antenna selection (0: single; 1 : dual)"]
    #[inline(always)]
    pub fn bit_ant_sel_v1(&mut self) -> BitAntSelV1W<'_, RegBtCoexTableHV1Spec> {
        BitAntSelV1W::new(self, 0)
    }
    #[doc = "Bit 1 - out of band indication (0: in band ; 1 : out of band)"]
    #[inline(always)]
    pub fn bit_oob_v1(&mut self) -> BitOobV1W<'_, RegBtCoexTableHV1Spec> {
        BitOobV1W::new(self, 1)
    }
    #[doc = "Bit 2 - Priority Mask for Rx response"]
    #[inline(always)]
    pub fn bit_pri_mask_rxresp(&mut self) -> BitPriMaskRxrespW<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskRxrespW::new(self, 2)
    }
    #[doc = "Bit 3 - Priority Mask for Tx response"]
    #[inline(always)]
    pub fn bit_pri_mask_txresp(&mut self) -> BitPriMaskTxrespW<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskTxrespW::new(self, 3)
    }
    #[doc = "Bit 4 - Priority Mask for P0 Beacon"]
    #[inline(always)]
    pub fn bit_pri_mask_tbtt_p0(&mut self) -> BitPriMaskTbttP0W<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskTbttP0W::new(self, 4)
    }
    #[doc = "Bit 5 - Priority Mask for P1 Beacon"]
    #[inline(always)]
    pub fn bit_pri_mask_tbtt_p1(&mut self) -> BitPriMaskTbttP1W<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskTbttP1W::new(self, 5)
    }
    #[doc = "Bits 6:9 - Priority Mask for Tx packet number"]
    #[inline(always)]
    pub fn bit_pri_mask_num(&mut self) -> BitPriMaskNumW<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskNumW::new(self, 6)
    }
    #[doc = "Bit 10 - Priority Mask for Tx Retry Packet"]
    #[inline(always)]
    pub fn bit_pri_mask_rty_v1(&mut self) -> BitPriMaskRtyV1W<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskRtyV1W::new(self, 10)
    }
    #[doc = "Bit 11 - Priority Mask for Tx OFDM"]
    #[inline(always)]
    pub fn bit_pri_mask_ofdm_v1(&mut self) -> BitPriMaskOfdmV1W<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskOfdmV1W::new(self, 11)
    }
    #[doc = "Bit 12 - Priority Mask for Tx CCK"]
    #[inline(always)]
    pub fn bit_pri_mask_cck_v1(&mut self) -> BitPriMaskCckV1W<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskCckV1W::new(self, 12)
    }
    #[doc = "Bits 13:20 - Priority Mask for Tx NAV"]
    #[inline(always)]
    pub fn bit_pri_mask_nav(&mut self) -> BitPriMaskNavW<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskNavW::new(self, 13)
    }
    #[doc = "Bits 21:27 - Priority Mask for Tx Queue"]
    #[inline(always)]
    pub fn bit_pri_mask_txac(&mut self) -> BitPriMaskTxacW<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskTxacW::new(self, 21)
    }
    #[doc = "Bit 28 - Priority Mask for RX CCK"]
    #[inline(always)]
    pub fn bit_pri_mask_rxcck_v1(&mut self) -> BitPriMaskRxcckV1W<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskRxcckV1W::new(self, 28)
    }
    #[doc = "Bit 29 - Priority Mask for RX OFDM"]
    #[inline(always)]
    pub fn bit_pri_mask_rxofdm_v1(&mut self) -> BitPriMaskRxofdmV1W<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskRxofdmV1W::new(self, 29)
    }
    #[doc = "Bit 30 - Priority Mask for RX Response Packet"]
    #[inline(always)]
    pub fn bit_pri_mask_rx_resp_v1(&mut self) -> BitPriMaskRxRespV1W<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskRxRespV1W::new(self, 30)
    }
    #[doc = "Bit 31 - Priority Mask for P2 NAN DW"]
    #[inline(always)]
    pub fn bit_pri_mask_dw_nan(&mut self) -> BitPriMaskDwNanW<'_, RegBtCoexTableHV1Spec> {
        BitPriMaskDwNanW::new(self, 31)
    }
}
#[doc = "BT Coexistence Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_coex_table_h_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_coex_table_h_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtCoexTableHV1Spec;
impl crate::RegisterSpec for RegBtCoexTableHV1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_coex_table_h_v1::R`](R) reader structure"]
impl crate::Readable for RegBtCoexTableHV1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_coex_table_h_v1::W`](W) writer structure"]
impl crate::Writable for RegBtCoexTableHV1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_COEX_TABLE_H_V1 to value 0"]
impl crate::Resettable for RegBtCoexTableHV1Spec {}
