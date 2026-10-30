#[doc = "Register `TIM_DIER` reader"]
pub type R = crate::R<TimDierSpec>;
#[doc = "Register `TIM_DIER` writer"]
pub type W = crate::W<TimDierSpec>;
#[doc = "Field `uie_timer0` reader - 1: enable update interrupt 0: disable update interrupt"]
pub type UieTimer0R = crate::BitReader;
#[doc = "Field `uie_timer0` writer - 1: enable update interrupt 0: disable update interrupt"]
pub type UieTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uie_timer1` reader - 1: enable update interrupt 0: disable update interrupt"]
pub type UieTimer1R = crate::BitReader;
#[doc = "Field `uie_timer1` writer - 1: enable update interrupt 0: disable update interrupt"]
pub type UieTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uie_timer2` reader - 1: enable update interrupt 0: disable update interrupt"]
pub type UieTimer2R = crate::BitReader;
#[doc = "Field `uie_timer2` writer - 1: enable update interrupt 0: disable update interrupt"]
pub type UieTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uie_timer3` reader - 1: enable update interrupt 0: disable update interrupt"]
pub type UieTimer3R = crate::BitReader;
#[doc = "Field `uie_timer3` writer - 1: enable update interrupt 0: disable update interrupt"]
pub type UieTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: enable update interrupt 0: disable update interrupt"]
    #[inline(always)]
    pub fn uie_timer0(&self) -> UieTimer0R {
        UieTimer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: enable update interrupt 0: disable update interrupt"]
    #[inline(always)]
    pub fn uie_timer1(&self) -> UieTimer1R {
        UieTimer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: enable update interrupt 0: disable update interrupt"]
    #[inline(always)]
    pub fn uie_timer2(&self) -> UieTimer2R {
        UieTimer2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: enable update interrupt 0: disable update interrupt"]
    #[inline(always)]
    pub fn uie_timer3(&self) -> UieTimer3R {
        UieTimer3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: enable update interrupt 0: disable update interrupt"]
    #[inline(always)]
    pub fn uie_timer0(&mut self) -> UieTimer0W<'_, TimDierSpec> {
        UieTimer0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: enable update interrupt 0: disable update interrupt"]
    #[inline(always)]
    pub fn uie_timer1(&mut self) -> UieTimer1W<'_, TimDierSpec> {
        UieTimer1W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: enable update interrupt 0: disable update interrupt"]
    #[inline(always)]
    pub fn uie_timer2(&mut self) -> UieTimer2W<'_, TimDierSpec> {
        UieTimer2W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: enable update interrupt 0: disable update interrupt"]
    #[inline(always)]
    pub fn uie_timer3(&mut self) -> UieTimer3W<'_, TimDierSpec> {
        UieTimer3W::new(self, 3)
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
