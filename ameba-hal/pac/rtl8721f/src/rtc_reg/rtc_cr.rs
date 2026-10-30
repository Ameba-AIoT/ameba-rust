#[doc = "Register `RTC_CR` reader"]
pub type R = crate::R<RtcCrSpec>;
#[doc = "Register `RTC_CR` writer"]
pub type W = crate::W<RtcCrSpec>;
#[doc = "Field `ADD1H` writer - Add one hour(summer time change),when this bit is set outside initialization mode,1 hour is add to the calendar time,this bit is always read as 0. *0:No effect *1:add one hour to the current time"]
pub type Add1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB1H` writer - Subtract one hour(winter time change),when this bit is set outside initialization mode,1 hour is subtracted to the calendar time if the current hour is not 0,this bit is always read as 0. *0:No effect *1:subtract one hour to the current time"]
pub type Sub1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers. *0: Calendar values (when reading from RTC_TR) are taken from the shadow register, which are updated once every two RTCCLK cycles. *1: Calendar values (when reading from RTC_TR) are taken directly from the calendar counters."]
pub type BypshadR = crate::BitReader;
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers. *0: Calendar values (when reading from RTC_TR) are taken from the shadow register, which are updated once every two RTCCLK cycles. *1: Calendar values (when reading from RTC_TR) are taken directly from the calendar counters."]
pub type BypshadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEL` reader - Output selection.There bits are used to select the flag to be routed to RTC_OUT output. *00: Output disabled *01: Alarm output enabled *10: Clock output is clk_spre (default:1Hz) *11: Clock output is clk_apre (default:256Hz)"]
pub type OselR = crate::FieldReader;
#[doc = "Field `OSEL` writer - Output selection.There bits are used to select the flag to be routed to RTC_OUT output. *00: Output disabled *01: Alarm output enabled *10: Clock output is clk_spre (default:1Hz) *11: Clock output is clk_apre (default:256Hz)"]
pub type OselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FMT` reader - Hour format *0: 24 hour/day format *1: AM/PM hour format"]
pub type FmtR = crate::BitReader;
#[doc = "Field `FMT` writer - Hour format *0: 24 hour/day format *1: AM/PM hour format"]
pub type FmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALME` reader - Alarm enable *0: Alarm disabled *1: Alarm enabled"]
pub type AlmeR = crate::BitReader;
#[doc = "Field `ALME` writer - Alarm enable *0: Alarm disabled *1: Alarm enabled"]
pub type AlmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTE` reader - Wakeup timer enable *0: Wakeup timer disabled *1: Wakeup timer enabled"]
pub type WuteR = crate::BitReader;
#[doc = "Field `WUTE` writer - Wakeup timer enable *0: Wakeup timer disabled *1: Wakeup timer enabled"]
pub type WuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMIE` reader - Alarm interrupt enable *0: Alarm interrupt disabled *1: Alarm interrupt enabled"]
pub type AlmieR = crate::BitReader;
#[doc = "Field `ALMIE` writer - Alarm interrupt enable *0: Alarm interrupt disabled *1: Alarm interrupt enabled"]
pub type AlmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable *0: Wakeup timer interrupt disabled *1: Wakeup timer interrupt enabled"]
pub type WutieR = crate::BitReader;
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable *0: Wakeup timer interrupt disabled *1: Wakeup timer interrupt enabled"]
pub type WutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOVTHIE` reader - Day over threshold interrupt enable *0: Day over threshold interrupt disabled *1: Day over threshold interrupt enabled"]
pub type DovthieR = crate::BitReader;
#[doc = "Field `DOVTHIE` writer - Day over threshold interrupt enable *0: Day over threshold interrupt disabled *1: Day over threshold interrupt enabled"]
pub type DovthieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV2_EN` reader - rtc_clk/rtc_bclk use divided clock enable *0: unuse div2 clk *1: use div2 clk"]
pub type Clkdiv2EnR = crate::BitReader;
#[doc = "Field `CLKDIV2_EN` writer - rtc_clk/rtc_bclk use divided clock enable *0: unuse div2 clk *1: use div2 clk"]
pub type Clkdiv2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAY_THRES` reader - Day threshold in binary format"]
pub type DayThresR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 2 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bypass the shadow registers. *0: Calendar values (when reading from RTC_TR) are taken from the shadow register, which are updated once every two RTCCLK cycles. *1: Calendar values (when reading from RTC_TR) are taken directly from the calendar counters."]
    #[inline(always)]
    pub fn bypshad(&self) -> BypshadR {
        BypshadR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Output selection.There bits are used to select the flag to be routed to RTC_OUT output. *00: Output disabled *01: Alarm output enabled *10: Clock output is clk_spre (default:1Hz) *11: Clock output is clk_apre (default:256Hz)"]
    #[inline(always)]
    pub fn osel(&self) -> OselR {
        OselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Hour format *0: 24 hour/day format *1: AM/PM hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm enable *0: Alarm disabled *1: Alarm enabled"]
    #[inline(always)]
    pub fn alme(&self) -> AlmeR {
        AlmeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable *0: Wakeup timer disabled *1: Wakeup timer enabled"]
    #[inline(always)]
    pub fn wute(&self) -> WuteR {
        WuteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm interrupt enable *0: Alarm interrupt disabled *1: Alarm interrupt enabled"]
    #[inline(always)]
    pub fn almie(&self) -> AlmieR {
        AlmieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable *0: Wakeup timer interrupt disabled *1: Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn wutie(&self) -> WutieR {
        WutieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Day over threshold interrupt enable *0: Day over threshold interrupt disabled *1: Day over threshold interrupt enabled"]
    #[inline(always)]
    pub fn dovthie(&self) -> DovthieR {
        DovthieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - rtc_clk/rtc_bclk use divided clock enable *0: unuse div2 clk *1: use div2 clk"]
    #[inline(always)]
    pub fn clkdiv2_en(&self) -> Clkdiv2EnR {
        Clkdiv2EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Day threshold in binary format"]
    #[inline(always)]
    pub fn day_thres(&self) -> DayThresR {
        DayThresR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Add one hour(summer time change),when this bit is set outside initialization mode,1 hour is add to the calendar time,this bit is always read as 0. *0:No effect *1:add one hour to the current time"]
    #[inline(always)]
    pub fn add1h(&mut self) -> Add1hW<'_, RtcCrSpec> {
        Add1hW::new(self, 0)
    }
    #[doc = "Bit 1 - Subtract one hour(winter time change),when this bit is set outside initialization mode,1 hour is subtracted to the calendar time if the current hour is not 0,this bit is always read as 0. *0:No effect *1:subtract one hour to the current time"]
    #[inline(always)]
    pub fn sub1h(&mut self) -> Sub1hW<'_, RtcCrSpec> {
        Sub1hW::new(self, 1)
    }
    #[doc = "Bit 2 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, RtcCrSpec> {
        BkpW::new(self, 2)
    }
    #[doc = "Bit 3 - Bypass the shadow registers. *0: Calendar values (when reading from RTC_TR) are taken from the shadow register, which are updated once every two RTCCLK cycles. *1: Calendar values (when reading from RTC_TR) are taken directly from the calendar counters."]
    #[inline(always)]
    pub fn bypshad(&mut self) -> BypshadW<'_, RtcCrSpec> {
        BypshadW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Output selection.There bits are used to select the flag to be routed to RTC_OUT output. *00: Output disabled *01: Alarm output enabled *10: Clock output is clk_spre (default:1Hz) *11: Clock output is clk_apre (default:256Hz)"]
    #[inline(always)]
    pub fn osel(&mut self) -> OselW<'_, RtcCrSpec> {
        OselW::new(self, 5)
    }
    #[doc = "Bit 7 - Hour format *0: 24 hour/day format *1: AM/PM hour format"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FmtW<'_, RtcCrSpec> {
        FmtW::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm enable *0: Alarm disabled *1: Alarm enabled"]
    #[inline(always)]
    pub fn alme(&mut self) -> AlmeW<'_, RtcCrSpec> {
        AlmeW::new(self, 8)
    }
    #[doc = "Bit 10 - Wakeup timer enable *0: Wakeup timer disabled *1: Wakeup timer enabled"]
    #[inline(always)]
    pub fn wute(&mut self) -> WuteW<'_, RtcCrSpec> {
        WuteW::new(self, 10)
    }
    #[doc = "Bit 12 - Alarm interrupt enable *0: Alarm interrupt disabled *1: Alarm interrupt enabled"]
    #[inline(always)]
    pub fn almie(&mut self) -> AlmieW<'_, RtcCrSpec> {
        AlmieW::new(self, 12)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable *0: Wakeup timer interrupt disabled *1: Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn wutie(&mut self) -> WutieW<'_, RtcCrSpec> {
        WutieW::new(self, 14)
    }
    #[doc = "Bit 16 - Day over threshold interrupt enable *0: Day over threshold interrupt disabled *1: Day over threshold interrupt enabled"]
    #[inline(always)]
    pub fn dovthie(&mut self) -> DovthieW<'_, RtcCrSpec> {
        DovthieW::new(self, 16)
    }
    #[doc = "Bit 17 - rtc_clk/rtc_bclk use divided clock enable *0: unuse div2 clk *1: use div2 clk"]
    #[inline(always)]
    pub fn clkdiv2_en(&mut self) -> Clkdiv2EnW<'_, RtcCrSpec> {
        Clkdiv2EnW::new(self, 17)
    }
}
#[doc = "RTC Ctrl Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCrSpec;
impl crate::RegisterSpec for RtcCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_cr::R`](R) reader structure"]
impl crate::Readable for RtcCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cr::W`](W) writer structure"]
impl crate::Writable for RtcCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CR to value 0xff82_0000"]
impl crate::Resettable for RtcCrSpec {
    const RESET_VALUE: u32 = 0xff82_0000;
}
