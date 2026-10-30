#[doc = "Register `TIM_DIER` reader"]
pub type R = crate::R<TimDierSpec>;
#[doc = "Register `TIM_DIER` writer"]
pub type W = crate::W<TimDierSpec>;
#[doc = "Field `UIE` reader - enable counter update interrupt"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - enable counter update interrupt"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0IE` reader - enable Capture/Compare 0 interrupt"]
pub type Cc0ieR = crate::BitReader;
#[doc = "Field `CC0IE` writer - enable Capture/Compare 0 interrupt"]
pub type Cc0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - enable Capture/Compare 1 interrupt"]
pub type Cc1ieR = crate::BitReader;
#[doc = "Field `CC1IE` writer - enable Capture/Compare 1 interrupt"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - enable Capture/Compare 2 interrupt"]
pub type Cc2ieR = crate::BitReader;
#[doc = "Field `CC2IE` writer - enable Capture/Compare 2 interrupt"]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IE` reader - enable Capture/Compare 3 interrupt"]
pub type Cc3ieR = crate::BitReader;
#[doc = "Field `CC3IE` writer - enable Capture/Compare 3 interrupt"]
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIE0` reader - enable channel 0 update interrupt"]
pub type Uie0R = crate::BitReader;
#[doc = "Field `UIE0` writer - enable channel 0 update interrupt"]
pub type Uie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIE1` reader - enable channel 1 update interrupt"]
pub type Uie1R = crate::BitReader;
#[doc = "Field `UIE1` writer - enable channel 1 update interrupt"]
pub type Uie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIE2` reader - enable channel 2 update interrupt"]
pub type Uie2R = crate::BitReader;
#[doc = "Field `UIE2` writer - enable channel 2 update interrupt"]
pub type Uie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIE3` reader - enable channel 3 update interrupt"]
pub type Uie3R = crate::BitReader;
#[doc = "Field `UIE3` writer - enable channel 3 update interrupt"]
pub type Uie3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable counter update interrupt"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable Capture/Compare 0 interrupt"]
    #[inline(always)]
    pub fn cc0ie(&self) -> Cc0ieR {
        Cc0ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable Capture/Compare 1 interrupt"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable Capture/Compare 2 interrupt"]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable Capture/Compare 3 interrupt"]
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - enable channel 0 update interrupt"]
    #[inline(always)]
    pub fn uie0(&self) -> Uie0R {
        Uie0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - enable channel 1 update interrupt"]
    #[inline(always)]
    pub fn uie1(&self) -> Uie1R {
        Uie1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - enable channel 2 update interrupt"]
    #[inline(always)]
    pub fn uie2(&self) -> Uie2R {
        Uie2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - enable channel 3 update interrupt"]
    #[inline(always)]
    pub fn uie3(&self) -> Uie3R {
        Uie3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable counter update interrupt"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, TimDierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - enable Capture/Compare 0 interrupt"]
    #[inline(always)]
    pub fn cc0ie(&mut self) -> Cc0ieW<'_, TimDierSpec> {
        Cc0ieW::new(self, 1)
    }
    #[doc = "Bit 2 - enable Capture/Compare 1 interrupt"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<'_, TimDierSpec> {
        Cc1ieW::new(self, 2)
    }
    #[doc = "Bit 3 - enable Capture/Compare 2 interrupt"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> Cc2ieW<'_, TimDierSpec> {
        Cc2ieW::new(self, 3)
    }
    #[doc = "Bit 4 - enable Capture/Compare 3 interrupt"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> Cc3ieW<'_, TimDierSpec> {
        Cc3ieW::new(self, 4)
    }
    #[doc = "Bit 16 - enable channel 0 update interrupt"]
    #[inline(always)]
    pub fn uie0(&mut self) -> Uie0W<'_, TimDierSpec> {
        Uie0W::new(self, 16)
    }
    #[doc = "Bit 17 - enable channel 1 update interrupt"]
    #[inline(always)]
    pub fn uie1(&mut self) -> Uie1W<'_, TimDierSpec> {
        Uie1W::new(self, 17)
    }
    #[doc = "Bit 18 - enable channel 2 update interrupt"]
    #[inline(always)]
    pub fn uie2(&mut self) -> Uie2W<'_, TimDierSpec> {
        Uie2W::new(self, 18)
    }
    #[doc = "Bit 19 - enable channel 3 update interrupt"]
    #[inline(always)]
    pub fn uie3(&mut self) -> Uie3W<'_, TimDierSpec> {
        Uie3W::new(self, 19)
    }
}
#[doc = "TIM interupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimDierSpec;
impl crate::RegisterSpec for TimDierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_dier::R`](R) reader structure"]
impl crate::Readable for TimDierSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_dier::W`](W) writer structure"]
impl crate::Writable for TimDierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_DIER to value 0"]
impl crate::Resettable for TimDierSpec {}
