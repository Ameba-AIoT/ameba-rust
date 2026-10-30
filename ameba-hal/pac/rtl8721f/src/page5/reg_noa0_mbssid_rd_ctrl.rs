#[doc = "Register `REG_NOA0_MBSSID_RD_CTRL` reader"]
pub type R = crate::R<RegNoa0MbssidRdCtrlSpec>;
#[doc = "Register `REG_NOA0_MBSSID_RD_CTRL` writer"]
pub type W = crate::W<RegNoa0MbssidRdCtrlSpec>;
#[doc = "Field `BIT_VOQ_RD_RESP_EN` reader - Enable VOQ RD responder function"]
pub type BitVoqRdRespEnR = crate::BitReader;
#[doc = "Field `BIT_VOQ_RD_RESP_EN` writer - Enable VOQ RD responder function"]
pub type BitVoqRdRespEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VIQ_RD_RESP_EN` reader - Enable VIQ RD responder function"]
pub type BitViqRdRespEnR = crate::BitReader;
#[doc = "Field `BIT_VIQ_RD_RESP_EN` writer - Enable VIQ RD responder function"]
pub type BitViqRdRespEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BEQ_RD_RESP_EN` reader - Enable BEQ RD responder function"]
pub type BitBeqRdRespEnR = crate::BitReader;
#[doc = "Field `BIT_BEQ_RD_RESP_EN` writer - Enable BEQ RD responder function"]
pub type BitBeqRdRespEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BKQ_RD_RESP_EN` reader - Enable BKQ RD responder function"]
pub type BitBkqRdRespEnR = crate::BitReader;
#[doc = "Field `BIT_BKQ_RD_RESP_EN` writer - Enable BKQ RD responder function"]
pub type BitBkqRdRespEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VOQ_RD_INIT_EN` reader - Enable VOQ RD initiator function"]
pub type BitVoqRdInitEnR = crate::BitReader;
#[doc = "Field `BIT_VOQ_RD_INIT_EN` writer - Enable VOQ RD initiator function"]
pub type BitVoqRdInitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_VIQ_RD_INIT_EN` reader - Enable VIQ RD initiator function"]
pub type BitViqRdInitEnR = crate::BitReader;
#[doc = "Field `BIT_VIQ_RD_INIT_EN` writer - Enable VIQ RD initiator function"]
pub type BitViqRdInitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BEQ_RD_INIT_EN` reader - Enable BEQ RD initiator function"]
pub type BitBeqRdInitEnR = crate::BitReader;
#[doc = "Field `BIT_BEQ_RD_INIT_EN` writer - Enable BEQ RD initiator function"]
pub type BitBeqRdInitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BKQ_RD_INIT_EN` reader - Enable BKQ RD initiator function"]
pub type BitBkqRdInitEnR = crate::BitReader;
#[doc = "Field `BIT_BKQ_RD_INIT_EN` writer - Enable BKQ RD initiator function"]
pub type BitBkqRdInitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_STBC_CFE` reader - Disable STBC CF-END function"]
pub type BitDisStbcCfeR = crate::BitReader;
#[doc = "Field `BIT_DIS_STBC_CFE` writer - Disable STBC CF-END function"]
pub type BitDisStbcCfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_LSIG_CFE` reader - Disable LSIG_TXOP CF-END function"]
pub type BitDisLsigCfeR = crate::BitReader;
#[doc = "Field `BIT_DIS_LSIG_CFE` writer - Disable LSIG_TXOP CF-END function"]
pub type BitDisLsigCfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXOP_CFE` reader - Disable TXOP_CFE function"]
pub type BitDisTxopCfeR = crate::BitReader;
#[doc = "Field `BIT_DIS_TXOP_CFE` writer - Disable TXOP_CFE function"]
pub type BitDisTxopCfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EDCCA_MSK_CNTDOWN_EN` reader - 1: disable EDCCA count down, 0: enable EDCCA count down Whether AC backoff needs to reference PHY_EDCCA 0: does not reference PHY_EDCCA 1: references PHY_EDCCA (default, backoff stops when PHY_EDCCA=1)"]
pub type BitEdccaMskCntdownEnR = crate::BitReader;
#[doc = "Field `BIT_EDCCA_MSK_CNTDOWN_EN` writer - 1: disable EDCCA count down, 0: enable EDCCA count down Whether AC backoff needs to reference PHY_EDCCA 0: does not reference PHY_EDCCA 1: references PHY_EDCCA (default, backoff stops when PHY_EDCCA=1)"]
pub type BitEdccaMskCntdownEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BCNERR_INEDCCA` reader - eneble force tx becon ignore EDCCA in disable contention area"]
pub type BitEnBcnerrInedccaR = crate::BitReader;
#[doc = "Field `BIT_EN_BCNERR_INEDCCA` writer - eneble force tx becon ignore EDCCA in disable contention area"]
pub type BitEnBcnerrInedccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BCNERR_INCCCA` reader - eneble force tx becon ignore CCA in disable contention area"]
pub type BitEnBcnerrIncccaR = crate::BitReader;
#[doc = "Field `BIT_EN_BCNERR_INCCCA` writer - eneble force tx becon ignore CCA in disable contention area"]
pub type BitEnBcnerrIncccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TX_OVER_BCNQ` reader - 1: Tx Packet with ignoring Beacon Area time 0: Disable"]
pub type BitDisTxOverBcnqR = crate::BitReader;
#[doc = "Field `BIT_DIS_TX_OVER_BCNQ` writer - 1: Tx Packet with ignoring Beacon Area time 0: Disable"]
pub type BitDisTxOverBcnqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_CLR_TXREQ_INCCA` reader - not valid."]
pub type BitEnClrTxreqInccaR = crate::BitReader;
#[doc = "Field `BIT_EN_CLR_TXREQ_INCCA` writer - not valid."]
pub type BitEnClrTxreqInccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCNQ0_EN` reader - When enable, auto MBSSID for root interface is enable."]
pub type BitMbidBcnq0EnR = crate::BitReader;
#[doc = "Field `BIT_MBID_BCNQ0_EN` writer - When enable, auto MBSSID for root interface is enable."]
pub type BitMbidBcnq0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCNQ1_EN` reader - When enable, auto MBSSID for virtual interface 1 is enable."]
pub type BitMbidBcnq1EnR = crate::BitReader;
#[doc = "Field `BIT_MBID_BCNQ1_EN` writer - When enable, auto MBSSID for virtual interface 1 is enable."]
pub type BitMbidBcnq1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCNQ2_EN` reader - When enable, auto MBSSID for virtual interface 2 is enable."]
pub type BitMbidBcnq2EnR = crate::BitReader;
#[doc = "Field `BIT_MBID_BCNQ2_EN` writer - When enable, auto MBSSID for virtual interface 2 is enable."]
pub type BitMbidBcnq2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCNQ3_EN` reader - When enable, auto MBSSID for virtual interface 3 is enable."]
pub type BitMbidBcnq3EnR = crate::BitReader;
#[doc = "Field `BIT_MBID_BCNQ3_EN` writer - When enable, auto MBSSID for virtual interface 3 is enable."]
pub type BitMbidBcnq3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCNQ4_EN` reader - When enable, auto MBSSID for virtual interface 4 is enable."]
pub type BitMbidBcnq4EnR = crate::BitReader;
#[doc = "Field `BIT_MBID_BCNQ4_EN` writer - When enable, auto MBSSID for virtual interface 4 is enable."]
pub type BitMbidBcnq4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCNQ5_EN` reader - When enable, auto MBSSID for virtual interface 5 is enable."]
pub type BitMbidBcnq5EnR = crate::BitReader;
#[doc = "Field `BIT_MBID_BCNQ5_EN` writer - When enable, auto MBSSID for virtual interface 5 is enable."]
pub type BitMbidBcnq5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCNQ6_EN` reader - When enable, auto MBSSID for virtual interface 6 is enable."]
pub type BitMbidBcnq6EnR = crate::BitReader;
#[doc = "Field `BIT_MBID_BCNQ6_EN` writer - When enable, auto MBSSID for virtual interface 6 is enable."]
pub type BitMbidBcnq6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCNQ7_EN` reader - When enable, auto MBSSID for virtual interface 7 is enable."]
pub type BitMbidBcnq7EnR = crate::BitReader;
#[doc = "Field `BIT_MBID_BCNQ7_EN` writer - When enable, auto MBSSID for virtual interface 7 is enable."]
pub type BitMbidBcnq7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TSFBIT32_RST_NOA0` reader - when enable, if tsft_bit32_int is asserted, reset power_cs of noa_unit0"]
pub type BitEnTsfbit32RstNoa0R = crate::BitReader;
#[doc = "Field `BIT_EN_TSFBIT32_RST_NOA0` writer - when enable, if tsft_bit32_int is asserted, reset power_cs of noa_unit0"]
pub type BitEnTsfbit32RstNoa0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA0_UNIT_SEL` reader - The selector for the source of noa unit0. 0 for port 0, 1 for port 1,2 for port2"]
pub type BitP2pNoa0UnitSelR = crate::FieldReader;
#[doc = "Field `BIT_P2P_NOA0_UNIT_SEL` writer - The selector for the source of noa unit0. 0 for port 0, 1 for port 1,2 for port2"]
pub type BitP2pNoa0UnitSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_P2P_NOA0_EN` reader - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
pub type BitP2pNoa0EnR = crate::FieldReader;
#[doc = "Field `BIT_P2P_NOA0_EN` writer - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
pub type BitP2pNoa0EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_P2P_NOA0_PWR_MGT_EN` reader - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
pub type BitP2pNoa0PwrMgtEnR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA0_PWR_MGT_EN` writer - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
pub type BitP2pNoa0PwrMgtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA0_DISTX_EN` reader - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
pub type BitP2pNoa0DistxEnR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA0_DISTX_EN` writer - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
pub type BitP2pNoa0DistxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA0_ALLSTASLEEP` reader - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
pub type BitP2pNoa0AllstasleepR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA0_ALLSTASLEEP` writer - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
pub type BitP2pNoa0AllstasleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable VOQ RD responder function"]
    #[inline(always)]
    pub fn bit_voq_rd_resp_en(&self) -> BitVoqRdRespEnR {
        BitVoqRdRespEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable VIQ RD responder function"]
    #[inline(always)]
    pub fn bit_viq_rd_resp_en(&self) -> BitViqRdRespEnR {
        BitViqRdRespEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable BEQ RD responder function"]
    #[inline(always)]
    pub fn bit_beq_rd_resp_en(&self) -> BitBeqRdRespEnR {
        BitBeqRdRespEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable BKQ RD responder function"]
    #[inline(always)]
    pub fn bit_bkq_rd_resp_en(&self) -> BitBkqRdRespEnR {
        BitBkqRdRespEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable VOQ RD initiator function"]
    #[inline(always)]
    pub fn bit_voq_rd_init_en(&self) -> BitVoqRdInitEnR {
        BitVoqRdInitEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable VIQ RD initiator function"]
    #[inline(always)]
    pub fn bit_viq_rd_init_en(&self) -> BitViqRdInitEnR {
        BitViqRdInitEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable BEQ RD initiator function"]
    #[inline(always)]
    pub fn bit_beq_rd_init_en(&self) -> BitBeqRdInitEnR {
        BitBeqRdInitEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable BKQ RD initiator function"]
    #[inline(always)]
    pub fn bit_bkq_rd_init_en(&self) -> BitBkqRdInitEnR {
        BitBkqRdInitEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable STBC CF-END function"]
    #[inline(always)]
    pub fn bit_dis_stbc_cfe(&self) -> BitDisStbcCfeR {
        BitDisStbcCfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable LSIG_TXOP CF-END function"]
    #[inline(always)]
    pub fn bit_dis_lsig_cfe(&self) -> BitDisLsigCfeR {
        BitDisLsigCfeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable TXOP_CFE function"]
    #[inline(always)]
    pub fn bit_dis_txop_cfe(&self) -> BitDisTxopCfeR {
        BitDisTxopCfeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: disable EDCCA count down, 0: enable EDCCA count down Whether AC backoff needs to reference PHY_EDCCA 0: does not reference PHY_EDCCA 1: references PHY_EDCCA (default, backoff stops when PHY_EDCCA=1)"]
    #[inline(always)]
    pub fn bit_edcca_msk_cntdown_en(&self) -> BitEdccaMskCntdownEnR {
        BitEdccaMskCntdownEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - eneble force tx becon ignore EDCCA in disable contention area"]
    #[inline(always)]
    pub fn bit_en_bcnerr_inedcca(&self) -> BitEnBcnerrInedccaR {
        BitEnBcnerrInedccaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - eneble force tx becon ignore CCA in disable contention area"]
    #[inline(always)]
    pub fn bit_en_bcnerr_inccca(&self) -> BitEnBcnerrIncccaR {
        BitEnBcnerrIncccaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Tx Packet with ignoring Beacon Area time 0: Disable"]
    #[inline(always)]
    pub fn bit_dis_tx_over_bcnq(&self) -> BitDisTxOverBcnqR {
        BitDisTxOverBcnqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - not valid."]
    #[inline(always)]
    pub fn bit_en_clr_txreq_incca(&self) -> BitEnClrTxreqInccaR {
        BitEnClrTxreqInccaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When enable, auto MBSSID for root interface is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq0_en(&self) -> BitMbidBcnq0EnR {
        BitMbidBcnq0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When enable, auto MBSSID for virtual interface 1 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq1_en(&self) -> BitMbidBcnq1EnR {
        BitMbidBcnq1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When enable, auto MBSSID for virtual interface 2 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq2_en(&self) -> BitMbidBcnq2EnR {
        BitMbidBcnq2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When enable, auto MBSSID for virtual interface 3 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq3_en(&self) -> BitMbidBcnq3EnR {
        BitMbidBcnq3EnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When enable, auto MBSSID for virtual interface 4 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq4_en(&self) -> BitMbidBcnq4EnR {
        BitMbidBcnq4EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When enable, auto MBSSID for virtual interface 5 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq5_en(&self) -> BitMbidBcnq5EnR {
        BitMbidBcnq5EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When enable, auto MBSSID for virtual interface 6 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq6_en(&self) -> BitMbidBcnq6EnR {
        BitMbidBcnq6EnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When enable, auto MBSSID for virtual interface 7 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq7_en(&self) -> BitMbidBcnq7EnR {
        BitMbidBcnq7EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - when enable, if tsft_bit32_int is asserted, reset power_cs of noa_unit0"]
    #[inline(always)]
    pub fn bit_en_tsfbit32_rst_noa0(&self) -> BitEnTsfbit32RstNoa0R {
        BitEnTsfbit32RstNoa0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - The selector for the source of noa unit0. 0 for port 0, 1 for port 1,2 for port2"]
    #[inline(always)]
    pub fn bit_p2p_noa0_unit_sel(&self) -> BitP2pNoa0UnitSelR {
        BitP2pNoa0UnitSelR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa0_en(&self) -> BitP2pNoa0EnR {
        BitP2pNoa0EnR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa0_pwr_mgt_en(&self) -> BitP2pNoa0PwrMgtEnR {
        BitP2pNoa0PwrMgtEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
    #[inline(always)]
    pub fn bit_p2p_noa0_distx_en(&self) -> BitP2pNoa0DistxEnR {
        BitP2pNoa0DistxEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
    #[inline(always)]
    pub fn bit_p2p_noa0_allstasleep(&self) -> BitP2pNoa0AllstasleepR {
        BitP2pNoa0AllstasleepR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable VOQ RD responder function"]
    #[inline(always)]
    pub fn bit_voq_rd_resp_en(&mut self) -> BitVoqRdRespEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitVoqRdRespEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable VIQ RD responder function"]
    #[inline(always)]
    pub fn bit_viq_rd_resp_en(&mut self) -> BitViqRdRespEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitViqRdRespEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable BEQ RD responder function"]
    #[inline(always)]
    pub fn bit_beq_rd_resp_en(&mut self) -> BitBeqRdRespEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitBeqRdRespEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable BKQ RD responder function"]
    #[inline(always)]
    pub fn bit_bkq_rd_resp_en(&mut self) -> BitBkqRdRespEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitBkqRdRespEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable VOQ RD initiator function"]
    #[inline(always)]
    pub fn bit_voq_rd_init_en(&mut self) -> BitVoqRdInitEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitVoqRdInitEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable VIQ RD initiator function"]
    #[inline(always)]
    pub fn bit_viq_rd_init_en(&mut self) -> BitViqRdInitEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitViqRdInitEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable BEQ RD initiator function"]
    #[inline(always)]
    pub fn bit_beq_rd_init_en(&mut self) -> BitBeqRdInitEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitBeqRdInitEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable BKQ RD initiator function"]
    #[inline(always)]
    pub fn bit_bkq_rd_init_en(&mut self) -> BitBkqRdInitEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitBkqRdInitEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Disable STBC CF-END function"]
    #[inline(always)]
    pub fn bit_dis_stbc_cfe(&mut self) -> BitDisStbcCfeW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitDisStbcCfeW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable LSIG_TXOP CF-END function"]
    #[inline(always)]
    pub fn bit_dis_lsig_cfe(&mut self) -> BitDisLsigCfeW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitDisLsigCfeW::new(self, 9)
    }
    #[doc = "Bit 10 - Disable TXOP_CFE function"]
    #[inline(always)]
    pub fn bit_dis_txop_cfe(&mut self) -> BitDisTxopCfeW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitDisTxopCfeW::new(self, 10)
    }
    #[doc = "Bit 11 - 1: disable EDCCA count down, 0: enable EDCCA count down Whether AC backoff needs to reference PHY_EDCCA 0: does not reference PHY_EDCCA 1: references PHY_EDCCA (default, backoff stops when PHY_EDCCA=1)"]
    #[inline(always)]
    pub fn bit_edcca_msk_cntdown_en(
        &mut self,
    ) -> BitEdccaMskCntdownEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitEdccaMskCntdownEnW::new(self, 11)
    }
    #[doc = "Bit 12 - eneble force tx becon ignore EDCCA in disable contention area"]
    #[inline(always)]
    pub fn bit_en_bcnerr_inedcca(&mut self) -> BitEnBcnerrInedccaW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitEnBcnerrInedccaW::new(self, 12)
    }
    #[doc = "Bit 13 - eneble force tx becon ignore CCA in disable contention area"]
    #[inline(always)]
    pub fn bit_en_bcnerr_inccca(&mut self) -> BitEnBcnerrIncccaW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitEnBcnerrIncccaW::new(self, 13)
    }
    #[doc = "Bit 14 - 1: Tx Packet with ignoring Beacon Area time 0: Disable"]
    #[inline(always)]
    pub fn bit_dis_tx_over_bcnq(&mut self) -> BitDisTxOverBcnqW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitDisTxOverBcnqW::new(self, 14)
    }
    #[doc = "Bit 15 - not valid."]
    #[inline(always)]
    pub fn bit_en_clr_txreq_incca(&mut self) -> BitEnClrTxreqInccaW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitEnClrTxreqInccaW::new(self, 15)
    }
    #[doc = "Bit 16 - When enable, auto MBSSID for root interface is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq0_en(&mut self) -> BitMbidBcnq0EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitMbidBcnq0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - When enable, auto MBSSID for virtual interface 1 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq1_en(&mut self) -> BitMbidBcnq1EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitMbidBcnq1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - When enable, auto MBSSID for virtual interface 2 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq2_en(&mut self) -> BitMbidBcnq2EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitMbidBcnq2EnW::new(self, 18)
    }
    #[doc = "Bit 19 - When enable, auto MBSSID for virtual interface 3 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq3_en(&mut self) -> BitMbidBcnq3EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitMbidBcnq3EnW::new(self, 19)
    }
    #[doc = "Bit 20 - When enable, auto MBSSID for virtual interface 4 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq4_en(&mut self) -> BitMbidBcnq4EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitMbidBcnq4EnW::new(self, 20)
    }
    #[doc = "Bit 21 - When enable, auto MBSSID for virtual interface 5 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq5_en(&mut self) -> BitMbidBcnq5EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitMbidBcnq5EnW::new(self, 21)
    }
    #[doc = "Bit 22 - When enable, auto MBSSID for virtual interface 6 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq6_en(&mut self) -> BitMbidBcnq6EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitMbidBcnq6EnW::new(self, 22)
    }
    #[doc = "Bit 23 - When enable, auto MBSSID for virtual interface 7 is enable."]
    #[inline(always)]
    pub fn bit_mbid_bcnq7_en(&mut self) -> BitMbidBcnq7EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitMbidBcnq7EnW::new(self, 23)
    }
    #[doc = "Bit 24 - when enable, if tsft_bit32_int is asserted, reset power_cs of noa_unit0"]
    #[inline(always)]
    pub fn bit_en_tsfbit32_rst_noa0(
        &mut self,
    ) -> BitEnTsfbit32RstNoa0W<'_, RegNoa0MbssidRdCtrlSpec> {
        BitEnTsfbit32RstNoa0W::new(self, 24)
    }
    #[doc = "Bits 25:26 - The selector for the source of noa unit0. 0 for port 0, 1 for port 1,2 for port2"]
    #[inline(always)]
    pub fn bit_p2p_noa0_unit_sel(&mut self) -> BitP2pNoa0UnitSelW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitP2pNoa0UnitSelW::new(self, 25)
    }
    #[doc = "Bits 27:28 - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa0_en(&mut self) -> BitP2pNoa0EnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitP2pNoa0EnW::new(self, 27)
    }
    #[doc = "Bit 29 - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa0_pwr_mgt_en(&mut self) -> BitP2pNoa0PwrMgtEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitP2pNoa0PwrMgtEnW::new(self, 29)
    }
    #[doc = "Bit 30 - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
    #[inline(always)]
    pub fn bit_p2p_noa0_distx_en(&mut self) -> BitP2pNoa0DistxEnW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitP2pNoa0DistxEnW::new(self, 30)
    }
    #[doc = "Bit 31 - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
    #[inline(always)]
    pub fn bit_p2p_noa0_allstasleep(
        &mut self,
    ) -> BitP2pNoa0AllstasleepW<'_, RegNoa0MbssidRdCtrlSpec> {
        BitP2pNoa0AllstasleepW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa0_mbssid_rd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa0_mbssid_rd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoa0MbssidRdCtrlSpec;
impl crate::RegisterSpec for RegNoa0MbssidRdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa0_mbssid_rd_ctrl::R`](R) reader structure"]
impl crate::Readable for RegNoa0MbssidRdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa0_mbssid_rd_ctrl::W`](W) writer structure"]
impl crate::Writable for RegNoa0MbssidRdCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA0_MBSSID_RD_CTRL to value 0x01ff_880f"]
impl crate::Resettable for RegNoa0MbssidRdCtrlSpec {
    const RESET_VALUE: u32 = 0x01ff_880f;
}
