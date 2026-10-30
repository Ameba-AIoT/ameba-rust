#[doc = "Register `REG_HIMR1` reader"]
pub type R = crate::R<RegHimr1Spec>;
#[doc = "Register `REG_HIMR1` writer"]
pub type W = crate::W<RegHimr1Spec>;
#[doc = "Field `BIT_HS_TXBCNERE9_INT_EN` reader - PORT1 TRX beacon Error interrupt (ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcnere9IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNERE9_INT_EN` writer - PORT1 TRX beacon Error interrupt (ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcnere9IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCNOK9_INT_EN` reader - PORT1 TRX beacon OK interrupt (ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcnok9IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCNOK9_INT_EN` writer - PORT1 TRX beacon OK interrupt (ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcnok9IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_GTINT3_EN` reader - When GTIMER3 expires, this bit is set to 1"]
pub type BitHsGtint3EnR = crate::BitReader;
#[doc = "Field `BIT_HS_GTINT3_EN` writer - When GTIMER3 expires, this bit is set to 1"]
pub type BitHsGtint3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_GTINT4_EN` reader - When GTIMER4 expires, this bit is set to 1"]
pub type BitHsGtint4EnR = crate::BitReader;
#[doc = "Field `BIT_HS_GTINT4_EN` writer - When GTIMER4 expires, this bit is set to 1"]
pub type BitHsGtint4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNELY0_AGGR_INT_EN` reader - port 0 Beacon early aggregation interrupt"]
pub type BitHsBcnely0AggrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNELY0_AGGR_INT_EN` writer - port 0 Beacon early aggregation interrupt"]
pub type BitHsBcnely0AggrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_A_INT_EN` reader - PS_Timer_A interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
pub type BitHsPsTimerAIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_A_INT_EN` writer - PS_Timer_A interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
pub type BitHsPsTimerAIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_B_INT_EN` reader - PS_Timer_B interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
pub type BitHsPsTimerBIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_B_INT_EN` writer - PS_Timer_B interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
pub type BitHsPsTimerBIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_PS_Timer_C_INT_EN` reader - PS_Timer_C interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
pub type BitHsPsTimerCIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_PS_Timer_C_INT_EN` writer - PS_Timer_C interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
pub type BitHsPsTimerCIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CPU_MGQ_TXDONE_INT_EN` reader - CPU_MQG_TXDONE ineterrupt source when HW TX CPUMGQ packet success, it issue interrupt to host"]
pub type BitHsCpuMgqTxdoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_CPU_MGQ_TXDONE_INT_EN` writer - CPU_MQG_TXDONE ineterrupt source when HW TX CPUMGQ packet success, it issue interrupt to host"]
pub type BitHsCpuMgqTxdoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNELY1_AGGR_INT_EN` reader - port 1 Beacon early aggregation interrupt"]
pub type BitHsBcnely1AggrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNELY1_AGGR_INT_EN` writer - port 1 Beacon early aggregation interrupt"]
pub type BitHsBcnely1AggrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY0_INT_EN` reader - This interrupt is issued at the time set by DRVERLYINT register before TBTT time for port 0 root bssid"]
pub type BitHsBcnerly0IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY0_INT_EN` writer - This interrupt is issued at the time set by DRVERLYINT register before TBTT time for port 0 root bssid"]
pub type BitHsBcnerly0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY1_INT_EN` reader - port0 BSSID1 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly1IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY1_INT_EN` writer - port0 BSSID1 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY2_INT_EN` reader - port0 BSSID2 beacon early interrupt, only valid under MBSSID mode"]
pub type BitHsBcnerly2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY2_INT_EN` writer - port0 BSSID2 beacon early interrupt, only valid under MBSSID mode"]
pub type BitHsBcnerly2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY3_INT_EN` reader - port0 BSSID3 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly3IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY3_INT_EN` writer - port0 BSSID3 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly3IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY4_INT_EN` reader - port0 BSSID4 beacon early interrupt, only valid under MBSSID mode"]
pub type BitHsBcnerly4IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY4_INT_EN` writer - port0 BSSID4 beacon early interrupt, only valid under MBSSID mode"]
pub type BitHsBcnerly4IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY5_INT_EN` reader - port0 BSSID5 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly5IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY5_INT_EN` writer - port0 BSSID5 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly5IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY6_INT_EN` reader - port0 BSSID6 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly6IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY6_INT_EN` writer - port0 BSSID6 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly6IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY7_INT_EN` reader - port0 BSSID7 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly7IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY7_INT_EN` writer - port0 BSSID7 beacon early interrupt , only valid under MBSSID mode"]
pub type BitHsBcnerly7IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_BCNERLY8_INT_EN` reader - port1 beacon early interrupt"]
pub type BitHsBcnerly8IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_BCNERLY8_INT_EN` writer - port1 beacon early interrupt"]
pub type BitHsBcnerly8IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND0_INT_EN` reader - port0 BSSID0 atimend interrupt"]
pub type BitHsAtimend0IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND0_INT_EN` writer - port0 BSSID0 atimend interrupt"]
pub type BitHsAtimend0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND1_INT_EN` reader - port0 BSSID1 atimend interrupt"]
pub type BitHsAtimend1IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND1_INT_EN` writer - port0 BSSID1 atimend interrupt"]
pub type BitHsAtimend1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND2_INT_EN` reader - port0 BSSID2 atimend interrupt"]
pub type BitHsAtimend2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND2_INT_EN` writer - port0 BSSID2 atimend interrupt"]
pub type BitHsAtimend2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND3_INT_EN` reader - port0 BSSID3 atimend interrupt"]
pub type BitHsAtimend3IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND3_INT_EN` writer - port0 BSSID3 atimend interrupt"]
pub type BitHsAtimend3IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND4_INT_EN` reader - port0 BSSID4 atimend interrupt"]
pub type BitHsAtimend4IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND4_INT_EN` writer - port0 BSSID4 atimend interrupt"]
pub type BitHsAtimend4IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND5_INT_EN` reader - port0 BSSID5 atimend interrupt"]
pub type BitHsAtimend5IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND5_INT_EN` writer - port0 BSSID5 atimend interrupt"]
pub type BitHsAtimend5IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND6_INT_EN` reader - port0 BSSID6 atimend interrupt"]
pub type BitHsAtimend6IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND6_INT_EN` writer - port0 BSSID6 atimend interrupt"]
pub type BitHsAtimend6IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND7_INT_EN` reader - port0 BSSID7 atimend interrupt"]
pub type BitHsAtimend7IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND7_INT_EN` writer - port0 BSSID7 atimend interrupt"]
pub type BitHsAtimend7IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_ATIMEND8_INT_EN` reader - port1 atimend interrupt"]
pub type BitHsAtimend8IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_ATIMEND8_INT_EN` writer - port1 atimend interrupt"]
pub type BitHsAtimend8IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CTWE_INT0_EN` reader - Client0 traffic window end interrupt for P2P"]
pub type BitHsCtweInt0EnR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWE_INT0_EN` writer - Client0 traffic window end interrupt for P2P"]
pub type BitHsCtweInt0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CTWE_INT1_EN` reader - Client1 traffic window end interrupt for P2P"]
pub type BitHsCtweInt1EnR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWE_INT1_EN` writer - Client1 traffic window end interrupt for P2P"]
pub type BitHsCtweInt1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - PORT1 TRX beacon Error interrupt (ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcnere9_int_en(&self) -> BitHsTxbcnere9IntEnR {
        BitHsTxbcnere9IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PORT1 TRX beacon OK interrupt (ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcnok9_int_en(&self) -> BitHsTxbcnok9IntEnR {
        BitHsTxbcnok9IntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When GTIMER3 expires, this bit is set to 1"]
    #[inline(always)]
    pub fn bit_hs_gtint3_en(&self) -> BitHsGtint3EnR {
        BitHsGtint3EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When GTIMER4 expires, this bit is set to 1"]
    #[inline(always)]
    pub fn bit_hs_gtint4_en(&self) -> BitHsGtint4EnR {
        BitHsGtint4EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - port 0 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnely0_aggr_int_en(&self) -> BitHsBcnely0AggrIntEnR {
        BitHsBcnely0AggrIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PS_Timer_A interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_a_int_en(&self) -> BitHsPsTimerAIntEnR {
        BitHsPsTimerAIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PS_Timer_B interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_b_int_en(&self) -> BitHsPsTimerBIntEnR {
        BitHsPsTimerBIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PS_Timer_C interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_c_int_en(&self) -> BitHsPsTimerCIntEnR {
        BitHsPsTimerCIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU_MQG_TXDONE ineterrupt source when HW TX CPUMGQ packet success, it issue interrupt to host"]
    #[inline(always)]
    pub fn bit_hs_cpu_mgq_txdone_int_en(&self) -> BitHsCpuMgqTxdoneIntEnR {
        BitHsCpuMgqTxdoneIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - port 1 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnely1_aggr_int_en(&self) -> BitHsBcnely1AggrIntEnR {
        BitHsBcnely1AggrIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This interrupt is issued at the time set by DRVERLYINT register before TBTT time for port 0 root bssid"]
    #[inline(always)]
    pub fn bit_hs_bcnerly0_int_en(&self) -> BitHsBcnerly0IntEnR {
        BitHsBcnerly0IntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - port0 BSSID1 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly1_int_en(&self) -> BitHsBcnerly1IntEnR {
        BitHsBcnerly1IntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - port0 BSSID2 beacon early interrupt, only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_int_en(&self) -> BitHsBcnerly2IntEnR {
        BitHsBcnerly2IntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - port0 BSSID3 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly3_int_en(&self) -> BitHsBcnerly3IntEnR {
        BitHsBcnerly3IntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - port0 BSSID4 beacon early interrupt, only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly4_int_en(&self) -> BitHsBcnerly4IntEnR {
        BitHsBcnerly4IntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - port0 BSSID5 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly5_int_en(&self) -> BitHsBcnerly5IntEnR {
        BitHsBcnerly5IntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - port0 BSSID6 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly6_int_en(&self) -> BitHsBcnerly6IntEnR {
        BitHsBcnerly6IntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - port0 BSSID7 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly7_int_en(&self) -> BitHsBcnerly7IntEnR {
        BitHsBcnerly7IntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - port1 beacon early interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnerly8_int_en(&self) -> BitHsBcnerly8IntEnR {
        BitHsBcnerly8IntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - port0 BSSID0 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend0_int_en(&self) -> BitHsAtimend0IntEnR {
        BitHsAtimend0IntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - port0 BSSID1 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend1_int_en(&self) -> BitHsAtimend1IntEnR {
        BitHsAtimend1IntEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - port0 BSSID2 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend2_int_en(&self) -> BitHsAtimend2IntEnR {
        BitHsAtimend2IntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - port0 BSSID3 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend3_int_en(&self) -> BitHsAtimend3IntEnR {
        BitHsAtimend3IntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - port0 BSSID4 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend4_int_en(&self) -> BitHsAtimend4IntEnR {
        BitHsAtimend4IntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - port0 BSSID5 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend5_int_en(&self) -> BitHsAtimend5IntEnR {
        BitHsAtimend5IntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - port0 BSSID6 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend6_int_en(&self) -> BitHsAtimend6IntEnR {
        BitHsAtimend6IntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - port0 BSSID7 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend7_int_en(&self) -> BitHsAtimend7IntEnR {
        BitHsAtimend7IntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - port1 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend8_int_en(&self) -> BitHsAtimend8IntEnR {
        BitHsAtimend8IntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Client0 traffic window end interrupt for P2P"]
    #[inline(always)]
    pub fn bit_hs_ctwe_int0_en(&self) -> BitHsCtweInt0EnR {
        BitHsCtweInt0EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Client1 traffic window end interrupt for P2P"]
    #[inline(always)]
    pub fn bit_hs_ctwe_int1_en(&self) -> BitHsCtweInt1EnR {
        BitHsCtweInt1EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PORT1 TRX beacon Error interrupt (ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcnere9_int_en(&mut self) -> BitHsTxbcnere9IntEnW<'_, RegHimr1Spec> {
        BitHsTxbcnere9IntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - PORT1 TRX beacon OK interrupt (ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcnok9_int_en(&mut self) -> BitHsTxbcnok9IntEnW<'_, RegHimr1Spec> {
        BitHsTxbcnok9IntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - When GTIMER3 expires, this bit is set to 1"]
    #[inline(always)]
    pub fn bit_hs_gtint3_en(&mut self) -> BitHsGtint3EnW<'_, RegHimr1Spec> {
        BitHsGtint3EnW::new(self, 3)
    }
    #[doc = "Bit 4 - When GTIMER4 expires, this bit is set to 1"]
    #[inline(always)]
    pub fn bit_hs_gtint4_en(&mut self) -> BitHsGtint4EnW<'_, RegHimr1Spec> {
        BitHsGtint4EnW::new(self, 4)
    }
    #[doc = "Bit 5 - port 0 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnely0_aggr_int_en(&mut self) -> BitHsBcnely0AggrIntEnW<'_, RegHimr1Spec> {
        BitHsBcnely0AggrIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - PS_Timer_A interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_a_int_en(&mut self) -> BitHsPsTimerAIntEnW<'_, RegHimr1Spec> {
        BitHsPsTimerAIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - PS_Timer_B interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_b_int_en(&mut self) -> BitHsPsTimerBIntEnW<'_, RegHimr1Spec> {
        BitHsPsTimerBIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - PS_Timer_C interrupt source When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_hs_ps_timer_c_int_en(&mut self) -> BitHsPsTimerCIntEnW<'_, RegHimr1Spec> {
        BitHsPsTimerCIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - CPU_MQG_TXDONE ineterrupt source when HW TX CPUMGQ packet success, it issue interrupt to host"]
    #[inline(always)]
    pub fn bit_hs_cpu_mgq_txdone_int_en(&mut self) -> BitHsCpuMgqTxdoneIntEnW<'_, RegHimr1Spec> {
        BitHsCpuMgqTxdoneIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - port 1 Beacon early aggregation interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnely1_aggr_int_en(&mut self) -> BitHsBcnely1AggrIntEnW<'_, RegHimr1Spec> {
        BitHsBcnely1AggrIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - This interrupt is issued at the time set by DRVERLYINT register before TBTT time for port 0 root bssid"]
    #[inline(always)]
    pub fn bit_hs_bcnerly0_int_en(&mut self) -> BitHsBcnerly0IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly0IntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - port0 BSSID1 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly1_int_en(&mut self) -> BitHsBcnerly1IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly1IntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - port0 BSSID2 beacon early interrupt, only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly2_int_en(&mut self) -> BitHsBcnerly2IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly2IntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - port0 BSSID3 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly3_int_en(&mut self) -> BitHsBcnerly3IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly3IntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - port0 BSSID4 beacon early interrupt, only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly4_int_en(&mut self) -> BitHsBcnerly4IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly4IntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - port0 BSSID5 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly5_int_en(&mut self) -> BitHsBcnerly5IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly5IntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - port0 BSSID6 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly6_int_en(&mut self) -> BitHsBcnerly6IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly6IntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - port0 BSSID7 beacon early interrupt , only valid under MBSSID mode"]
    #[inline(always)]
    pub fn bit_hs_bcnerly7_int_en(&mut self) -> BitHsBcnerly7IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly7IntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - port1 beacon early interrupt"]
    #[inline(always)]
    pub fn bit_hs_bcnerly8_int_en(&mut self) -> BitHsBcnerly8IntEnW<'_, RegHimr1Spec> {
        BitHsBcnerly8IntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - port0 BSSID0 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend0_int_en(&mut self) -> BitHsAtimend0IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend0IntEnW::new(self, 20)
    }
    #[doc = "Bit 21 - port0 BSSID1 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend1_int_en(&mut self) -> BitHsAtimend1IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend1IntEnW::new(self, 21)
    }
    #[doc = "Bit 22 - port0 BSSID2 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend2_int_en(&mut self) -> BitHsAtimend2IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend2IntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - port0 BSSID3 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend3_int_en(&mut self) -> BitHsAtimend3IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend3IntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - port0 BSSID4 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend4_int_en(&mut self) -> BitHsAtimend4IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend4IntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - port0 BSSID5 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend5_int_en(&mut self) -> BitHsAtimend5IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend5IntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - port0 BSSID6 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend6_int_en(&mut self) -> BitHsAtimend6IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend6IntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - port0 BSSID7 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend7_int_en(&mut self) -> BitHsAtimend7IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend7IntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - port1 atimend interrupt"]
    #[inline(always)]
    pub fn bit_hs_atimend8_int_en(&mut self) -> BitHsAtimend8IntEnW<'_, RegHimr1Spec> {
        BitHsAtimend8IntEnW::new(self, 28)
    }
    #[doc = "Bit 30 - Client0 traffic window end interrupt for P2P"]
    #[inline(always)]
    pub fn bit_hs_ctwe_int0_en(&mut self) -> BitHsCtweInt0EnW<'_, RegHimr1Spec> {
        BitHsCtweInt0EnW::new(self, 30)
    }
    #[doc = "Bit 31 - Client1 traffic window end interrupt for P2P"]
    #[inline(always)]
    pub fn bit_hs_ctwe_int1_en(&mut self) -> BitHsCtweInt1EnW<'_, RegHimr1Spec> {
        BitHsCtweInt1EnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_himr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_himr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHimr1Spec;
impl crate::RegisterSpec for RegHimr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_himr1::R`](R) reader structure"]
impl crate::Readable for RegHimr1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_himr1::W`](W) writer structure"]
impl crate::Writable for RegHimr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HIMR1 to value 0"]
impl crate::Resettable for RegHimr1Spec {}
