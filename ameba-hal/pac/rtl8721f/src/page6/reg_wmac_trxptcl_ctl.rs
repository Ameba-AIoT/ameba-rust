#[doc = "Register `REG_WMAC_TRXPTCL_CTL` reader"]
pub type R = crate::R<RegWmacTrxptclCtlSpec>;
#[doc = "Register `REG_WMAC_TRXPTCL_CTL` writer"]
pub type W = crate::W<RegWmacTrxptclCtlSpec>;
#[doc = "Field `BIT_ORIG_DCTS_CHK` reader - Txok condition for dual CTS frame exchange: 00: Either one Rx ok 01: 1st one Rx ok 10: 2nd one Rx ok 11: both Rx ok"]
pub type BitOrigDctsChkR = crate::FieldReader;
#[doc = "Field `BIT_ORIG_DCTS_CHK` writer - Txok condition for dual CTS frame exchange: 00: Either one Rx ok 01: 1st one Rx ok 10: 2nd one Rx ok 11: both Rx ok"]
pub type BitOrigDctsChkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TXACK_WITH_A1MATCH` reader - SET TO 1 :check every bit of a1 of rxpkt when need to response"]
pub type BitTxackWithA1matchR = crate::BitReader;
#[doc = "Field `BIT_TXACK_WITH_A1MATCH` writer - SET TO 1 :check every bit of a1 of rxpkt when need to response"]
pub type BitTxackWithA1matchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXRESP_BY_RXANTSEL` reader - 8188E/8812 Enable responder send the response frame with the antenna selection information which from the phy status of the previous rx frame."]
pub type BitTxrespByRxantselR = crate::BitReader;
#[doc = "Field `BIT_TXRESP_BY_RXANTSEL` writer - 8188E/8812 Enable responder send the response frame with the antenna selection information which from the phy status of the previous rx frame."]
pub type BitTxrespByRxantselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DLY_TX_WAIT_RXANTSEL` reader - 8188E/8812 In some Cases, the phy status may be received too late to be used in the response frame. Enable this bit to postpone the tx of response frame until the phy status is obtained. Otherwise, If the phy status unavailable for its delay, The IOREG value of 0x06D8\\[6\\] will be used as before."]
pub type BitDlyTxWaitRxantselR = crate::BitReader;
#[doc = "Field `BIT_DLY_TX_WAIT_RXANTSEL` writer - 8188E/8812 In some Cases, the phy status may be received too late to be used in the response frame. Enable this bit to postpone the tx of response frame until the phy status is obtained. Otherwise, If the phy status unavailable for its delay, The IOREG value of 0x06D8\\[6\\] will be used as before."]
pub type BitDlyTxWaitRxantselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CTS_DYNBW_SEL` reader - 8812 Response cts bw selection when dynamic rts frame is received 00: current maximum bw indicated by secondary cca 01: bw fall back by one level based on bw in 2\"b00 if not reach 20M 10: bw fall back by one level based on bw in 2\"b01 if not reach 20M 11: bw fall back by one level based on bw in 2\"b10 if not reach 20M e.g, current maximum bw indicated by secondary_cca is 80M, when set to 2\"b00, the response cts bw is 80M, when set to 2\"b01, cts bw is 40M, when set to 2\"b10 or 2\"b11 , cts bw is 20M"]
pub type BitRespCtsDynbwSelR = crate::FieldReader;
#[doc = "Field `BIT_RESP_CTS_DYNBW_SEL` writer - 8812 Response cts bw selection when dynamic rts frame is received 00: current maximum bw indicated by secondary cca 01: bw fall back by one level based on bw in 2\"b00 if not reach 20M 10: bw fall back by one level based on bw in 2\"b01 if not reach 20M 11: bw fall back by one level based on bw in 2\"b10 if not reach 20M e.g, current maximum bw indicated by secondary_cca is 80M, when set to 2\"b00, the response cts bw is 80M, when set to 2\"b01, cts bw is 40M, when set to 2\"b10 or 2\"b11 , cts bw is 20M"]
pub type BitRespCtsDynbwSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RFMOD` reader - 8812 RF_BW indication, Used to decide response duplicated cts txsc 2\"b00: 20M, 2\"b01: 40M, 2\"b10: 80M e.g: when RF_BW= 80M, duplicated cts BW= 40M, txsc = 9 or 10 when RF_BW = 40M, duplicated cts BW = 40M, txsc = 0"]
pub type BitRfmodR = crate::FieldReader;
#[doc = "Field `BIT_RFMOD` writer - 8812 RF_BW indication, Used to decide response duplicated cts txsc 2\"b00: 20M, 2\"b01: 40M, 2\"b10: 80M e.g: when RF_BW= 80M, duplicated cts BW= 40M, txsc = 9 or 10 when RF_BW = 40M, duplicated cts BW = 40M, txsc = 0"]
pub type BitRfmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RESP_CHK_A2_REFUGEE` reader - if port 1 isacting as AP,this bit should set 0,otherwise should set 1 if relay mode enable and port1 is not AP mode"]
pub type BitRespChkA2RefugeeR = crate::BitReader;
#[doc = "Field `BIT_RESP_CHK_A2_REFUGEE` writer - if port 1 isacting as AP,this bit should set 0,otherwise should set 1 if relay mode enable and port1 is not AP mode"]
pub type BitRespChkA2RefugeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SECOND_CCA_TIMER` reader - 8812 The maximum time interval in us that secondary cca is pull high before primary cca When receive duplicated rts. When station response with cts, if the time that secondary cca pull high before primary cca is larger than this threshold, then secondary cca is judged to be interference, otherwise it is judged to the received RTS frame. default: 3\"b0"]
pub type BitSecondCcaTimerR = crate::FieldReader;
#[doc = "Field `BIT_SECOND_CCA_TIMER` writer - 8812 The maximum time interval in us that secondary cca is pull high before primary cca When receive duplicated rts. When station response with cts, if the time that secondary cca pull high before primary cca is larger than this threshold, then secondary cca is judged to be interference, otherwise it is judged to the received RTS frame. default: 3\"b0"]
pub type BitSecondCcaTimerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_DIS_RESP_CTSINCCA` reader - 8812 Disable to response cts when cca goes high after SIFS"]
pub type BitDisRespCtsinccaR = crate::BitReader;
#[doc = "Field `BIT_DIS_RESP_CTSINCCA` writer - 8812 Disable to response cts when cca goes high after SIFS"]
pub type BitDisRespCtsinccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_RESP_ACKINCCA` reader - Disable to response ack when cca goes high after SIFS"]
pub type BitDisRespAckinccaR = crate::BitReader;
#[doc = "Field `BIT_DIS_RESP_ACKINCCA` writer - Disable to response ack when cca goes high after SIFS"]
pub type BitDisRespAckinccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_LDPC_EN` reader - if rxpkt with ldpc, response pkt with ldpc"]
pub type BitRespLdpcEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_LDPC_EN` writer - if rxpkt with ldpc, response pkt with ldpc"]
pub type BitRespLdpcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_SGIEN` reader - Enable responder send SGI HT-OFDM response for acknowledging Rx SGI HT-OFDM packet"]
pub type BitRespSgienR = crate::BitReader;
#[doc = "Field `BIT_RESP_SGIEN` writer - Enable responder send SGI HT-OFDM response for acknowledging Rx SGI HT-OFDM packet"]
pub type BitRespSgienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_SPLCPEN` reader - Enable responder send SPLCP CCK response for acknowledging Rx SPLCP CCK packet"]
pub type BitRespSplcpenR = crate::BitReader;
#[doc = "Field `BIT_RESP_SPLCPEN` writer - Enable responder send SPLCP CCK response for acknowledging Rx SPLCP CCK packet"]
pub type BitRespSplcpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_DCFE_EN` reader - Enable responder send dual CF-END for responding RX CF-END"]
pub type BitRespDcfeEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_DCFE_EN` writer - Enable responder send dual CF-END for responding RX CF-END"]
pub type BitRespDcfeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_DCTS_EN` reader - Enable responder send dual CTS for responding Rx RTS"]
pub type BitRespDctsEnR = crate::BitReader;
#[doc = "Field `BIT_RESP_DCTS_EN` writer - Enable responder send dual CTS for responding Rx RTS"]
pub type BitRespDctsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RESP_CHNBUSY` reader - Define channel busy condition in responder to determine RX busy 1: PHYRXON (OFDM_VBON/CCKCCA) 0: CCA (CCKCCA/OFDMCCA)"]
pub type BitRespChnbusyR = crate::FieldReader;
#[doc = "Field `BIT_RESP_CHNBUSY` writer - Define channel busy condition in responder to determine RX busy 1: PHYRXON (OFDM_VBON/CCKCCA) 0: CCA (CCKCCA/OFDMCCA)"]
pub type BitRespChnbusyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_EARLY_TXBA` reader - While phyrxon down, mac_rxactive on. Ignore mac rx busy, respond BA immediately."]
pub type BitEarlyTxbaR = crate::BitReader;
#[doc = "Field `BIT_EARLY_TXBA` writer - While phyrxon down, mac_rxactive on. Ignore mac rx busy, respond BA immediately."]
pub type BitEarlyTxbaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_UPD_MYRXPKTNAV` reader - Do not update NAV, while receiving my packet."]
pub type BitDisUpdMyrxpktnavR = crate::BitReader;
#[doc = "Field `BIT_DIS_UPD_MYRXPKTNAV` writer - Do not update NAV, while receiving my packet."]
pub type BitDisUpdMyrxpktnavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CCA_RST_EIFS` reader - Reset EIFS counter while CCA on"]
pub type BitCcaRstEifsR = crate::BitReader;
#[doc = "Field `BIT_CCA_RST_EIFS` writer - Reset EIFS counter while CCA on"]
pub type BitCcaRstEifsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PLCPCHK_RST_EIFS` reader - Reset EIFS counter while receive correct PLCP."]
pub type BitPlcpchkRstEifsR = crate::BitReader;
#[doc = "Field `BIT_PLCPCHK_RST_EIFS` writer - Reset EIFS counter while receive correct PLCP."]
pub type BitPlcpchkRstEifsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RST_EDCA_EIFS` reader - While in EIFS duration, reset scheduler top to TX"]
pub type BitRstEdcaEifsR = crate::BitReader;
#[doc = "Field `BIT_RST_EDCA_EIFS` writer - While in EIFS duration, reset scheduler top to TX"]
pub type BitRstEdcaEifsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RST_EDCA_ACKTO` reader - While in ACKTO duration, reset scheduler top to TX"]
pub type BitRstEdcaAcktoR = crate::BitReader;
#[doc = "Field `BIT_RST_EDCA_ACKTO` writer - While in ACKTO duration, reset scheduler top to TX"]
pub type BitRstEdcaAcktoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_STOP_UPDNAV` reader - When this bit set 1. If RX packet with EDCA_REF is low, not update NAV."]
pub type BitEnStopUpdnavR = crate::BitReader;
#[doc = "Field `BIT_EN_STOP_UPDNAV` writer - When this bit set 1. If RX packet with EDCA_REF is low, not update NAV."]
pub type BitEnStopUpdnavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BLK_EDCA_BBSBY` reader - Block EDCA Tx while BB in standby state"]
pub type BitBlkEdcaBbsbyR = crate::BitReader;
#[doc = "Field `BIT_BLK_EDCA_BBSBY` writer - Block EDCA Tx while BB in standby state"]
pub type BitBlkEdcaBbsbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BLK_EDCA_BBSLP` reader - Block EDCA Tx while BB in sleep state"]
pub type BitBlkEdcaBbslpR = crate::BitReader;
#[doc = "Field `BIT_BLK_EDCA_BBSLP` writer - Block EDCA Tx while BB in sleep state"]
pub type BitBlkEdcaBbslpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Txok condition for dual CTS frame exchange: 00: Either one Rx ok 01: 1st one Rx ok 10: 2nd one Rx ok 11: both Rx ok"]
    #[inline(always)]
    pub fn bit_orig_dcts_chk(&self) -> BitOrigDctsChkR {
        BitOrigDctsChkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - SET TO 1 :check every bit of a1 of rxpkt when need to response"]
    #[inline(always)]
    pub fn bit_txack_with_a1match(&self) -> BitTxackWithA1matchR {
        BitTxackWithA1matchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8188E/8812 Enable responder send the response frame with the antenna selection information which from the phy status of the previous rx frame."]
    #[inline(always)]
    pub fn bit_txresp_by_rxantsel(&self) -> BitTxrespByRxantselR {
        BitTxrespByRxantselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8188E/8812 In some Cases, the phy status may be received too late to be used in the response frame. Enable this bit to postpone the tx of response frame until the phy status is obtained. Otherwise, If the phy status unavailable for its delay, The IOREG value of 0x06D8\\[6\\] will be used as before."]
    #[inline(always)]
    pub fn bit_dly_tx_wait_rxantsel(&self) -> BitDlyTxWaitRxantselR {
        BitDlyTxWaitRxantselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 8812 Response cts bw selection when dynamic rts frame is received 00: current maximum bw indicated by secondary cca 01: bw fall back by one level based on bw in 2\"b00 if not reach 20M 10: bw fall back by one level based on bw in 2\"b01 if not reach 20M 11: bw fall back by one level based on bw in 2\"b10 if not reach 20M e.g, current maximum bw indicated by secondary_cca is 80M, when set to 2\"b00, the response cts bw is 80M, when set to 2\"b01, cts bw is 40M, when set to 2\"b10 or 2\"b11 , cts bw is 20M"]
    #[inline(always)]
    pub fn bit_resp_cts_dynbw_sel(&self) -> BitRespCtsDynbwSelR {
        BitRespCtsDynbwSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - 8812 RF_BW indication, Used to decide response duplicated cts txsc 2\"b00: 20M, 2\"b01: 40M, 2\"b10: 80M e.g: when RF_BW= 80M, duplicated cts BW= 40M, txsc = 9 or 10 when RF_BW = 40M, duplicated cts BW = 40M, txsc = 0"]
    #[inline(always)]
    pub fn bit_rfmod(&self) -> BitRfmodR {
        BitRfmodR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - if port 1 isacting as AP,this bit should set 0,otherwise should set 1 if relay mode enable and port1 is not AP mode"]
    #[inline(always)]
    pub fn bit_resp_chk_a2_refugee(&self) -> BitRespChkA2RefugeeR {
        BitRespChkA2RefugeeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - 8812 The maximum time interval in us that secondary cca is pull high before primary cca When receive duplicated rts. When station response with cts, if the time that secondary cca pull high before primary cca is larger than this threshold, then secondary cca is judged to be interference, otherwise it is judged to the received RTS frame. default: 3\"b0"]
    #[inline(always)]
    pub fn bit_second_cca_timer(&self) -> BitSecondCcaTimerR {
        BitSecondCcaTimerR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 8812 Disable to response cts when cca goes high after SIFS"]
    #[inline(always)]
    pub fn bit_dis_resp_ctsincca(&self) -> BitDisRespCtsinccaR {
        BitDisRespCtsinccaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable to response ack when cca goes high after SIFS"]
    #[inline(always)]
    pub fn bit_dis_resp_ackincca(&self) -> BitDisRespAckinccaR {
        BitDisRespAckinccaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - if rxpkt with ldpc, response pkt with ldpc"]
    #[inline(always)]
    pub fn bit_resp_ldpc_en(&self) -> BitRespLdpcEnR {
        BitRespLdpcEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable responder send SGI HT-OFDM response for acknowledging Rx SGI HT-OFDM packet"]
    #[inline(always)]
    pub fn bit_resp_sgien(&self) -> BitRespSgienR {
        BitRespSgienR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable responder send SPLCP CCK response for acknowledging Rx SPLCP CCK packet"]
    #[inline(always)]
    pub fn bit_resp_splcpen(&self) -> BitRespSplcpenR {
        BitRespSplcpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable responder send dual CF-END for responding RX CF-END"]
    #[inline(always)]
    pub fn bit_resp_dcfe_en(&self) -> BitRespDcfeEnR {
        BitRespDcfeEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable responder send dual CTS for responding Rx RTS"]
    #[inline(always)]
    pub fn bit_resp_dcts_en(&self) -> BitRespDctsEnR {
        BitRespDctsEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Define channel busy condition in responder to determine RX busy 1: PHYRXON (OFDM_VBON/CCKCCA) 0: CCA (CCKCCA/OFDMCCA)"]
    #[inline(always)]
    pub fn bit_resp_chnbusy(&self) -> BitRespChnbusyR {
        BitRespChnbusyR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - While phyrxon down, mac_rxactive on. Ignore mac rx busy, respond BA immediately."]
    #[inline(always)]
    pub fn bit_early_txba(&self) -> BitEarlyTxbaR {
        BitEarlyTxbaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Do not update NAV, while receiving my packet."]
    #[inline(always)]
    pub fn bit_dis_upd_myrxpktnav(&self) -> BitDisUpdMyrxpktnavR {
        BitDisUpdMyrxpktnavR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset EIFS counter while CCA on"]
    #[inline(always)]
    pub fn bit_cca_rst_eifs(&self) -> BitCcaRstEifsR {
        BitCcaRstEifsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset EIFS counter while receive correct PLCP."]
    #[inline(always)]
    pub fn bit_plcpchk_rst_eifs(&self) -> BitPlcpchkRstEifsR {
        BitPlcpchkRstEifsR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - While in EIFS duration, reset scheduler top to TX"]
    #[inline(always)]
    pub fn bit_rst_edca_eifs(&self) -> BitRstEdcaEifsR {
        BitRstEdcaEifsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - While in ACKTO duration, reset scheduler top to TX"]
    #[inline(always)]
    pub fn bit_rst_edca_ackto(&self) -> BitRstEdcaAcktoR {
        BitRstEdcaAcktoR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When this bit set 1. If RX packet with EDCA_REF is low, not update NAV."]
    #[inline(always)]
    pub fn bit_en_stop_updnav(&self) -> BitEnStopUpdnavR {
        BitEnStopUpdnavR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Block EDCA Tx while BB in standby state"]
    #[inline(always)]
    pub fn bit_blk_edca_bbsby(&self) -> BitBlkEdcaBbsbyR {
        BitBlkEdcaBbsbyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Block EDCA Tx while BB in sleep state"]
    #[inline(always)]
    pub fn bit_blk_edca_bbslp(&self) -> BitBlkEdcaBbslpR {
        BitBlkEdcaBbslpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Txok condition for dual CTS frame exchange: 00: Either one Rx ok 01: 1st one Rx ok 10: 2nd one Rx ok 11: both Rx ok"]
    #[inline(always)]
    pub fn bit_orig_dcts_chk(&mut self) -> BitOrigDctsChkW<'_, RegWmacTrxptclCtlSpec> {
        BitOrigDctsChkW::new(self, 0)
    }
    #[doc = "Bit 2 - SET TO 1 :check every bit of a1 of rxpkt when need to response"]
    #[inline(always)]
    pub fn bit_txack_with_a1match(&mut self) -> BitTxackWithA1matchW<'_, RegWmacTrxptclCtlSpec> {
        BitTxackWithA1matchW::new(self, 2)
    }
    #[doc = "Bit 3 - 8188E/8812 Enable responder send the response frame with the antenna selection information which from the phy status of the previous rx frame."]
    #[inline(always)]
    pub fn bit_txresp_by_rxantsel(&mut self) -> BitTxrespByRxantselW<'_, RegWmacTrxptclCtlSpec> {
        BitTxrespByRxantselW::new(self, 3)
    }
    #[doc = "Bit 4 - 8188E/8812 In some Cases, the phy status may be received too late to be used in the response frame. Enable this bit to postpone the tx of response frame until the phy status is obtained. Otherwise, If the phy status unavailable for its delay, The IOREG value of 0x06D8\\[6\\] will be used as before."]
    #[inline(always)]
    pub fn bit_dly_tx_wait_rxantsel(&mut self) -> BitDlyTxWaitRxantselW<'_, RegWmacTrxptclCtlSpec> {
        BitDlyTxWaitRxantselW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 8812 Response cts bw selection when dynamic rts frame is received 00: current maximum bw indicated by secondary cca 01: bw fall back by one level based on bw in 2\"b00 if not reach 20M 10: bw fall back by one level based on bw in 2\"b01 if not reach 20M 11: bw fall back by one level based on bw in 2\"b10 if not reach 20M e.g, current maximum bw indicated by secondary_cca is 80M, when set to 2\"b00, the response cts bw is 80M, when set to 2\"b01, cts bw is 40M, when set to 2\"b10 or 2\"b11 , cts bw is 20M"]
    #[inline(always)]
    pub fn bit_resp_cts_dynbw_sel(&mut self) -> BitRespCtsDynbwSelW<'_, RegWmacTrxptclCtlSpec> {
        BitRespCtsDynbwSelW::new(self, 5)
    }
    #[doc = "Bits 7:8 - 8812 RF_BW indication, Used to decide response duplicated cts txsc 2\"b00: 20M, 2\"b01: 40M, 2\"b10: 80M e.g: when RF_BW= 80M, duplicated cts BW= 40M, txsc = 9 or 10 when RF_BW = 40M, duplicated cts BW = 40M, txsc = 0"]
    #[inline(always)]
    pub fn bit_rfmod(&mut self) -> BitRfmodW<'_, RegWmacTrxptclCtlSpec> {
        BitRfmodW::new(self, 7)
    }
    #[doc = "Bit 9 - if port 1 isacting as AP,this bit should set 0,otherwise should set 1 if relay mode enable and port1 is not AP mode"]
    #[inline(always)]
    pub fn bit_resp_chk_a2_refugee(&mut self) -> BitRespChkA2RefugeeW<'_, RegWmacTrxptclCtlSpec> {
        BitRespChkA2RefugeeW::new(self, 9)
    }
    #[doc = "Bits 10:12 - 8812 The maximum time interval in us that secondary cca is pull high before primary cca When receive duplicated rts. When station response with cts, if the time that secondary cca pull high before primary cca is larger than this threshold, then secondary cca is judged to be interference, otherwise it is judged to the received RTS frame. default: 3\"b0"]
    #[inline(always)]
    pub fn bit_second_cca_timer(&mut self) -> BitSecondCcaTimerW<'_, RegWmacTrxptclCtlSpec> {
        BitSecondCcaTimerW::new(self, 10)
    }
    #[doc = "Bit 13 - 8812 Disable to response cts when cca goes high after SIFS"]
    #[inline(always)]
    pub fn bit_dis_resp_ctsincca(&mut self) -> BitDisRespCtsinccaW<'_, RegWmacTrxptclCtlSpec> {
        BitDisRespCtsinccaW::new(self, 13)
    }
    #[doc = "Bit 14 - Disable to response ack when cca goes high after SIFS"]
    #[inline(always)]
    pub fn bit_dis_resp_ackincca(&mut self) -> BitDisRespAckinccaW<'_, RegWmacTrxptclCtlSpec> {
        BitDisRespAckinccaW::new(self, 14)
    }
    #[doc = "Bit 15 - if rxpkt with ldpc, response pkt with ldpc"]
    #[inline(always)]
    pub fn bit_resp_ldpc_en(&mut self) -> BitRespLdpcEnW<'_, RegWmacTrxptclCtlSpec> {
        BitRespLdpcEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable responder send SGI HT-OFDM response for acknowledging Rx SGI HT-OFDM packet"]
    #[inline(always)]
    pub fn bit_resp_sgien(&mut self) -> BitRespSgienW<'_, RegWmacTrxptclCtlSpec> {
        BitRespSgienW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable responder send SPLCP CCK response for acknowledging Rx SPLCP CCK packet"]
    #[inline(always)]
    pub fn bit_resp_splcpen(&mut self) -> BitRespSplcpenW<'_, RegWmacTrxptclCtlSpec> {
        BitRespSplcpenW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable responder send dual CF-END for responding RX CF-END"]
    #[inline(always)]
    pub fn bit_resp_dcfe_en(&mut self) -> BitRespDcfeEnW<'_, RegWmacTrxptclCtlSpec> {
        BitRespDcfeEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable responder send dual CTS for responding Rx RTS"]
    #[inline(always)]
    pub fn bit_resp_dcts_en(&mut self) -> BitRespDctsEnW<'_, RegWmacTrxptclCtlSpec> {
        BitRespDctsEnW::new(self, 19)
    }
    #[doc = "Bits 20:21 - Define channel busy condition in responder to determine RX busy 1: PHYRXON (OFDM_VBON/CCKCCA) 0: CCA (CCKCCA/OFDMCCA)"]
    #[inline(always)]
    pub fn bit_resp_chnbusy(&mut self) -> BitRespChnbusyW<'_, RegWmacTrxptclCtlSpec> {
        BitRespChnbusyW::new(self, 20)
    }
    #[doc = "Bit 22 - While phyrxon down, mac_rxactive on. Ignore mac rx busy, respond BA immediately."]
    #[inline(always)]
    pub fn bit_early_txba(&mut self) -> BitEarlyTxbaW<'_, RegWmacTrxptclCtlSpec> {
        BitEarlyTxbaW::new(self, 22)
    }
    #[doc = "Bit 23 - Do not update NAV, while receiving my packet."]
    #[inline(always)]
    pub fn bit_dis_upd_myrxpktnav(&mut self) -> BitDisUpdMyrxpktnavW<'_, RegWmacTrxptclCtlSpec> {
        BitDisUpdMyrxpktnavW::new(self, 23)
    }
    #[doc = "Bit 24 - Reset EIFS counter while CCA on"]
    #[inline(always)]
    pub fn bit_cca_rst_eifs(&mut self) -> BitCcaRstEifsW<'_, RegWmacTrxptclCtlSpec> {
        BitCcaRstEifsW::new(self, 24)
    }
    #[doc = "Bit 25 - Reset EIFS counter while receive correct PLCP."]
    #[inline(always)]
    pub fn bit_plcpchk_rst_eifs(&mut self) -> BitPlcpchkRstEifsW<'_, RegWmacTrxptclCtlSpec> {
        BitPlcpchkRstEifsW::new(self, 25)
    }
    #[doc = "Bit 26 - While in EIFS duration, reset scheduler top to TX"]
    #[inline(always)]
    pub fn bit_rst_edca_eifs(&mut self) -> BitRstEdcaEifsW<'_, RegWmacTrxptclCtlSpec> {
        BitRstEdcaEifsW::new(self, 26)
    }
    #[doc = "Bit 27 - While in ACKTO duration, reset scheduler top to TX"]
    #[inline(always)]
    pub fn bit_rst_edca_ackto(&mut self) -> BitRstEdcaAcktoW<'_, RegWmacTrxptclCtlSpec> {
        BitRstEdcaAcktoW::new(self, 27)
    }
    #[doc = "Bit 28 - When this bit set 1. If RX packet with EDCA_REF is low, not update NAV."]
    #[inline(always)]
    pub fn bit_en_stop_updnav(&mut self) -> BitEnStopUpdnavW<'_, RegWmacTrxptclCtlSpec> {
        BitEnStopUpdnavW::new(self, 28)
    }
    #[doc = "Bit 30 - Block EDCA Tx while BB in standby state"]
    #[inline(always)]
    pub fn bit_blk_edca_bbsby(&mut self) -> BitBlkEdcaBbsbyW<'_, RegWmacTrxptclCtlSpec> {
        BitBlkEdcaBbsbyW::new(self, 30)
    }
    #[doc = "Bit 31 - Block EDCA Tx while BB in sleep state"]
    #[inline(always)]
    pub fn bit_blk_edca_bbslp(&mut self) -> BitBlkEdcaBbslpW<'_, RegWmacTrxptclCtlSpec> {
        BitBlkEdcaBbslpW::new(self, 31)
    }
}
#[doc = "WMAC TX/RX Protocol Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_trxptcl_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_trxptcl_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacTrxptclCtlSpec;
impl crate::RegisterSpec for RegWmacTrxptclCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_trxptcl_ctl::R`](R) reader structure"]
impl crate::Readable for RegWmacTrxptclCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_trxptcl_ctl::W`](W) writer structure"]
impl crate::Writable for RegWmacTrxptclCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_TRXPTCL_CTL to value 0x0ea0_1000"]
impl crate::Resettable for RegWmacTrxptclCtlSpec {
    const RESET_VALUE: u32 = 0x0ea0_1000;
}
