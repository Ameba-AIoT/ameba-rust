#[doc = "Register `REG_P2P_NOA2_CTRL_STATE` reader"]
pub type R = crate::R<RegP2pNoa2CtrlStateSpec>;
#[doc = "Register `REG_P2P_NOA2_CTRL_STATE` writer"]
pub type W = crate::W<RegP2pNoa2CtrlStateSpec>;
#[doc = "Field `BIT_EN_TSFBIT32_RST_NOA2` reader - "]
pub type BitEnTsfbit32RstNoa2R = crate::BitReader;
#[doc = "Field `BIT_EN_TSFBIT32_RST_NOA2` writer - "]
pub type BitEnTsfbit32RstNoa2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA2_UNIT_SEL` reader - The selector for the source of noa unit0. 0 for port 0, 1 for port 1, 2 for port 2"]
pub type BitP2pNoa2UnitSelR = crate::FieldReader;
#[doc = "Field `BIT_P2P_NOA2_UNIT_SEL` writer - The selector for the source of noa unit0. 0 for port 0, 1 for port 1, 2 for port 2"]
pub type BitP2pNoa2UnitSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_P2P_NOA2_EN` reader - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
pub type BitP2pNoa2EnR = crate::FieldReader;
#[doc = "Field `BIT_P2P_NOA2_EN` writer - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
pub type BitP2pNoa2EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_P2P_NOA2_PWR_MGT_EN` reader - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
pub type BitP2pNoa2PwrMgtEnR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA2_PWR_MGT_EN` writer - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
pub type BitP2pNoa2PwrMgtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA2_DISTX_EN` reader - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
pub type BitP2pNoa2DistxEnR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA2_DISTX_EN` writer - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
pub type BitP2pNoa2DistxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_NOA2_ALLSTASLEEP` reader - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
pub type BitP2pNoa2AllstasleepR = crate::BitReader;
#[doc = "Field `BIT_P2P_NOA2_ALLSTASLEEP` writer - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
pub type BitP2pNoa2AllstasleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SPEC_FORCE_DOZE0_2` reader - If NoA_Count == 1, this value is 1. else 0."]
pub type BitSpecForceDoze0_2R = crate::BitReader;
#[doc = "Field `BIT_SPEC_NOA0_OFF_PERIOD_2` reader - Spec time"]
pub type BitSpecNoa0OffPeriod2R = crate::BitReader;
#[doc = "Field `BIT_SPEC_FORCE_DOZE1_2` reader - If NoA_Count == 1, this value is 1. else 0."]
pub type BitSpecForceDoze1_2R = crate::BitReader;
#[doc = "Field `BIT_SPEC_NOA1_OFF_PERIOD_2` reader - Spec time"]
pub type BitSpecNoa1OffPeriod2R = crate::BitReader;
#[doc = "Field `BIT_SPEC_CTWIN_EARLY_DISTX_2` reader - ctwin_early_distx"]
pub type BitSpecCtwinEarlyDistx2R = crate::BitReader;
#[doc = "Field `BIT_SPEC_BEACON_AREA_ON_2` reader - The time is from TBTT to BeaconTx or Timeout"]
pub type BitSpecBeaconAreaOn2R = crate::BitReader;
#[doc = "Field `BIT_SPEC_CTWINDOW_ON_2` reader - The time from TBTT <-> CTWEND (with BCNDMATIM early time)"]
pub type BitSpecCtwindowOn2R = crate::BitReader;
#[doc = "Field `BIT_SPEC_POWER_STATE_2` reader - 1: On, 0: Off"]
pub type BitSpecPowerState2R = crate::BitReader;
#[doc = "Field `BIT_FORCE_DOZE0_2` reader - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
pub type BitForceDoze0_2R = crate::BitReader;
#[doc = "Field `BIT_NOA0_OFF_PERIOD_2` reader - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
pub type BitNoa0OffPeriod2R = crate::BitReader;
#[doc = "Field `BIT_FORCE_DOZE1_2` reader - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
pub type BitForceDoze1_2R = crate::BitReader;
#[doc = "Field `BIT_NOA1_OFF_PERIOD_2` reader - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
pub type BitNoa1OffPeriod2R = crate::BitReader;
#[doc = "Field `BIT_CTWIN_EARLY_DISTX_2` reader - ctwin_early_distx. (It is including early time. )"]
pub type BitCtwinEarlyDistx2R = crate::BitReader;
#[doc = "Field `BIT_BEACON_AREA_ON_2` reader - The time is from HOLD Time to BeaconTx or Timeout. (It is including early time. )"]
pub type BitBeaconAreaOn2R = crate::BitReader;
#[doc = "Field `BIT_CTWINDOW_ON_2` reader - Spec time and Beacon Early Time to TBTT Time. (It is including early time. )"]
pub type BitCtwindowOn2R = crate::BitReader;
#[doc = "Field `BIT_POWER_STATE_2` reader - 1: On, 0: Off. (It is including early time. )"]
pub type BitPowerState2R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bit_en_tsfbit32_rst_noa2(&self) -> BitEnTsfbit32RstNoa2R {
        BitEnTsfbit32RstNoa2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - The selector for the source of noa unit0. 0 for port 0, 1 for port 1, 2 for port 2"]
    #[inline(always)]
    pub fn bit_p2p_noa2_unit_sel(&self) -> BitP2pNoa2UnitSelR {
        BitP2pNoa2UnitSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa2_en(&self) -> BitP2pNoa2EnR {
        BitP2pNoa2EnR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa2_pwr_mgt_en(&self) -> BitP2pNoa2PwrMgtEnR {
        BitP2pNoa2PwrMgtEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
    #[inline(always)]
    pub fn bit_p2p_noa2_distx_en(&self) -> BitP2pNoa2DistxEnR {
        BitP2pNoa2DistxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
    #[inline(always)]
    pub fn bit_p2p_noa2_allstasleep(&self) -> BitP2pNoa2AllstasleepR {
        BitP2pNoa2AllstasleepR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - If NoA_Count == 1, this value is 1. else 0."]
    #[inline(always)]
    pub fn bit_spec_force_doze0_2(&self) -> BitSpecForceDoze0_2R {
        BitSpecForceDoze0_2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Spec time"]
    #[inline(always)]
    pub fn bit_spec_noa0_off_period_2(&self) -> BitSpecNoa0OffPeriod2R {
        BitSpecNoa0OffPeriod2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If NoA_Count == 1, this value is 1. else 0."]
    #[inline(always)]
    pub fn bit_spec_force_doze1_2(&self) -> BitSpecForceDoze1_2R {
        BitSpecForceDoze1_2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Spec time"]
    #[inline(always)]
    pub fn bit_spec_noa1_off_period_2(&self) -> BitSpecNoa1OffPeriod2R {
        BitSpecNoa1OffPeriod2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ctwin_early_distx"]
    #[inline(always)]
    pub fn bit_spec_ctwin_early_distx_2(&self) -> BitSpecCtwinEarlyDistx2R {
        BitSpecCtwinEarlyDistx2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The time is from TBTT to BeaconTx or Timeout"]
    #[inline(always)]
    pub fn bit_spec_beacon_area_on_2(&self) -> BitSpecBeaconAreaOn2R {
        BitSpecBeaconAreaOn2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The time from TBTT <-> CTWEND (with BCNDMATIM early time)"]
    #[inline(always)]
    pub fn bit_spec_ctwindow_on_2(&self) -> BitSpecCtwindowOn2R {
        BitSpecCtwindowOn2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: On, 0: Off"]
    #[inline(always)]
    pub fn bit_spec_power_state_2(&self) -> BitSpecPowerState2R {
        BitSpecPowerState2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_force_doze0_2(&self) -> BitForceDoze0_2R {
        BitForceDoze0_2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_noa0_off_period_2(&self) -> BitNoa0OffPeriod2R {
        BitNoa0OffPeriod2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_force_doze1_2(&self) -> BitForceDoze1_2R {
        BitForceDoze1_2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_noa1_off_period_2(&self) -> BitNoa1OffPeriod2R {
        BitNoa1OffPeriod2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ctwin_early_distx. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_ctwin_early_distx_2(&self) -> BitCtwinEarlyDistx2R {
        BitCtwinEarlyDistx2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The time is from HOLD Time to BeaconTx or Timeout. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_beacon_area_on_2(&self) -> BitBeaconAreaOn2R {
        BitBeaconAreaOn2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Spec time and Beacon Early Time to TBTT Time. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_ctwindow_on_2(&self) -> BitCtwindowOn2R {
        BitCtwindowOn2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: On, 0: Off. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_power_state_2(&self) -> BitPowerState2R {
        BitPowerState2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bit_en_tsfbit32_rst_noa2(
        &mut self,
    ) -> BitEnTsfbit32RstNoa2W<'_, RegP2pNoa2CtrlStateSpec> {
        BitEnTsfbit32RstNoa2W::new(self, 0)
    }
    #[doc = "Bits 1:2 - The selector for the source of noa unit0. 0 for port 0, 1 for port 1, 2 for port 2"]
    #[inline(always)]
    pub fn bit_p2p_noa2_unit_sel(&mut self) -> BitP2pNoa2UnitSelW<'_, RegP2pNoa2CtrlStateSpec> {
        BitP2pNoa2UnitSelW::new(self, 1)
    }
    #[doc = "Bits 3:4 - The condition that noa_unit0, \\[1\\]-NOA1, \\[0\\]-NOA0 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa2_en(&mut self) -> BitP2pNoa2EnW<'_, RegP2pNoa2CtrlStateSpec> {
        BitP2pNoa2EnW::new(self, 3)
    }
    #[doc = "Bit 5 - A main switch for P2P PowerSaving 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn bit_p2p_noa2_pwr_mgt_en(&mut self) -> BitP2pNoa2PwrMgtEnW<'_, RegP2pNoa2CtrlStateSpec> {
        BitP2pNoa2PwrMgtEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Hw will disable TX in P2P_off period 0: Do Nothing"]
    #[inline(always)]
    pub fn bit_p2p_noa2_distx_en(&mut self) -> BitP2pNoa2DistxEnW<'_, RegP2pNoa2CtrlStateSpec> {
        BitP2pNoa2DistxEnW::new(self, 6)
    }
    #[doc = "Bit 7 - It indicates that the state from CTWindow End to BcnEarly 1: Sleep 0: Wake"]
    #[inline(always)]
    pub fn bit_p2p_noa2_allstasleep(
        &mut self,
    ) -> BitP2pNoa2AllstasleepW<'_, RegP2pNoa2CtrlStateSpec> {
        BitP2pNoa2AllstasleepW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_p2p_noa2_ctrl_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_p2p_noa2_ctrl_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegP2pNoa2CtrlStateSpec;
impl crate::RegisterSpec for RegP2pNoa2CtrlStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_p2p_noa2_ctrl_state::R`](R) reader structure"]
impl crate::Readable for RegP2pNoa2CtrlStateSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_p2p_noa2_ctrl_state::W`](W) writer structure"]
impl crate::Writable for RegP2pNoa2CtrlStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_P2P_NOA2_CTRL_STATE to value 0x01"]
impl crate::Resettable for RegP2pNoa2CtrlStateSpec {
    const RESET_VALUE: u32 = 0x01;
}
