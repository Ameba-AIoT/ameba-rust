#[doc = "Register `RTC_CALIBR` reader"]
pub type R = crate::R<RtcCalibrSpec>;
#[doc = "Register `RTC_CALIBR` writer"]
pub type W = crate::W<RtcCalibrSpec>;
#[doc = "Field `DC` reader - Digital calibration"]
pub type DcR = crate::FieldReader;
#[doc = "Field `DC` writer - Digital calibration"]
pub type DcW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DCS` reader - Digital calibration signal *0: Positive calibration, time update frequency is increased. *1: Negative calibration, time update frequency is decreased."]
pub type DcsR = crate::BitReader;
#[doc = "Field `DCS` writer - Digital calibration signal *0: Positive calibration, time update frequency is increased. *1: Negative calibration, time update frequency is decreased."]
pub type DcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCE` reader - Digital calibration enable *0: Digital calibration disabled *1: Digital calibration enabled"]
pub type DceR = crate::BitReader;
#[doc = "Field `DCE` writer - Digital calibration enable *0: Digital calibration disabled *1: Digital calibration enabled"]
pub type DceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALP` reader - Calibration period"]
pub type CalpR = crate::FieldReader;
#[doc = "Field `CALP` writer - Calibration period"]
pub type CalpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - Digital calibration"]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 14 - Digital calibration signal *0: Positive calibration, time update frequency is increased. *1: Negative calibration, time update frequency is decreased."]
    #[inline(always)]
    pub fn dcs(&self) -> DcsR {
        DcsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Digital calibration enable *0: Digital calibration disabled *1: Digital calibration enabled"]
    #[inline(always)]
    pub fn dce(&self) -> DceR {
        DceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Calibration period"]
    #[inline(always)]
    pub fn calp(&self) -> CalpR {
        CalpR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Digital calibration"]
    #[inline(always)]
    pub fn dc(&mut self) -> DcW<'_, RtcCalibrSpec> {
        DcW::new(self, 0)
    }
    #[doc = "Bit 14 - Digital calibration signal *0: Positive calibration, time update frequency is increased. *1: Negative calibration, time update frequency is decreased."]
    #[inline(always)]
    pub fn dcs(&mut self) -> DcsW<'_, RtcCalibrSpec> {
        DcsW::new(self, 14)
    }
    #[doc = "Bit 15 - Digital calibration enable *0: Digital calibration disabled *1: Digital calibration enabled"]
    #[inline(always)]
    pub fn dce(&mut self) -> DceW<'_, RtcCalibrSpec> {
        DceW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Calibration period"]
    #[inline(always)]
    pub fn calp(&mut self) -> CalpW<'_, RtcCalibrSpec> {
        CalpW::new(self, 16)
    }
}
#[doc = "RTC Calibration Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_calibr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_calibr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCalibrSpec;
impl crate::RegisterSpec for RtcCalibrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_calibr::R`](R) reader structure"]
impl crate::Readable for RtcCalibrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_calibr::W`](W) writer structure"]
impl crate::Writable for RtcCalibrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CALIBR to value 0"]
impl crate::Resettable for RtcCalibrSpec {}
