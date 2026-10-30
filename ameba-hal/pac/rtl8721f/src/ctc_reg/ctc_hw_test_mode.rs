#[doc = "Register `CTC_HW_TestMode` reader"]
pub type R = crate::R<CtcHwTestModeSpec>;
#[doc = "Register `CTC_HW_TestMode` writer"]
pub type W = crate::W<CtcHwTestModeSpec>;
#[doc = "Field `hw_tmode_en` reader - 0: hardware test mode disabled 1: hardware test mode enabled, the clock sent to AUX Analog from captouch is tie to fixed value, which is depent on hw_clock_tie_value( 410h\\[1\\] )"]
pub type HwTmodeEnR = crate::BitReader;
#[doc = "Field `hw_tmode_en` writer - 0: hardware test mode disabled 1: hardware test mode enabled, the clock sent to AUX Analog from captouch is tie to fixed value, which is depent on hw_clock_tie_value( 410h\\[1\\] )"]
pub type HwTmodeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hw_clock_tie_value` reader - this value is the clock tied value in captouch hardware test mode"]
pub type HwClockTieValueR = crate::BitReader;
#[doc = "Field `hw_clock_tie_value` writer - this value is the clock tied value in captouch hardware test mode"]
pub type HwClockTieValueW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: hardware test mode disabled 1: hardware test mode enabled, the clock sent to AUX Analog from captouch is tie to fixed value, which is depent on hw_clock_tie_value( 410h\\[1\\] )"]
    #[inline(always)]
    pub fn hw_tmode_en(&self) -> HwTmodeEnR {
        HwTmodeEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this value is the clock tied value in captouch hardware test mode"]
    #[inline(always)]
    pub fn hw_clock_tie_value(&self) -> HwClockTieValueR {
        HwClockTieValueR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: hardware test mode disabled 1: hardware test mode enabled, the clock sent to AUX Analog from captouch is tie to fixed value, which is depent on hw_clock_tie_value( 410h\\[1\\] )"]
    #[inline(always)]
    pub fn hw_tmode_en(&mut self) -> HwTmodeEnW<'_, CtcHwTestModeSpec> {
        HwTmodeEnW::new(self, 0)
    }
    #[doc = "Bit 1 - this value is the clock tied value in captouch hardware test mode"]
    #[inline(always)]
    pub fn hw_clock_tie_value(&mut self) -> HwClockTieValueW<'_, CtcHwTestModeSpec> {
        HwClockTieValueW::new(self, 1)
    }
}
#[doc = "Captouch_Hardware_Test_Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_hw_test_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_hw_test_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcHwTestModeSpec;
impl crate::RegisterSpec for CtcHwTestModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_hw_test_mode::R`](R) reader structure"]
impl crate::Readable for CtcHwTestModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_hw_test_mode::W`](W) writer structure"]
impl crate::Writable for CtcHwTestModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_HW_TestMode to value 0"]
impl crate::Resettable for CtcHwTestModeSpec {}
