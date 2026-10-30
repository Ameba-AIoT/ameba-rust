#[doc = "Register `REG_HISR1` reader"]
pub type R = crate::R<RegHisr1Spec>;
#[doc = "Field `BIT_HS_TXBCNERR9_INT` reader - PORT1 TX beacon Error interrupt"]
pub type BitHsTxbcnerr9IntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNOK9_INT` reader - PORT1 TX beacon OK interrupt"]
pub type BitHsTxbcnok9IntR = crate::BitReader;
#[doc = "Field `BIT_HS_GTINT3` reader - When GTIMER3 expires, this bit is set to 1"]
pub type BitHsGtint3R = crate::BitReader;
#[doc = "Field `BIT_HS_GTINT4` reader - When GTIMER4 expires, this bit is set to 1"]
pub type BitHsGtint4R = crate::BitReader;
#[doc = "Field `BIT_HS_BCNELY0_AGGR_INT` reader - port 0 Beacon early aggregation interrupt"]
pub type BitHsBcnely0AggrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_A_INT` reader - PS_Timer_A interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
pub type BitHsPsTimerAIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_B_INT` reader - PS_Timer_B interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
pub type BitHsPsTimerBIntR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_C_INT` reader - PS_Timer_C interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
pub type BitHsPsTimerCIntR = crate::BitReader;
#[doc = "Field `BIT_HS_CPU_MGQ_TXDONE_INT` reader - CPU_MQG_TXDONE ineterrupt source when HW TX CPUMGQ packet success, it issue interrupt to host"]
pub type BitHsCpuMgqTxdoneIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNELY1_AGGR_INT` reader - port 1 Beacon early aggregation interrupt"]
pub type BitHsBcnely1AggrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY0_INT` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT time for port 0 root bssid"]
pub type BitHsBcnerly0IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY1_INT` reader - port0 BSSID1 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly1IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY2_INT` reader - port0 BSSID2 beacon early interrupt, only valid under MBSSID mode"]
pub type BitHsBcnerly2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY3_INT` reader - port0 BSSID3 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly3IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY4_INT` reader - port0 BSSID4 beacon early interrupt, only valid under MBSSID mode"]
pub type BitHsBcnerly4IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY5_INT` reader - port0 BSSID5 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly5IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY6_INT` reader - port0 BSSID6 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly6IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY7_INT` reader - port0 BSSID7 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly7IntR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY8_INT` reader - port1 beacon early interrupt"]
pub type BitHsBcnerly8IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND0_INT` reader - port0 BSSID0 atimend interrupt"]
pub type BitHsAtimend0IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND1_INT` reader - port0 BSSID1 atimend interrupt"]
pub type BitHsAtimend1IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND2_INT` reader - port0 BSSID2 atimend interrupt"]
pub type BitHsAtimend2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND3_INT` reader - port0 BSSID3 atimend interrupt"]
pub type BitHsAtimend3IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND4_INT` reader - port0 BSSID4 atimend interrupt"]
pub type BitHsAtimend4IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND5_INT` reader - port0 BSSID5 atimend interrupt"]
pub type BitHsAtimend5IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND6_INT` reader - port0 BSSID6 atimend interrupt"]
pub type BitHsAtimend6IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND7_INT` reader - port0 BSSID7 atimend interrupt"]
pub type BitHsAtimend7IntR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND8_INT` reader - port1 atimend interrupt"]
pub type BitHsAtimend8IntR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWE_INT0` reader - port0 CTWindow end interrupt for p2p."]
pub type BitHsCtweInt0R = crate::BitReader;
#[doc = "Field `BIT_HS_CTWE_INT1` reader - port1 CTWindow end interrupt for p2p."]
pub type BitHsCtweInt1R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - PORT1 TX beacon Error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnerr9_int(&self) -> BitHsTxbcnerr9IntR {
        BitHsTxbcnerr9IntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PORT1 TX beacon OK interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcnok9_int(&self) -> BitHsTxbcnok9IntR {
        BitHsTxbcnok9IntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When GTIMER3 expires, this bit is set to 1"]
    #[inline(always)]
    pub fn bit_hs_gtint3(&self) -> BitHsGtint3R {
        BitHsGtint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When GTIMER4 expires, this bit is set to 1"]
    #[inline(always)]
    pub fn bit_hs_gtint4(&self) -> BitHsGtint4R {
        BitHsGtint4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - port 0 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnely0_aggr_int(&self) -> BitHsBcnely0AggrIntR {
        BitHsBcnely0AggrIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PS_Timer_A interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_a_int(&self) -> BitHsPsTimerAIntR {
        BitHsPsTimerAIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PS_Timer_B interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_b_int(&self) -> BitHsPsTimerBIntR {
        BitHsPsTimerBIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PS_Timer_C interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_c_int(&self) -> BitHsPsTimerCIntR {
        BitHsPsTimerCIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU_MQG_TXDONE ineterrupt source when HW TX CPUMGQ packet success, it issue interrupt to host"]
    #[inline(always)]
    pub fn bit_hs_cpu_mgq_txdone_int(&self) -> BitHsCpuMgqTxdoneIntR {
        BitHsCpuMgqTxdoneIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - port 1 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnely1_aggr_int(&self) -> BitHsBcnely1AggrIntR {
        BitHsBcnely1AggrIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This interrupt is issued at the time set by DRVERLYINT register before TBTT time for port 0 root bssid"]
    #[inline(always)]
    pub fn bit_hs_bcnerly0_int(&self) -> BitHsBcnerly0IntR {
        BitHsBcnerly0IntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - port0 BSSID1 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly1_int(&self) -> BitHsBcnerly1IntR {
        BitHsBcnerly1IntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - port0 BSSID2 beacon early interrupt, only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_int(&self) -> BitHsBcnerly2IntR {
        BitHsBcnerly2IntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - port0 BSSID3 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly3_int(&self) -> BitHsBcnerly3IntR {
        BitHsBcnerly3IntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - port0 BSSID4 beacon early interrupt, only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly4_int(&self) -> BitHsBcnerly4IntR {
        BitHsBcnerly4IntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - port0 BSSID5 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly5_int(&self) -> BitHsBcnerly5IntR {
        BitHsBcnerly5IntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - port0 BSSID6 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly6_int(&self) -> BitHsBcnerly6IntR {
        BitHsBcnerly6IntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - port0 BSSID7 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly7_int(&self) -> BitHsBcnerly7IntR {
        BitHsBcnerly7IntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - port1 beacon early interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnerly8_int(&self) -> BitHsBcnerly8IntR {
        BitHsBcnerly8IntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - port0 BSSID0 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend0_int(&self) -> BitHsAtimend0IntR {
        BitHsAtimend0IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - port0 BSSID1 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend1_int(&self) -> BitHsAtimend1IntR {
        BitHsAtimend1IntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - port0 BSSID2 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend2_int(&self) -> BitHsAtimend2IntR {
        BitHsAtimend2IntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - port0 BSSID3 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend3_int(&self) -> BitHsAtimend3IntR {
        BitHsAtimend3IntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - port0 BSSID4 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend4_int(&self) -> BitHsAtimend4IntR {
        BitHsAtimend4IntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - port0 BSSID5 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend5_int(&self) -> BitHsAtimend5IntR {
        BitHsAtimend5IntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - port0 BSSID6 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend6_int(&self) -> BitHsAtimend6IntR {
        BitHsAtimend6IntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - port0 BSSID7 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend7_int(&self) -> BitHsAtimend7IntR {
        BitHsAtimend7IntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - port1 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend8_int(&self) -> BitHsAtimend8IntR {
        BitHsAtimend8IntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - port0 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_hs_ctwe_int0(&self) -> BitHsCtweInt0R {
        BitHsCtweInt0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - port1 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_hs_ctwe_int1(&self) -> BitHsCtweInt1R {
        BitHsCtweInt1R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hisr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHisr1Spec;
impl crate::RegisterSpec for RegHisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hisr1::R`](R) reader structure"]
impl crate::Readable for RegHisr1Spec {}
#[doc = "`reset()` method sets REG_HISR1 to value 0"]
impl crate::Resettable for RegHisr1Spec {}
