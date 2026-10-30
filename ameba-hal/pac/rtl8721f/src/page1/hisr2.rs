#[doc = "Register `HISR2` reader"]
pub type R = crate::R<Hisr2Spec>;
#[doc = "Field `BIT_HS_TIM_NO_HIT_INT` reader - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
pub type BitHsTimNoHitIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCN_IE_1_HIT_INT` reader - rx bcn with ie1 hit"]
pub type BitHsBcnIe1HitIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCN_IE_0_HIT_INT` reader - rx bcn with ie0 hit"]
pub type BitHsBcnIe0HitIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BC_TWT_PARA_CHG_INT` reader - RX broadcast twt with parameter changing,notify fw to read twt report"]
pub type BitHsBcTwtParaChgIntR = crate::BitReader;
#[doc = "Field `BIT_HS_DFS_PKT_INT` reader - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
pub type BitHsDfsPktIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TIM_HIT_INT` reader - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
pub type BitHsTimHitIntR = crate::BitReader;
#[doc = "Field `BIT_HS_RX_PHYSTS_DONE_INT` reader - indicate physts is successfully write into physts_mem"]
pub type BitHsRxPhystsDoneIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_A_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
pub type BitHsPsTimerAEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_B_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
pub type BitHsPsTimerBEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_C_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
pub type BitHsPsTimerCEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_D_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
pub type BitHsPsTimerDEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_E_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
pub type BitHsPsTimerEEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_D_INT` reader - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
pub type BitHsPsTimerDIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_E_INT` reader - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
pub type BitHsPsTimerEIntR = crate::BitReader;
#[doc = "Field `BIT_HS_RXBCN_NAN_RPT_OK` reader - port2 rxbcn rpt and write to txrptbuf ok"]
pub type BitHsRxbcnNanRptOkR = crate::BitReader;
#[doc = "Field `BIT_HS_DWWIN_END_INT` reader - discovery window end interrupt for NAN"]
pub type BitHsDwwinEndIntR = crate::BitReader;
#[doc = "Field `BIT_HS_EVTQ_START_INT` reader - event queue start interrupt for NAN"]
pub type BitHsEvtqStartIntR = crate::BitReader;
#[doc = "Field `BIT_HS_EVTQ_TXDONE_INT` reader - event queue done interrupt for NAN"]
pub type BitHsEvtqTxdoneIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNERR_PORT2_INT` reader - PORT2 TX DISC beacon Error interrupt"]
pub type BitHsTxbcnerrPort2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNERR_PORT2_SYNC_INT` reader - PORT2 TX SYNC beacon Error interrupt"]
pub type BitHsTxbcnerrPort2SyncIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNOK_PORT2_INT` reader - PORT2 TX DISC beacon OK interrupt"]
pub type BitHsTxbcnokPort2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNOK_PORT2_SYNC_INT` reader - PORT2 TX SYNC beacon OK interrupt"]
pub type BitHsTxbcnokPort2SyncIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNDMA2_INT` reader - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndma2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNDMA_P2_2G_INT` reader - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndmaP2_2gIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNDMA_P2_5G_INT` reader - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndmaP2_5gIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TBTT2_DISC_INT` reader - the interrupt from the exact TBTT time for P2."]
pub type BitHsTbtt2DiscIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TBTT2_SYNC_2G_INT` reader - the interrupt from the exact TBTT time for P2 SYNC_2G."]
pub type BitHsTbtt2Sync2gIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TBTT2_SYNC_5G_INT` reader - the interrupt from the exact TBTT time for P2 SYNC_5G."]
pub type BitHsTbtt2Sync5gIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY2_DSIC_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
pub type BitHsBcnerly2DsicIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY2_SYNC_2G_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
pub type BitHsBcnerly2Sync2gIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY2_SYNC_5G_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
pub type BitHsBcnerly2Sync5gIntR = crate::BitReader;
#[doc = "Field `BIT_KM0_INT` reader - The gathering of interrupts of KM0"]
pub type BitKm0IntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
    #[inline(always)]
    pub fn bit_hs_tim_no_hit_int(&self) -> BitHsTimNoHitIntR {
        BitHsTimNoHitIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx bcn with ie1 hit"]
    #[inline(always)]
    pub fn bit_hs_bcn_ie_1_hit_int(&self) -> BitHsBcnIe1HitIntR {
        BitHsBcnIe1HitIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx bcn with ie0 hit"]
    #[inline(always)]
    pub fn bit_hs_bcn_ie_0_hit_int(&self) -> BitHsBcnIe0HitIntR {
        BitHsBcnIe0HitIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX broadcast twt with parameter changing,notify fw to read twt report"]
    #[inline(always)]
    pub fn bit_hs_bc_twt_para_chg_int(&self) -> BitHsBcTwtParaChgIntR {
        BitHsBcTwtParaChgIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
    #[inline(always)]
    pub fn bit_hs_dfs_pkt_int(&self) -> BitHsDfsPktIntR {
        BitHsDfsPktIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
    #[inline(always)]
    pub fn bit_hs_tim_hit_int(&self) -> BitHsTimHitIntR {
        BitHsTimHitIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - indicate physts is successfully write into physts_mem"]
    #[inline(always)]
    pub fn bit_hs_rx_physts_done_int(&self) -> BitHsRxPhystsDoneIntR {
        BitHsRxPhystsDoneIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_a_early_int(&self) -> BitHsPsTimerAEarlyIntR {
        BitHsPsTimerAEarlyIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_b_early_int(&self) -> BitHsPsTimerBEarlyIntR {
        BitHsPsTimerBEarlyIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_c_early_int(&self) -> BitHsPsTimerCEarlyIntR {
        BitHsPsTimerCEarlyIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_d_early_int(&self) -> BitHsPsTimerDEarlyIntR {
        BitHsPsTimerDEarlyIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_e_early_int(&self) -> BitHsPsTimerEEarlyIntR {
        BitHsPsTimerEEarlyIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_d_int(&self) -> BitHsPsTimerDIntR {
        BitHsPsTimerDIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_e_int(&self) -> BitHsPsTimerEIntR {
        BitHsPsTimerEIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - port2 rxbcn rpt and write to txrptbuf ok"]
    #[inline(always)]
    pub fn bit_hs_rxbcn_nan_rpt_ok(&self) -> BitHsRxbcnNanRptOkR {
        BitHsRxbcnNanRptOkR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - discovery window end interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_dwwin_end_int(&self) -> BitHsDwwinEndIntR {
        BitHsDwwinEndIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - event queue start interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_evtq_start_int(&self) -> BitHsEvtqStartIntR {
        BitHsEvtqStartIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - event queue done interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_evtq_txdone_int(&self) -> BitHsEvtqTxdoneIntR {
        BitHsEvtqTxdoneIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PORT2 TX DISC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnerr_port2_int(&self) -> BitHsTxbcnerrPort2IntR {
        BitHsTxbcnerrPort2IntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PORT2 TX SYNC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnerr_port2_sync_int(&self) -> BitHsTxbcnerrPort2SyncIntR {
        BitHsTxbcnerrPort2SyncIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PORT2 TX DISC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnok_port2_int(&self) -> BitHsTxbcnokPort2IntR {
        BitHsTxbcnokPort2IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PORT2 TX SYNC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnok_port2_sync_int(&self) -> BitHsTxbcnokPort2SyncIntR {
        BitHsTxbcnokPort2SyncIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma2_int(&self) -> BitHsBcndma2IntR {
        BitHsBcndma2IntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma_p2_2g_int(&self) -> BitHsBcndmaP2_2gIntR {
        BitHsBcndmaP2_2gIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma_p2_5g_int(&self) -> BitHsBcndmaP2_5gIntR {
        BitHsBcndmaP2_5gIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - the interrupt from the exact TBTT time for P2."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_disc_int(&self) -> BitHsTbtt2DiscIntR {
        BitHsTbtt2DiscIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - the interrupt from the exact TBTT time for P2 SYNC_2G."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_sync_2g_int(&self) -> BitHsTbtt2Sync2gIntR {
        BitHsTbtt2Sync2gIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the interrupt from the exact TBTT time for P2 SYNC_5G."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_sync_5g_int(&self) -> BitHsTbtt2Sync5gIntR {
        BitHsTbtt2Sync5gIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_dsic_int(&self) -> BitHsBcnerly2DsicIntR {
        BitHsBcnerly2DsicIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_sync_2g_int(&self) -> BitHsBcnerly2Sync2gIntR {
        BitHsBcnerly2Sync2gIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_sync_5g_int(&self) -> BitHsBcnerly2Sync5gIntR {
        BitHsBcnerly2Sync5gIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The gathering of interrupts of KM0"]
    #[inline(always)]
    pub fn bit_km0_int(&self) -> BitKm0IntR {
        BitKm0IntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hisr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hisr2Spec;
impl crate::RegisterSpec for Hisr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hisr2::R`](R) reader structure"]
impl crate::Readable for Hisr2Spec {}
#[doc = "`reset()` method sets HISR2 to value 0"]
impl crate::Resettable for Hisr2Spec {}
