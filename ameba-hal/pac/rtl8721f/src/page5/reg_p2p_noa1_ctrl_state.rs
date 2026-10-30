#[doc = "Register `REG_P2P_NOA1_CTRL_STATE` reader"]
pub type R = crate::R<RegP2pNoa1CtrlStateSpec>;
#[doc = "Register `REG_P2P_NOA1_CTRL_STATE` writer"]
pub type W = crate::W<RegP2pNoa1CtrlStateSpec>;
#[doc = "Field `BIT_EN_TSFBIT32_RST_NOA1` reader - "]
pub type BitEnTsfbit32RstNoa1R = crate::BitReader;
#[doc = "Field `BIT_EN_TSFBIT32_RST_NOA1` writer - "]
pub type BitEnTsfbit32RstNoa1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA1_UNIT_SEL` reader - The selector for the source of noa unit0. 0 for port 0, 1 for port 1"]
pub type BitP2pNoa1UnitSelR = crate::FieldReader;
#[doc = "Field `BIT_P2P_NOA1_UNIT_SEL` writer - The selector for the source of noa unit0. 0 for port 0, 1 for port 1"]
pub type BitP2pNoa1UnitSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_P2P_NOA1_EN` reader - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
pub type BitP2pNoa1EnR = crate::FieldReader;
#[doc = "Field `BIT_P2P_NOA1_EN` writer - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
pub type BitP2pNoa1EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_P2P_NOA1_PWR_MGT_EN` reader - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
pub type BitP2pNoa1PwrMgtEnR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA1_PWR_MGT_EN` writer - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
pub type BitP2pNoa1PwrMgtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA1_DISTX_EN` reader - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
pub type BitP2pNoa1DistxEnR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA1_DISTX_EN` writer - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
pub type BitP2pNoa1DistxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA1_ALLSTASLEEP` reader - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
pub type BitP2pNoa1AllstasleepR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA1_ALLSTASLEEP` writer - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
pub type BitP2pNoa1AllstasleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SPEC_FORCE_DOZE0_1` reader - If NoA_Count == 1, this value is 1. else 0."]
pub type BitSpecForceDoze0_1R = crate::BitReader;
#[doc = "Field `BIT_SPEC_NOA0_OFF_PERIOD_1` reader - Spec time"]
pub type BitSpecNoa0OffPeriod1R = crate::BitReader;
#[doc = "Field `BIT_SPEC_FORCE_DOZE1_1` reader - If NoA_Count == 1, this value is 1. else 0."]
pub type BitSpecForceDoze1_1R = crate::BitReader;
#[doc = "Field `BIT_SPEC_NOA1_OFF_PERIOD_1` reader - Spec time"]
pub type BitSpecNoa1OffPeriod1R = crate::BitReader;
#[doc = "Field `BIT_SPEC_CTWIN_EARLY_DISTX_1` reader - ctwin_early_distx"]
pub type BitSpecCtwinEarlyDistx1R = crate::BitReader;
#[doc = "Field `BIT_SPEC_BEACON_AREA_ON_1` reader - The time is from TBTT to BeaconTx or Timeout"]
pub type BitSpecBeaconAreaOn1R = crate::BitReader;
#[doc = "Field `BIT_SPEC_CTWINDOW_ON_1` reader - The time from TBTT <-> CTWEND (with BCNDMATIM early time)"]
pub type BitSpecCtwindowOn1R = crate::BitReader;
#[doc = "Field `BIT_SPEC_POWER_STATE_1` reader - 1: On, 0: Off"]
pub type BitSpecPowerState1R = crate::BitReader;
#[doc = "Field `BIT_FORCE_DOZE0_1` reader - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
pub type BitForceDoze0_1R = crate::BitReader;
#[doc = "Field `BIT_NOA0_OFF_PERIOD_1` reader - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
pub type BitNoa0OffPeriod1R = crate::BitReader;
#[doc = "Field `BIT_FORCE_DOZE1_1` reader - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
pub type BitForceDoze1_1R = crate::BitReader;
#[doc = "Field `BIT_NOA1_OFF_PERIOD_1` reader - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
pub type BitNoa1OffPeriod1R = crate::BitReader;
#[doc = "Field `BIT_CTWIN_EARLY_DISTX_1` reader - ctwin_early_distx. (It is including early time. )"]
pub type BitCtwinEarlyDistx1R = crate::BitReader;
#[doc = "Field `BIT_BEACON_AREA_ON_1` reader - The time is from HOLD Time to BeaconTx or Timeout. (It is including early time. )"]
pub type BitBeaconAreaOn1R = crate::BitReader;
#[doc = "Field `BIT_CTWINDOW_ON_1` reader - Spec time and Beacon Early Time to TBTT Time. (It is including early time. )"]
pub type BitCtwindowOn1R = crate::BitReader;
#[doc = "Field `BIT_POWER_STATE_1` reader - 1: On, 0: Off. (It is including early time. )"]
pub type BitPowerState1R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bit_en_tsfbit32_rst_noa1(&self) -> BitEnTsfbit32RstNoa1R {
        BitEnTsfbit32RstNoa1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - The selector for the source of noa unit0. 0 for port 0, 1 for port 1"]
    #[inline(always)]
    pub fn bit_p2p_noa1_unit_sel(&self) -> BitP2pNoa1UnitSelR {
        BitP2pNoa1UnitSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa1_en(&self) -> BitP2pNoa1EnR {
        BitP2pNoa1EnR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa1_pwr_mgt_en(&self) -> BitP2pNoa1PwrMgtEnR {
        BitP2pNoa1PwrMgtEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
    #[inline(always)]
    pub fn bit_p2p_noa1_distx_en(&self) -> BitP2pNoa1DistxEnR {
        BitP2pNoa1DistxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
    #[inline(always)]
    pub fn bit_p2p_noa1_allstasleep(&self) -> BitP2pNoa1AllstasleepR {
        BitP2pNoa1AllstasleepR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - If NoA_Count == 1, this value is 1. else 0."]
    #[inline(always)]
    pub fn bit_spec_force_doze0_1(&self) -> BitSpecForceDoze0_1R {
        BitSpecForceDoze0_1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Spec time"]
    #[inline(always)]
    pub fn bit_spec_noa0_off_period_1(&self) -> BitSpecNoa0OffPeriod1R {
        BitSpecNoa0OffPeriod1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If NoA_Count == 1, this value is 1. else 0."]
    #[inline(always)]
    pub fn bit_spec_force_doze1_1(&self) -> BitSpecForceDoze1_1R {
        BitSpecForceDoze1_1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Spec time"]
    #[inline(always)]
    pub fn bit_spec_noa1_off_period_1(&self) -> BitSpecNoa1OffPeriod1R {
        BitSpecNoa1OffPeriod1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ctwin_early_distx"]
    #[inline(always)]
    pub fn bit_spec_ctwin_early_distx_1(&self) -> BitSpecCtwinEarlyDistx1R {
        BitSpecCtwinEarlyDistx1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The time is from TBTT to BeaconTx or Timeout"]
    #[inline(always)]
    pub fn bit_spec_beacon_area_on_1(&self) -> BitSpecBeaconAreaOn1R {
        BitSpecBeaconAreaOn1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The time from TBTT <-> CTWEND (with BCNDMATIM early time)"]
    #[inline(always)]
    pub fn bit_spec_ctwindow_on_1(&self) -> BitSpecCtwindowOn1R {
        BitSpecCtwindowOn1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: On, 0: Off"]
    #[inline(always)]
    pub fn bit_spec_power_state_1(&self) -> BitSpecPowerState1R {
        BitSpecPowerState1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_force_doze0_1(&self) -> BitForceDoze0_1R {
        BitForceDoze0_1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_noa0_off_period_1(&self) -> BitNoa0OffPeriod1R {
        BitNoa0OffPeriod1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_force_doze1_1(&self) -> BitForceDoze1_1R {
        BitForceDoze1_1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_noa1_off_period_1(&self) -> BitNoa1OffPeriod1R {
        BitNoa1OffPeriod1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ctwin_early_distx. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_ctwin_early_distx_1(&self) -> BitCtwinEarlyDistx1R {
        BitCtwinEarlyDistx1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The time is from HOLD Time to BeaconTx or Timeout. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_beacon_area_on_1(&self) -> BitBeaconAreaOn1R {
        BitBeaconAreaOn1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Spec time and Beacon Early Time to TBTT Time. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_ctwindow_on_1(&self) -> BitCtwindowOn1R {
        BitCtwindowOn1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: On, 0: Off. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_power_state_1(&self) -> BitPowerState1R {
        BitPowerState1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bit_en_tsfbit32_rst_noa1(
        &mut self,
    ) -> BitEnTsfbit32RstNoa1W<'_, RegP2pNoa1CtrlStateSpec> {
        BitEnTsfbit32RstNoa1W::new(self, 0)
    }
    #[doc = "Bits 1:2 - The selector for the source of noa unit0. 0 for port 0, 1 for port 1"]
    #[inline(always)]
    pub fn bit_p2p_noa1_unit_sel(&mut self) -> BitP2pNoa1UnitSelW<'_, RegP2pNoa1CtrlStateSpec> {
        BitP2pNoa1UnitSelW::new(self, 1)
    }
    #[doc = "Bits 3:4 - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa1_en(&mut self) -> BitP2pNoa1EnW<'_, RegP2pNoa1CtrlStateSpec> {
        BitP2pNoa1EnW::new(self, 3)
    }
    #[doc = "Bit 5 - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa1_pwr_mgt_en(&mut self) -> BitP2pNoa1PwrMgtEnW<'_, RegP2pNoa1CtrlStateSpec> {
        BitP2pNoa1PwrMgtEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
    #[inline(always)]
    pub fn bit_p2p_noa1_distx_en(&mut self) -> BitP2pNoa1DistxEnW<'_, RegP2pNoa1CtrlStateSpec> {
        BitP2pNoa1DistxEnW::new(self, 6)
    }
    #[doc = "Bit 7 - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
    #[inline(always)]
    pub fn bit_p2p_noa1_allstasleep(
        &mut self,
    ) -> BitP2pNoa1AllstasleepW<'_, RegP2pNoa1CtrlStateSpec> {
        BitP2pNoa1AllstasleepW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p2p_noa1_ctrl_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p2p_noa1_ctrl_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP2pNoa1CtrlStateSpec;
impl crate::RegisterSpec for RegP2pNoa1CtrlStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p2p_noa1_ctrl_state::R`](R) reader structure"]
impl crate::Readable for RegP2pNoa1CtrlStateSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p2p_noa1_ctrl_state::W`](W) writer structure"]
impl crate::Writable for RegP2pNoa1CtrlStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P2P_NOA1_CTRL_STATE to value 0x01"]
impl crate::Resettable for RegP2pNoa1CtrlStateSpec {
    const RESET_VALUE: u32 = 0x01;
}
