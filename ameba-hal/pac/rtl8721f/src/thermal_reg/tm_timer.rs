#[doc = "Register `TM_TIMER` reader"]
pub type R = crate::R<TmTimerSpec>;
#[doc = "Register `TM_TIMER` writer"]
pub type W = crate::W<TmTimerSpec>;
#[doc = "Field `time_period` reader - Time period of auto power on"]
pub type TimePeriodR = crate::FieldReader;
#[doc = "Field `time_period` writer - Time period of auto power on"]
pub type TimePeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Time period of auto power on"]
    #[inline(always)]
    pub fn time_period(&self) -> TimePeriodR {
        TimePeriodR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time period of auto power on"]
    #[inline(always)]
    pub fn time_period(&mut self) -> TimePeriodW<'_, TmTimerSpec> {
        TimePeriodW::new(self, 0)
    }
}
#[doc = "Thermal Meter Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmTimerSpec;
impl crate::RegisterSpec for TmTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_timer::R`](R) reader structure"]
impl crate::Readable for TmTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`tm_timer::W`](W) writer structure"]
impl crate::Writable for TmTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_TIMER to value 0x0f"]
impl crate::Resettable for TmTimerSpec {
    const RESET_VALUE: u32 = 0x0f;
}
