#[doc = "Register `REG_RTC_DET_CTRL0` reader"]
pub type R = crate::R<RegRtcDetCtrl0Spec>;
#[doc = "Register `REG_RTC_DET_CTRL0` writer"]
pub type W = crate::W<RegRtcDetCtrl0Spec>;
#[doc = "Field `rtc_det_en` reader - AON detect RTC 131k to see RTC domain is abnormal function enable . 1:enable 0:disable"]
pub type RtcDetEnR = crate::BitReader;
#[doc = "Field `rtc_det_en` writer - AON detect RTC 131k to see RTC domain is abnormal function enable . 1:enable 0:disable"]
pub type RtcDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_force_msk` reader - Software control to send a reset sequence to RTC domain mask bit. 1: force_rst/force_clr register will be valid 0: force_rst/force_clr register is invalid"]
pub type RtcForceMskR = crate::BitReader;
#[doc = "Field `rtc_force_msk` writer - Software control to send a reset sequence to RTC domain mask bit. 1: force_rst/force_clr register will be valid 0: force_rst/force_clr register is invalid"]
pub type RtcForceMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtc_rst_period` reader - Aon det rtc module will periodic send reset seq to RTC if no 131k detect, base on 100K cycle. The period is (this valie + 1)*10us"]
pub type RtcRstPeriodR = crate::FieldReader<u16>;
#[doc = "Field `rtc_rst_period` writer - Aon det rtc module will periodic send reset seq to RTC if no 131k detect, base on 100K cycle. The period is (this valie + 1)*10us"]
pub type RtcRstPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - AON detect RTC 131k to see RTC domain is abnormal function enable . 1:enable 0:disable"]
    #[inline(always)]
    pub fn rtc_det_en(&self) -> RtcDetEnR {
        RtcDetEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software control to send a reset sequence to RTC domain mask bit. 1: force_rst/force_clr register will be valid 0: force_rst/force_clr register is invalid"]
    #[inline(always)]
    pub fn rtc_force_msk(&self) -> RtcForceMskR {
        RtcForceMskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Aon det rtc module will periodic send reset seq to RTC if no 131k detect, base on 100K cycle. The period is (this valie + 1)*10us"]
    #[inline(always)]
    pub fn rtc_rst_period(&self) -> RtcRstPeriodR {
        RtcRstPeriodR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - AON detect RTC 131k to see RTC domain is abnormal function enable . 1:enable 0:disable"]
    #[inline(always)]
    pub fn rtc_det_en(&mut self) -> RtcDetEnW<'_, RegRtcDetCtrl0Spec> {
        RtcDetEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Software control to send a reset sequence to RTC domain mask bit. 1: force_rst/force_clr register will be valid 0: force_rst/force_clr register is invalid"]
    #[inline(always)]
    pub fn rtc_force_msk(&mut self) -> RtcForceMskW<'_, RegRtcDetCtrl0Spec> {
        RtcForceMskW::new(self, 1)
    }
    #[doc = "Bits 16:31 - Aon det rtc module will periodic send reset seq to RTC if no 131k detect, base on 100K cycle. The period is (this valie + 1)*10us"]
    #[inline(always)]
    pub fn rtc_rst_period(&mut self) -> RtcRstPeriodW<'_, RegRtcDetCtrl0Spec> {
        RtcRstPeriodW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rtc_det_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rtc_det_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRtcDetCtrl0Spec;
impl crate::RegisterSpec for RegRtcDetCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rtc_det_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegRtcDetCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rtc_det_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegRtcDetCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RTC_DET_CTRL0 to value 0x0fff_0001"]
impl crate::Resettable for RegRtcDetCtrl0Spec {
    const RESET_VALUE: u32 = 0x0fff_0001;
}
