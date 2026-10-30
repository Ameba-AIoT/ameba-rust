#[doc = "Register `REG_TCR` reader"]
pub type R = crate::R<RegTcrSpec>;
#[doc = "Register `REG_TCR` writer"]
pub type W = crate::W<RegTcrSpec>;
#[doc = "Field `BIT_WMAC_TCR_DISGCLK` reader - when set to 1,disable MACTX clock gating control."]
pub type BitWmacTcrDisgclkR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_DISGCLK` writer - when set to 1,disable MACTX clock gating control."]
pub type BitWmacTcrDisgclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_PADSEL` reader - AMPDU Padding pattern selection 0: 32-bit Zero padding, 1: Zero length delimiter."]
pub type BitWmacTcrPadselR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_PADSEL` writer - AMPDU Padding pattern selection 0: 32-bit Zero padding, 1: Zero length delimiter."]
pub type BitWmacTcrPadselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_VHTSIGA1_TXPS` reader - This bit controls TXOP_PS_NOT_ALLOWED bit value of VHT-SIG-A1 field in TX VHT packets 1: TXOP_PS_NOT_ALLOWED is set to 1 0: TXOP_PS_NOT_ALLOWED is set to 0"]
pub type BitWmacTcrVhtsiga1TxpsR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_VHTSIGA1_TXPS` writer - This bit controls TXOP_PS_NOT_ALLOWED bit value of VHT-SIG-A1 field in TX VHT packets 1: TXOP_PS_NOT_ALLOWED is set to 1 0: TXOP_PS_NOT_ALLOWED is set to 0"]
pub type BitWmacTcrVhtsiga1TxpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_UPD_HGQMD` reader - when set to 1, enable HGQ set moredata bit in Frame control field"]
pub type BitWmacTcrUpdHgqmdR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_UPD_HGQMD` writer - when set to 1, enable HGQ set moredata bit in Frame control field"]
pub type BitWmacTcrUpdHgqmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_UPD_TIMIE` reader - when set to 1, enable hw to update beacon's time field"]
pub type BitWmacTcrUpdTimieR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_UPD_TIMIE` writer - when set to 1, enable hw to update beacon's time field"]
pub type BitWmacTcrUpdTimieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_PWRMGT_VAL` reader - MAC will overwrite power bit of frame control according to PWR_ST.(only for data frame)"]
pub type BitWmacTcrPwrmgtValR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_PWRMGT_VAL` writer - MAC will overwrite power bit of frame control according to PWR_ST.(only for data frame)"]
pub type BitWmacTcrPwrmgtValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_PWRMGT_CTL` reader - If this bit is set to 1, MAC will overwrite power bit of frame control according to PWR_ST. (only for data frame)"]
pub type BitWmacTcrPwrmgtCtlR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_PWRMGT_CTL` writer - If this bit is set to 1, MAC will overwrite power bit of frame control according to PWR_ST. (only for data frame)"]
pub type BitWmacTcrPwrmgtCtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_CRC` reader - Append 32-bit Cyclic Redundancy Check. This bit indicates the existence of a CRC32 appended to a packet. 1: No CRC32 appended 0: A CRC32 is appended"]
pub type BitWmacTcrCrcR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_CRC` writer - Append 32-bit Cyclic Redundancy Check. This bit indicates the existence of a CRC32 appended to a packet. 1: No CRC32 appended 0: A CRC32 is appended"]
pub type BitWmacTcrCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_CFENDFOR` reader - CF-End Frame Format.The Address 2 field of CF-End is defined to be the address of the STA contained in the AP, but in 802.11n a STA can also send CF-End.The Address 2 field is not specified clearly in this case. This register chooses between MAC ID of the STA or BSSID of the network. 0: BSSID 1: MAC ID"]
pub type BitWmacCfendforR = crate::BitReader;
#[doc = "Field `BIT_WMAC_CFENDFOR` writer - CF-End Frame Format.The Address 2 field of CF-End is defined to be the address of the STA contained in the AP, but in 802.11n a STA can also send CF-End.The Address 2 field is not specified clearly in this case. This register chooses between MAC ID of the STA or BSSID of the network. 0: BSSID 1: MAC ID"]
pub type BitWmacCfendforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_ICV` reader - Integrity Check Value. This bit indicates the existence of an ICV appended to an encipherment packet. 1: No ICV appended 0: ICV appended"]
pub type BitWmacTcrIcvR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_ICV` writer - Integrity Check Value. This bit indicates the existence of an ICV appended to an encipherment packet. 1: No ICV appended 0: ICV appended"]
pub type BitWmacTcrIcvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_TXSK_PERPKT` reader - Search key for each mpdu"]
pub type BitWmacTcrTxskPerpktR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_TXSK_PERPKT` writer - Search key for each mpdu"]
pub type BitWmacTcrTxskPerpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_ERRSTEN_0` reader - Phytxend_ps comes but mactx still active, reset mactx state machine"]
pub type BitWmacTcrErrsten0R = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_ERRSTEN_0` writer - Phytxend_ps comes but mactx still active, reset mactx state machine"]
pub type BitWmacTcrErrsten0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_ERRSTEN_1` reader - If txd fifo underflow, reset mactx state machine"]
pub type BitWmacTcrErrsten1R = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_ERRSTEN_1` writer - If txd fifo underflow, reset mactx state machine"]
pub type BitWmacTcrErrsten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_ERRSTEN_2` reader - If txd fifo underflow when txtype is cmpba, reset mactx state machine"]
pub type BitWmacTcrErrsten2R = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_ERRSTEN_2` writer - If txd fifo underflow when txtype is cmpba, reset mactx state machine"]
pub type BitWmacTcrErrsten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_ERRSTEN_3` reader - Use phytxend_ps to reset mactx state machine"]
pub type BitWmacTcrErrsten3R = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_ERRSTEN_3` writer - Use phytxend_ps to reset mactx state machine"]
pub type BitWmacTcrErrsten3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_MSK_SIGBCRC` reader - when set to 1, mask sigb crc in service field for vht packet"]
pub type BitWmacMskSigbcrcR = crate::BitReader;
#[doc = "Field `BIT_WMAC_MSK_SIGBCRC` writer - when set to 1, mask sigb crc in service field for vht packet"]
pub type BitWmacMskSigbcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_DIS_A2B0` reader - when set to 1, not check bit 0 of rx a2"]
pub type BitWmacDisA2b0R = crate::BitReader;
#[doc = "Field `BIT_WMAC_DIS_A2B0` writer - when set to 1, not check bit 0 of rx a2"]
pub type BitWmacDisA2b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_DIS_SigTa` reader - when set to 1, disable signaling TA information for response packet"]
pub type BitWmacDisSigTaR = crate::BitReader;
#[doc = "Field `BIT_WMAC_DIS_SigTa` writer - when set to 1, disable signaling TA information for response packet"]
pub type BitWmacDisSigTaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCR_EN_20MST` reader - when set to 1, enable ndpa signaling TA when BW =20MHz"]
pub type BitWmacTcrEn20mstR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCR_EN_20MST` writer - when set to 1, enable ndpa signaling TA when BW =20MHz"]
pub type BitWmacTcrEn20mstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FETCH_MPDU_AFTER_WSECRDY` reader - If set to 1,mactx_mpdu_gen will fetch mpdu after wsec is ready when sectype is not tkip/wep40/wep104"]
pub type BitFetchMpduAfterWsecrdyR = crate::BitReader;
#[doc = "Field `BIT_FETCH_MPDU_AFTER_WSECRDY` writer - If set to 1,mactx_mpdu_gen will fetch mpdu after wsec is ready when sectype is not tkip/wep40/wep104"]
pub type BitFetchMpduAfterWsecrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_UNDERFLOWEN_CMPLEN_SEL` reader - compare length selector for enable condition of underflow, consider plcp header when set 1"]
pub type BitUnderflowenCmplenSelR = crate::BitReader;
#[doc = "Field `BIT_UNDERFLOWEN_CMPLEN_SEL` writer - compare length selector for enable condition of underflow, consider plcp header when set 1"]
pub type BitUnderflowenCmplenSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_EN_SCRAM_INC` reader - Increase 1 in service field scrambler seed per tx end"]
pub type BitWmacEnScramIncR = crate::BitReader;
#[doc = "Field `BIT_WMAC_EN_SCRAM_INC` writer - Increase 1 in service field scrambler seed per tx end"]
pub type BitWmacEnScramIncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_SMOOTH_VAL` reader - smooth value of plcp header HTSIG2"]
pub type BitWmacSmoothValR = crate::BitReader;
#[doc = "Field `BIT_WMAC_SMOOTH_VAL` writer - smooth value of plcp header HTSIG2"]
pub type BitWmacSmoothValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_TCRPWRMGT_HWCTL` reader - If this bit is set to 1, MAC will overwrite power bit of frame control according to PWR_ST. (only for RTS/CTS/ACK/CFEND/BA/BAR frames)"]
pub type BitWmacTcrpwrmgtHwctlR = crate::BitReader;
#[doc = "Field `BIT_WMAC_TCRPWRMGT_HWCTL` writer - If this bit is set to 1, MAC will overwrite power bit of frame control according to PWR_ST. (only for RTS/CTS/ACK/CFEND/BA/BAR frames)"]
pub type BitWmacTcrpwrmgtHwctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_ANTMODE_SEL` reader - 0:4x1 mode, 1:8x1 mode"]
pub type BitWmacAntmodeSelR = crate::BitReader;
#[doc = "Field `BIT_WMAC_ANTMODE_SEL` writer - 0:4x1 mode, 1:8x1 mode"]
pub type BitWmacAntmodeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_BF_SEL` reader - 1: set ndp_rx_standby when rx ppdu of ndpa end; 0: set ndp_rx_standby when rx ndpa packet fcs end;"]
pub type BitWmacBfSelR = crate::BitReader;
#[doc = "Field `BIT_WMAC_BF_SEL` writer - 1: set ndp_rx_standby when rx ppdu of ndpa end; 0: set ndp_rx_standby when rx ndpa packet fcs end;"]
pub type BitWmacBfSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_EN_EOF` reader - when set to 1, enable single ampdu for qos packet. In early mode,not set to 1."]
pub type BitWmacEnEofR = crate::BitReader;
#[doc = "Field `BIT_WMAC_EN_EOF` writer - when set to 1, enable single ampdu for qos packet. In early mode,not set to 1."]
pub type BitWmacEnEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_NOTX_IN_RXNDP` reader - 1: block original tx when receiving NDP packet"]
pub type BitWmacNotxInRxndpR = crate::BitReader;
#[doc = "Field `BIT_WMAC_NOTX_IN_RXNDP` writer - 1: block original tx when receiving NDP packet"]
pub type BitWmacNotxInRxndpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_RAW_LEN` reader - 1: use raw ppdu_len 0: use 4byte alignment ppdu_len"]
pub type BitWmacRawLenR = crate::BitReader;
#[doc = "Field `BIT_WMAC_RAW_LEN` writer - 1: use raw ppdu_len 0: use 4byte alignment ppdu_len"]
pub type BitWmacRawLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_DISABLE_CCK` reader - when set to 1, disable cck rate packet"]
pub type BitWmacDisableCckR = crate::BitReader;
#[doc = "Field `BIT_WMAC_DISABLE_CCK` writer - when set to 1, disable cck rate packet"]
pub type BitWmacDisableCckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_EN_RTS_ADDR` reader - when set to 1, enable change RTS multicast address"]
pub type BitWmacEnRtsAddrR = crate::BitReader;
#[doc = "Field `BIT_WMAC_EN_RTS_ADDR` writer - when set to 1, enable change RTS multicast address"]
pub type BitWmacEnRtsAddrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - when set to 1,disable MACTX clock gating control."]
    #[inline(always)]
    pub fn bit_wmac_tcr_disgclk(&self) -> BitWmacTcrDisgclkR {
        BitWmacTcrDisgclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AMPDU Padding pattern selection 0: 32-bit Zero padding, 1: Zero length delimiter."]
    #[inline(always)]
    pub fn bit_wmac_tcr_padsel(&self) -> BitWmacTcrPadselR {
        BitWmacTcrPadselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit controls TXOP_PS_NOT_ALLOWED bit value of VHT-SIG-A1 field in TX VHT packets 1: TXOP_PS_NOT_ALLOWED is set to 1 0: TXOP_PS_NOT_ALLOWED is set to 0"]
    #[inline(always)]
    pub fn bit_wmac_tcr_vhtsiga1_txps(&self) -> BitWmacTcrVhtsiga1TxpsR {
        BitWmacTcrVhtsiga1TxpsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - when set to 1, enable HGQ set moredata bit in Frame control field"]
    #[inline(always)]
    pub fn bit_wmac_tcr_upd_hgqmd(&self) -> BitWmacTcrUpdHgqmdR {
        BitWmacTcrUpdHgqmdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when set to 1, enable hw to update beacon's time field"]
    #[inline(always)]
    pub fn bit_wmac_tcr_upd_timie(&self) -> BitWmacTcrUpdTimieR {
        BitWmacTcrUpdTimieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MAC will overwrite power bit of frame control according to PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_wmac_tcr_pwrmgt_val(&self) -> BitWmacTcrPwrmgtValR {
        BitWmacTcrPwrmgtValR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If this bit is set to 1, MAC will overwrite power bit of frame control according to PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_wmac_tcr_pwrmgt_ctl(&self) -> BitWmacTcrPwrmgtCtlR {
        BitWmacTcrPwrmgtCtlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Append 32-bit Cyclic Redundancy Check. This bit indicates the existence of a CRC32 appended to a packet. 1: No CRC32 appended 0: A CRC32 is appended"]
    #[inline(always)]
    pub fn bit_wmac_tcr_crc(&self) -> BitWmacTcrCrcR {
        BitWmacTcrCrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CF-End Frame Format.The Address 2 field of CF-End is defined to be the address of the STA contained in the AP, but in 802.11n a STA can also send CF-End.The Address 2 field is not specified clearly in this case. This register chooses between MAC ID of the STA or BSSID of the network. 0: BSSID 1: MAC ID"]
    #[inline(always)]
    pub fn bit_wmac_cfendfor(&self) -> BitWmacCfendforR {
        BitWmacCfendforR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Integrity Check Value. This bit indicates the existence of an ICV appended to an encipherment packet. 1: No ICV appended 0: ICV appended"]
    #[inline(always)]
    pub fn bit_wmac_tcr_icv(&self) -> BitWmacTcrIcvR {
        BitWmacTcrIcvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Search key for each mpdu"]
    #[inline(always)]
    pub fn bit_wmac_tcr_txsk_perpkt(&self) -> BitWmacTcrTxskPerpktR {
        BitWmacTcrTxskPerpktR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Phytxend_ps comes but mactx still active, reset mactx state machine"]
    #[inline(always)]
    pub fn bit_wmac_tcr_errsten_0(&self) -> BitWmacTcrErrsten0R {
        BitWmacTcrErrsten0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If txd fifo underflow, reset mactx state machine"]
    #[inline(always)]
    pub fn bit_wmac_tcr_errsten_1(&self) -> BitWmacTcrErrsten1R {
        BitWmacTcrErrsten1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If txd fifo underflow when txtype is cmpba, reset mactx state machine"]
    #[inline(always)]
    pub fn bit_wmac_tcr_errsten_2(&self) -> BitWmacTcrErrsten2R {
        BitWmacTcrErrsten2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Use phytxend_ps to reset mactx state machine"]
    #[inline(always)]
    pub fn bit_wmac_tcr_errsten_3(&self) -> BitWmacTcrErrsten3R {
        BitWmacTcrErrsten3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - when set to 1, mask sigb crc in service field for vht packet"]
    #[inline(always)]
    pub fn bit_wmac_msk_sigbcrc(&self) -> BitWmacMskSigbcrcR {
        BitWmacMskSigbcrcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - when set to 1, not check bit 0 of rx a2"]
    #[inline(always)]
    pub fn bit_wmac_dis_a2b0(&self) -> BitWmacDisA2b0R {
        BitWmacDisA2b0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - when set to 1, disable signaling TA information for response packet"]
    #[inline(always)]
    pub fn bit_wmac_dis_sig_ta(&self) -> BitWmacDisSigTaR {
        BitWmacDisSigTaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - when set to 1, enable ndpa signaling TA when BW =20MHz"]
    #[inline(always)]
    pub fn bit_wmac_tcr_en_20mst(&self) -> BitWmacTcrEn20mstR {
        BitWmacTcrEn20mstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If set to 1,mactx_mpdu_gen will fetch mpdu after wsec is ready when sectype is not tkip/wep40/wep104"]
    #[inline(always)]
    pub fn bit_fetch_mpdu_after_wsecrdy(&self) -> BitFetchMpduAfterWsecrdyR {
        BitFetchMpduAfterWsecrdyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - compare length selector for enable condition of underflow, consider plcp header when set 1"]
    #[inline(always)]
    pub fn bit_underflowen_cmplen_sel(&self) -> BitUnderflowenCmplenSelR {
        BitUnderflowenCmplenSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Increase 1 in service field scrambler seed per tx end"]
    #[inline(always)]
    pub fn bit_wmac_en_scram_inc(&self) -> BitWmacEnScramIncR {
        BitWmacEnScramIncR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - smooth value of plcp header HTSIG2"]
    #[inline(always)]
    pub fn bit_wmac_smooth_val(&self) -> BitWmacSmoothValR {
        BitWmacSmoothValR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If this bit is set to 1, MAC will overwrite power bit of frame control according to PWR_ST. (only for RTS/CTS/ACK/CFEND/BA/BAR frames)"]
    #[inline(always)]
    pub fn bit_wmac_tcrpwrmgt_hwctl(&self) -> BitWmacTcrpwrmgtHwctlR {
        BitWmacTcrpwrmgtHwctlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0:4x1 mode, 1:8x1 mode"]
    #[inline(always)]
    pub fn bit_wmac_antmode_sel(&self) -> BitWmacAntmodeSelR {
        BitWmacAntmodeSelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: set ndp_rx_standby when rx ppdu of ndpa end; 0: set ndp_rx_standby when rx ndpa packet fcs end;"]
    #[inline(always)]
    pub fn bit_wmac_bf_sel(&self) -> BitWmacBfSelR {
        BitWmacBfSelR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - when set to 1, enable single ampdu for qos packet. In early mode,not set to 1."]
    #[inline(always)]
    pub fn bit_wmac_en_eof(&self) -> BitWmacEnEofR {
        BitWmacEnEofR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: block original tx when receiving NDP packet"]
    #[inline(always)]
    pub fn bit_wmac_notx_in_rxndp(&self) -> BitWmacNotxInRxndpR {
        BitWmacNotxInRxndpR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: use raw ppdu_len 0: use 4byte alignment ppdu_len"]
    #[inline(always)]
    pub fn bit_wmac_raw_len(&self) -> BitWmacRawLenR {
        BitWmacRawLenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - when set to 1, disable cck rate packet"]
    #[inline(always)]
    pub fn bit_wmac_disable_cck(&self) -> BitWmacDisableCckR {
        BitWmacDisableCckR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - when set to 1, enable change RTS multicast address"]
    #[inline(always)]
    pub fn bit_wmac_en_rts_addr(&self) -> BitWmacEnRtsAddrR {
        BitWmacEnRtsAddrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - when set to 1,disable MACTX clock gating control."]
    #[inline(always)]
    pub fn bit_wmac_tcr_disgclk(&mut self) -> BitWmacTcrDisgclkW<'_, RegTcrSpec> {
        BitWmacTcrDisgclkW::new(self, 1)
    }
    #[doc = "Bit 2 - AMPDU Padding pattern selection 0: 32-bit Zero padding, 1: Zero length delimiter."]
    #[inline(always)]
    pub fn bit_wmac_tcr_padsel(&mut self) -> BitWmacTcrPadselW<'_, RegTcrSpec> {
        BitWmacTcrPadselW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit controls TXOP_PS_NOT_ALLOWED bit value of VHT-SIG-A1 field in TX VHT packets 1: TXOP_PS_NOT_ALLOWED is set to 1 0: TXOP_PS_NOT_ALLOWED is set to 0"]
    #[inline(always)]
    pub fn bit_wmac_tcr_vhtsiga1_txps(&mut self) -> BitWmacTcrVhtsiga1TxpsW<'_, RegTcrSpec> {
        BitWmacTcrVhtsiga1TxpsW::new(self, 3)
    }
    #[doc = "Bit 4 - when set to 1, enable HGQ set moredata bit in Frame control field"]
    #[inline(always)]
    pub fn bit_wmac_tcr_upd_hgqmd(&mut self) -> BitWmacTcrUpdHgqmdW<'_, RegTcrSpec> {
        BitWmacTcrUpdHgqmdW::new(self, 4)
    }
    #[doc = "Bit 5 - when set to 1, enable hw to update beacon's time field"]
    #[inline(always)]
    pub fn bit_wmac_tcr_upd_timie(&mut self) -> BitWmacTcrUpdTimieW<'_, RegTcrSpec> {
        BitWmacTcrUpdTimieW::new(self, 5)
    }
    #[doc = "Bit 6 - MAC will overwrite power bit of frame control according to PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_wmac_tcr_pwrmgt_val(&mut self) -> BitWmacTcrPwrmgtValW<'_, RegTcrSpec> {
        BitWmacTcrPwrmgtValW::new(self, 6)
    }
    #[doc = "Bit 7 - If this bit is set to 1, MAC will overwrite power bit of frame control according to PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_wmac_tcr_pwrmgt_ctl(&mut self) -> BitWmacTcrPwrmgtCtlW<'_, RegTcrSpec> {
        BitWmacTcrPwrmgtCtlW::new(self, 7)
    }
    #[doc = "Bit 8 - Append 32-bit Cyclic Redundancy Check. This bit indicates the existence of a CRC32 appended to a packet. 1: No CRC32 appended 0: A CRC32 is appended"]
    #[inline(always)]
    pub fn bit_wmac_tcr_crc(&mut self) -> BitWmacTcrCrcW<'_, RegTcrSpec> {
        BitWmacTcrCrcW::new(self, 8)
    }
    #[doc = "Bit 9 - CF-End Frame Format.The Address 2 field of CF-End is defined to be the address of the STA contained in the AP, but in 802.11n a STA can also send CF-End.The Address 2 field is not specified clearly in this case. This register chooses between MAC ID of the STA or BSSID of the network. 0: BSSID 1: MAC ID"]
    #[inline(always)]
    pub fn bit_wmac_cfendfor(&mut self) -> BitWmacCfendforW<'_, RegTcrSpec> {
        BitWmacCfendforW::new(self, 9)
    }
    #[doc = "Bit 10 - Integrity Check Value. This bit indicates the existence of an ICV appended to an encipherment packet. 1: No ICV appended 0: ICV appended"]
    #[inline(always)]
    pub fn bit_wmac_tcr_icv(&mut self) -> BitWmacTcrIcvW<'_, RegTcrSpec> {
        BitWmacTcrIcvW::new(self, 10)
    }
    #[doc = "Bit 11 - Search key for each mpdu"]
    #[inline(always)]
    pub fn bit_wmac_tcr_txsk_perpkt(&mut self) -> BitWmacTcrTxskPerpktW<'_, RegTcrSpec> {
        BitWmacTcrTxskPerpktW::new(self, 11)
    }
    #[doc = "Bit 12 - Phytxend_ps comes but mactx still active, reset mactx state machine"]
    #[inline(always)]
    pub fn bit_wmac_tcr_errsten_0(&mut self) -> BitWmacTcrErrsten0W<'_, RegTcrSpec> {
        BitWmacTcrErrsten0W::new(self, 12)
    }
    #[doc = "Bit 13 - If txd fifo underflow, reset mactx state machine"]
    #[inline(always)]
    pub fn bit_wmac_tcr_errsten_1(&mut self) -> BitWmacTcrErrsten1W<'_, RegTcrSpec> {
        BitWmacTcrErrsten1W::new(self, 13)
    }
    #[doc = "Bit 14 - If txd fifo underflow when txtype is cmpba, reset mactx state machine"]
    #[inline(always)]
    pub fn bit_wmac_tcr_errsten_2(&mut self) -> BitWmacTcrErrsten2W<'_, RegTcrSpec> {
        BitWmacTcrErrsten2W::new(self, 14)
    }
    #[doc = "Bit 15 - Use phytxend_ps to reset mactx state machine"]
    #[inline(always)]
    pub fn bit_wmac_tcr_errsten_3(&mut self) -> BitWmacTcrErrsten3W<'_, RegTcrSpec> {
        BitWmacTcrErrsten3W::new(self, 15)
    }
    #[doc = "Bit 16 - when set to 1, mask sigb crc in service field for vht packet"]
    #[inline(always)]
    pub fn bit_wmac_msk_sigbcrc(&mut self) -> BitWmacMskSigbcrcW<'_, RegTcrSpec> {
        BitWmacMskSigbcrcW::new(self, 16)
    }
    #[doc = "Bit 17 - when set to 1, not check bit 0 of rx a2"]
    #[inline(always)]
    pub fn bit_wmac_dis_a2b0(&mut self) -> BitWmacDisA2b0W<'_, RegTcrSpec> {
        BitWmacDisA2b0W::new(self, 17)
    }
    #[doc = "Bit 18 - when set to 1, disable signaling TA information for response packet"]
    #[inline(always)]
    pub fn bit_wmac_dis_sig_ta(&mut self) -> BitWmacDisSigTaW<'_, RegTcrSpec> {
        BitWmacDisSigTaW::new(self, 18)
    }
    #[doc = "Bit 19 - when set to 1, enable ndpa signaling TA when BW =20MHz"]
    #[inline(always)]
    pub fn bit_wmac_tcr_en_20mst(&mut self) -> BitWmacTcrEn20mstW<'_, RegTcrSpec> {
        BitWmacTcrEn20mstW::new(self, 19)
    }
    #[doc = "Bit 20 - If set to 1,mactx_mpdu_gen will fetch mpdu after wsec is ready when sectype is not tkip/wep40/wep104"]
    #[inline(always)]
    pub fn bit_fetch_mpdu_after_wsecrdy(&mut self) -> BitFetchMpduAfterWsecrdyW<'_, RegTcrSpec> {
        BitFetchMpduAfterWsecrdyW::new(self, 20)
    }
    #[doc = "Bit 21 - compare length selector for enable condition of underflow, consider plcp header when set 1"]
    #[inline(always)]
    pub fn bit_underflowen_cmplen_sel(&mut self) -> BitUnderflowenCmplenSelW<'_, RegTcrSpec> {
        BitUnderflowenCmplenSelW::new(self, 21)
    }
    #[doc = "Bit 22 - Increase 1 in service field scrambler seed per tx end"]
    #[inline(always)]
    pub fn bit_wmac_en_scram_inc(&mut self) -> BitWmacEnScramIncW<'_, RegTcrSpec> {
        BitWmacEnScramIncW::new(self, 22)
    }
    #[doc = "Bit 23 - smooth value of plcp header HTSIG2"]
    #[inline(always)]
    pub fn bit_wmac_smooth_val(&mut self) -> BitWmacSmoothValW<'_, RegTcrSpec> {
        BitWmacSmoothValW::new(self, 23)
    }
    #[doc = "Bit 24 - If this bit is set to 1, MAC will overwrite power bit of frame control according to PWR_ST. (only for RTS/CTS/ACK/CFEND/BA/BAR frames)"]
    #[inline(always)]
    pub fn bit_wmac_tcrpwrmgt_hwctl(&mut self) -> BitWmacTcrpwrmgtHwctlW<'_, RegTcrSpec> {
        BitWmacTcrpwrmgtHwctlW::new(self, 24)
    }
    #[doc = "Bit 25 - 0:4x1 mode, 1:8x1 mode"]
    #[inline(always)]
    pub fn bit_wmac_antmode_sel(&mut self) -> BitWmacAntmodeSelW<'_, RegTcrSpec> {
        BitWmacAntmodeSelW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: set ndp_rx_standby when rx ppdu of ndpa end; 0: set ndp_rx_standby when rx ndpa packet fcs end;"]
    #[inline(always)]
    pub fn bit_wmac_bf_sel(&mut self) -> BitWmacBfSelW<'_, RegTcrSpec> {
        BitWmacBfSelW::new(self, 26)
    }
    #[doc = "Bit 27 - when set to 1, enable single ampdu for qos packet. In early mode,not set to 1."]
    #[inline(always)]
    pub fn bit_wmac_en_eof(&mut self) -> BitWmacEnEofW<'_, RegTcrSpec> {
        BitWmacEnEofW::new(self, 27)
    }
    #[doc = "Bit 28 - 1: block original tx when receiving NDP packet"]
    #[inline(always)]
    pub fn bit_wmac_notx_in_rxndp(&mut self) -> BitWmacNotxInRxndpW<'_, RegTcrSpec> {
        BitWmacNotxInRxndpW::new(self, 28)
    }
    #[doc = "Bit 29 - 1: use raw ppdu_len 0: use 4byte alignment ppdu_len"]
    #[inline(always)]
    pub fn bit_wmac_raw_len(&mut self) -> BitWmacRawLenW<'_, RegTcrSpec> {
        BitWmacRawLenW::new(self, 29)
    }
    #[doc = "Bit 30 - when set to 1, disable cck rate packet"]
    #[inline(always)]
    pub fn bit_wmac_disable_cck(&mut self) -> BitWmacDisableCckW<'_, RegTcrSpec> {
        BitWmacDisableCckW::new(self, 30)
    }
    #[doc = "Bit 31 - when set to 1, enable change RTS multicast address"]
    #[inline(always)]
    pub fn bit_wmac_en_rts_addr(&mut self) -> BitWmacEnRtsAddrW<'_, RegTcrSpec> {
        BitWmacEnRtsAddrW::new(self, 31)
    }
}
#[doc = "Transmission Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTcrSpec;
impl crate::RegisterSpec for RegTcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tcr::R`](R) reader structure"]
impl crate::Readable for RegTcrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tcr::W`](W) writer structure"]
impl crate::Writable for RegTcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TCR to value 0x2830_0000"]
impl crate::Resettable for RegTcrSpec {
    const RESET_VALUE: u32 = 0x2830_0000;
}
