#[doc = "Register `RTC_YEAR` reader"]
pub type R = crate::R<RtcYearSpec>;
#[doc = "Register `RTC_YEAR` writer"]
pub type W = crate::W<RtcYearSpec>;
#[doc = "Field `YEAR` reader - Year in binary format"]
pub type YearR = crate::FieldReader;
#[doc = "Field `YEAR` writer - Year in binary format"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESTORE` reader - Reset Flag. Indicates whether any reset conditions have occurred (except POR/PDR/BOD), so that the bit is set to '1' before reset, and then the global variable that determines whether the recovery time information is needed depending on whether the bit is '1'."]
pub type RestoreR = crate::BitReader;
#[doc = "Field `RESTORE` writer - Reset Flag. Indicates whether any reset conditions have occurred (except POR/PDR/BOD), so that the bit is set to '1' before reset, and then the global variable that determines whether the recovery time information is needed depending on whether the bit is '1'."]
pub type RestoreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Year in binary format"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Reset Flag. Indicates whether any reset conditions have occurred (except POR/PDR/BOD), so that the bit is set to '1' before reset, and then the global variable that determines whether the recovery time information is needed depending on whether the bit is '1'."]
    #[inline(always)]
    pub fn restore(&self) -> RestoreR {
        RestoreR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Year in binary format"]
    #[inline(always)]
    pub fn year(&mut self) -> YearW<'_, RtcYearSpec> {
        YearW::new(self, 0)
    }
    #[doc = "Bit 31 - Reset Flag. Indicates whether any reset conditions have occurred (except POR/PDR/BOD), so that the bit is set to '1' before reset, and then the global variable that determines whether the recovery time information is needed depending on whether the bit is '1'."]
    #[inline(always)]
    pub fn restore(&mut self) -> RestoreW<'_, RtcYearSpec> {
        RestoreW::new(self, 31)
    }
}
#[doc = "RTC Year Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_year::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_year::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcYearSpec;
impl crate::RegisterSpec for RtcYearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_year::R`](R) reader structure"]
impl crate::Readable for RtcYearSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_year::W`](W) writer structure"]
impl crate::Writable for RtcYearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_YEAR to value 0"]
impl crate::Resettable for RtcYearSpec {}
