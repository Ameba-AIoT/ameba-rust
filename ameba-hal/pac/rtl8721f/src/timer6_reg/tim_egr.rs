#[doc = "Register `TIM_EGR` writer"]
pub type W = crate::W<TimEgrSpec>;
#[doc = "Field `UG` writer - update generation. Re-initialize the counter and generate an update of registers. Note that the prescaler counter is cleared too.(anyway the prescaler ratio is not affected)"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0G` writer - Capture/Compare 0 generation This bit is set by software and is automatically cleared by hardware. channel CC0 configured as output: The CC0IF flag is set, the corresponding interrupt or request is sent if enabled. Channel CC0 configured as input: The current value of counter is capture in CCR0 field of TIM_CCR0 register. The CC0IF flag is set, the corresponding interrupt or request is sent if enabled."]
pub type Cc0gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - refer to CC0G description"]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - refer to CC0G description"]
pub type Cc2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - refer to CC0G description"]
pub type Cc3gW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - update generation. Re-initialize the counter and generate an update of registers. Note that the prescaler counter is cleared too.(anyway the prescaler ratio is not affected)"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<'_, TimEgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 generation This bit is set by software and is automatically cleared by hardware. channel CC0 configured as output: The CC0IF flag is set, the corresponding interrupt or request is sent if enabled. Channel CC0 configured as input: The current value of counter is capture in CCR0 field of TIM_CCR0 register. The CC0IF flag is set, the corresponding interrupt or request is sent if enabled."]
    #[inline(always)]
    pub fn cc0g(&mut self) -> Cc0gW<'_, TimEgrSpec> {
        Cc0gW::new(self, 1)
    }
    #[doc = "Bit 2 - refer to CC0G description"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<'_, TimEgrSpec> {
        Cc1gW::new(self, 2)
    }
    #[doc = "Bit 3 - refer to CC0G description"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> Cc2gW<'_, TimEgrSpec> {
        Cc2gW::new(self, 3)
    }
    #[doc = "Bit 4 - refer to CC0G description"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> Cc3gW<'_, TimEgrSpec> {
        Cc3gW::new(self, 4)
    }
}
#[doc = "TIM event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimEgrSpec;
impl crate::RegisterSpec for TimEgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tim_egr::W`](W) writer structure"]
impl crate::Writable for TimEgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_EGR to value 0"]
impl crate::Resettable for TimEgrSpec {}
