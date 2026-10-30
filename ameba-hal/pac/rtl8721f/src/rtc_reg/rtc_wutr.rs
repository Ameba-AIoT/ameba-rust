#[doc = "Register `RTC_WUTR` reader"]
pub type R = crate::R<RtcWutrSpec>;
#[doc = "Register `RTC_WUTR` writer"]
pub type W = crate::W<RtcWutrSpec>;
#[doc = "Field `WUT` reader - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE is set to '1'), the WUTF flag is set every (WUT\\[16:0\\]+1) cycles. The first assertion of WUTF occurs between WUT and (WUT+1) cycles after WUTE is set."]
pub type WutR = crate::FieldReader<u32>;
#[doc = "Field `WUT` writer - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE is set to '1'), the WUTF flag is set every (WUT\\[16:0\\]+1) cycles. The first assertion of WUTF occurs between WUT and (WUT+1) cycles after WUTE is set."]
pub type WutW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE is set to '1'), the WUTF flag is set every (WUT\\[16:0\\]+1) cycles. The first assertion of WUTF occurs between WUT and (WUT+1) cycles after WUTE is set."]
    #[inline(always)]
    pub fn wut(&self) -> WutR {
        WutR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Wakeup auto-reload value bits When the wakeup timer is enabled (WUTE is set to '1'), the WUTF flag is set every (WUT\\[16:0\\]+1) cycles. The first assertion of WUTF occurs between WUT and (WUT+1) cycles after WUTE is set."]
    #[inline(always)]
    pub fn wut(&mut self) -> WutW<'_, RtcWutrSpec> {
        WutW::new(self, 0)
    }
}
#[doc = "RTC Wakeup Timer Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_wutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcWutrSpec;
impl crate::RegisterSpec for RtcWutrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_wutr::R`](R) reader structure"]
impl crate::Readable for RtcWutrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_wutr::W`](W) writer structure"]
impl crate::Writable for RtcWutrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_WUTR to value 0"]
impl crate::Resettable for RtcWutrSpec {}
