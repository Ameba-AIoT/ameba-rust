#[doc = "Register `TIM_EN` reader"]
pub type R = crate::R<TimEnSpec>;
#[doc = "Register `TIM_EN` writer"]
pub type W = crate::W<TimEnSpec>;
#[doc = "Field `cnt_start_timer0` reader - enable the timer0 counter. 1:enable 0:disable"]
pub type CntStartTimer0R = crate::BitReader;
#[doc = "Field `cnt_start_timer0` writer - enable the timer0 counter. 1:enable 0:disable"]
pub type CntStartTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnt_start_timer1` reader - enable the timer1 counter. 1:enable 0:disable"]
pub type CntStartTimer1R = crate::BitReader;
#[doc = "Field `cnt_start_timer1` writer - enable the timer1 counter. 1:enable 0:disable"]
pub type CntStartTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnt_start_timer2` reader - enable the timer2 counter. 1:enable 0:disable"]
pub type CntStartTimer2R = crate::BitReader;
#[doc = "Field `cnt_start_timer2` writer - enable the timer2 counter. 1:enable 0:disable"]
pub type CntStartTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnt_start_timer3` reader - enable the timer3 counter. 1:enable 0:disable"]
pub type CntStartTimer3R = crate::BitReader;
#[doc = "Field `cnt_start_timer3` writer - enable the timer3 counter. 1:enable 0:disable"]
pub type CntStartTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `config_valid_timer0` reader - TIM_SET configuration valid or not 1: valid 0: not valid"]
pub type ConfigValidTimer0R = crate::BitReader;
#[doc = "Field `config_valid_timer1` reader - TIM_SET configuration valid or not 1: valid 0: not valid"]
pub type ConfigValidTimer1R = crate::BitReader;
#[doc = "Field `config_valid_timer2` reader - TIM_SET configuration valid or not 1: valid 0: not valid"]
pub type ConfigValidTimer2R = crate::BitReader;
#[doc = "Field `config_valid_timer3` reader - TIM_SET configuration valid or not. 1: valid 0: not valid if compare option enable, 1: vaild value is seted 0: set option is on going"]
pub type ConfigValidTimer3R = crate::BitReader;
#[doc = "Field `reset_timer0` writer - Writing 1 will cause a pulse reset timer0. Please wait for 3 work clk before new timer configuration 1: valid 0: not valid"]
pub type ResetTimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_timer1` writer - Writing 1 will cause a pulse reset timer1. Please wait for 3 work clk before new timer configuration 1: valid 0: not valid"]
pub type ResetTimer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_timer2` writer - Writing 1 will cause a pulse reset timer1. Please wait for 3 work clk before new timer configuration 1: valid 0: not valid"]
pub type ResetTimer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset_timer3` writer - Writing 1 will cause a pulse reset timer1. Please wait for 3 work clk before new timer configuration 1: valid 0: not valid"]
pub type ResetTimer3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable the timer0 counter. 1:enable 0:disable"]
    #[inline(always)]
    pub fn cnt_start_timer0(&self) -> CntStartTimer0R {
        CntStartTimer0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable the timer1 counter. 1:enable 0:disable"]
    #[inline(always)]
    pub fn cnt_start_timer1(&self) -> CntStartTimer1R {
        CntStartTimer1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable the timer2 counter. 1:enable 0:disable"]
    #[inline(always)]
    pub fn cnt_start_timer2(&self) -> CntStartTimer2R {
        CntStartTimer2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable the timer3 counter. 1:enable 0:disable"]
    #[inline(always)]
    pub fn cnt_start_timer3(&self) -> CntStartTimer3R {
        CntStartTimer3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM_SET configuration valid or not 1: valid 0: not valid"]
    #[inline(always)]
    pub fn config_valid_timer0(&self) -> ConfigValidTimer0R {
        ConfigValidTimer0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIM_SET configuration valid or not 1: valid 0: not valid"]
    #[inline(always)]
    pub fn config_valid_timer1(&self) -> ConfigValidTimer1R {
        ConfigValidTimer1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TIM_SET configuration valid or not 1: valid 0: not valid"]
    #[inline(always)]
    pub fn config_valid_timer2(&self) -> ConfigValidTimer2R {
        ConfigValidTimer2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM_SET configuration valid or not. 1: valid 0: not valid if compare option enable, 1: vaild value is seted 0: set option is on going"]
    #[inline(always)]
    pub fn config_valid_timer3(&self) -> ConfigValidTimer3R {
        ConfigValidTimer3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable the timer0 counter. 1:enable 0:disable"]
    #[inline(always)]
    pub fn cnt_start_timer0(&mut self) -> CntStartTimer0W<'_, TimEnSpec> {
        CntStartTimer0W::new(self, 0)
    }
    #[doc = "Bit 1 - enable the timer1 counter. 1:enable 0:disable"]
    #[inline(always)]
    pub fn cnt_start_timer1(&mut self) -> CntStartTimer1W<'_, TimEnSpec> {
        CntStartTimer1W::new(self, 1)
    }
    #[doc = "Bit 2 - enable the timer2 counter. 1:enable 0:disable"]
    #[inline(always)]
    pub fn cnt_start_timer2(&mut self) -> CntStartTimer2W<'_, TimEnSpec> {
        CntStartTimer2W::new(self, 2)
    }
    #[doc = "Bit 3 - enable the timer3 counter. 1:enable 0:disable"]
    #[inline(always)]
    pub fn cnt_start_timer3(&mut self) -> CntStartTimer3W<'_, TimEnSpec> {
        CntStartTimer3W::new(self, 3)
    }
    #[doc = "Bit 28 - Writing 1 will cause a pulse reset timer0. Please wait for 3 work clk before new timer configuration 1: valid 0: not valid"]
    #[inline(always)]
    pub fn reset_timer0(&mut self) -> ResetTimer0W<'_, TimEnSpec> {
        ResetTimer0W::new(self, 28)
    }
    #[doc = "Bit 29 - Writing 1 will cause a pulse reset timer1. Please wait for 3 work clk before new timer configuration 1: valid 0: not valid"]
    #[inline(always)]
    pub fn reset_timer1(&mut self) -> ResetTimer1W<'_, TimEnSpec> {
        ResetTimer1W::new(self, 29)
    }
    #[doc = "Bit 30 - Writing 1 will cause a pulse reset timer1. Please wait for 3 work clk before new timer configuration 1: valid 0: not valid"]
    #[inline(always)]
    pub fn reset_timer2(&mut self) -> ResetTimer2W<'_, TimEnSpec> {
        ResetTimer2W::new(self, 30)
    }
    #[doc = "Bit 31 - Writing 1 will cause a pulse reset timer1. Please wait for 3 work clk before new timer configuration 1: valid 0: not valid"]
    #[inline(always)]
    pub fn reset_timer3(&mut self) -> ResetTimer3W<'_, TimEnSpec> {
        ResetTimer3W::new(self, 31)
    }
}
#[doc = "TIM enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimEnSpec;
impl crate::RegisterSpec for TimEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_en::R`](R) reader structure"]
impl crate::Readable for TimEnSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_en::W`](W) writer structure"]
impl crate::Writable for TimEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_EN to value 0x0f0f"]
impl crate::Resettable for TimEnSpec {
    const RESET_VALUE: u32 = 0x0f0f;
}
