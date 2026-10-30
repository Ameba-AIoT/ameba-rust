#[doc = "Register `REG_WMAC_DBG_FTM_CTL` reader"]
pub type R = crate::R<RegWmacDbgFtmCtlSpec>;
#[doc = "Register `REG_WMAC_DBG_FTM_CTL` writer"]
pub type W = crate::W<RegWmacDbgFtmCtlSpec>;
#[doc = "Field `BIT_FTM_EN` reader - Fine timing measurement function; 0: Disable 1:enable"]
pub type BitFtmEnR = crate::BitReader;
#[doc = "Field `BIT_FTM_EN` writer - Fine timing measurement function; 0: Disable 1:enable"]
pub type BitFtmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXFTMREQ_EN` reader - If BIT_FTM_TYPE = 1 && BIT_RXFTMREQ_EN=1, HW will parsing rx packet, to find FTM request frame."]
pub type BitRxftmreqEnR = crate::BitReader;
#[doc = "Field `BIT_RXFTMREQ_EN` writer - If BIT_FTM_TYPE = 1 && BIT_RXFTMREQ_EN=1, HW will parsing rx packet, to find FTM request frame."]
pub type BitRxftmreqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXFTMREQ_BYDRV` reader - If set this bit to 1,Rx FTMREQ/FTM can be receive into rxpktbuffer."]
pub type BitRxftmreqBydrvR = crate::BitReader;
#[doc = "Field `BIT_RXFTMREQ_BYDRV` writer - If set this bit to 1,Rx FTMREQ/FTM can be receive into rxpktbuffer."]
pub type BitRxftmreqBydrvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXFTM_EN` reader - If BIT_FTM_TYPE = 1 && BIT_RXFTM_EN=1, HW will parsing rx packet, to find FTM request frame."]
pub type BitRxftmEnR = crate::BitReader;
#[doc = "Field `BIT_RXFTM_EN` writer - If BIT_FTM_TYPE = 1 && BIT_RXFTM_EN=1, HW will parsing rx packet, to find FTM request frame."]
pub type BitRxftmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TDE_DEBUG` reader - LA MODE function. enable txdma debug mode"]
pub type BitEnTdeDebugR = crate::BitReader;
#[doc = "Field `BIT_EN_TDE_DEBUG` writer - LA MODE function. enable txdma debug mode"]
pub type BitEnTdeDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXFTM_TXACK_BW` reader - When HW receive FTM pkt, HW will use RX FTM SC as response ACK TXSC."]
pub type BitRxftmTxackBwR = crate::BitReader;
#[doc = "Field `BIT_RXFTM_TXACK_BW` writer - When HW receive FTM pkt, HW will use RX FTM SC as response ACK TXSC."]
pub type BitRxftmTxackBwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXFTM_TXACK_SC` reader - When HW receive FTM pkt, HW will use RX FTM BW as response ACK TXBW."]
pub type BitRxftmTxackScR = crate::BitReader;
#[doc = "Field `BIT_RXFTM_TXACK_SC` writer - When HW receive FTM pkt, HW will use RX FTM BW as response ACK TXBW."]
pub type BitRxftmTxackScW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TRX_TRIG_SEL_WMAC` reader - LA MODE function. *1: select trig_stop signals from mactx or macrx, in MACTX/MACRX mode"]
pub type BitTrxTrigSelWmacR = crate::BitReader;
#[doc = "Field `BIT_TRX_TRIG_SEL_WMAC` writer - LA MODE function. *1: select trig_stop signals from mactx or macrx, in MACTX/MACRX mode"]
pub type BitTrxTrigSelWmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_IQD_LENGTH_REF` reader - LA MODE function. Valid when BIT_EN_IQD_LEN_BREAK = 1 Used to finish IQDUMP, unit is (8KB + 1bit)"]
pub type BitIqdLengthRefR = crate::FieldReader;
#[doc = "Field `BIT_IQD_LENGTH_REF` writer - LA MODE function. Valid when BIT_EN_IQD_LEN_BREAK = 1 Used to finish IQDUMP, unit is (8KB + 1bit)"]
pub type BitIqdLengthRefW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_EN_IQD_LEN_BREAK` reader - LA MODE function. *1: after \"trig_stop\" signal asserted, if the iqdata's length over the BIT_IQD_LENGTH_REF, finish the iqdump process *0: only used TRIG_TIME finish the iqdump process"]
pub type BitEnIqdLenBreakR = crate::BitReader;
#[doc = "Field `BIT_EN_IQD_LEN_BREAK` writer - LA MODE function. *1: after \"trig_stop\" signal asserted, if the iqdata's length over the BIT_IQD_LENGTH_REF, finish the iqdump process *0: only used TRIG_TIME finish the iqdump process"]
pub type BitEnIqdLenBreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TRX_TRIG_SEL_PATT` reader - LA MODE function. *1: select trig_stop signals from pattern match (see 0x17F0~0x17FC), in MACTX/MACRX mode"]
pub type BitTrxTrigSelPattR = crate::BitReader;
#[doc = "Field `BIT_TRX_TRIG_SEL_PATT` writer - LA MODE function. *1: select trig_stop signals from pattern match (see 0x17F0~0x17FC), in MACTX/MACRX mode"]
pub type BitTrxTrigSelPattW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TRX_TRIG_SEL_DEBUG` reader - LA MODE function. *1: select trig_stop signals from debug port, in MACTX/MACRX mode"]
pub type BitTrxTrigSelDebugR = crate::BitReader;
#[doc = "Field `BIT_TRX_TRIG_SEL_DEBUG` writer - LA MODE function. *1: select trig_stop signals from debug port, in MACTX/MACRX mode"]
pub type BitTrxTrigSelDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_RXD_DEBUG` reader - LA MODE function. enable macrx debug mode"]
pub type BitEnRxdDebugR = crate::BitReader;
#[doc = "Field `BIT_EN_RXD_DEBUG` writer - LA MODE function. enable macrx debug mode"]
pub type BitEnRxdDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TXD_DEBUG` reader - LA MODE function. enable mactx debug mode"]
pub type BitEnTxdDebugR = crate::BitReader;
#[doc = "Field `BIT_EN_TXD_DEBUG` writer - LA MODE function. enable mactx debug mode"]
pub type BitEnTxdDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TRIG_TIME_UNIT_SEL` reader - Select trigger time unit of 0x7C0\\[14:8\\]. 3'b000 : 1us , 3'b001 : 2us , 3'b010 : 4us , 3'b011 : 8us , 3'b100 : 16us , 3'b101 : 32us , 3'b110 : 64us, 3'b111 : RSV"]
pub type BitTrigTimeUnitSelR = crate::FieldReader;
#[doc = "Field `BIT_TRIG_TIME_UNIT_SEL` writer - Select trigger time unit of 0x7C0\\[14:8\\]. 3'b000 : 1us , 3'b001 : 2us , 3'b010 : 4us , 3'b011 : 8us , 3'b100 : 16us , 3'b101 : 32us , 3'b110 : 64us, 3'b111 : RSV"]
pub type BitTrigTimeUnitSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_LEVEL_PULSE_SEL_EXT` reader - LA mode triggered by mac debug port timeout event"]
pub type BitLevelPulseSelExtR = crate::BitReader;
#[doc = "Field `BIT_LEVEL_PULSE_SEL_EXT` writer - LA mode triggered by mac debug port timeout event"]
pub type BitLevelPulseSelExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LA_MAC_TIMEOUT_VALUE` reader - LA mode triggered by mac debug port timeout value, available with non-zero value."]
pub type BitLaMacTimeoutValueR = crate::FieldReader;
#[doc = "Field `BIT_LA_MAC_TIMEOUT_VALUE` writer - LA mode triggered by mac debug port timeout value, available with non-zero value."]
pub type BitLaMacTimeoutValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_LA_MAC_TIMEOUT_UNIT_SEL` reader - LA mode triggered by mac debug port timeout value unit selection. 2'b00: 1us, 2'b01:32us, 2'b10: 1ms, 2'b11:RSVD"]
pub type BitLaMacTimeoutUnitSelR = crate::FieldReader;
#[doc = "Field `BIT_LA_MAC_TIMEOUT_UNIT_SEL` writer - LA mode triggered by mac debug port timeout value unit selection. 2'b00: 1us, 2'b01:32us, 2'b10: 1ms, 2'b11:RSVD"]
pub type BitLaMacTimeoutUnitSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_EN_RE_START` reader - Before setting 0x7C0\\[1:0\\] =2'b11, set this bit to 1 to enable MDPK access mode"]
pub type BitEnReStartR = crate::BitReader;
#[doc = "Field `BIT_EN_RE_START` writer - Before setting 0x7C0\\[1:0\\] =2'b11, set this bit to 1 to enable MDPK access mode"]
pub type BitEnReStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TIMEOUT` reader - Set this bit to 1 to reset mdpk read pointer"]
pub type BitEnTimeoutR = crate::BitReader;
#[doc = "Field `BIT_EN_TIMEOUT` writer - Set this bit to 1 to reset mdpk read pointer"]
pub type BitEnTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SEL_DEBUG_DATA` reader - 11: data write to sram 10: data after sync_fifo 01: input data (TXD/RXD/IQ_DMA_DATA from BB) 00: RSVD"]
pub type BitSelDebugDataR = crate::FieldReader;
#[doc = "Field `BIT_SEL_DEBUG_DATA` writer - 11: data write to sram 10: data after sync_fifo 01: input data (TXD/RXD/IQ_DMA_DATA from BB) 00: RSVD"]
pub type BitSelDebugDataW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Fine timing measurement function; 0: Disable 1:enable"]
    #[inline(always)]
    pub fn bit_ftm_en(&self) -> BitFtmEnR {
        BitFtmEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If BIT_FTM_TYPE = 1 && BIT_RXFTMREQ_EN=1, HW will parsing rx packet, to find FTM request frame."]
    #[inline(always)]
    pub fn bit_rxftmreq_en(&self) -> BitRxftmreqEnR {
        BitRxftmreqEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set this bit to 1,Rx FTMREQ/FTM can be receive into rxpktbuffer."]
    #[inline(always)]
    pub fn bit_rxftmreq_bydrv(&self) -> BitRxftmreqBydrvR {
        BitRxftmreqBydrvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If BIT_FTM_TYPE = 1 && BIT_RXFTM_EN=1, HW will parsing rx packet, to find FTM request frame."]
    #[inline(always)]
    pub fn bit_rxftm_en(&self) -> BitRxftmEnR {
        BitRxftmEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LA MODE function. enable txdma debug mode"]
    #[inline(always)]
    pub fn bit_en_tde_debug(&self) -> BitEnTdeDebugR {
        BitEnTdeDebugR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When HW receive FTM pkt, HW will use RX FTM SC as response ACK TXSC."]
    #[inline(always)]
    pub fn bit_rxftm_txack_bw(&self) -> BitRxftmTxackBwR {
        BitRxftmTxackBwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When HW receive FTM pkt, HW will use RX FTM BW as response ACK TXBW."]
    #[inline(always)]
    pub fn bit_rxftm_txack_sc(&self) -> BitRxftmTxackScR {
        BitRxftmTxackScR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LA MODE function. *1: select trig_stop signals from mactx or macrx, in MACTX/MACRX mode"]
    #[inline(always)]
    pub fn bit_trx_trig_sel_wmac(&self) -> BitTrxTrigSelWmacR {
        BitTrxTrigSelWmacR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - LA MODE function. Valid when BIT_EN_IQD_LEN_BREAK = 1 Used to finish IQDUMP, unit is (8KB + 1bit)"]
    #[inline(always)]
    pub fn bit_iqd_length_ref(&self) -> BitIqdLengthRefR {
        BitIqdLengthRefR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - LA MODE function. *1: after \"trig_stop\" signal asserted, if the iqdata's length over the BIT_IQD_LENGTH_REF, finish the iqdump process *0: only used TRIG_TIME finish the iqdump process"]
    #[inline(always)]
    pub fn bit_en_iqd_len_break(&self) -> BitEnIqdLenBreakR {
        BitEnIqdLenBreakR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LA MODE function. *1: select trig_stop signals from pattern match (see 0x17F0~0x17FC), in MACTX/MACRX mode"]
    #[inline(always)]
    pub fn bit_trx_trig_sel_patt(&self) -> BitTrxTrigSelPattR {
        BitTrxTrigSelPattR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LA MODE function. *1: select trig_stop signals from debug port, in MACTX/MACRX mode"]
    #[inline(always)]
    pub fn bit_trx_trig_sel_debug(&self) -> BitTrxTrigSelDebugR {
        BitTrxTrigSelDebugR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LA MODE function. enable macrx debug mode"]
    #[inline(always)]
    pub fn bit_en_rxd_debug(&self) -> BitEnRxdDebugR {
        BitEnRxdDebugR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LA MODE function. enable mactx debug mode"]
    #[inline(always)]
    pub fn bit_en_txd_debug(&self) -> BitEnTxdDebugR {
        BitEnTxdDebugR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select trigger time unit of 0x7C0\\[14:8\\]. 3'b000 : 1us , 3'b001 : 2us , 3'b010 : 4us , 3'b011 : 8us , 3'b100 : 16us , 3'b101 : 32us , 3'b110 : 64us, 3'b111 : RSV"]
    #[inline(always)]
    pub fn bit_trig_time_unit_sel(&self) -> BitTrigTimeUnitSelR {
        BitTrigTimeUnitSelR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - LA mode triggered by mac debug port timeout event"]
    #[inline(always)]
    pub fn bit_level_pulse_sel_ext(&self) -> BitLevelPulseSelExtR {
        BitLevelPulseSelExtR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - LA mode triggered by mac debug port timeout value, available with non-zero value."]
    #[inline(always)]
    pub fn bit_la_mac_timeout_value(&self) -> BitLaMacTimeoutValueR {
        BitLaMacTimeoutValueR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:27 - LA mode triggered by mac debug port timeout value unit selection. 2'b00: 1us, 2'b01:32us, 2'b10: 1ms, 2'b11:RSVD"]
    #[inline(always)]
    pub fn bit_la_mac_timeout_unit_sel(&self) -> BitLaMacTimeoutUnitSelR {
        BitLaMacTimeoutUnitSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Before setting 0x7C0\\[1:0\\] =2'b11, set this bit to 1 to enable MDPK access mode"]
    #[inline(always)]
    pub fn bit_en_re_start(&self) -> BitEnReStartR {
        BitEnReStartR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to 1 to reset mdpk read pointer"]
    #[inline(always)]
    pub fn bit_en_timeout(&self) -> BitEnTimeoutR {
        BitEnTimeoutR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 11: data write to sram 10: data after sync_fifo 01: input data (TXD/RXD/IQ_DMA_DATA from BB) 00: RSVD"]
    #[inline(always)]
    pub fn bit_sel_debug_data(&self) -> BitSelDebugDataR {
        BitSelDebugDataR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fine timing measurement function; 0: Disable 1:enable"]
    #[inline(always)]
    pub fn bit_ftm_en(&mut self) -> BitFtmEnW<'_, RegWmacDbgFtmCtlSpec> {
        BitFtmEnW::new(self, 0)
    }
    #[doc = "Bit 1 - If BIT_FTM_TYPE = 1 && BIT_RXFTMREQ_EN=1, HW will parsing rx packet, to find FTM request frame."]
    #[inline(always)]
    pub fn bit_rxftmreq_en(&mut self) -> BitRxftmreqEnW<'_, RegWmacDbgFtmCtlSpec> {
        BitRxftmreqEnW::new(self, 1)
    }
    #[doc = "Bit 2 - If set this bit to 1,Rx FTMREQ/FTM can be receive into rxpktbuffer."]
    #[inline(always)]
    pub fn bit_rxftmreq_bydrv(&mut self) -> BitRxftmreqBydrvW<'_, RegWmacDbgFtmCtlSpec> {
        BitRxftmreqBydrvW::new(self, 2)
    }
    #[doc = "Bit 3 - If BIT_FTM_TYPE = 1 && BIT_RXFTM_EN=1, HW will parsing rx packet, to find FTM request frame."]
    #[inline(always)]
    pub fn bit_rxftm_en(&mut self) -> BitRxftmEnW<'_, RegWmacDbgFtmCtlSpec> {
        BitRxftmEnW::new(self, 3)
    }
    #[doc = "Bit 4 - LA MODE function. enable txdma debug mode"]
    #[inline(always)]
    pub fn bit_en_tde_debug(&mut self) -> BitEnTdeDebugW<'_, RegWmacDbgFtmCtlSpec> {
        BitEnTdeDebugW::new(self, 4)
    }
    #[doc = "Bit 5 - When HW receive FTM pkt, HW will use RX FTM SC as response ACK TXSC."]
    #[inline(always)]
    pub fn bit_rxftm_txack_bw(&mut self) -> BitRxftmTxackBwW<'_, RegWmacDbgFtmCtlSpec> {
        BitRxftmTxackBwW::new(self, 5)
    }
    #[doc = "Bit 6 - When HW receive FTM pkt, HW will use RX FTM BW as response ACK TXBW."]
    #[inline(always)]
    pub fn bit_rxftm_txack_sc(&mut self) -> BitRxftmTxackScW<'_, RegWmacDbgFtmCtlSpec> {
        BitRxftmTxackScW::new(self, 6)
    }
    #[doc = "Bit 7 - LA MODE function. *1: select trig_stop signals from mactx or macrx, in MACTX/MACRX mode"]
    #[inline(always)]
    pub fn bit_trx_trig_sel_wmac(&mut self) -> BitTrxTrigSelWmacW<'_, RegWmacDbgFtmCtlSpec> {
        BitTrxTrigSelWmacW::new(self, 7)
    }
    #[doc = "Bits 8:10 - LA MODE function. Valid when BIT_EN_IQD_LEN_BREAK = 1 Used to finish IQDUMP, unit is (8KB + 1bit)"]
    #[inline(always)]
    pub fn bit_iqd_length_ref(&mut self) -> BitIqdLengthRefW<'_, RegWmacDbgFtmCtlSpec> {
        BitIqdLengthRefW::new(self, 8)
    }
    #[doc = "Bit 11 - LA MODE function. *1: after \"trig_stop\" signal asserted, if the iqdata's length over the BIT_IQD_LENGTH_REF, finish the iqdump process *0: only used TRIG_TIME finish the iqdump process"]
    #[inline(always)]
    pub fn bit_en_iqd_len_break(&mut self) -> BitEnIqdLenBreakW<'_, RegWmacDbgFtmCtlSpec> {
        BitEnIqdLenBreakW::new(self, 11)
    }
    #[doc = "Bit 12 - LA MODE function. *1: select trig_stop signals from pattern match (see 0x17F0~0x17FC), in MACTX/MACRX mode"]
    #[inline(always)]
    pub fn bit_trx_trig_sel_patt(&mut self) -> BitTrxTrigSelPattW<'_, RegWmacDbgFtmCtlSpec> {
        BitTrxTrigSelPattW::new(self, 12)
    }
    #[doc = "Bit 13 - LA MODE function. *1: select trig_stop signals from debug port, in MACTX/MACRX mode"]
    #[inline(always)]
    pub fn bit_trx_trig_sel_debug(&mut self) -> BitTrxTrigSelDebugW<'_, RegWmacDbgFtmCtlSpec> {
        BitTrxTrigSelDebugW::new(self, 13)
    }
    #[doc = "Bit 14 - LA MODE function. enable macrx debug mode"]
    #[inline(always)]
    pub fn bit_en_rxd_debug(&mut self) -> BitEnRxdDebugW<'_, RegWmacDbgFtmCtlSpec> {
        BitEnRxdDebugW::new(self, 14)
    }
    #[doc = "Bit 15 - LA MODE function. enable mactx debug mode"]
    #[inline(always)]
    pub fn bit_en_txd_debug(&mut self) -> BitEnTxdDebugW<'_, RegWmacDbgFtmCtlSpec> {
        BitEnTxdDebugW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Select trigger time unit of 0x7C0\\[14:8\\]. 3'b000 : 1us , 3'b001 : 2us , 3'b010 : 4us , 3'b011 : 8us , 3'b100 : 16us , 3'b101 : 32us , 3'b110 : 64us, 3'b111 : RSV"]
    #[inline(always)]
    pub fn bit_trig_time_unit_sel(&mut self) -> BitTrigTimeUnitSelW<'_, RegWmacDbgFtmCtlSpec> {
        BitTrigTimeUnitSelW::new(self, 16)
    }
    #[doc = "Bit 19 - LA mode triggered by mac debug port timeout event"]
    #[inline(always)]
    pub fn bit_level_pulse_sel_ext(&mut self) -> BitLevelPulseSelExtW<'_, RegWmacDbgFtmCtlSpec> {
        BitLevelPulseSelExtW::new(self, 19)
    }
    #[doc = "Bits 20:25 - LA mode triggered by mac debug port timeout value, available with non-zero value."]
    #[inline(always)]
    pub fn bit_la_mac_timeout_value(&mut self) -> BitLaMacTimeoutValueW<'_, RegWmacDbgFtmCtlSpec> {
        BitLaMacTimeoutValueW::new(self, 20)
    }
    #[doc = "Bits 26:27 - LA mode triggered by mac debug port timeout value unit selection. 2'b00: 1us, 2'b01:32us, 2'b10: 1ms, 2'b11:RSVD"]
    #[inline(always)]
    pub fn bit_la_mac_timeout_unit_sel(
        &mut self,
    ) -> BitLaMacTimeoutUnitSelW<'_, RegWmacDbgFtmCtlSpec> {
        BitLaMacTimeoutUnitSelW::new(self, 26)
    }
    #[doc = "Bit 28 - Before setting 0x7C0\\[1:0\\] =2'b11, set this bit to 1 to enable MDPK access mode"]
    #[inline(always)]
    pub fn bit_en_re_start(&mut self) -> BitEnReStartW<'_, RegWmacDbgFtmCtlSpec> {
        BitEnReStartW::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to 1 to reset mdpk read pointer"]
    #[inline(always)]
    pub fn bit_en_timeout(&mut self) -> BitEnTimeoutW<'_, RegWmacDbgFtmCtlSpec> {
        BitEnTimeoutW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 11: data write to sram 10: data after sync_fifo 01: input data (TXD/RXD/IQ_DMA_DATA from BB) 00: RSVD"]
    #[inline(always)]
    pub fn bit_sel_debug_data(&mut self) -> BitSelDebugDataW<'_, RegWmacDbgFtmCtlSpec> {
        BitSelDebugDataW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_dbg_ftm_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_dbg_ftm_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacDbgFtmCtlSpec;
impl crate::RegisterSpec for RegWmacDbgFtmCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_dbg_ftm_ctl::R`](R) reader structure"]
impl crate::Readable for RegWmacDbgFtmCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_dbg_ftm_ctl::W`](W) writer structure"]
impl crate::Writable for RegWmacDbgFtmCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_DBG_FTM_CTL to value 0xc000_0080"]
impl crate::Resettable for RegWmacDbgFtmCtlSpec {
    const RESET_VALUE: u32 = 0xc000_0080;
}
