#[doc = "Register `REG_HIMR2` reader"]
pub type R = crate::R<RegHimr2Spec>;
#[doc = "Register `REG_HIMR2` writer"]
pub type W = crate::W<RegHimr2Spec>;
#[doc = "Field `BIT_HS_TIM_NO_HIT_INT_EN` reader - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
pub type BitHsTimNoHitIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TIM_NO_HIT_INT_EN` writer - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
pub type BitHsTimNoHitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCN_IE_1_HIT_INT_EN` reader - rx bcn with ie1 hit"]
pub type BitHsBcnIe1HitIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCN_IE_1_HIT_INT_EN` writer - rx bcn with ie1 hit"]
pub type BitHsBcnIe1HitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCN_IE_0_HIT_INT_EN` reader - rx bcn with ie0 hit"]
pub type BitHsBcnIe0HitIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCN_IE_0_HIT_INT_EN` writer - rx bcn with ie0 hit"]
pub type BitHsBcnIe0HitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BC_TWT_PARA_CHG_INT_EN` reader - RX broadcast twt with parameter changing,notify fw to read twt report"]
pub type BitHsBcTwtParaChgIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BC_TWT_PARA_CHG_INT_EN` writer - RX broadcast twt with parameter changing,notify fw to read twt report"]
pub type BitHsBcTwtParaChgIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_DFS_PKT_INT_EN` reader - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
pub type BitHsDfsPktIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_DFS_PKT_INT_EN` writer - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
pub type BitHsDfsPktIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TIM_HIT_INT_EN` reader - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
pub type BitHsTimHitIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TIM_HIT_INT_EN` writer - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
pub type BitHsTimHitIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_RX_PHYSTS_DONE_INT_EN` reader - indicate physts is successfully write into physts_mem"]
pub type BitHsRxPhystsDoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_RX_PHYSTS_DONE_INT_EN` writer - indicate physts is successfully write into physts_mem"]
pub type BitHsRxPhystsDoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_A_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
pub type BitHsPsTimerAEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_A_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
pub type BitHsPsTimerAEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_B_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
pub type BitHsPsTimerBEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_B_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
pub type BitHsPsTimerBEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_C_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
pub type BitHsPsTimerCEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_C_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
pub type BitHsPsTimerCEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_D_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
pub type BitHsPsTimerDEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_D_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
pub type BitHsPsTimerDEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_E_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
pub type BitHsPsTimerEEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_E_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
pub type BitHsPsTimerEEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_D_INT_EN` reader - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
pub type BitHsPsTimerDIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_D_INT_EN` writer - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
pub type BitHsPsTimerDIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_E_INT_EN` reader - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
pub type BitHsPsTimerEIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_E_INT_EN` writer - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
pub type BitHsPsTimerEIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_RXBCN_NAN_RPT_OK_EN` reader - port2 rxbcn rpt and write to txrptbuf ok"]
pub type BitHsRxbcnNanRptOkEnR = crate::BitReader;
#[doc = "Field `BIT_HS_RXBCN_NAN_RPT_OK_EN` writer - port2 rxbcn rpt and write to txrptbuf ok"]
pub type BitHsRxbcnNanRptOkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_DWWIN_END_INT_EN` reader - discovery window end interrupt for NAN"]
pub type BitHsDwwinEndIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_DWWIN_END_INT_EN` writer - discovery window end interrupt for NAN"]
pub type BitHsDwwinEndIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_EVTQ_START_INT_EN` reader - event queue start interrupt for NAN"]
pub type BitHsEvtqStartIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_EVTQ_START_INT_EN` writer - event queue start interrupt for NAN"]
pub type BitHsEvtqStartIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_EVTQ_TXDONE_INT_EN` reader - event queue done interrupt for NAN"]
pub type BitHsEvtqTxdoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_EVTQ_TXDONE_INT_EN` writer - event queue done interrupt for NAN"]
pub type BitHsEvtqTxdoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCNERR_PORT2_INT_EN` reader - PORT2 TX DISC beacon Error interrupt"]
pub type BitHsTxbcnerrPort2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNERR_PORT2_INT_EN` writer - PORT2 TX DISC beacon Error interrupt"]
pub type BitHsTxbcnerrPort2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCNERR_PORT2_SYNC_INT_EN` reader - PORT2 TX SYNC beacon Error interrupt"]
pub type BitHsTxbcnerrPort2SyncIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNERR_PORT2_SYNC_INT_EN` writer - PORT2 TX SYNC beacon Error interrupt"]
pub type BitHsTxbcnerrPort2SyncIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCNOK_PORT2_INT_EN` reader - PORT2 TX DISC beacon OK interrupt"]
pub type BitHsTxbcnokPort2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNOK_PORT2_INT_EN` writer - PORT2 TX DISC beacon OK interrupt"]
pub type BitHsTxbcnokPort2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCNOK_PORT2_SYNC_INT_EN` reader - PORT2 TX SYNC beacon OK interrupt"]
pub type BitHsTxbcnokPort2SyncIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNOK_PORT2_SYNC_INT_EN` writer - PORT2 TX SYNC beacon OK interrupt"]
pub type BitHsTxbcnokPort2SyncIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNDMA2_INT_EN` reader - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndma2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNDMA2_INT_EN` writer - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndma2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNDMA_P2_2G_INT_EN` reader - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndmaP2_2gIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNDMA_P2_2G_INT_EN` writer - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndmaP2_2gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNDMA_P2_5G_INT_EN` reader - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndmaP2_5gIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNDMA_P2_5G_INT_EN` writer - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
pub type BitHsBcndmaP2_5gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TBTT2_DISC_INT_EN` reader - the interrupt from the exact TBTT time for P2."]
pub type BitHsTbtt2DiscIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TBTT2_DISC_INT_EN` writer - the interrupt from the exact TBTT time for P2."]
pub type BitHsTbtt2DiscIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TBTT2_SYNC_2G_INT_EN` reader - the interrupt from the exact TBTT time for P2 SYNC_2G."]
pub type BitHsTbtt2Sync2gIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TBTT2_SYNC_2G_INT_EN` writer - the interrupt from the exact TBTT time for P2 SYNC_2G."]
pub type BitHsTbtt2Sync2gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TBTT2_SYNC_5G_INT_EN` reader - the interrupt from the exact TBTT time for P2 SYNC_5G."]
pub type BitHsTbtt2Sync5gIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TBTT2_SYNC_5G_INT_EN` writer - the interrupt from the exact TBTT time for P2 SYNC_5G."]
pub type BitHsTbtt2Sync5gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY2_DISC_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
pub type BitHsBcnerly2DiscIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY2_DISC_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
pub type BitHsBcnerly2DiscIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY2_SYNC_2G_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
pub type BitHsBcnerly2Sync2gIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY2_SYNC_2G_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
pub type BitHsBcnerly2Sync2gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY2_SYNC_5G_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
pub type BitHsBcnerly2Sync5gIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY2_SYNC_5G_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
pub type BitHsBcnerly2Sync5gIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_KM0_INT_EN` reader - The gathering of interrupts of KM0"]
pub type BitKm0IntEnR = crate::BitReader;
#[doc = "Field `BIT_KM0_INT_EN` writer - The gathering of interrupts of KM0"]
pub type BitKm0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
    #[inline(always)]
    pub fn bit_hs_tim_no_hit_int_en(&self) -> BitHsTimNoHitIntEnR {
        BitHsTimNoHitIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx bcn with ie1 hit"]
    #[inline(always)]
    pub fn bit_hs_bcn_ie_1_hit_int_en(&self) -> BitHsBcnIe1HitIntEnR {
        BitHsBcnIe1HitIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx bcn with ie0 hit"]
    #[inline(always)]
    pub fn bit_hs_bcn_ie_0_hit_int_en(&self) -> BitHsBcnIe0HitIntEnR {
        BitHsBcnIe0HitIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX broadcast twt with parameter changing,notify fw to read twt report"]
    #[inline(always)]
    pub fn bit_hs_bc_twt_para_chg_int_en(&self) -> BitHsBcTwtParaChgIntEnR {
        BitHsBcTwtParaChgIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
    #[inline(always)]
    pub fn bit_hs_dfs_pkt_int_en(&self) -> BitHsDfsPktIntEnR {
        BitHsDfsPktIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
    #[inline(always)]
    pub fn bit_hs_tim_hit_int_en(&self) -> BitHsTimHitIntEnR {
        BitHsTimHitIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - indicate physts is successfully write into physts_mem"]
    #[inline(always)]
    pub fn bit_hs_rx_physts_done_int_en(&self) -> BitHsRxPhystsDoneIntEnR {
        BitHsRxPhystsDoneIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_a_early_int_en(&self) -> BitHsPsTimerAEarlyIntEnR {
        BitHsPsTimerAEarlyIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_b_early_int_en(&self) -> BitHsPsTimerBEarlyIntEnR {
        BitHsPsTimerBEarlyIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_c_early_int_en(&self) -> BitHsPsTimerCEarlyIntEnR {
        BitHsPsTimerCEarlyIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_d_early_int_en(&self) -> BitHsPsTimerDEarlyIntEnR {
        BitHsPsTimerDEarlyIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_e_early_int_en(&self) -> BitHsPsTimerEEarlyIntEnR {
        BitHsPsTimerEEarlyIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_d_int_en(&self) -> BitHsPsTimerDIntEnR {
        BitHsPsTimerDIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_e_int_en(&self) -> BitHsPsTimerEIntEnR {
        BitHsPsTimerEIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - port2 rxbcn rpt and write to txrptbuf ok"]
    #[inline(always)]
    pub fn bit_hs_rxbcn_nan_rpt_ok_en(&self) -> BitHsRxbcnNanRptOkEnR {
        BitHsRxbcnNanRptOkEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - discovery window end interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_dwwin_end_int_en(&self) -> BitHsDwwinEndIntEnR {
        BitHsDwwinEndIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - event queue start interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_evtq_start_int_en(&self) -> BitHsEvtqStartIntEnR {
        BitHsEvtqStartIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - event queue done interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_evtq_txdone_int_en(&self) -> BitHsEvtqTxdoneIntEnR {
        BitHsEvtqTxdoneIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PORT2 TX DISC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnerr_port2_int_en(&self) -> BitHsTxbcnerrPort2IntEnR {
        BitHsTxbcnerrPort2IntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PORT2 TX SYNC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnerr_port2_sync_int_en(&self) -> BitHsTxbcnerrPort2SyncIntEnR {
        BitHsTxbcnerrPort2SyncIntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PORT2 TX DISC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnok_port2_int_en(&self) -> BitHsTxbcnokPort2IntEnR {
        BitHsTxbcnokPort2IntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PORT2 TX SYNC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnok_port2_sync_int_en(&self) -> BitHsTxbcnokPort2SyncIntEnR {
        BitHsTxbcnokPort2SyncIntEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma2_int_en(&self) -> BitHsBcndma2IntEnR {
        BitHsBcndma2IntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma_p2_2g_int_en(&self) -> BitHsBcndmaP2_2gIntEnR {
        BitHsBcndmaP2_2gIntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma_p2_5g_int_en(&self) -> BitHsBcndmaP2_5gIntEnR {
        BitHsBcndmaP2_5gIntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - the interrupt from the exact TBTT time for P2."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_disc_int_en(&self) -> BitHsTbtt2DiscIntEnR {
        BitHsTbtt2DiscIntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - the interrupt from the exact TBTT time for P2 SYNC_2G."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_sync_2g_int_en(&self) -> BitHsTbtt2Sync2gIntEnR {
        BitHsTbtt2Sync2gIntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the interrupt from the exact TBTT time for P2 SYNC_5G."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_sync_5g_int_en(&self) -> BitHsTbtt2Sync5gIntEnR {
        BitHsTbtt2Sync5gIntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_disc_int_en(&self) -> BitHsBcnerly2DiscIntEnR {
        BitHsBcnerly2DiscIntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_sync_2g_int_en(&self) -> BitHsBcnerly2Sync2gIntEnR {
        BitHsBcnerly2Sync2gIntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_sync_5g_int_en(&self) -> BitHsBcnerly2Sync5gIntEnR {
        BitHsBcnerly2Sync5gIntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The gathering of interrupts of KM0"]
    #[inline(always)]
    pub fn bit_km0_int_en(&self) -> BitKm0IntEnR {
        BitKm0IntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - when no my aid or aid is 0(0x16DC\\[8\\]) or my bssid is not in tim element(0x16DB\\[31\\]),generate this interrupt"]
    #[inline(always)]
    pub fn bit_hs_tim_no_hit_int_en(&mut self) -> BitHsTimNoHitIntEnW<'_, RegHimr2Spec> {
        BitHsTimNoHitIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - rx bcn with ie1 hit"]
    #[inline(always)]
    pub fn bit_hs_bcn_ie_1_hit_int_en(&mut self) -> BitHsBcnIe1HitIntEnW<'_, RegHimr2Spec> {
        BitHsBcnIe1HitIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - rx bcn with ie0 hit"]
    #[inline(always)]
    pub fn bit_hs_bcn_ie_0_hit_int_en(&mut self) -> BitHsBcnIe0HitIntEnW<'_, RegHimr2Spec> {
        BitHsBcnIe0HitIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - RX broadcast twt with parameter changing,notify fw to read twt report"]
    #[inline(always)]
    pub fn bit_hs_bc_twt_para_chg_int_en(&mut self) -> BitHsBcTwtParaChgIntEnW<'_, RegHimr2Spec> {
        BitHsBcTwtParaChgIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - enable interrupt when receiving dfs pkt and about to reach threshold of given pkt number or timeout"]
    #[inline(always)]
    pub fn bit_hs_dfs_pkt_int_en(&mut self) -> BitHsDfsPktIntEnW<'_, RegHimr2Spec> {
        BitHsDfsPktIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - when my aid or aid is 0 (0x16DC\\[8\\]) or my bssid is in tim element,generate this interrupt"]
    #[inline(always)]
    pub fn bit_hs_tim_hit_int_en(&mut self) -> BitHsTimHitIntEnW<'_, RegHimr2Spec> {
        BitHsTimHitIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - indicate physts is successfully write into physts_mem"]
    #[inline(always)]
    pub fn bit_hs_rx_physts_done_int_en(&mut self) -> BitHsRxPhystsDoneIntEnW<'_, RegHimr2Spec> {
        BitHsRxPhystsDoneIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_a_early_int_en(&mut self) -> BitHsPsTimerAEarlyIntEnW<'_, RegHimr2Spec> {
        BitHsPsTimerAEarlyIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_b_early_int_en(&mut self) -> BitHsPsTimerBEarlyIntEnW<'_, RegHimr2Spec> {
        BitHsPsTimerBEarlyIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_c_early_int_en(&mut self) -> BitHsPsTimerCEarlyIntEnW<'_, RegHimr2Spec> {
        BitHsPsTimerCEarlyIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_d_early_int_en(&mut self) -> BitHsPsTimerDEarlyIntEnW<'_, RegHimr2Spec> {
        BitHsPsTimerDEarlyIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
    #[inline(always)]
    pub fn bit_hs_ps_timer_e_early_int_en(&mut self) -> BitHsPsTimerEEarlyIntEnW<'_, RegHimr2Spec> {
        BitHsPsTimerEEarlyIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_d_int_en(&mut self) -> BitHsPsTimerDIntEnW<'_, RegHimr2Spec> {
        BitHsPsTimerDIntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_e_int_en(&mut self) -> BitHsPsTimerEIntEnW<'_, RegHimr2Spec> {
        BitHsPsTimerEIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - port2 rxbcn rpt and write to txrptbuf ok"]
    #[inline(always)]
    pub fn bit_hs_rxbcn_nan_rpt_ok_en(&mut self) -> BitHsRxbcnNanRptOkEnW<'_, RegHimr2Spec> {
        BitHsRxbcnNanRptOkEnW::new(self, 14)
    }
    #[doc = "Bit 15 - discovery window end interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_dwwin_end_int_en(&mut self) -> BitHsDwwinEndIntEnW<'_, RegHimr2Spec> {
        BitHsDwwinEndIntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - event queue start interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_evtq_start_int_en(&mut self) -> BitHsEvtqStartIntEnW<'_, RegHimr2Spec> {
        BitHsEvtqStartIntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - event queue done interrupt for NAN"]
    #[inline(always)]
    pub fn bit_hs_evtq_txdone_int_en(&mut self) -> BitHsEvtqTxdoneIntEnW<'_, RegHimr2Spec> {
        BitHsEvtqTxdoneIntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - PORT2 TX DISC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnerr_port2_int_en(&mut self) -> BitHsTxbcnerrPort2IntEnW<'_, RegHimr2Spec> {
        BitHsTxbcnerrPort2IntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - PORT2 TX SYNC beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnerr_port2_sync_int_en(
        &mut self,
    ) -> BitHsTxbcnerrPort2SyncIntEnW<'_, RegHimr2Spec> {
        BitHsTxbcnerrPort2SyncIntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - PORT2 TX DISC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnok_port2_int_en(&mut self) -> BitHsTxbcnokPort2IntEnW<'_, RegHimr2Spec> {
        BitHsTxbcnokPort2IntEnW::new(self, 20)
    }
    #[doc = "Bit 21 - PORT2 TX SYNC beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnok_port2_sync_int_en(
        &mut self,
    ) -> BitHsTxbcnokPort2SyncIntEnW<'_, RegHimr2Spec> {
        BitHsTxbcnokPort2SyncIntEnW::new(self, 21)
    }
    #[doc = "Bit 22 - When BCNDMA interval arrives before TBTT2_DISC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma2_int_en(&mut self) -> BitHsBcndma2IntEnW<'_, RegHimr2Spec> {
        BitHsBcndma2IntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - When BCNDMA interval arrives before TBTT2_2G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma_p2_2g_int_en(&mut self) -> BitHsBcndmaP2_2gIntEnW<'_, RegHimr2Spec> {
        BitHsBcndmaP2_2gIntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - When BCNDMA interval arrives before TBTT2_5G_SYNC, this interrupt informs MCU to prepare related beacon tasks."]
    #[inline(always)]
    pub fn bit_hs_bcndma_p2_5g_int_en(&mut self) -> BitHsBcndmaP2_5gIntEnW<'_, RegHimr2Spec> {
        BitHsBcndmaP2_5gIntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - the interrupt from the exact TBTT time for P2."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_disc_int_en(&mut self) -> BitHsTbtt2DiscIntEnW<'_, RegHimr2Spec> {
        BitHsTbtt2DiscIntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - the interrupt from the exact TBTT time for P2 SYNC_2G."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_sync_2g_int_en(&mut self) -> BitHsTbtt2Sync2gIntEnW<'_, RegHimr2Spec> {
        BitHsTbtt2Sync2gIntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - the interrupt from the exact TBTT time for P2 SYNC_5G."]
    #[inline(always)]
    pub fn bit_hs_tbtt2_sync_5g_int_en(&mut self) -> BitHsTbtt2Sync5gIntEnW<'_, RegHimr2Spec> {
        BitHsTbtt2Sync5gIntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 DISC time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_disc_int_en(&mut self) -> BitHsBcnerly2DiscIntEnW<'_, RegHimr2Spec> {
        BitHsBcnerly2DiscIntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 2.4G time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_sync_2g_int_en(
        &mut self,
    ) -> BitHsBcnerly2Sync2gIntEnW<'_, RegHimr2Spec> {
        BitHsBcnerly2Sync2gIntEnW::new(self, 29)
    }
    #[doc = "Bit 30 - This interrupt is issued at the time set by DRVERLYINT register before TBTT2 SYNC 5G time."]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_sync_5g_int_en(
        &mut self,
    ) -> BitHsBcnerly2Sync5gIntEnW<'_, RegHimr2Spec> {
        BitHsBcnerly2Sync5gIntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - The gathering of interrupts of KM0"]
    #[inline(always)]
    pub fn bit_km0_int_en(&mut self) -> BitKm0IntEnW<'_, RegHimr2Spec> {
        BitKm0IntEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_himr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_himr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHimr2Spec;
impl crate::RegisterSpec for RegHimr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_himr2::R`](R) reader structure"]
impl crate::Readable for RegHimr2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_himr2::W`](W) writer structure"]
impl crate::Writable for RegHimr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HIMR2 to value 0"]
impl crate::Resettable for RegHimr2Spec {}
