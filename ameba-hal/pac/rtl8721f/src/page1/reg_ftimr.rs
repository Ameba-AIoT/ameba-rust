#[doc = "Register `REG_FTIMR` reader"]
pub type R = crate::R<RegFtimrSpec>;
#[doc = "Register `REG_FTIMR` writer"]
pub type W = crate::W<RegFtimrSpec>;
#[doc = "Field `BIT_FS_GTINT0_INT_EN` reader - General Timer\\[0\\] interrupt"]
pub type BitFsGtint0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT0_INT_EN` writer - General Timer\\[0\\] interrupt"]
pub type BitFsGtint0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_GTINT1_INT_EN` reader - General Timer\\[1\\] interrupt"]
pub type BitFsGtint1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT1_INT_EN` writer - General Timer\\[1\\] interrupt"]
pub type BitFsGtint1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_GTINT2_INT_EN` reader - General Timer\\[2\\] interrupt"]
pub type BitFsGtint2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT2_INT_EN` writer - General Timer\\[2\\] interrupt"]
pub type BitFsGtint2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_GTINT3_INT_EN` reader - General Timer\\[3\\] interrupt"]
pub type BitFsGtint3IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT3_INT_EN` writer - General Timer\\[3\\] interrupt"]
pub type BitFsGtint3IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_GTINT4_INT_EN` reader - General Timer\\[4\\] interrupt"]
pub type BitFsGtint4IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT4_INT_EN` writer - General Timer\\[4\\] interrupt"]
pub type BitFsGtint4IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_GTINT5_INT_EN` reader - General Timer\\[5\\] interrupt"]
pub type BitFsGtint5IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT5_INT_EN` writer - General Timer\\[5\\] interrupt"]
pub type BitFsGtint5IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_GTINT6_INT_EN` reader - General Timer\\[6\\] interrupt"]
pub type BitFsGtint6IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT6_INT_EN` writer - General Timer\\[6\\] interrupt"]
pub type BitFsGtint6IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_FTM_PTT_INT_EN` reader - Partial tsf timer"]
pub type BitFsFtmPttIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_FTM_PTT_INT_EN` writer - Partial tsf timer"]
pub type BitFsFtmPttIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_5` reader - RSVD"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `RSVD_5` writer - RSVD"]
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_4` reader - RSVD"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_4` writer - RSVD"]
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_3` reader - RSVD"]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD_3` writer - RSVD"]
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_TIMEOUT0_INT_EN` reader - PS_TimeOut0 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b4\\[2:0\\]) matches the content of this register(0x580), the FW PS_TimeOut0 interrupt is issued."]
pub type BitFsPsTimeout0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_TIMEOUT0_INT_EN` writer - PS_TimeOut0 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b4\\[2:0\\]) matches the content of this register(0x580), the FW PS_TimeOut0 interrupt is issued."]
pub type BitFsPsTimeout0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_TIMEOUT1_INT_EN` reader - PS_TimeOut1 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b1\\[5:3\\]) matches the content of this register(0x584), the FW PS_TimeOut1 interrupt is issued."]
pub type BitFsPsTimeout1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_TIMEOUT1_INT_EN` writer - PS_TimeOut1 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b1\\[5:3\\]) matches the content of this register(0x584), the FW PS_TimeOut1 interrupt is issued."]
pub type BitFsPsTimeout1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_TIMEOUT2_INT_EN` reader - PS_TimeOut2 interrupt . When the lower 4 bytes of FREERUN CNT matches the content of this register(0x588), the FW PS_TimeOut2 interrupt is issued."]
pub type BitFsPsTimeout2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_TIMEOUT2_INT_EN` writer - PS_TimeOut2 interrupt . When the lower 4 bytes of FREERUN CNT matches the content of this register(0x588), the FW PS_TimeOut2 interrupt is issued."]
pub type BitFsPsTimeout2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_A_INT_EN` reader - PS_Timer_A interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
pub type BitFsPsTimerAIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_A_INT_EN` writer - PS_Timer_A interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
pub type BitFsPsTimerAIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_B_INT_EN` reader - PS_Timer_B interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
pub type BitFsPsTimerBIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_B_INT_EN` writer - PS_Timer_B interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
pub type BitFsPsTimerBIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_C_INT_EN` reader - PS_Timer_C interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
pub type BitFsPsTimerCIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_C_INT_EN` writer - PS_Timer_C interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
pub type BitFsPsTimerCIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_A_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
pub type BitFsPsTimerAEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_A_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
pub type BitFsPsTimerAEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_B_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
pub type BitFsPsTimerBEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_B_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
pub type BitFsPsTimerBEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_C_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
pub type BitFsPsTimerCEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_C_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
pub type BitFsPsTimerCEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_D_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
pub type BitFsPsTimerDEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_D_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
pub type BitFsPsTimerDEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_E_Early_INT_EN` reader - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
pub type BitFsPsTimerEEarlyIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_E_Early_INT_EN` writer - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
pub type BitFsPsTimerEEarlyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_D_INT_EN` reader - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
pub type BitFsPsTimerDIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_D_INT_EN` writer - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
pub type BitFsPsTimerDIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PS_Timer_E_INT_EN` reader - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
pub type BitFsPsTimerEIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_E_INT_EN` writer - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
pub type BitFsPsTimerEIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RPWM_INT_EN` reader - RPWM interrupt"]
pub type BitFsRpwmIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RPWM_INT_EN` writer - RPWM interrupt"]
pub type BitFsRpwmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FTSR2_INT_EN` reader - FTMR2 Indicator (FTMR2 and FTMR2 are true, this bit is set to 1)"]
pub type BitFtsr2IntEnR = crate::BitReader;
#[doc = "Field `BIT_FTSR2_INT_EN` writer - FTMR2 Indicator (FTMR2 and FTMR2 are true, this bit is set to 1)"]
pub type BitFtsr2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FTSR1_INT_EN` reader - FTMR1 Indicator (FTMR1 and FTMR1 are true, this bit is set to 1)"]
pub type BitFtsr1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FTSR1_INT_EN` writer - FTMR1 Indicator (FTMR1 and FTMR1 are true, this bit is set to 1)"]
pub type BitFtsr1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - General Timer\\[0\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint0_int_en(&self) -> BitFsGtint0IntEnR {
        BitFsGtint0IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General Timer\\[1\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint1_int_en(&self) -> BitFsGtint1IntEnR {
        BitFsGtint1IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General Timer\\[2\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint2_int_en(&self) -> BitFsGtint2IntEnR {
        BitFsGtint2IntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Timer\\[3\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint3_int_en(&self) -> BitFsGtint3IntEnR {
        BitFsGtint3IntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - General Timer\\[4\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint4_int_en(&self) -> BitFsGtint4IntEnR {
        BitFsGtint4IntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Timer\\[5\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint5_int_en(&self) -> BitFsGtint5IntEnR {
        BitFsGtint5IntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Timer\\[6\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint6_int_en(&self) -> BitFsGtint6IntEnR {
        BitFsGtint6IntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Partial tsf timer"]
    #[inline(always)]
    pub fn bit_fs_ftm_ptt_int_en(&self) -> BitFsFtmPttIntEnR {
        BitFsFtmPttIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PS_TimeOut0 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b4\\[2:0\\]) matches the content of this register(0x580), the FW PS_TimeOut0 interrupt is issued."]
    #[inline(always)]
    pub fn bit_fs_ps_timeout0_int_en(&self) -> BitFsPsTimeout0IntEnR {
        BitFsPsTimeout0IntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PS_TimeOut1 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b1\\[5:3\\]) matches the content of this register(0x584), the FW PS_TimeOut1 interrupt is issued."]
    #[inline(always)]
    pub fn bit_fs_ps_timeout1_int_en(&self) -> BitFsPsTimeout1IntEnR {
        BitFsPsTimeout1IntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PS_TimeOut2 interrupt . When the lower 4 bytes of FREERUN CNT matches the content of this register(0x588), the FW PS_TimeOut2 interrupt is issued."]
    #[inline(always)]
    pub fn bit_fs_ps_timeout2_int_en(&self) -> BitFsPsTimeout2IntEnR {
        BitFsPsTimeout2IntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - PS_Timer_A interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_a_int_en(&self) -> BitFsPsTimerAIntEnR {
        BitFsPsTimerAIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PS_Timer_B interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_b_int_en(&self) -> BitFsPsTimerBIntEnR {
        BitFsPsTimerBIntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PS_Timer_C interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_c_int_en(&self) -> BitFsPsTimerCIntEnR {
        BitFsPsTimerCIntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_a_early_int_en(&self) -> BitFsPsTimerAEarlyIntEnR {
        BitFsPsTimerAEarlyIntEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_b_early_int_en(&self) -> BitFsPsTimerBEarlyIntEnR {
        BitFsPsTimerBEarlyIntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_c_early_int_en(&self) -> BitFsPsTimerCEarlyIntEnR {
        BitFsPsTimerCEarlyIntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_d_early_int_en(&self) -> BitFsPsTimerDEarlyIntEnR {
        BitFsPsTimerDEarlyIntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_e_early_int_en(&self) -> BitFsPsTimerEEarlyIntEnR {
        BitFsPsTimerEEarlyIntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_d_int_en(&self) -> BitFsPsTimerDIntEnR {
        BitFsPsTimerDIntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_e_int_en(&self) -> BitFsPsTimerEIntEnR {
        BitFsPsTimerEIntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RPWM interrupt"]
    #[inline(always)]
    pub fn bit_fs_rpwm_int_en(&self) -> BitFsRpwmIntEnR {
        BitFsRpwmIntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - FTMR2 Indicator (FTMR2 and FTMR2 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_ftsr2_int_en(&self) -> BitFtsr2IntEnR {
        BitFtsr2IntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FTMR1 Indicator (FTMR1 and FTMR1 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_ftsr1_int_en(&self) -> BitFtsr1IntEnR {
        BitFtsr1IntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - General Timer\\[0\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint0_int_en(&mut self) -> BitFsGtint0IntEnW<'_, RegFtimrSpec> {
        BitFsGtint0IntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - General Timer\\[1\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint1_int_en(&mut self) -> BitFsGtint1IntEnW<'_, RegFtimrSpec> {
        BitFsGtint1IntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - General Timer\\[2\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint2_int_en(&mut self) -> BitFsGtint2IntEnW<'_, RegFtimrSpec> {
        BitFsGtint2IntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - General Timer\\[3\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint3_int_en(&mut self) -> BitFsGtint3IntEnW<'_, RegFtimrSpec> {
        BitFsGtint3IntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - General Timer\\[4\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint4_int_en(&mut self) -> BitFsGtint4IntEnW<'_, RegFtimrSpec> {
        BitFsGtint4IntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - General Timer\\[5\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint5_int_en(&mut self) -> BitFsGtint5IntEnW<'_, RegFtimrSpec> {
        BitFsGtint5IntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - General Timer\\[6\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint6_int_en(&mut self) -> BitFsGtint6IntEnW<'_, RegFtimrSpec> {
        BitFsGtint6IntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Partial tsf timer"]
    #[inline(always)]
    pub fn bit_fs_ftm_ptt_int_en(&mut self) -> BitFsFtmPttIntEnW<'_, RegFtimrSpec> {
        BitFsFtmPttIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&mut self) -> Rsvd5W<'_, RegFtimrSpec> {
        Rsvd5W::new(self, 8)
    }
    #[doc = "Bit 9 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&mut self) -> Rsvd4W<'_, RegFtimrSpec> {
        Rsvd4W::new(self, 9)
    }
    #[doc = "Bit 10 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> Rsvd3W<'_, RegFtimrSpec> {
        Rsvd3W::new(self, 10)
    }
    #[doc = "Bit 11 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegFtimrSpec> {
        Rsvd2W::new(self, 11)
    }
    #[doc = "Bit 12 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFtimrSpec> {
        RsvdW::new(self, 12)
    }
    #[doc = "Bit 13 - PS_TimeOut0 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b4\\[2:0\\]) matches the content of this register(0x580), the FW PS_TimeOut0 interrupt is issued."]
    #[inline(always)]
    pub fn bit_fs_ps_timeout0_int_en(&mut self) -> BitFsPsTimeout0IntEnW<'_, RegFtimrSpec> {
        BitFsPsTimeout0IntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - PS_TimeOut1 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b1\\[5:3\\]) matches the content of this register(0x584), the FW PS_TimeOut1 interrupt is issued."]
    #[inline(always)]
    pub fn bit_fs_ps_timeout1_int_en(&mut self) -> BitFsPsTimeout1IntEnW<'_, RegFtimrSpec> {
        BitFsPsTimeout1IntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - PS_TimeOut2 interrupt . When the lower 4 bytes of FREERUN CNT matches the content of this register(0x588), the FW PS_TimeOut2 interrupt is issued."]
    #[inline(always)]
    pub fn bit_fs_ps_timeout2_int_en(&mut self) -> BitFsPsTimeout2IntEnW<'_, RegFtimrSpec> {
        BitFsPsTimeout2IntEnW::new(self, 15)
    }
    #[doc = "Bit 17 - PS_Timer_A interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_a_int_en(&mut self) -> BitFsPsTimerAIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerAIntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - PS_Timer_B interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_b_int_en(&mut self) -> BitFsPsTimerBIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerBIntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - PS_Timer_C interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_c_int_en(&mut self) -> BitFsPsTimerCIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerCIntEnW::new(self, 19)
    }
    #[doc = "Bit 21 - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_a_early_int_en(&mut self) -> BitFsPsTimerAEarlyIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerAEarlyIntEnW::new(self, 21)
    }
    #[doc = "Bit 22 - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_b_early_int_en(&mut self) -> BitFsPsTimerBEarlyIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerBEarlyIntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_c_early_int_en(&mut self) -> BitFsPsTimerCEarlyIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerCEarlyIntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_d_early_int_en(&mut self) -> BitFsPsTimerDEarlyIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerDEarlyIntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_e_early_int_en(&mut self) -> BitFsPsTimerEEarlyIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerEEarlyIntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_d_int_en(&mut self) -> BitFsPsTimerDIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerDIntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_e_int_en(&mut self) -> BitFsPsTimerEIntEnW<'_, RegFtimrSpec> {
        BitFsPsTimerEIntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - RPWM interrupt"]
    #[inline(always)]
    pub fn bit_fs_rpwm_int_en(&mut self) -> BitFsRpwmIntEnW<'_, RegFtimrSpec> {
        BitFsRpwmIntEnW::new(self, 28)
    }
    #[doc = "Bit 30 - FTMR2 Indicator (FTMR2 and FTMR2 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_ftsr2_int_en(&mut self) -> BitFtsr2IntEnW<'_, RegFtimrSpec> {
        BitFtsr2IntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - FTMR1 Indicator (FTMR1 and FTMR1 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_ftsr1_int_en(&mut self) -> BitFtsr1IntEnW<'_, RegFtimrSpec> {
        BitFtsr1IntEnW::new(self, 31)
    }
}
#[doc = "Firmware Timer Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ftimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ftimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFtimrSpec;
impl crate::RegisterSpec for RegFtimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ftimr::R`](R) reader structure"]
impl crate::Readable for RegFtimrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ftimr::W`](W) writer structure"]
impl crate::Writable for RegFtimrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FTIMR to value 0"]
impl crate::Resettable for RegFtimrSpec {}
