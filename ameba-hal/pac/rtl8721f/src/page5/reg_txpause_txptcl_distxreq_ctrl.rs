#[doc = "Register `REG_TXPAUSE_TXPTCL_DISTXREQ_CTRL` reader"]
pub type R = crate::R<RegTxpauseTxptclDistxreqCtrlSpec>;
#[doc = "Register `REG_TXPAUSE_TXPTCL_DISTXREQ_CTRL` writer"]
pub type W = crate::W<RegTxpauseTxptclDistxreqCtrlSpec>;
#[doc = "Field `BIT_BCN_RTSMF_EN` reader - When this bit is set, BCNQ will be TX, not wait until RTS or MF transaction finished."]
pub type BitBcnRtsmfEnR = crate::BitReader;
#[doc = "Field `BIT_BCN_RTSMF_EN` writer - When this bit is set, BCNQ will be TX, not wait until RTS or MF transaction finished."]
pub type BitBcnRtsmfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIQ_RTSMF_EN` reader - When this bit is set, HIQ will be TX, not wait until RTS or MF transaction finished."]
pub type BitHiqRtsmfEnR = crate::BitReader;
#[doc = "Field `BIT_HIQ_RTSMF_EN` writer - When this bit is set, HIQ will be TX, not wait until RTS or MF transaction finished."]
pub type BitHiqRtsmfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MGTQ_RTSMF_EN` reader - When this bit is set, MTGQ will be TX, not wait until RTS or MF transaction finished."]
pub type BitMgtqRtsmfEnR = crate::BitReader;
#[doc = "Field `BIT_MGTQ_RTSMF_EN` writer - When this bit is set, MTGQ will be TX, not wait until RTS or MF transaction finished."]
pub type BitMgtqRtsmfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MGTQ_TXOP_EN` reader - When this bit is set, MGTQ may sent packets by using ACQ TXOP."]
pub type BitMgtqTxopEnR = crate::BitReader;
#[doc = "Field `BIT_MGTQ_TXOP_EN` writer - When this bit is set, MGTQ may sent packets by using ACQ TXOP."]
pub type BitMgtqTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RDG_END_TXOP` reader - 1: In tx RDG mode, Nav protect all txop 0: In tx RDG mode, Nav protect to next pkt"]
pub type BitRdgEndTxopR = crate::BitReader;
#[doc = "Field `BIT_RDG_END_TXOP` writer - 1: In tx RDG mode, Nav protect all txop 0: In tx RDG mode, Nav protect to next pkt"]
pub type BitRdgEndTxopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_NAV_END_TXOP` reader - 1: In txop mode, Nav protect all txop 0: In txop mode, Nav protect to next pkt"]
pub type BitNavEndTxopR = crate::BitReader;
#[doc = "Field `BIT_NAV_END_TXOP` writer - 1: In txop mode, Nav protect all txop 0: In txop mode, Nav protect to next pkt"]
pub type BitNavEndTxopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_CW` reader - When this bit is set, disable CW."]
pub type BitDisCwR = crate::BitReader;
#[doc = "Field `BIT_DIS_CW` writer - When this bit is set, disable CW."]
pub type BitDisCwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXQ_NAV_MSK` reader - 1: Queue BKF must wait TX Nav finished 0: Queue not need wait"]
pub type BitTxqNavMskR = crate::FieldReader;
#[doc = "Field `BIT_TXQ_NAV_MSK` writer - 1: Queue BKF must wait TX Nav finished 0: Queue not need wait"]
pub type BitTxqNavMskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_SIFS_BK_EN` reader - SIFS Break Check Criteria enable for AMPDU aggregation"]
pub type BitSifsBkEnR = crate::BitReader;
#[doc = "Field `BIT_SIFS_BK_EN` writer - SIFS Break Check Criteria enable for AMPDU aggregation"]
pub type BitSifsBkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LSIG_TXOP_TXCMD_NAV` reader - Set LSIG_TXOP from TXCMD_NAV field"]
pub type BitLsigTxopTxcmdNavR = crate::BitReader;
#[doc = "Field `BIT_LSIG_TXOP_TXCMD_NAV` writer - Set LSIG_TXOP from TXCMD_NAV field"]
pub type BitLsigTxopTxcmdNavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_STOPVO` reader - Stop VO Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopvoR = crate::BitReader;
#[doc = "Field `BIT_MAC_STOPVO` writer - Stop VO Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopvoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_STOPVI` reader - Stop VI Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopviR = crate::BitReader;
#[doc = "Field `BIT_MAC_STOPVI` writer - Stop VI Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopviW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_STOPBE` reader - Stop BE Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopbeR = crate::BitReader;
#[doc = "Field `BIT_MAC_STOPBE` writer - Stop BE Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_STOPBK` reader - Stop BK Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopbkR = crate::BitReader;
#[doc = "Field `BIT_MAC_STOPBK` writer - Stop BK Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_STOPMGQ` reader - Stop Management Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopmgqR = crate::BitReader;
#[doc = "Field `BIT_MAC_STOPMGQ` writer - Stop Management Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopmgqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_STOPHIQ` reader - Stop High Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStophiqR = crate::BitReader;
#[doc = "Field `BIT_MAC_STOPHIQ` writer - Stop High Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStophiqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MAC_STOPBCNQ` reader - Stop BCNQ Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopbcnqR = crate::BitReader;
#[doc = "Field `BIT_MAC_STOPBCNQ` writer - Stop BCNQ Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
pub type BitMacStopbcnqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STOP_CPUMGQ` reader - To control CPUMGQ queues contention circuit. 1: CPUMGQ queues all stop. 0: Enable CPUMGQ queues contention circuit"]
pub type BitStopCpumgqR = crate::BitReader;
#[doc = "Field `BIT_STOP_CPUMGQ` writer - To control CPUMGQ queues contention circuit. 1: CPUMGQ queues all stop. 0: Enable CPUMGQ queues contention circuit"]
pub type BitStopCpumgqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXREQ_CLR_VO` reader - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrVoR = crate::BitReader;
#[doc = "Field `BIT_DIS_TXREQ_CLR_VO` writer - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrVoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXREQ_CLR_VI` reader - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrViR = crate::BitReader;
#[doc = "Field `BIT_DIS_TXREQ_CLR_VI` writer - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrViW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXREQ_CLR_BE` reader - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrBeR = crate::BitReader;
#[doc = "Field `BIT_DIS_TXREQ_CLR_BE` writer - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrBeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXREQ_CLR_BK` reader - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrBkR = crate::BitReader;
#[doc = "Field `BIT_DIS_TXREQ_CLR_BK` writer - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrBkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXREQ_CLR_MGQ` reader - 1: if this queue can't own channel ,txcmd will be cleared 0:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrMgqR = crate::BitReader;
#[doc = "Field `BIT_DIS_TXREQ_CLR_MGQ` writer - 1: if this queue can't own channel ,txcmd will be cleared 0:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrMgqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXREQ_CLR_HI` reader - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrHiR = crate::BitReader;
#[doc = "Field `BIT_DIS_TXREQ_CLR_HI` writer - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrHiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_TXREQ_CLR_CPUMGQ` reader - 1: if this queue can't own channel ,txcmd will be cleared 0:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrCpumgqR = crate::BitReader;
#[doc = "Field `BIT_DIS_TXREQ_CLR_CPUMGQ` writer - 1: if this queue can't own channel ,txcmd will be cleared 0:Only TX_finish can clear txreq"]
pub type BitDisTxreqClrCpumgqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_BT_CCA` reader - Disable BT_CCA. (NOT USED, SEE 0x1594)"]
pub type BitDisBtCcaR = crate::BitReader;
#[doc = "Field `BIT_DIS_BT_CCA` writer - Disable BT_CCA. (NOT USED, SEE 0x1594)"]
pub type BitDisBtCcaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When this bit is set, BCNQ will be TX, not wait until RTS or MF transaction finished."]
    #[inline(always)]
    pub fn bit_bcn_rtsmf_en(&self) -> BitBcnRtsmfEnR {
        BitBcnRtsmfEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is set, HIQ will be TX, not wait until RTS or MF transaction finished."]
    #[inline(always)]
    pub fn bit_hiq_rtsmf_en(&self) -> BitHiqRtsmfEnR {
        BitHiqRtsmfEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set, MTGQ will be TX, not wait until RTS or MF transaction finished."]
    #[inline(always)]
    pub fn bit_mgtq_rtsmf_en(&self) -> BitMgtqRtsmfEnR {
        BitMgtqRtsmfEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set, MGTQ may sent packets by using ACQ TXOP."]
    #[inline(always)]
    pub fn bit_mgtq_txop_en(&self) -> BitMgtqTxopEnR {
        BitMgtqTxopEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: In tx RDG mode, Nav protect all txop 0: In tx RDG mode, Nav protect to next pkt"]
    #[inline(always)]
    pub fn bit_rdg_end_txop(&self) -> BitRdgEndTxopR {
        BitRdgEndTxopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: In txop mode, Nav protect all txop 0: In txop mode, Nav protect to next pkt"]
    #[inline(always)]
    pub fn bit_nav_end_txop(&self) -> BitNavEndTxopR {
        BitNavEndTxopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit is set, disable CW."]
    #[inline(always)]
    pub fn bit_dis_cw(&self) -> BitDisCwR {
        BitDisCwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 1: Queue BKF must wait TX Nav finished 0: Queue not need wait"]
    #[inline(always)]
    pub fn bit_txq_nav_msk(&self) -> BitTxqNavMskR {
        BitTxqNavMskR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - SIFS Break Check Criteria enable for AMPDU aggregation"]
    #[inline(always)]
    pub fn bit_sifs_bk_en(&self) -> BitSifsBkEnR {
        BitSifsBkEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set LSIG_TXOP from TXCMD_NAV field"]
    #[inline(always)]
    pub fn bit_lsig_txop_txcmd_nav(&self) -> BitLsigTxopTxcmdNavR {
        BitLsigTxopTxcmdNavR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Stop VO Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopvo(&self) -> BitMacStopvoR {
        BitMacStopvoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop VI Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopvi(&self) -> BitMacStopviR {
        BitMacStopviR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Stop BE Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopbe(&self) -> BitMacStopbeR {
        BitMacStopbeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stop BK Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopbk(&self) -> BitMacStopbkR {
        BitMacStopbkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stop Management Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopmgq(&self) -> BitMacStopmgqR {
        BitMacStopmgqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stop High Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stophiq(&self) -> BitMacStophiqR {
        BitMacStophiqR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stop BCNQ Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopbcnq(&self) -> BitMacStopbcnqR {
        BitMacStopbcnqR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - To control CPUMGQ queues contention circuit. 1: CPUMGQ queues all stop. 0: Enable CPUMGQ queues contention circuit"]
    #[inline(always)]
    pub fn bit_stop_cpumgq(&self) -> BitStopCpumgqR {
        BitStopCpumgqR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_vo(&self) -> BitDisTxreqClrVoR {
        BitDisTxreqClrVoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_vi(&self) -> BitDisTxreqClrViR {
        BitDisTxreqClrViR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_be(&self) -> BitDisTxreqClrBeR {
        BitDisTxreqClrBeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_bk(&self) -> BitDisTxreqClrBkR {
        BitDisTxreqClrBkR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: if this queue can't own channel ,txcmd will be cleared 0:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_mgq(&self) -> BitDisTxreqClrMgqR {
        BitDisTxreqClrMgqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_hi(&self) -> BitDisTxreqClrHiR {
        BitDisTxreqClrHiR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: if this queue can't own channel ,txcmd will be cleared 0:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_cpumgq(&self) -> BitDisTxreqClrCpumgqR {
        BitDisTxreqClrCpumgqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable BT_CCA. (NOT USED, SEE 0x1594)"]
    #[inline(always)]
    pub fn bit_dis_bt_cca(&self) -> BitDisBtCcaR {
        BitDisBtCcaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is set, BCNQ will be TX, not wait until RTS or MF transaction finished."]
    #[inline(always)]
    pub fn bit_bcn_rtsmf_en(&mut self) -> BitBcnRtsmfEnW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitBcnRtsmfEnW::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit is set, HIQ will be TX, not wait until RTS or MF transaction finished."]
    #[inline(always)]
    pub fn bit_hiq_rtsmf_en(&mut self) -> BitHiqRtsmfEnW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitHiqRtsmfEnW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit is set, MTGQ will be TX, not wait until RTS or MF transaction finished."]
    #[inline(always)]
    pub fn bit_mgtq_rtsmf_en(&mut self) -> BitMgtqRtsmfEnW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMgtqRtsmfEnW::new(self, 2)
    }
    #[doc = "Bit 3 - When this bit is set, MGTQ may sent packets by using ACQ TXOP."]
    #[inline(always)]
    pub fn bit_mgtq_txop_en(&mut self) -> BitMgtqTxopEnW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMgtqTxopEnW::new(self, 3)
    }
    #[doc = "Bit 5 - 1: In tx RDG mode, Nav protect all txop 0: In tx RDG mode, Nav protect to next pkt"]
    #[inline(always)]
    pub fn bit_rdg_end_txop(&mut self) -> BitRdgEndTxopW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitRdgEndTxopW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: In txop mode, Nav protect all txop 0: In txop mode, Nav protect to next pkt"]
    #[inline(always)]
    pub fn bit_nav_end_txop(&mut self) -> BitNavEndTxopW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitNavEndTxopW::new(self, 6)
    }
    #[doc = "Bit 7 - When this bit is set, disable CW."]
    #[inline(always)]
    pub fn bit_dis_cw(&mut self) -> BitDisCwW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisCwW::new(self, 7)
    }
    #[doc = "Bits 8:11 - 1: Queue BKF must wait TX Nav finished 0: Queue not need wait"]
    #[inline(always)]
    pub fn bit_txq_nav_msk(&mut self) -> BitTxqNavMskW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitTxqNavMskW::new(self, 8)
    }
    #[doc = "Bit 12 - SIFS Break Check Criteria enable for AMPDU aggregation"]
    #[inline(always)]
    pub fn bit_sifs_bk_en(&mut self) -> BitSifsBkEnW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitSifsBkEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Set LSIG_TXOP from TXCMD_NAV field"]
    #[inline(always)]
    pub fn bit_lsig_txop_txcmd_nav(
        &mut self,
    ) -> BitLsigTxopTxcmdNavW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitLsigTxopTxcmdNavW::new(self, 13)
    }
    #[doc = "Bit 16 - Stop VO Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopvo(&mut self) -> BitMacStopvoW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMacStopvoW::new(self, 16)
    }
    #[doc = "Bit 17 - Stop VI Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopvi(&mut self) -> BitMacStopviW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMacStopviW::new(self, 17)
    }
    #[doc = "Bit 18 - Stop BE Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopbe(&mut self) -> BitMacStopbeW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMacStopbeW::new(self, 18)
    }
    #[doc = "Bit 19 - Stop BK Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopbk(&mut self) -> BitMacStopbkW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMacStopbkW::new(self, 19)
    }
    #[doc = "Bit 20 - Stop Management Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopmgq(&mut self) -> BitMacStopmgqW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMacStopmgqW::new(self, 20)
    }
    #[doc = "Bit 21 - Stop High Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stophiq(&mut self) -> BitMacStophiqW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMacStophiqW::new(self, 21)
    }
    #[doc = "Bit 22 - Stop BCNQ Queue Write a 1 to this bit via software to immediately stop the transmission of packet from this queue including retry attempts."]
    #[inline(always)]
    pub fn bit_mac_stopbcnq(&mut self) -> BitMacStopbcnqW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitMacStopbcnqW::new(self, 22)
    }
    #[doc = "Bit 23 - To control CPUMGQ queues contention circuit. 1: CPUMGQ queues all stop. 0: Enable CPUMGQ queues contention circuit"]
    #[inline(always)]
    pub fn bit_stop_cpumgq(&mut self) -> BitStopCpumgqW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitStopCpumgqW::new(self, 23)
    }
    #[doc = "Bit 24 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_vo(
        &mut self,
    ) -> BitDisTxreqClrVoW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisTxreqClrVoW::new(self, 24)
    }
    #[doc = "Bit 25 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_vi(
        &mut self,
    ) -> BitDisTxreqClrViW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisTxreqClrViW::new(self, 25)
    }
    #[doc = "Bit 26 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_be(
        &mut self,
    ) -> BitDisTxreqClrBeW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisTxreqClrBeW::new(self, 26)
    }
    #[doc = "Bit 27 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_bk(
        &mut self,
    ) -> BitDisTxreqClrBkW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisTxreqClrBkW::new(self, 27)
    }
    #[doc = "Bit 28 - 1: if this queue can't own channel ,txcmd will be cleared 0:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_mgq(
        &mut self,
    ) -> BitDisTxreqClrMgqW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisTxreqClrMgqW::new(self, 28)
    }
    #[doc = "Bit 29 - 0: if this queue can't own channel ,txcmd will be cleared 1:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_hi(
        &mut self,
    ) -> BitDisTxreqClrHiW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisTxreqClrHiW::new(self, 29)
    }
    #[doc = "Bit 30 - 1: if this queue can't own channel ,txcmd will be cleared 0:Only TX_finish can clear txreq"]
    #[inline(always)]
    pub fn bit_dis_txreq_clr_cpumgq(
        &mut self,
    ) -> BitDisTxreqClrCpumgqW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisTxreqClrCpumgqW::new(self, 30)
    }
    #[doc = "Bit 31 - Disable BT_CCA. (NOT USED, SEE 0x1594)"]
    #[inline(always)]
    pub fn bit_dis_bt_cca(&mut self) -> BitDisBtCcaW<'_, RegTxpauseTxptclDistxreqCtrlSpec> {
        BitDisBtCcaW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txpause_txptcl_distxreq_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txpause_txptcl_distxreq_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxpauseTxptclDistxreqCtrlSpec;
impl crate::RegisterSpec for RegTxpauseTxptclDistxreqCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txpause_txptcl_distxreq_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTxpauseTxptclDistxreqCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txpause_txptcl_distxreq_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTxpauseTxptclDistxreqCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXPAUSE_TXPTCL_DISTXREQ_CTRL to value 0x3f09"]
impl crate::Resettable for RegTxpauseTxptclDistxreqCtrlSpec {
    const RESET_VALUE: u32 = 0x3f09;
}
