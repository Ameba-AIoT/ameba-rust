#[doc = "Register `CTC_OV_MODE` reader"]
pub type R = crate::R<CtcOvModeSpec>;
#[doc = "Register `CTC_OV_MODE` writer"]
pub type W = crate::W<CtcOvModeSpec>;
#[doc = "Field `ov_wait_timer_cnt` reader - when sample mode is taggered sequence,it determines how long to wait before starting the next periodic sampling after each chn has been sampled in turn. It can the unit is sample clock. When this register is set to 0 , HW will scan continuously and have no sleep time."]
pub type OvWaitTimerCntR = crate::FieldReader<u32>;
#[doc = "Field `ov_wait_timer_cnt` writer - when sample mode is taggered sequence,it determines how long to wait before starting the next periodic sampling after each chn has been sampled in turn. It can the unit is sample clock. When this register is set to 0 , HW will scan continuously and have no sleep time."]
pub type OvWaitTimerCntW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `ctc_trig_mode` reader - 0:ch_en trig 1: adc dummy chn trig"]
pub type CtcTrigModeR = crate::BitReader;
#[doc = "Field `ctc_trig_mode` writer - 0:ch_en trig 1: adc dummy chn trig"]
pub type CtcTrigModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ov_mode` reader - sample mode 1-All sample conversions done in staggered sequence 0-All sample conversions done in regular sequence"]
pub type OvModeR = crate::BitReader;
#[doc = "Field `ov_mode` writer - sample mode 1-All sample conversions done in staggered sequence 0-All sample conversions done in regular sequence"]
pub type OvModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - when sample mode is taggered sequence,it determines how long to wait before starting the next periodic sampling after each chn has been sampled in turn. It can the unit is sample clock. When this register is set to 0 , HW will scan continuously and have no sleep time."]
    #[inline(always)]
    pub fn ov_wait_timer_cnt(&self) -> OvWaitTimerCntR {
        OvWaitTimerCntR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 30 - 0:ch_en trig 1: adc dummy chn trig"]
    #[inline(always)]
    pub fn ctc_trig_mode(&self) -> CtcTrigModeR {
        CtcTrigModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - sample mode 1-All sample conversions done in staggered sequence 0-All sample conversions done in regular sequence"]
    #[inline(always)]
    pub fn ov_mode(&self) -> OvModeR {
        OvModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - when sample mode is taggered sequence,it determines how long to wait before starting the next periodic sampling after each chn has been sampled in turn. It can the unit is sample clock. When this register is set to 0 , HW will scan continuously and have no sleep time."]
    #[inline(always)]
    pub fn ov_wait_timer_cnt(&mut self) -> OvWaitTimerCntW<'_, CtcOvModeSpec> {
        OvWaitTimerCntW::new(self, 0)
    }
    #[doc = "Bit 30 - 0:ch_en trig 1: adc dummy chn trig"]
    #[inline(always)]
    pub fn ctc_trig_mode(&mut self) -> CtcTrigModeW<'_, CtcOvModeSpec> {
        CtcTrigModeW::new(self, 30)
    }
    #[doc = "Bit 31 - sample mode 1-All sample conversions done in staggered sequence 0-All sample conversions done in regular sequence"]
    #[inline(always)]
    pub fn ov_mode(&mut self) -> OvModeW<'_, CtcOvModeSpec> {
        OvModeW::new(self, 31)
    }
}
#[doc = "Captouch Overmaple Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_ov_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_ov_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcOvModeSpec;
impl crate::RegisterSpec for CtcOvModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_ov_mode::R`](R) reader structure"]
impl crate::Readable for CtcOvModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_ov_mode::W`](W) writer structure"]
impl crate::Writable for CtcOvModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_OV_MODE to value 0"]
impl crate::Resettable for CtcOvModeSpec {}
