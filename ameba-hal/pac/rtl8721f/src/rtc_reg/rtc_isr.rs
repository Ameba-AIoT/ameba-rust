#[doc = "Register `RTC_ISR` reader"]
pub type R = crate::R<RtcIsrSpec>;
#[doc = "Register `RTC_ISR` writer"]
pub type W = crate::W<RtcIsrSpec>;
#[doc = "Field `ALMWF` reader - Alarm write flag This bit is set by hardware when Alarm values can be changed, after the ALME bit has been set to '0' in RTC_CR. It is cleared by hardware when ALME bit has been set to '1' in RTC_CR. *0: Alarm update not allowed *1: Alarm update allowed"]
pub type AlmwfR = crate::BitReader;
#[doc = "Field `WUTRSF` reader - This bit is set by hardware each time the WUTE bit is copied into the shadow register. This bit is cleared by hardware in initialization mode. This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. *0: WUTE is not yet synchronized. *1: WUTE is synchronized."]
pub type WutrsfR = crate::BitReader;
#[doc = "Field `WUTWF` reader - Wakeup timer write flag This flag is set by hardware when WUT value can be changed, after the WUTE bit has been set to '0' in RTC_CR. *0: Wakeup timer configuration update not allowd except in initialization mode *1: Wakeup timer configuration update allowed"]
pub type WutwfR = crate::BitReader;
#[doc = "Field `INITS` reader - This bit is set by hardware when the calendar day field is different from 0 (RTC domain reset state). *0: Calendar has not been initialized. *1: Calendar has been initialized."]
pub type InitsR = crate::BitReader;
#[doc = "Field `RSF` reader - Registers synchronization flag. This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_TR). This bit is cleared by hardware in initialization mode or when in bypass shadow register mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. *0: Calendar shadow registers are not yet synchronized. *1: Calendar shadow registers are synchronized."]
pub type RsfR = crate::BitReader;
#[doc = "Field `INITF` reader - Initialization flag. When this bit is set to '1', the RTC is in initialization state, and the time, date, and prescaler registers can be updated. *0: Calendar registers update is not allowed. *1: Calendar registers update is allowed."]
pub type InitfR = crate::BitReader;
#[doc = "Field `INIT` reader - Initialization mode *0: Free running mode. *1: Initialization mode used to program time and date register (RTC_TR), and prescaler register (RTC_PRER). Counters are stoped and start counting from the new value when INIT is set."]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization mode *0: Free running mode. *1: Initialization mode used to program time and date register (RTC_TR), and prescaler register (RTC_PRER). Counters are stoped and start counting from the new value when INIT is set."]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMF` reader - Alarm flag This flag is set by hardware when the time register (RTC_TR) matches the Alarm registers (RTC_ALMR1L and RTC_ALMR1H)."]
pub type AlmfR = crate::BitReader;
#[doc = "Field `WUTF` reader - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches to. This flag must be cleared by software at least 2 RTCCLK periods before WUTF is set to '1' again."]
pub type WutfR = crate::BitReader;
#[doc = "Field `DOVTHF` reader - Day over threshold flag This flag is set by hardware when the Day in RTC_TR over the DAY_THRES set in RTC_CR register."]
pub type DovthfR = crate::BitReader;
#[doc = "Field `RECALPF` reader - Recalibration pending flag The RECALPF status flag is automatically set to '1' when software writes to the RTC_CALIBR register, indicating that the RTCCALIBR register is blocked. When the new calibration settings are taken into account, this bit returns to '0'. Refer to Re-calibration on-the-fly."]
pub type RecalpfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm write flag This bit is set by hardware when Alarm values can be changed, after the ALME bit has been set to '0' in RTC_CR. It is cleared by hardware when ALME bit has been set to '1' in RTC_CR. *0: Alarm update not allowed *1: Alarm update allowed"]
    #[inline(always)]
    pub fn almwf(&self) -> AlmwfR {
        AlmwfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set by hardware each time the WUTE bit is copied into the shadow register. This bit is cleared by hardware in initialization mode. This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. *0: WUTE is not yet synchronized. *1: WUTE is synchronized."]
    #[inline(always)]
    pub fn wutrsf(&self) -> WutrsfR {
        WutrsfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag This flag is set by hardware when WUT value can be changed, after the WUTE bit has been set to '0' in RTC_CR. *0: Wakeup timer configuration update not allowd except in initialization mode *1: Wakeup timer configuration update allowed"]
    #[inline(always)]
    pub fn wutwf(&self) -> WutwfR {
        WutwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set by hardware when the calendar day field is different from 0 (RTC domain reset state). *0: Calendar has not been initialized. *1: Calendar has been initialized."]
    #[inline(always)]
    pub fn inits(&self) -> InitsR {
        InitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag. This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_TR). This bit is cleared by hardware in initialization mode or when in bypass shadow register mode (BYPSHAD=1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode. *0: Calendar shadow registers are not yet synchronized. *1: Calendar shadow registers are synchronized."]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag. When this bit is set to '1', the RTC is in initialization state, and the time, date, and prescaler registers can be updated. *0: Calendar registers update is not allowed. *1: Calendar registers update is allowed."]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode *0: Free running mode. *1: Initialization mode used to program time and date register (RTC_TR), and prescaler register (RTC_PRER). Counters are stoped and start counting from the new value when INIT is set."]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm flag This flag is set by hardware when the time register (RTC_TR) matches the Alarm registers (RTC_ALMR1L and RTC_ALMR1H)."]
    #[inline(always)]
    pub fn almf(&self) -> AlmfR {
        AlmfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag This flag is set by hardware when the wakeup auto-reload counter reaches to. This flag must be cleared by software at least 2 RTCCLK periods before WUTF is set to '1' again."]
    #[inline(always)]
    pub fn wutf(&self) -> WutfR {
        WutfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Day over threshold flag This flag is set by hardware when the Day in RTC_TR over the DAY_THRES set in RTC_CR register."]
    #[inline(always)]
    pub fn dovthf(&self) -> DovthfR {
        DovthfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending flag The RECALPF status flag is automatically set to '1' when software writes to the RTC_CALIBR register, indicating that the RTCCALIBR register is blocked. When the new calibration settings are taken into account, this bit returns to '0'. Refer to Re-calibration on-the-fly."]
    #[inline(always)]
    pub fn recalpf(&self) -> RecalpfR {
        RecalpfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Initialization mode *0: Free running mode. *1: Initialization mode used to program time and date register (RTC_TR), and prescaler register (RTC_PRER). Counters are stoped and start counting from the new value when INIT is set."]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, RtcIsrSpec> {
        InitW::new(self, 7)
    }
}
#[doc = "RTC Init And Status Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcIsrSpec;
impl crate::RegisterSpec for RtcIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_isr::R`](R) reader structure"]
impl crate::Readable for RtcIsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_isr::W`](W) writer structure"]
impl crate::Writable for RtcIsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_ISR to value 0"]
impl crate::Resettable for RtcIsrSpec {}
