#[doc = "Register `REG_FTISR` reader"]
pub type R = crate::R<RegFtisrSpec>;
#[doc = "Field `BIT_FS_GTINT0_INT` reader - General Timer\\[0\\] interrupt"]
pub type BitFsGtint0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT1_INT` reader - General Timer\\[1\\] interrupt"]
pub type BitFsGtint1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT2_INT` reader - General Timer\\[2\\] interrupt"]
pub type BitFsGtint2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT3_INT` reader - General Timer\\[3\\] interrupt"]
pub type BitFsGtint3IntR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT4_INT` reader - General Timer\\[4\\] interrupt"]
pub type BitFsGtint4IntR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT5_INT` reader - General Timer\\[5\\] interrupt"]
pub type BitFsGtint5IntR = crate::BitReader;
#[doc = "Field `BIT_FS_GTINT6_INT` reader - General Timer\\[6\\] interrupt"]
pub type BitFsGtint6IntR = crate::BitReader;
#[doc = "Field `BIT_FS_FTM_PTT_INT` reader - Partial tsf timer"]
pub type BitFsFtmPttIntR = crate::BitReader;
#[doc = "Field `RSVD_5` reader - RSVD"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `RSVD_4` reader - RSVD"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_3` reader - RSVD"]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_TIMEOUT0_INT` reader - PS_TimeOut0 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b4\\[2:0\\]) matches the content of this register(0x580), the FW PS_TimeOut0 interrupt is issued."]
pub type BitFsPsTimeout0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_TIMEOUT1_INT` reader - PS_TimeOut1 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b1\\[5:3\\]) matches the content of this register(0x584), the FW PS_TimeOut1 interrupt is issued."]
pub type BitFsPsTimeout1IntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_TIMEOUT2_INT` reader - PS_TimeOut2 interrupt . When the lower 4 bytes of FREERUN CNT matches the content of this register(0x588), the FW PS_TimeOut2 interrupt is issued."]
pub type BitFsPsTimeout2IntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_A_INT` reader - PS_Timer_A interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
pub type BitFsPsTimerAIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_B_INT` reader - PS_Timer_B interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
pub type BitFsPsTimerBIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_C_INT` reader - PS_Timer_C interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
pub type BitFsPsTimerCIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_A_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
pub type BitFsPsTimerAEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_B_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
pub type BitFsPsTimerBEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_C_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
pub type BitFsPsTimerCEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_D_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
pub type BitFsPsTimerDEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_E_Early_INT` reader - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
pub type BitFsPsTimerEEarlyIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_D_INT` reader - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
pub type BitFsPsTimerDIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PS_Timer_E_INT` reader - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
pub type BitFsPsTimerEIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RPWM_INT` reader - RPWM interrupt"]
pub type BitFsRpwmIntR = crate::BitReader;
#[doc = "Field `BIT_FTSR2_INT` reader - FTMR2 Indicator (FTMR2 and FTMR2 are true, this bit is set to 1)"]
pub type BitFtsr2IntR = crate::BitReader;
#[doc = "Field `BIT_FTSR1_INT` reader - FTMR1 Indicator (FTMR1 and FTMR1 are true, this bit is set to 1)"]
pub type BitFtsr1IntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - General Timer\\[0\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint0_int(&self) -> BitFsGtint0IntR {
        BitFsGtint0IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General Timer\\[1\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint1_int(&self) -> BitFsGtint1IntR {
        BitFsGtint1IntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General Timer\\[2\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint2_int(&self) -> BitFsGtint2IntR {
        BitFsGtint2IntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Timer\\[3\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint3_int(&self) -> BitFsGtint3IntR {
        BitFsGtint3IntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - General Timer\\[4\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint4_int(&self) -> BitFsGtint4IntR {
        BitFsGtint4IntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Timer\\[5\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint5_int(&self) -> BitFsGtint5IntR {
        BitFsGtint5IntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Timer\\[6\\] interrupt"]
    #[inline(always)]
    pub fn bit_fs_gtint6_int(&self) -> BitFsGtint6IntR {
        BitFsGtint6IntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Partial tsf timer"]
    #[inline(always)]
    pub fn bit_fs_ftm_ptt_int(&self) -> BitFsFtmPttIntR {
        BitFsFtmPttIntR::new(((self.bits >> 7) & 1) != 0)
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
    pub fn bit_fs_ps_timeout0_int(&self) -> BitFsPsTimeout0IntR {
        BitFsPsTimeout0IntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PS_TimeOut1 interrupt . When the lower 4 bytes of TSFTR(that select by 0x5b1\\[5:3\\]) matches the content of this register(0x584), the FW PS_TimeOut1 interrupt is issued."]
    #[inline(always)]
    pub fn bit_fs_ps_timeout1_int(&self) -> BitFsPsTimeout1IntR {
        BitFsPsTimeout1IntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PS_TimeOut2 interrupt . When the lower 4 bytes of FREERUN CNT matches the content of this register(0x588), the FW PS_TimeOut2 interrupt is issued."]
    #[inline(always)]
    pub fn bit_fs_ps_timeout2_int(&self) -> BitFsPsTimeout2IntR {
        BitFsPsTimeout2IntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - PS_Timer_A interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[2:0\\]) matches the content of this register, the PS_Timer_A_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_a_int(&self) -> BitFsPsTimerAIntR {
        BitFsPsTimerAIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PS_Timer_B interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[10:8\\]) matches the content of this register, the PS_Timer_B_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_b_int(&self) -> BitFsPsTimerBIntR {
        BitFsPsTimerBIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PS_Timer_C interrupt When the lower 4 bytes of TSF(that select by reg 0x1510\\[18:16\\]) matches the content of this register, the PS_Timer_C_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_c_int(&self) -> BitFsPsTimerCIntR {
        BitFsPsTimerCIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - This interrupt is issued at the time set by PS_Timer_A_Early register before PS_Timer_A time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_a_early_int(&self) -> BitFsPsTimerAEarlyIntR {
        BitFsPsTimerAEarlyIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This interrupt is issued at the time set by PS_Timer_B_Early register before PS_Timer_B time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_b_early_int(&self) -> BitFsPsTimerBEarlyIntR {
        BitFsPsTimerBEarlyIntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This interrupt is issued at the time set by PS_Timer_C_Early register before PS_Timer_C time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_c_early_int(&self) -> BitFsPsTimerCEarlyIntR {
        BitFsPsTimerCEarlyIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This interrupt is issued at the time set by PS_Timer_D_Early register before PS_Timer_D time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_d_early_int(&self) -> BitFsPsTimerDEarlyIntR {
        BitFsPsTimerDEarlyIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This interrupt is issued at the time set by PS_Timer_E_Early register before PS_Timer_E time."]
    #[inline(always)]
    pub fn bit_fs_ps_timer_e_early_int(&self) -> BitFsPsTimerEEarlyIntR {
        BitFsPsTimerEEarlyIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PS_Timer_D interrupt When the lower 4 bytes of TSF(that select by reg 0x1520\\[17:16\\]) matches the content of this register, the PS_Timer_D_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_d_int(&self) -> BitFsPsTimerDIntR {
        BitFsPsTimerDIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PS_Timer_E interrupt When the TSF(that select by reg 0x1520\\[13:12\\]) matches the content of this register, the PS_Timer_E_INT interrupt is issued"]
    #[inline(always)]
    pub fn bit_fs_ps_timer_e_int(&self) -> BitFsPsTimerEIntR {
        BitFsPsTimerEIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RPWM interrupt"]
    #[inline(always)]
    pub fn bit_fs_rpwm_int(&self) -> BitFsRpwmIntR {
        BitFsRpwmIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - FTMR2 Indicator (FTMR2 and FTMR2 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_ftsr2_int(&self) -> BitFtsr2IntR {
        BitFtsr2IntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FTMR1 Indicator (FTMR1 and FTMR1 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_ftsr1_int(&self) -> BitFtsr1IntR {
        BitFtsr1IntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Firmware Timer Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ftisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFtisrSpec;
impl crate::RegisterSpec for RegFtisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ftisr::R`](R) reader structure"]
impl crate::Readable for RegFtisrSpec {}
#[doc = "`reset()` method sets REG_FTISR to value 0"]
impl crate::Resettable for RegFtisrSpec {}
