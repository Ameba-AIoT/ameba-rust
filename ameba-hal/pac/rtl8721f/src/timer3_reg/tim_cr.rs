#[doc = "Register `TIM_CR` reader"]
pub type R = crate::R<TimCrSpec>;
#[doc = "Register `TIM_CR` writer"]
pub type W = crate::W<TimCrSpec>;
#[doc = "Field `UDIS` reader - disable UEV. Shadow registers keep their values. Otherwise, buffered registers are then loaded with their preload values when UEV happen."]
pub type UdisR = crate::BitReader;
#[doc = "Field `UDIS` writer - disable UEV. Shadow registers keep their values. Otherwise, buffered registers are then loaded with their preload values when UEV happen."]
pub type UdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - update request source * 0: counter overflow and setting the UG bit * 1: counter overflow"]
pub type UrsR = crate::BitReader;
#[doc = "Field `URS` writer - update request source * 0: counter overflow and setting the UG bit * 1: counter overflow"]
pub type UrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM` reader - one-pulse mode * 0: counter is not stopped at UEV * 1: counter stops counting at next UEV"]
pub type OpmR = crate::BitReader;
#[doc = "Field `OPM` writer - one-pulse mode * 0: counter is not stopped at UEV * 1: counter stops counting at next UEV"]
pub type OpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARPE` reader - enable auto-reload preloading * 0: TIM_ARR register is not buffered * 1: TIM_ARR register is buffered"]
pub type ArpeR = crate::BitReader;
#[doc = "Field `ARPE` writer - enable auto-reload preloading * 0: TIM_ARR register is not buffered * 1: TIM_ARR register is buffered"]
pub type ArpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - external trigger polarity(TRGI) * 0: positive edge is active * 1: negative edge is active * 2 or 3: both edge is active Note: these bits are only valid in one-pulse mode."]
pub type EtpR = crate::FieldReader;
#[doc = "Field `ETP` writer - external trigger polarity(TRGI) * 0: positive edge is active * 1: negative edge is active * 2 or 3: both edge is active Note: these bits are only valid in one-pulse mode."]
pub type EtpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - disable UEV. Shadow registers keep their values. Otherwise, buffered registers are then loaded with their preload values when UEV happen."]
    #[inline(always)]
    pub fn udis(&self) -> UdisR {
        UdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - update request source * 0: counter overflow and setting the UG bit * 1: counter overflow"]
    #[inline(always)]
    pub fn urs(&self) -> UrsR {
        UrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - one-pulse mode * 0: counter is not stopped at UEV * 1: counter stops counting at next UEV"]
    #[inline(always)]
    pub fn opm(&self) -> OpmR {
        OpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable auto-reload preloading * 0: TIM_ARR register is not buffered * 1: TIM_ARR register is buffered"]
    #[inline(always)]
    pub fn arpe(&self) -> ArpeR {
        ArpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - external trigger polarity(TRGI) * 0: positive edge is active * 1: negative edge is active * 2 or 3: both edge is active Note: these bits are only valid in one-pulse mode."]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - disable UEV. Shadow registers keep their values. Otherwise, buffered registers are then loaded with their preload values when UEV happen."]
    #[inline(always)]
    pub fn udis(&mut self) -> UdisW<'_, TimCrSpec> {
        UdisW::new(self, 1)
    }
    #[doc = "Bit 2 - update request source * 0: counter overflow and setting the UG bit * 1: counter overflow"]
    #[inline(always)]
    pub fn urs(&mut self) -> UrsW<'_, TimCrSpec> {
        UrsW::new(self, 2)
    }
    #[doc = "Bit 3 - one-pulse mode * 0: counter is not stopped at UEV * 1: counter stops counting at next UEV"]
    #[inline(always)]
    pub fn opm(&mut self) -> OpmW<'_, TimCrSpec> {
        OpmW::new(self, 3)
    }
    #[doc = "Bit 4 - enable auto-reload preloading * 0: TIM_ARR register is not buffered * 1: TIM_ARR register is buffered"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ArpeW<'_, TimCrSpec> {
        ArpeW::new(self, 4)
    }
    #[doc = "Bits 8:9 - external trigger polarity(TRGI) * 0: positive edge is active * 1: negative edge is active * 2 or 3: both edge is active Note: these bits are only valid in one-pulse mode."]
    #[inline(always)]
    pub fn etp(&mut self) -> EtpW<'_, TimCrSpec> {
        EtpW::new(self, 8)
    }
}
#[doc = "TIM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCrSpec;
impl crate::RegisterSpec for TimCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_cr::R`](R) reader structure"]
impl crate::Readable for TimCrSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_cr::W`](W) writer structure"]
impl crate::Writable for TimCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_CR to value 0"]
impl crate::Resettable for TimCrSpec {}
