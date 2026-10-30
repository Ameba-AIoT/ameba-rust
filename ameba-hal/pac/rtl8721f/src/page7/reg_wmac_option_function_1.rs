#[doc = "Register `REG_WMAC_OPTION_FUNCTION_1` reader"]
pub type R = crate::R<RegWmacOptionFunction1Spec>;
#[doc = "Register `REG_WMAC_OPTION_FUNCTION_1` writer"]
pub type W = crate::W<RegWmacOptionFunction1Spec>;
#[doc = "Field `BIT_r_WMAC_CHK_CCK_LEN_1` reader - enable to check rx cck len"]
pub type BitRWmacChkCckLen1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_CHK_CCK_LEN_1` writer - enable to check rx cck len"]
pub type BitRWmacChkCckLen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_CHK_OFDM_LEN_1` reader - enable to check ofdm len"]
pub type BitRWmacChkOfdmLen1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_CHK_OFDM_LEN_1` writer - enable to check ofdm len"]
pub type BitRWmacChkOfdmLen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_OFDM_FILTER_1` reader - RSVD"]
pub type BitROfdmFilter1R = crate::BitReader;
#[doc = "Field `BIT_r_OFDM_FILTER_1` writer - RSVD"]
pub type BitROfdmFilter1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RXLEN_SEL1_1` reader - rx vht len sel lsig len if sigb len is more than lsig len"]
pub type BitRWmacRxlenSel1_1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RXLEN_SEL1_1` writer - rx vht len sel lsig len if sigb len is more than lsig len"]
pub type BitRWmacRxlenSel1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RXLEN_SEL_1` reader - rx vht len sel lsig len VHT PPDU length selection 0: when a VHT PPDU packet is received, parsing and calculation are based on the VHTSIGB PPDU length 1: when a VHT PPDU packet is received, parsing and calculation are based on the LSIG length computed by BB"]
pub type BitRWmacRxlenSel1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RXLEN_SEL_1` writer - rx vht len sel lsig len VHT PPDU length selection 0: when a VHT PPDU packet is received, parsing and calculation are based on the VHTSIGB PPDU length 1: when a VHT PPDU packet is received, parsing and calculation are based on the LSIG length computed by BB"]
pub type BitRWmacRxlenSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_NDP_FILTER_1` reader - mac rx ndp pkt set ppdu len to 0"]
pub type BitRWmacNdpFilter1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_NDP_FILTER_1` writer - mac rx ndp pkt set ppdu len to 0"]
pub type BitRWmacNdpFilter1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_TCR_MPAR_NDP_1` reader - nop ok check bb ndp vld & mac rx ndp check ok"]
pub type BitRWmacTcrMparNdp1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_TCR_MPAR_NDP_1` writer - nop ok check bb ndp vld & mac rx ndp check ok"]
pub type BitRWmacTcrMparNdp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_MAC_TCR_VBONF_RD_1` reader - vbon neg edge to read all data of mac rx sync fifo for rx parser"]
pub type BitRMacTcrVbonfRd1R = crate::BitReader;
#[doc = "Field `BIT_r_MAC_TCR_VBONF_RD_1` writer - vbon neg edge to read all data of mac rx sync fifo for rx parser"]
pub type BitRMacTcrVbonfRd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_PHYSTS_PLCP_1` reader - append plcp header after rxdesc"]
pub type BitRWmacPhystsPlcp1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_PHYSTS_PLCP_1` writer - append plcp header after rxdesc"]
pub type BitRWmacPhystsPlcp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_PHYSTS_SNIF_1` reader - append sniff info after rxdesc"]
pub type BitRWmacPhystsSnif1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_PHYSTS_SNIF_1` writer - append sniff info after rxdesc"]
pub type BitRWmacPhystsSnif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_FIL_FCPROVER_1` reader - drop the ofdm pkt if the rx ofdm leg pkt fc version illegal"]
pub type BitRWmacFilFcprover1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_FIL_FCPROVER_1` writer - drop the ofdm pkt if the rx ofdm leg pkt fc version illegal"]
pub type BitRWmacFilFcprover1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_FIL_FCTYPE_1` reader - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
pub type BitRWmacFilFctype1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_FIL_FCTYPE_1` writer - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
pub type BitRWmacFilFctype1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_FIL_CTLPKTLEN_1` reader - drop the rx pkt if the rx ctrl pkt len large than r_WMAC_RX_FIL_LEN"]
pub type BitRWmacFilCtlpktlen1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_FIL_CTLPKTLEN_1` writer - drop the rx pkt if the rx ctrl pkt len large than r_WMAC_RX_FIL_LEN"]
pub type BitRWmacFilCtlpktlen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_FIL_SECERR_1` reader - sec len error drop(sec len is zero)"]
pub type BitRWmacFilSecerr1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_FIL_SECERR_1` writer - sec len error drop(sec len is zero)"]
pub type BitRWmacFilSecerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_PFIN_TOEN_1` reader - rx fifo fsm flush state timeout to error stage"]
pub type BitRWmacPfinToen1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_PFIN_TOEN_1` writer - rx fifo fsm flush state timeout to error stage"]
pub type BitRWmacPfinToen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_MID_1` reader - enable search txrpt buffer for macid"]
pub type BitRWmacSrchTxrptMid1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_MID_1` writer - enable search txrpt buffer for macid"]
pub type BitRWmacSrchTxrptMid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_PERPKT_1` reader - enable search txrpt buffer for macid every pkt"]
pub type BitRWmacSrchTxrptPerpkt1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_PERPKT_1` writer - enable search txrpt buffer for macid every pkt"]
pub type BitRWmacSrchTxrptPerpkt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_POWINT_EN_1` reader - enable rx pkt fc column power bit int for FISR"]
pub type BitRWmacPowintEn1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_POWINT_EN_1` writer - enable rx pkt fc column power bit int for FISR"]
pub type BitRWmacPowintEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_NDP_RST_1` reader - rx pkt is ndp to clr macrx_active"]
pub type BitRWmacNdpRst1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_NDP_RST_1` writer - rx pkt is ndp to clr macrx_active"]
pub type BitRWmacNdpRst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_TYPE_1` reader - for rx ack/cts pkt not to search txrpt buffer macid"]
pub type BitRWmacSrchTxrptType1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_TYPE_1` writer - for rx ack/cts pkt not to search txrpt buffer macid"]
pub type BitRWmacSrchTxrptType1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_UA1_1` reader - when this bit is set 1, use tx A1 to search txrpt when rx ack/cts, when BIT_r_WMAC_SRCH_TXRPT_TYPE is set 0, BIT_r_WMAC_SRCH_TXRPT_UA1 should be set 1"]
pub type BitRWmacSrchTxrptUa1_1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_UA1_1` writer - when this bit is set 1, use tx A1 to search txrpt when rx ack/cts, when BIT_r_WMAC_SRCH_TXRPT_TYPE is set 0, BIT_r_WMAC_SRCH_TXRPT_UA1 should be set 1"]
pub type BitRWmacSrchTxrptUa1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_REF_DROP_1` reader - when this bit is set 1, reference macid drop information when macid search"]
pub type BitRWmacSrchTxrptRefDrop1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_SRCH_TXRPT_REF_DROP_1` writer - when this bit is set 1, reference macid drop information when macid search"]
pub type BitRWmacSrchTxrptRefDrop1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RXRST_DLY_1` reader - when this bit is set 1, rxrst pulse lasts for 20 cycle of MAC clock, when this bit is set 0, rxrst pulse lasts for 10 cycle of MAC clock,"]
pub type BitRWmacRxrstDly1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RXRST_DLY_1` writer - when this bit is set 1, rxrst pulse lasts for 20 cycle of MAC clock, when this bit is set 0, rxrst pulse lasts for 10 cycle of MAC clock,"]
pub type BitRWmacRxrstDly1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RX_SYNCFIFO_SYNC_1` reader - "]
pub type BitRWmacRxSyncfifoSync1R = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_RX_SYNCFIFO_SYNC_1` writer - "]
pub type BitRWmacRxSyncfifoSync1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_WMAC_RXFIFO_FULL_TH_1` reader - csi report mac pause bb almost full threshold"]
pub type BitRWmacRxfifoFullTh1R = crate::FieldReader;
#[doc = "Field `BIT_r_WMAC_RXFIFO_FULL_TH_1` writer - csi report mac pause bb almost full threshold"]
pub type BitRWmacRxfifoFullTh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - enable to check rx cck len"]
    #[inline(always)]
    pub fn bit_r_wmac_chk_cck_len_1(&self) -> BitRWmacChkCckLen1R {
        BitRWmacChkCckLen1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable to check ofdm len"]
    #[inline(always)]
    pub fn bit_r_wmac_chk_ofdm_len_1(&self) -> BitRWmacChkOfdmLen1R {
        BitRWmacChkOfdmLen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RSVD"]
    #[inline(always)]
    pub fn bit_r_ofdm_filter_1(&self) -> BitROfdmFilter1R {
        BitROfdmFilter1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - rx vht len sel lsig len if sigb len is more than lsig len"]
    #[inline(always)]
    pub fn bit_r_wmac_rxlen_sel1_1(&self) -> BitRWmacRxlenSel1_1R {
        BitRWmacRxlenSel1_1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rx vht len sel lsig len VHT PPDU length selection 0: when a VHT PPDU packet is received, parsing and calculation are based on the VHTSIGB PPDU length 1: when a VHT PPDU packet is received, parsing and calculation are based on the LSIG length computed by BB"]
    #[inline(always)]
    pub fn bit_r_wmac_rxlen_sel_1(&self) -> BitRWmacRxlenSel1R {
        BitRWmacRxlenSel1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - mac rx ndp pkt set ppdu len to 0"]
    #[inline(always)]
    pub fn bit_r_wmac_ndp_filter_1(&self) -> BitRWmacNdpFilter1R {
        BitRWmacNdpFilter1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - nop ok check bb ndp vld & mac rx ndp check ok"]
    #[inline(always)]
    pub fn bit_r_wmac_tcr_mpar_ndp_1(&self) -> BitRWmacTcrMparNdp1R {
        BitRWmacTcrMparNdp1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - vbon neg edge to read all data of mac rx sync fifo for rx parser"]
    #[inline(always)]
    pub fn bit_r_mac_tcr_vbonf_rd_1(&self) -> BitRMacTcrVbonfRd1R {
        BitRMacTcrVbonfRd1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - append plcp header after rxdesc"]
    #[inline(always)]
    pub fn bit_r_wmac_physts_plcp_1(&self) -> BitRWmacPhystsPlcp1R {
        BitRWmacPhystsPlcp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - append sniff info after rxdesc"]
    #[inline(always)]
    pub fn bit_r_wmac_physts_snif_1(&self) -> BitRWmacPhystsSnif1R {
        BitRWmacPhystsSnif1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - drop the ofdm pkt if the rx ofdm leg pkt fc version illegal"]
    #[inline(always)]
    pub fn bit_r_wmac_fil_fcprover_1(&self) -> BitRWmacFilFcprover1R {
        BitRWmacFilFcprover1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
    #[inline(always)]
    pub fn bit_r_wmac_fil_fctype_1(&self) -> BitRWmacFilFctype1R {
        BitRWmacFilFctype1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - drop the rx pkt if the rx ctrl pkt len large than r_WMAC_RX_FIL_LEN"]
    #[inline(always)]
    pub fn bit_r_wmac_fil_ctlpktlen_1(&self) -> BitRWmacFilCtlpktlen1R {
        BitRWmacFilCtlpktlen1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - sec len error drop(sec len is zero)"]
    #[inline(always)]
    pub fn bit_r_wmac_fil_secerr_1(&self) -> BitRWmacFilSecerr1R {
        BitRWmacFilSecerr1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - rx fifo fsm flush state timeout to error stage"]
    #[inline(always)]
    pub fn bit_r_wmac_pfin_toen_1(&self) -> BitRWmacPfinToen1R {
        BitRWmacPfinToen1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - enable search txrpt buffer for macid"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_mid_1(&self) -> BitRWmacSrchTxrptMid1R {
        BitRWmacSrchTxrptMid1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - enable search txrpt buffer for macid every pkt"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_perpkt_1(&self) -> BitRWmacSrchTxrptPerpkt1R {
        BitRWmacSrchTxrptPerpkt1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - enable rx pkt fc column power bit int for FISR"]
    #[inline(always)]
    pub fn bit_r_wmac_powint_en_1(&self) -> BitRWmacPowintEn1R {
        BitRWmacPowintEn1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - rx pkt is ndp to clr macrx_active"]
    #[inline(always)]
    pub fn bit_r_wmac_ndp_rst_1(&self) -> BitRWmacNdpRst1R {
        BitRWmacNdpRst1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - for rx ack/cts pkt not to search txrpt buffer macid"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_type_1(&self) -> BitRWmacSrchTxrptType1R {
        BitRWmacSrchTxrptType1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - when this bit is set 1, use tx A1 to search txrpt when rx ack/cts, when BIT_r_WMAC_SRCH_TXRPT_TYPE is set 0, BIT_r_WMAC_SRCH_TXRPT_UA1 should be set 1"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_ua1_1(&self) -> BitRWmacSrchTxrptUa1_1R {
        BitRWmacSrchTxrptUa1_1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - when this bit is set 1, reference macid drop information when macid search"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_ref_drop_1(&self) -> BitRWmacSrchTxrptRefDrop1R {
        BitRWmacSrchTxrptRefDrop1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - when this bit is set 1, rxrst pulse lasts for 20 cycle of MAC clock, when this bit is set 0, rxrst pulse lasts for 10 cycle of MAC clock,"]
    #[inline(always)]
    pub fn bit_r_wmac_rxrst_dly_1(&self) -> BitRWmacRxrstDly1R {
        BitRWmacRxrstDly1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bit_r_wmac_rx_syncfifo_sync_1(&self) -> BitRWmacRxSyncfifoSync1R {
        BitRWmacRxSyncfifoSync1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - csi report mac pause bb almost full threshold"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfifo_full_th_1(&self) -> BitRWmacRxfifoFullTh1R {
        BitRWmacRxfifoFullTh1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable to check rx cck len"]
    #[inline(always)]
    pub fn bit_r_wmac_chk_cck_len_1(
        &mut self,
    ) -> BitRWmacChkCckLen1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacChkCckLen1W::new(self, 0)
    }
    #[doc = "Bit 1 - enable to check ofdm len"]
    #[inline(always)]
    pub fn bit_r_wmac_chk_ofdm_len_1(
        &mut self,
    ) -> BitRWmacChkOfdmLen1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacChkOfdmLen1W::new(self, 1)
    }
    #[doc = "Bit 2 - RSVD"]
    #[inline(always)]
    pub fn bit_r_ofdm_filter_1(&mut self) -> BitROfdmFilter1W<'_, RegWmacOptionFunction1Spec> {
        BitROfdmFilter1W::new(self, 2)
    }
    #[doc = "Bit 3 - rx vht len sel lsig len if sigb len is more than lsig len"]
    #[inline(always)]
    pub fn bit_r_wmac_rxlen_sel1_1(
        &mut self,
    ) -> BitRWmacRxlenSel1_1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacRxlenSel1_1W::new(self, 3)
    }
    #[doc = "Bit 4 - rx vht len sel lsig len VHT PPDU length selection 0: when a VHT PPDU packet is received, parsing and calculation are based on the VHTSIGB PPDU length 1: when a VHT PPDU packet is received, parsing and calculation are based on the LSIG length computed by BB"]
    #[inline(always)]
    pub fn bit_r_wmac_rxlen_sel_1(&mut self) -> BitRWmacRxlenSel1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacRxlenSel1W::new(self, 4)
    }
    #[doc = "Bit 5 - mac rx ndp pkt set ppdu len to 0"]
    #[inline(always)]
    pub fn bit_r_wmac_ndp_filter_1(
        &mut self,
    ) -> BitRWmacNdpFilter1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacNdpFilter1W::new(self, 5)
    }
    #[doc = "Bit 6 - nop ok check bb ndp vld & mac rx ndp check ok"]
    #[inline(always)]
    pub fn bit_r_wmac_tcr_mpar_ndp_1(
        &mut self,
    ) -> BitRWmacTcrMparNdp1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacTcrMparNdp1W::new(self, 6)
    }
    #[doc = "Bit 7 - vbon neg edge to read all data of mac rx sync fifo for rx parser"]
    #[inline(always)]
    pub fn bit_r_mac_tcr_vbonf_rd_1(
        &mut self,
    ) -> BitRMacTcrVbonfRd1W<'_, RegWmacOptionFunction1Spec> {
        BitRMacTcrVbonfRd1W::new(self, 7)
    }
    #[doc = "Bit 8 - append plcp header after rxdesc"]
    #[inline(always)]
    pub fn bit_r_wmac_physts_plcp_1(
        &mut self,
    ) -> BitRWmacPhystsPlcp1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacPhystsPlcp1W::new(self, 8)
    }
    #[doc = "Bit 9 - append sniff info after rxdesc"]
    #[inline(always)]
    pub fn bit_r_wmac_physts_snif_1(
        &mut self,
    ) -> BitRWmacPhystsSnif1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacPhystsSnif1W::new(self, 9)
    }
    #[doc = "Bit 10 - drop the ofdm pkt if the rx ofdm leg pkt fc version illegal"]
    #[inline(always)]
    pub fn bit_r_wmac_fil_fcprover_1(
        &mut self,
    ) -> BitRWmacFilFcprover1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacFilFcprover1W::new(self, 10)
    }
    #[doc = "Bit 11 - drop the ofdm pkt if the rx ofdm leg pkt fc type is 2'b11"]
    #[inline(always)]
    pub fn bit_r_wmac_fil_fctype_1(
        &mut self,
    ) -> BitRWmacFilFctype1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacFilFctype1W::new(self, 11)
    }
    #[doc = "Bit 12 - drop the rx pkt if the rx ctrl pkt len large than r_WMAC_RX_FIL_LEN"]
    #[inline(always)]
    pub fn bit_r_wmac_fil_ctlpktlen_1(
        &mut self,
    ) -> BitRWmacFilCtlpktlen1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacFilCtlpktlen1W::new(self, 12)
    }
    #[doc = "Bit 13 - sec len error drop(sec len is zero)"]
    #[inline(always)]
    pub fn bit_r_wmac_fil_secerr_1(
        &mut self,
    ) -> BitRWmacFilSecerr1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacFilSecerr1W::new(self, 13)
    }
    #[doc = "Bit 14 - rx fifo fsm flush state timeout to error stage"]
    #[inline(always)]
    pub fn bit_r_wmac_pfin_toen_1(&mut self) -> BitRWmacPfinToen1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacPfinToen1W::new(self, 14)
    }
    #[doc = "Bit 15 - enable search txrpt buffer for macid"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_mid_1(
        &mut self,
    ) -> BitRWmacSrchTxrptMid1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacSrchTxrptMid1W::new(self, 15)
    }
    #[doc = "Bit 16 - enable search txrpt buffer for macid every pkt"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_perpkt_1(
        &mut self,
    ) -> BitRWmacSrchTxrptPerpkt1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacSrchTxrptPerpkt1W::new(self, 16)
    }
    #[doc = "Bit 17 - enable rx pkt fc column power bit int for FISR"]
    #[inline(always)]
    pub fn bit_r_wmac_powint_en_1(&mut self) -> BitRWmacPowintEn1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacPowintEn1W::new(self, 17)
    }
    #[doc = "Bit 18 - rx pkt is ndp to clr macrx_active"]
    #[inline(always)]
    pub fn bit_r_wmac_ndp_rst_1(&mut self) -> BitRWmacNdpRst1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacNdpRst1W::new(self, 18)
    }
    #[doc = "Bit 19 - for rx ack/cts pkt not to search txrpt buffer macid"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_type_1(
        &mut self,
    ) -> BitRWmacSrchTxrptType1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacSrchTxrptType1W::new(self, 19)
    }
    #[doc = "Bit 20 - when this bit is set 1, use tx A1 to search txrpt when rx ack/cts, when BIT_r_WMAC_SRCH_TXRPT_TYPE is set 0, BIT_r_WMAC_SRCH_TXRPT_UA1 should be set 1"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_ua1_1(
        &mut self,
    ) -> BitRWmacSrchTxrptUa1_1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacSrchTxrptUa1_1W::new(self, 20)
    }
    #[doc = "Bit 21 - when this bit is set 1, reference macid drop information when macid search"]
    #[inline(always)]
    pub fn bit_r_wmac_srch_txrpt_ref_drop_1(
        &mut self,
    ) -> BitRWmacSrchTxrptRefDrop1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacSrchTxrptRefDrop1W::new(self, 21)
    }
    #[doc = "Bit 22 - when this bit is set 1, rxrst pulse lasts for 20 cycle of MAC clock, when this bit is set 0, rxrst pulse lasts for 10 cycle of MAC clock,"]
    #[inline(always)]
    pub fn bit_r_wmac_rxrst_dly_1(&mut self) -> BitRWmacRxrstDly1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacRxrstDly1W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bit_r_wmac_rx_syncfifo_sync_1(
        &mut self,
    ) -> BitRWmacRxSyncfifoSync1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacRxSyncfifoSync1W::new(self, 23)
    }
    #[doc = "Bits 24:31 - csi report mac pause bb almost full threshold"]
    #[inline(always)]
    pub fn bit_r_wmac_rxfifo_full_th_1(
        &mut self,
    ) -> BitRWmacRxfifoFullTh1W<'_, RegWmacOptionFunction1Spec> {
        BitRWmacRxfifoFullTh1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_option_function_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_option_function_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacOptionFunction1Spec;
impl crate::RegisterSpec for RegWmacOptionFunction1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_option_function_1::R`](R) reader structure"]
impl crate::Readable for RegWmacOptionFunction1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_option_function_1::W`](W) writer structure"]
impl crate::Writable for RegWmacOptionFunction1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_OPTION_FUNCTION_1 to value 0x5480_a000"]
impl crate::Resettable for RegWmacOptionFunction1Spec {
    const RESET_VALUE: u32 = 0x5480_a000;
}
