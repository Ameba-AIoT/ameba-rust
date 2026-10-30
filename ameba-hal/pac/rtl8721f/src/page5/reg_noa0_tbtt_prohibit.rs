#[doc = "Register `REG_NOA0_TBTT_PROHIBIT` reader"]
pub type R = crate::R<RegNoa0TbttProhibitSpec>;
#[doc = "Register `REG_NOA0_TBTT_PROHIBIT` writer"]
pub type W = crate::W<RegNoa0TbttProhibitSpec>;
#[doc = "Field `BIT_TBTT_PROHIBIT_SETUP` reader - TBTT_prohibit_setup, unit is 32us"]
pub type BitTbttProhibitSetupR = crate::FieldReader;
#[doc = "Field `BIT_TBTT_PROHIBIT_SETUP` writer - TBTT_prohibit_setup, unit is 32us"]
pub type BitTbttProhibitSetupW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_TBTT_HOLD_TIME_INFRA` reader - TBTT_prohibit_hold time for infrastructure mode , unit is 2 ms"]
pub type BitTbttHoldTimeInfraR = crate::FieldReader;
#[doc = "Field `BIT_TBTT_HOLD_TIME_INFRA` writer - TBTT_prohibit_hold time for infrastructure mode , unit is 2 ms"]
pub type BitTbttHoldTimeInfraW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_TBTT_HOLD_TIME` reader - TBTT_prohibit_hold time for AP mode, unit is 32 us"]
pub type BitTbttHoldTimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TBTT_HOLD_TIME` writer - TBTT_prohibit_hold time for AP mode, unit is 32 us"]
pub type BitTbttHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_EN_TBTT_AREA_FOR_BB` reader - enable notify bb that mac p0 is in tbtt area"]
pub type BitEnTbttAreaForBbR = crate::BitReader;
#[doc = "Field `BIT_EN_TBTT_AREA_FOR_BB` writer - enable notify bb that mac p0 is in tbtt area"]
pub type BitEnTbttAreaForBbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::BitReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_RXPRSP_UPD` reader - 1: enable update tsft when RX probe response frame"]
pub type BitEnRxprspUpdR = crate::BitReader;
#[doc = "Field `BIT_EN_RXPRSP_UPD` writer - 1: enable update tsft when RX probe response frame"]
pub type BitEnRxprspUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_AUTO_MASK_RXPRSP_UPD` reader - valid when BIT_EN_RXPRSP_UPD = 1 1: auto disable RXPRSP's tsft update when rx probe response frame during bcn_early to tbtt"]
pub type BitEnAutoMaskRxprspUpdR = crate::BitReader;
#[doc = "Field `BIT_EN_AUTO_MASK_RXPRSP_UPD` writer - valid when BIT_EN_RXPRSP_UPD = 1 1: auto disable RXPRSP's tsft update when rx probe response frame during bcn_early to tbtt"]
pub type BitEnAutoMaskRxprspUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SPEC_FORCE_DOZE0_0` reader - If NoA_Count == 1, this value is 1. else 0."]
pub type BitSpecForceDoze0_0R = crate::BitReader;
#[doc = "Field `BIT_SPEC_NOA0_OFF_PERIOD_0` reader - Spec time"]
pub type BitSpecNoa0OffPeriod0R = crate::BitReader;
#[doc = "Field `BIT_SPEC_FORCE_DOZE1_0` reader - If NoA_Count == 1, this value is 1. else 0."]
pub type BitSpecForceDoze1_0R = crate::BitReader;
#[doc = "Field `BIT_SPEC_NOA1_OFF_PERIOD_0` reader - Spec time"]
pub type BitSpecNoa1OffPeriod0R = crate::BitReader;
#[doc = "Field `BIT_SPEC_CTWIN_EARLY_DISTX_0` reader - ctwin_early_distx"]
pub type BitSpecCtwinEarlyDistx0R = crate::BitReader;
#[doc = "Field `BIT_SPEC_BEACON_AREA_ON_0` reader - The time is from TBTT to BeaconTx or Timeout"]
pub type BitSpecBeaconAreaOn0R = crate::BitReader;
#[doc = "Field `BIT_SPEC_CTWINDOW_ON_0` reader - The time from TBTT <-> CTWEND (with BCNDMATIM early time)"]
pub type BitSpecCtwindowOn0R = crate::BitReader;
#[doc = "Field `BIT_SPEC_POWER_STATE_0` reader - 1: On, 0: Off"]
pub type BitSpecPowerState0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - TBTT_prohibit_setup, unit is 32us"]
    #[inline(always)]
    pub fn bit_tbtt_prohibit_setup(&self) -> BitTbttProhibitSetupR {
        BitTbttProhibitSetupR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - TBTT_prohibit_hold time for infrastructure mode , unit is 2 ms"]
    #[inline(always)]
    pub fn bit_tbtt_hold_time_infra(&self) -> BitTbttHoldTimeInfraR {
        BitTbttHoldTimeInfraR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:19 - TBTT_prohibit_hold time for AP mode, unit is 32 us"]
    #[inline(always)]
    pub fn bit_tbtt_hold_time(&self) -> BitTbttHoldTimeR {
        BitTbttHoldTimeR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 20 - enable notify bb that mac p0 is in tbtt area"]
    #[inline(always)]
    pub fn bit_en_tbtt_area_for_bb(&self) -> BitEnTbttAreaForBbR {
        BitEnTbttAreaForBbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: enable update tsft when RX probe response frame"]
    #[inline(always)]
    pub fn bit_en_rxprsp_upd(&self) -> BitEnRxprspUpdR {
        BitEnRxprspUpdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - valid when BIT_EN_RXPRSP_UPD = 1 1: auto disable RXPRSP's tsft update when rx probe response frame during bcn_early to tbtt"]
    #[inline(always)]
    pub fn bit_en_auto_mask_rxprsp_upd(&self) -> BitEnAutoMaskRxprspUpdR {
        BitEnAutoMaskRxprspUpdR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If NoA_Count == 1, this value is 1. else 0."]
    #[inline(always)]
    pub fn bit_spec_force_doze0_0(&self) -> BitSpecForceDoze0_0R {
        BitSpecForceDoze0_0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Spec time"]
    #[inline(always)]
    pub fn bit_spec_noa0_off_period_0(&self) -> BitSpecNoa0OffPeriod0R {
        BitSpecNoa0OffPeriod0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If NoA_Count == 1, this value is 1. else 0."]
    #[inline(always)]
    pub fn bit_spec_force_doze1_0(&self) -> BitSpecForceDoze1_0R {
        BitSpecForceDoze1_0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Spec time"]
    #[inline(always)]
    pub fn bit_spec_noa1_off_period_0(&self) -> BitSpecNoa1OffPeriod0R {
        BitSpecNoa1OffPeriod0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ctwin_early_distx"]
    #[inline(always)]
    pub fn bit_spec_ctwin_early_distx_0(&self) -> BitSpecCtwinEarlyDistx0R {
        BitSpecCtwinEarlyDistx0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The time is from TBTT to BeaconTx or Timeout"]
    #[inline(always)]
    pub fn bit_spec_beacon_area_on_0(&self) -> BitSpecBeaconAreaOn0R {
        BitSpecBeaconAreaOn0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The time from TBTT <-> CTWEND (with BCNDMATIM early time)"]
    #[inline(always)]
    pub fn bit_spec_ctwindow_on_0(&self) -> BitSpecCtwindowOn0R {
        BitSpecCtwindowOn0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: On, 0: Off"]
    #[inline(always)]
    pub fn bit_spec_power_state_0(&self) -> BitSpecPowerState0R {
        BitSpecPowerState0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TBTT_prohibit_setup, unit is 32us"]
    #[inline(always)]
    pub fn bit_tbtt_prohibit_setup(
        &mut self,
    ) -> BitTbttProhibitSetupW<'_, RegNoa0TbttProhibitSpec> {
        BitTbttProhibitSetupW::new(self, 0)
    }
    #[doc = "Bits 4:7 - TBTT_prohibit_hold time for infrastructure mode , unit is 2 ms"]
    #[inline(always)]
    pub fn bit_tbtt_hold_time_infra(
        &mut self,
    ) -> BitTbttHoldTimeInfraW<'_, RegNoa0TbttProhibitSpec> {
        BitTbttHoldTimeInfraW::new(self, 4)
    }
    #[doc = "Bits 8:19 - TBTT_prohibit_hold time for AP mode, unit is 32 us"]
    #[inline(always)]
    pub fn bit_tbtt_hold_time(&mut self) -> BitTbttHoldTimeW<'_, RegNoa0TbttProhibitSpec> {
        BitTbttHoldTimeW::new(self, 8)
    }
    #[doc = "Bit 20 - enable notify bb that mac p0 is in tbtt area"]
    #[inline(always)]
    pub fn bit_en_tbtt_area_for_bb(&mut self) -> BitEnTbttAreaForBbW<'_, RegNoa0TbttProhibitSpec> {
        BitEnTbttAreaForBbW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegNoa0TbttProhibitSpec> {
        DummyW::new(self, 21)
    }
    #[doc = "Bit 22 - 1: enable update tsft when RX probe response frame"]
    #[inline(always)]
    pub fn bit_en_rxprsp_upd(&mut self) -> BitEnRxprspUpdW<'_, RegNoa0TbttProhibitSpec> {
        BitEnRxprspUpdW::new(self, 22)
    }
    #[doc = "Bit 23 - valid when BIT_EN_RXPRSP_UPD = 1 1: auto disable RXPRSP's tsft update when rx probe response frame during bcn_early to tbtt"]
    #[inline(always)]
    pub fn bit_en_auto_mask_rxprsp_upd(
        &mut self,
    ) -> BitEnAutoMaskRxprspUpdW<'_, RegNoa0TbttProhibitSpec> {
        BitEnAutoMaskRxprspUpdW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa0_tbtt_prohibit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa0_tbtt_prohibit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoa0TbttProhibitSpec;
impl crate::RegisterSpec for RegNoa0TbttProhibitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa0_tbtt_prohibit::R`](R) reader structure"]
impl crate::Readable for RegNoa0TbttProhibitSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa0_tbtt_prohibit::W`](W) writer structure"]
impl crate::Writable for RegNoa0TbttProhibitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA0_TBTT_PROHIBIT to value 0x00c0_c812"]
impl crate::Resettable for RegNoa0TbttProhibitSpec {
    const RESET_VALUE: u32 = 0x00c0_c812;
}
