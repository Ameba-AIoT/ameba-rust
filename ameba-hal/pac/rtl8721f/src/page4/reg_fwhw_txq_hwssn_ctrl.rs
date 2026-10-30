#[doc = "Register `REG_FWHW_TXQ_HWSSN_CTRL` reader"]
pub type R = crate::R<RegFwhwTxqHwssnCtrlSpec>;
#[doc = "Register `REG_FWHW_TXQ_HWSSN_CTRL` writer"]
pub type W = crate::W<RegFwhwTxqHwssnCtrlSpec>;
#[doc = "Field `BIT_PROTECT_BMC_NAV` reader - Enable the NAV protection when transmit broadcast/multicast"]
pub type BitProtectBmcNavR = crate::BitReader;
#[doc = "Field `BIT_PROTECT_BMC_NAV` writer - Enable the NAV protection when transmit broadcast/multicast"]
pub type BitProtectBmcNavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_FTM_RPT` reader - When this bit asserted, HW will report FTM rpt"]
pub type BitEnFtmRptR = crate::BitReader;
#[doc = "Field `BIT_EN_FTM_RPT` writer - When this bit asserted, HW will report FTM rpt"]
pub type BitEnFtmRptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BCN_TXRPT` reader - 1: HW will report bcn tx_status 0: no report"]
pub type BitEnBcnTxrptR = crate::BitReader;
#[doc = "Field `BIT_EN_BCN_TXRPT` writer - 1: HW will report bcn tx_status 0: no report"]
pub type BitEnBcnTxrptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MACID_MATCH_RTS` reader - Enable the packet that follow the RTS have same MACID"]
pub type BitMacidMatchRtsR = crate::BitReader;
#[doc = "Field `BIT_MACID_MATCH_RTS` writer - Enable the packet that follow the RTS have same MACID"]
pub type BitMacidMatchRtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_SSN_CHECK` reader - Disable checker the SSN rule when generating AMPDU TX cmd."]
pub type BitDisSsnCheckR = crate::BitReader;
#[doc = "Field `BIT_DIS_SSN_CHECK` writer - Disable checker the SSN rule when generating AMPDU TX cmd."]
pub type BitDisSsnCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_RTS_NAV_BK` reader - Control The RTS NAV criteria for TX. When this bit is clear, TX may aggregates only according to the number of MPDUs existing in the TXPKTBUF, the limit of aggregation size and number, and also the break time. When this bit is set, TX will be under the RTS NAV criteria. No excess RTS NAV is allowed."]
pub type BitEnRtsNavBkR = crate::BitReader;
#[doc = "Field `BIT_EN_RTS_NAV_BK` writer - Control The RTS NAV criteria for TX. When this bit is clear, TX may aggregates only according to the number of MPDUs existing in the TXPKTBUF, the limit of aggregation size and number, and also the break time. When this bit is set, TX will be under the RTS NAV criteria. No excess RTS NAV is allowed."]
pub type BitEnRtsNavBkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_USE_INI_RAT` reader - 0: If Txdesc's USE RATE = 0 and this field is equal to 0, the datarate of TXRPT SRAM is used as tx pkt's datarate. (ECO flow) 1: TXInfo's datarate is used as tx pkt's datarate."]
pub type BitEnUseIniRatR = crate::BitReader;
#[doc = "Field `BIT_EN_USE_INI_RAT` writer - 0: If Txdesc's USE RATE = 0 and this field is equal to 0, the datarate of TXRPT SRAM is used as tx pkt's datarate. (ECO flow) 1: TXInfo's datarate is used as tx pkt's datarate."]
pub type BitEnUseIniRatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_RTY_BK` reader - AMPDU aggregation mode with retry MPDUs and new MPDUs. When this bit is clear enabled, an AMPDU aggregates retried MPDU and new MPDU. All the aggregation criteria is still the same with normal AMPDU operation."]
pub type BitEnRtyBkR = crate::BitReader;
#[doc = "Field `BIT_EN_RTY_BK` writer - AMPDU aggregation mode with retry MPDUs and new MPDUs. When this bit is clear enabled, an AMPDU aggregates retried MPDU and new MPDU. All the aggregation criteria is still the same with normal AMPDU operation."]
pub type BitEnRtyBkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_QUEUE_RPT` reader - 8723A/8812 1: HW will report tx_status 0: no report BIT0: BK Queue BIT1: BE Queue BIT2: VI Queue BIT3: VO Queue BIT4: MGT Queue BIT5: HI Queue BIT6: BCN Queue BIT7: CPU_MGT Queue"]
pub type BitEnQueueRptR = crate::FieldReader;
#[doc = "Field `BIT_EN_QUEUE_RPT` writer - 8723A/8812 1: HW will report tx_status 0: no report BIT0: BK Queue BIT1: BE Queue BIT2: VI Queue BIT3: VO Queue BIT4: MGT Queue BIT5: HI Queue BIT6: BCN Queue BIT7: CPU_MGT Queue"]
pub type BitEnQueueRptW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EN_RTY_BK_COND` reader - AMPDU aggregation mode with retry MPDUs and new MPDUs. When this bit is enabled, if new MPDU's data rate is not the same as retry MPDUs' fallback data rate, the aggregation will be broken."]
pub type BitEnRtyBkCondR = crate::BitReader;
#[doc = "Field `BIT_EN_RTY_BK_COND` writer - AMPDU aggregation mode with retry MPDUs and new MPDUs. When this bit is enabled, if new MPDU's data rate is not the same as retry MPDUs' fallback data rate, the aggregation will be broken."]
pub type BitEnRtyBkCondW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_RTY_BC` reader - When this bit is enabled, retry the broadcast packet"]
pub type BitEnRtyBcR = crate::BitReader;
#[doc = "Field `BIT_EN_RTY_BC` writer - When this bit is enabled, retry the broadcast packet"]
pub type BitEnRtyBcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXFAILRPT_DIS` reader - when this bit is enabled, HW will not generate txrpt for tx failed packets"]
pub type BitTxfailrptDisR = crate::BitReader;
#[doc = "Field `BIT_TXFAILRPT_DIS` writer - when this bit is enabled, HW will not generate txrpt for tx failed packets"]
pub type BitTxfailrptDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXRPT_DIS` reader - when this bit is enabled, HW will not generate txrpt for driver's fixed rate packets"]
pub type BitTxrptDisR = crate::BitReader;
#[doc = "Field `BIT_TXRPT_DIS` writer - when this bit is enabled, HW will not generate txrpt for driver's fixed rate packets"]
pub type BitTxrptDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_WR_FREE_TAIL` reader - 1: Drive write Bcnq_boundary, Hw will set free_tail in Boundary-1 0: no update"]
pub type BitEnWrFreeTailR = crate::BitReader;
#[doc = "Field `BIT_EN_WR_FREE_TAIL` writer - 1: Drive write Bcnq_boundary, Hw will set free_tail in Boundary-1 0: no update"]
pub type BitEnWrFreeTailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_RD_RESP_NAV_BK` reader - Control the RD responder NAV criteria for TX. When this bit is clear, RD responder may aggregates only according to the number of MPDUs existing in the TXPKTBUF, the limit of aggregation size and number, and also the break time. When this bit is set, RD responder will be under the RX NAV criteria for RD response sequence. No excess NAV is allowed"]
pub type BitEnRdRespNavBkR = crate::BitReader;
#[doc = "Field `BIT_EN_RD_RESP_NAV_BK` writer - Control the RD responder NAV criteria for TX. When this bit is clear, RD responder may aggregates only according to the number of MPDUs existing in the TXPKTBUF, the limit of aggregation size and number, and also the break time. When this bit is set, RD responder will be under the RX NAV criteria for RD response sequence. No excess NAV is allowed"]
pub type BitEnRdRespNavBkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BCNQ_DL` reader - 1: ptcl consider there is bcn pkt in TXBUF when bcn pkt first be transferd in; 0: mask bcn pkt num in bcn queue"]
pub type BitEnBcnqDlR = crate::BitReader;
#[doc = "Field `BIT_EN_BCNQ_DL` writer - 1: ptcl consider there is bcn pkt in TXBUF when bcn pkt first be transferd in; 0: mask bcn pkt num in bcn queue"]
pub type BitEnBcnqDlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RTS_LIMIT_IN_OFDM` reader - Rts rate will not down to cck rate"]
pub type BitRtsLimitInOfdmR = crate::BitReader;
#[doc = "Field `BIT_RTS_LIMIT_IN_OFDM` writer - Rts rate will not down to cck rate"]
pub type BitRtsLimitInOfdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HW_SSN_EN` reader - HW update the SSN \\[7:0\\], When this bit enable and EN_HWSEQ of TXDESC, MAC will overwrite MACHDR SEQ according to TXDESC QSEL \\[0\\] BIT_HWSEQ_VO_EN \\[1\\] BIT_HWSEQ_VI_EN \\[2\\] BIT_HWSEQ_BE_EN \\[3\\] BIT_HWSEQ_BK_EN \\[4\\] BIT_HWSEQ_MGT_EN \\[5\\] BIT_HWSEQ_HI_EN \\[6\\] BIT_HWSEQ_BCN_EN,don't care EN_HWSEQ of TXDESC \\[7\\] BIT_HWSEQ_CPUM_EN"]
pub type BitHwSsnEnR = crate::FieldReader;
#[doc = "Field `BIT_HW_SSN_EN` writer - HW update the SSN \\[7:0\\], When this bit enable and EN_HWSEQ of TXDESC, MAC will overwrite MACHDR SEQ according to TXDESC QSEL \\[0\\] BIT_HWSEQ_VO_EN \\[1\\] BIT_HWSEQ_VI_EN \\[2\\] BIT_HWSEQ_BE_EN \\[3\\] BIT_HWSEQ_BK_EN \\[4\\] BIT_HWSEQ_MGT_EN \\[5\\] BIT_HWSEQ_HI_EN \\[6\\] BIT_HWSEQ_BCN_EN,don't care EN_HWSEQ of TXDESC \\[7\\] BIT_HWSEQ_CPUM_EN"]
pub type BitHwSsnEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable the NAV protection when transmit broadcast/multicast"]
    #[inline(always)]
    pub fn bit_protect_bmc_nav(&self) -> BitProtectBmcNavR {
        BitProtectBmcNavR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit asserted, HW will report FTM rpt"]
    #[inline(always)]
    pub fn bit_en_ftm_rpt(&self) -> BitEnFtmRptR {
        BitEnFtmRptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: HW will report bcn tx_status 0: no report"]
    #[inline(always)]
    pub fn bit_en_bcn_txrpt(&self) -> BitEnBcnTxrptR {
        BitEnBcnTxrptR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the packet that follow the RTS have same MACID"]
    #[inline(always)]
    pub fn bit_macid_match_rts(&self) -> BitMacidMatchRtsR {
        BitMacidMatchRtsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable checker the SSN rule when generating AMPDU TX cmd."]
    #[inline(always)]
    pub fn bit_dis_ssn_check(&self) -> BitDisSsnCheckR {
        BitDisSsnCheckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control The RTS NAV criteria for TX. When this bit is clear, TX may aggregates only according to the number of MPDUs existing in the TXPKTBUF, the limit of aggregation size and number, and also the break time. When this bit is set, TX will be under the RTS NAV criteria. No excess RTS NAV is allowed."]
    #[inline(always)]
    pub fn bit_en_rts_nav_bk(&self) -> BitEnRtsNavBkR {
        BitEnRtsNavBkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: If Txdesc's USE RATE = 0 and this field is equal to 0, the datarate of TXRPT SRAM is used as tx pkt's datarate. (ECO flow) 1: TXInfo's datarate is used as tx pkt's datarate."]
    #[inline(always)]
    pub fn bit_en_use_ini_rat(&self) -> BitEnUseIniRatR {
        BitEnUseIniRatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AMPDU aggregation mode with retry MPDUs and new MPDUs. When this bit is clear enabled, an AMPDU aggregates retried MPDU and new MPDU. All the aggregation criteria is still the same with normal AMPDU operation."]
    #[inline(always)]
    pub fn bit_en_rty_bk(&self) -> BitEnRtyBkR {
        BitEnRtyBkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 8723A/8812 1: HW will report tx_status 0: no report BIT0: BK Queue BIT1: BE Queue BIT2: VI Queue BIT3: VO Queue BIT4: MGT Queue BIT5: HI Queue BIT6: BCN Queue BIT7: CPU_MGT Queue"]
    #[inline(always)]
    pub fn bit_en_queue_rpt(&self) -> BitEnQueueRptR {
        BitEnQueueRptR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - AMPDU aggregation mode with retry MPDUs and new MPDUs. When this bit is enabled, if new MPDU's data rate is not the same as retry MPDUs' fallback data rate, the aggregation will be broken."]
    #[inline(always)]
    pub fn bit_en_rty_bk_cond(&self) -> BitEnRtyBkCondR {
        BitEnRtyBkCondR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When this bit is enabled, retry the broadcast packet"]
    #[inline(always)]
    pub fn bit_en_rty_bc(&self) -> BitEnRtyBcR {
        BitEnRtyBcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - when this bit is enabled, HW will not generate txrpt for tx failed packets"]
    #[inline(always)]
    pub fn bit_txfailrpt_dis(&self) -> BitTxfailrptDisR {
        BitTxfailrptDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - when this bit is enabled, HW will not generate txrpt for driver's fixed rate packets"]
    #[inline(always)]
    pub fn bit_txrpt_dis(&self) -> BitTxrptDisR {
        BitTxrptDisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Drive write Bcnq_boundary, Hw will set free_tail in Boundary-1 0: no update"]
    #[inline(always)]
    pub fn bit_en_wr_free_tail(&self) -> BitEnWrFreeTailR {
        BitEnWrFreeTailR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Control the RD responder NAV criteria for TX. When this bit is clear, RD responder may aggregates only according to the number of MPDUs existing in the TXPKTBUF, the limit of aggregation size and number, and also the break time. When this bit is set, RD responder will be under the RX NAV criteria for RD response sequence. No excess NAV is allowed"]
    #[inline(always)]
    pub fn bit_en_rd_resp_nav_bk(&self) -> BitEnRdRespNavBkR {
        BitEnRdRespNavBkR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: ptcl consider there is bcn pkt in TXBUF when bcn pkt first be transferd in; 0: mask bcn pkt num in bcn queue"]
    #[inline(always)]
    pub fn bit_en_bcnq_dl(&self) -> BitEnBcnqDlR {
        BitEnBcnqDlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rts rate will not down to cck rate"]
    #[inline(always)]
    pub fn bit_rts_limit_in_ofdm(&self) -> BitRtsLimitInOfdmR {
        BitRtsLimitInOfdmR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - HW update the SSN \\[7:0\\], When this bit enable and EN_HWSEQ of TXDESC, MAC will overwrite MACHDR SEQ according to TXDESC QSEL \\[0\\] BIT_HWSEQ_VO_EN \\[1\\] BIT_HWSEQ_VI_EN \\[2\\] BIT_HWSEQ_BE_EN \\[3\\] BIT_HWSEQ_BK_EN \\[4\\] BIT_HWSEQ_MGT_EN \\[5\\] BIT_HWSEQ_HI_EN \\[6\\] BIT_HWSEQ_BCN_EN,don't care EN_HWSEQ of TXDESC \\[7\\] BIT_HWSEQ_CPUM_EN"]
    #[inline(always)]
    pub fn bit_hw_ssn_en(&self) -> BitHwSsnEnR {
        BitHwSsnEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the NAV protection when transmit broadcast/multicast"]
    #[inline(always)]
    pub fn bit_protect_bmc_nav(&mut self) -> BitProtectBmcNavW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitProtectBmcNavW::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit asserted, HW will report FTM rpt"]
    #[inline(always)]
    pub fn bit_en_ftm_rpt(&mut self) -> BitEnFtmRptW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnFtmRptW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: HW will report bcn tx_status 0: no report"]
    #[inline(always)]
    pub fn bit_en_bcn_txrpt(&mut self) -> BitEnBcnTxrptW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnBcnTxrptW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable the packet that follow the RTS have same MACID"]
    #[inline(always)]
    pub fn bit_macid_match_rts(&mut self) -> BitMacidMatchRtsW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitMacidMatchRtsW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable checker the SSN rule when generating AMPDU TX cmd."]
    #[inline(always)]
    pub fn bit_dis_ssn_check(&mut self) -> BitDisSsnCheckW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitDisSsnCheckW::new(self, 4)
    }
    #[doc = "Bit 5 - Control The RTS NAV criteria for TX. When this bit is clear, TX may aggregates only according to the number of MPDUs existing in the TXPKTBUF, the limit of aggregation size and number, and also the break time. When this bit is set, TX will be under the RTS NAV criteria. No excess RTS NAV is allowed."]
    #[inline(always)]
    pub fn bit_en_rts_nav_bk(&mut self) -> BitEnRtsNavBkW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnRtsNavBkW::new(self, 5)
    }
    #[doc = "Bit 6 - 0: If Txdesc's USE RATE = 0 and this field is equal to 0, the datarate of TXRPT SRAM is used as tx pkt's datarate. (ECO flow) 1: TXInfo's datarate is used as tx pkt's datarate."]
    #[inline(always)]
    pub fn bit_en_use_ini_rat(&mut self) -> BitEnUseIniRatW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnUseIniRatW::new(self, 6)
    }
    #[doc = "Bit 7 - AMPDU aggregation mode with retry MPDUs and new MPDUs. When this bit is clear enabled, an AMPDU aggregates retried MPDU and new MPDU. All the aggregation criteria is still the same with normal AMPDU operation."]
    #[inline(always)]
    pub fn bit_en_rty_bk(&mut self) -> BitEnRtyBkW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnRtyBkW::new(self, 7)
    }
    #[doc = "Bits 8:15 - 8723A/8812 1: HW will report tx_status 0: no report BIT0: BK Queue BIT1: BE Queue BIT2: VI Queue BIT3: VO Queue BIT4: MGT Queue BIT5: HI Queue BIT6: BCN Queue BIT7: CPU_MGT Queue"]
    #[inline(always)]
    pub fn bit_en_queue_rpt(&mut self) -> BitEnQueueRptW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnQueueRptW::new(self, 8)
    }
    #[doc = "Bit 16 - AMPDU aggregation mode with retry MPDUs and new MPDUs. When this bit is enabled, if new MPDU's data rate is not the same as retry MPDUs' fallback data rate, the aggregation will be broken."]
    #[inline(always)]
    pub fn bit_en_rty_bk_cond(&mut self) -> BitEnRtyBkCondW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnRtyBkCondW::new(self, 16)
    }
    #[doc = "Bit 17 - When this bit is enabled, retry the broadcast packet"]
    #[inline(always)]
    pub fn bit_en_rty_bc(&mut self) -> BitEnRtyBcW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnRtyBcW::new(self, 17)
    }
    #[doc = "Bit 18 - when this bit is enabled, HW will not generate txrpt for tx failed packets"]
    #[inline(always)]
    pub fn bit_txfailrpt_dis(&mut self) -> BitTxfailrptDisW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitTxfailrptDisW::new(self, 18)
    }
    #[doc = "Bit 19 - when this bit is enabled, HW will not generate txrpt for driver's fixed rate packets"]
    #[inline(always)]
    pub fn bit_txrpt_dis(&mut self) -> BitTxrptDisW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitTxrptDisW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Drive write Bcnq_boundary, Hw will set free_tail in Boundary-1 0: no update"]
    #[inline(always)]
    pub fn bit_en_wr_free_tail(&mut self) -> BitEnWrFreeTailW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnWrFreeTailW::new(self, 20)
    }
    #[doc = "Bit 21 - Control the RD responder NAV criteria for TX. When this bit is clear, RD responder may aggregates only according to the number of MPDUs existing in the TXPKTBUF, the limit of aggregation size and number, and also the break time. When this bit is set, RD responder will be under the RX NAV criteria for RD response sequence. No excess NAV is allowed"]
    #[inline(always)]
    pub fn bit_en_rd_resp_nav_bk(&mut self) -> BitEnRdRespNavBkW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnRdRespNavBkW::new(self, 21)
    }
    #[doc = "Bit 22 - 1: ptcl consider there is bcn pkt in TXBUF when bcn pkt first be transferd in; 0: mask bcn pkt num in bcn queue"]
    #[inline(always)]
    pub fn bit_en_bcnq_dl(&mut self) -> BitEnBcnqDlW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitEnBcnqDlW::new(self, 22)
    }
    #[doc = "Bit 23 - Rts rate will not down to cck rate"]
    #[inline(always)]
    pub fn bit_rts_limit_in_ofdm(&mut self) -> BitRtsLimitInOfdmW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitRtsLimitInOfdmW::new(self, 23)
    }
    #[doc = "Bits 24:31 - HW update the SSN \\[7:0\\], When this bit enable and EN_HWSEQ of TXDESC, MAC will overwrite MACHDR SEQ according to TXDESC QSEL \\[0\\] BIT_HWSEQ_VO_EN \\[1\\] BIT_HWSEQ_VI_EN \\[2\\] BIT_HWSEQ_BE_EN \\[3\\] BIT_HWSEQ_BK_EN \\[4\\] BIT_HWSEQ_MGT_EN \\[5\\] BIT_HWSEQ_HI_EN \\[6\\] BIT_HWSEQ_BCN_EN,don't care EN_HWSEQ of TXDESC \\[7\\] BIT_HWSEQ_CPUM_EN"]
    #[inline(always)]
    pub fn bit_hw_ssn_en(&mut self) -> BitHwSsnEnW<'_, RegFwhwTxqHwssnCtrlSpec> {
        BitHwSsnEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fwhw_txq_hwssn_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fwhw_txq_hwssn_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwhwTxqHwssnCtrlSpec;
impl crate::RegisterSpec for RegFwhwTxqHwssnCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fwhw_txq_hwssn_ctrl::R`](R) reader structure"]
impl crate::Readable for RegFwhwTxqHwssnCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fwhw_txq_hwssn_ctrl::W`](W) writer structure"]
impl crate::Writable for RegFwhwTxqHwssnCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FWHW_TXQ_HWSSN_CTRL to value 0x0074_3f87"]
impl crate::Resettable for RegFwhwTxqHwssnCtrlSpec {
    const RESET_VALUE: u32 = 0x0074_3f87;
}
