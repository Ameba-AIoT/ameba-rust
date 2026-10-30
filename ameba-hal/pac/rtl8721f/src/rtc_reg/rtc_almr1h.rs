#[doc = "Register `RTC_ALMR1H` reader"]
pub type R = crate::R<RtcAlmr1hSpec>;
#[doc = "Register `RTC_ALMR1H` writer"]
pub type W = crate::W<RtcAlmr1hSpec>;
#[doc = "Field `ALR_DAY` reader - DAY in binary format"]
pub type AlrDayR = crate::FieldReader<u16>;
#[doc = "Field `ALR_DAY` writer - DAY in binary format"]
pub type AlrDayW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MSK3` reader - Alarm day mask *0: Alarm set if the days match *1: Days don't care in Alarm comprison"]
pub type Msk3R = crate::BitReader;
#[doc = "Field `MSK3` writer - Alarm day mask *0: Alarm set if the days match *1: Days don't care in Alarm comprison"]
pub type Msk3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - DAY in binary format"]
    #[inline(always)]
    pub fn alr_day(&self) -> AlrDayR {
        AlrDayR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Alarm day mask *0: Alarm set if the days match *1: Days don't care in Alarm comprison"]
    #[inline(always)]
    pub fn msk3(&self) -> Msk3R {
        Msk3R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - DAY in binary format"]
    #[inline(always)]
    pub fn alr_day(&mut self) -> AlrDayW<'_, RtcAlmr1hSpec> {
        AlrDayW::new(self, 0)
    }
    #[doc = "Bit 9 - Alarm day mask *0: Alarm set if the days match *1: Days don't care in Alarm comprison"]
    #[inline(always)]
    pub fn msk3(&mut self) -> Msk3W<'_, RtcAlmr1hSpec> {
        Msk3W::new(self, 9)
    }
}
#[doc = "- RTC Alarm 1 Reg High\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_almr1h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_almr1h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlmr1hSpec;
impl crate::RegisterSpec for RtcAlmr1hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_almr1h::R`](R) reader structure"]
impl crate::Readable for RtcAlmr1hSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_almr1h::W`](W) writer structure"]
impl crate::Writable for RtcAlmr1hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_ALMR1H to value 0"]
impl crate::Resettable for RtcAlmr1hSpec {}
