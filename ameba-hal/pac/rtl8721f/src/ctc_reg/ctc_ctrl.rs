#[doc = "Register `CTC_CTRL` reader"]
pub type R = crate::R<CtcCtrlSpec>;
#[doc = "Register `CTC_CTRL` writer"]
pub type W = crate::W<CtcCtrlSpec>;
#[doc = "Field `ct_enable` reader - 0: CTC disable 1: CTC enable"]
pub type CtEnableR = crate::BitReader;
#[doc = "Field `ct_enable` writer - 0: CTC disable 1: CTC enable"]
pub type CtEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `debounce_en` reader - 0: Disable 1: Enable"]
pub type DebounceEnR = crate::BitReader;
#[doc = "Field `debounce_en` writer - 0: Disable 1: Enable"]
pub type DebounceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `debounce_time` reader - 2'b00: 2 times 2'b01: 3 times 2'b10: 4 times 2'b11: 5 times"]
pub type DebounceTimeR = crate::FieldReader;
#[doc = "Field `debounce_time` writer - 2'b00: 2 times 2'b01: 3 times 2'b10: 4 times 2'b11: 5 times"]
pub type DebounceTimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `baseline_ini` reader - 1: Baseline initial function enable, HW will clear this bit to \"0\" after baseline initial"]
pub type BaselineIniR = crate::BitReader;
#[doc = "Field `baseline_ini` writer - 1: Baseline initial function enable, HW will clear this bit to \"0\" after baseline initial"]
pub type BaselineIniW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: CTC disable 1: CTC enable"]
    #[inline(always)]
    pub fn ct_enable(&self) -> CtEnableR {
        CtEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn debounce_en(&self) -> DebounceEnR {
        DebounceEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 2'b00: 2 times 2'b01: 3 times 2'b10: 4 times 2'b11: 5 times"]
    #[inline(always)]
    pub fn debounce_time(&self) -> DebounceTimeR {
        DebounceTimeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - 1: Baseline initial function enable, HW will clear this bit to \"0\" after baseline initial"]
    #[inline(always)]
    pub fn baseline_ini(&self) -> BaselineIniR {
        BaselineIniR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: CTC disable 1: CTC enable"]
    #[inline(always)]
    pub fn ct_enable(&mut self) -> CtEnableW<'_, CtcCtrlSpec> {
        CtEnableW::new(self, 0)
    }
    #[doc = "Bit 4 - 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn debounce_en(&mut self) -> DebounceEnW<'_, CtcCtrlSpec> {
        DebounceEnW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 2'b00: 2 times 2'b01: 3 times 2'b10: 4 times 2'b11: 5 times"]
    #[inline(always)]
    pub fn debounce_time(&mut self) -> DebounceTimeW<'_, CtcCtrlSpec> {
        DebounceTimeW::new(self, 5)
    }
    #[doc = "Bit 8 - 1: Baseline initial function enable, HW will clear this bit to \"0\" after baseline initial"]
    #[inline(always)]
    pub fn baseline_ini(&mut self) -> BaselineIniW<'_, CtcCtrlSpec> {
        BaselineIniW::new(self, 8)
    }
}
#[doc = "Captouch Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcCtrlSpec;
impl crate::RegisterSpec for CtcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_ctrl::R`](R) reader structure"]
impl crate::Readable for CtcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_ctrl::W`](W) writer structure"]
impl crate::Writable for CtcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_CTRL to value 0x0100"]
impl crate::Resettable for CtcCtrlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
