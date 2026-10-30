#[doc = "Register `RTC_ALMR1L` reader"]
pub type R = crate::R<RtcAlmr1lSpec>;
#[doc = "Register `RTC_ALMR1L` writer"]
pub type W = crate::W<RtcAlmr1lSpec>;
#[doc = "Field `ALR_SU` reader - Second units in BCD format"]
pub type AlrSuR = crate::FieldReader;
#[doc = "Field `ALR_SU` writer - Second units in BCD format"]
pub type AlrSuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALR_ST` reader - Second tens in BCD format"]
pub type AlrStR = crate::FieldReader;
#[doc = "Field `ALR_ST` writer - Second tens in BCD format"]
pub type AlrStW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK0` reader - Alarm seconds mask *0: Alarm set if the seconds match *1: Seconds don't care in Alarm comprison"]
pub type Msk0R = crate::BitReader;
#[doc = "Field `MSK0` writer - Alarm seconds mask *0: Alarm set if the seconds match *1: Seconds don't care in Alarm comprison"]
pub type Msk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALR_MNU` reader - Minute units in BCD format"]
pub type AlrMnuR = crate::FieldReader;
#[doc = "Field `ALR_MNU` writer - Minute units in BCD format"]
pub type AlrMnuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALR_MNT` reader - Minute tens in BCD format"]
pub type AlrMntR = crate::FieldReader;
#[doc = "Field `ALR_MNT` writer - Minute tens in BCD format"]
pub type AlrMntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK1` reader - Alarm minutes mask. *0: Alarm set if the minutes match *1: Minutes don't care in Alarm comprison"]
pub type Msk1R = crate::BitReader;
#[doc = "Field `MSK1` writer - Alarm minutes mask. *0: Alarm set if the minutes match *1: Minutes don't care in Alarm comprison"]
pub type Msk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALR_HU` reader - Hour units in BCD format"]
pub type AlrHuR = crate::FieldReader;
#[doc = "Field `ALR_HU` writer - Hour units in BCD format"]
pub type AlrHuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALR_HT` reader - Hour tens in BCD format"]
pub type AlrHtR = crate::FieldReader;
#[doc = "Field `ALR_HT` writer - Hour tens in BCD format"]
pub type AlrHtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALR_PM` reader - AM/PM notation. *0: AM or 24-hour format *1: PM"]
pub type AlrPmR = crate::BitReader;
#[doc = "Field `ALR_PM` writer - AM/PM notation. *0: AM or 24-hour format *1: PM"]
pub type AlrPmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSK2` reader - Alarm hour mask *0: Alarm set if the hours match *1: Hours don't care in Alarm comparison"]
pub type Msk2R = crate::BitReader;
#[doc = "Field `MSK2` writer - Alarm hour mask *0: Alarm set if the hours match *1: Hours don't care in Alarm comparison"]
pub type Msk2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn alr_su(&self) -> AlrSuR {
        AlrSuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn alr_st(&self) -> AlrStR {
        AlrStR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm seconds mask *0: Alarm set if the seconds match *1: Seconds don't care in Alarm comprison"]
    #[inline(always)]
    pub fn msk0(&self) -> Msk0R {
        Msk0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn alr_mnu(&self) -> AlrMnuR {
        AlrMnuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn alr_mnt(&self) -> AlrMntR {
        AlrMntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes mask. *0: Alarm set if the minutes match *1: Minutes don't care in Alarm comprison"]
    #[inline(always)]
    pub fn msk1(&self) -> Msk1R {
        Msk1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn alr_hu(&self) -> AlrHuR {
        AlrHuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn alr_ht(&self) -> AlrHtR {
        AlrHtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation. *0: AM or 24-hour format *1: PM"]
    #[inline(always)]
    pub fn alr_pm(&self) -> AlrPmR {
        AlrPmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm hour mask *0: Alarm set if the hours match *1: Hours don't care in Alarm comparison"]
    #[inline(always)]
    pub fn msk2(&self) -> Msk2R {
        Msk2R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn alr_su(&mut self) -> AlrSuW<'_, RtcAlmr1lSpec> {
        AlrSuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn alr_st(&mut self) -> AlrStW<'_, RtcAlmr1lSpec> {
        AlrStW::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm seconds mask *0: Alarm set if the seconds match *1: Seconds don't care in Alarm comprison"]
    #[inline(always)]
    pub fn msk0(&mut self) -> Msk0W<'_, RtcAlmr1lSpec> {
        Msk0W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn alr_mnu(&mut self) -> AlrMnuW<'_, RtcAlmr1lSpec> {
        AlrMnuW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn alr_mnt(&mut self) -> AlrMntW<'_, RtcAlmr1lSpec> {
        AlrMntW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm minutes mask. *0: Alarm set if the minutes match *1: Minutes don't care in Alarm comprison"]
    #[inline(always)]
    pub fn msk1(&mut self) -> Msk1W<'_, RtcAlmr1lSpec> {
        Msk1W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn alr_hu(&mut self) -> AlrHuW<'_, RtcAlmr1lSpec> {
        AlrHuW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn alr_ht(&mut self) -> AlrHtW<'_, RtcAlmr1lSpec> {
        AlrHtW::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation. *0: AM or 24-hour format *1: PM"]
    #[inline(always)]
    pub fn alr_pm(&mut self) -> AlrPmW<'_, RtcAlmr1lSpec> {
        AlrPmW::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm hour mask *0: Alarm set if the hours match *1: Hours don't care in Alarm comparison"]
    #[inline(always)]
    pub fn msk2(&mut self) -> Msk2W<'_, RtcAlmr1lSpec> {
        Msk2W::new(self, 23)
    }
}
#[doc = "RTC Alarm 1 Reg Low\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_almr1l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_almr1l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAlmr1lSpec;
impl crate::RegisterSpec for RtcAlmr1lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_almr1l::R`](R) reader structure"]
impl crate::Readable for RtcAlmr1lSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_almr1l::W`](W) writer structure"]
impl crate::Writable for RtcAlmr1lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_ALMR1L to value 0"]
impl crate::Resettable for RtcAlmr1lSpec {}
