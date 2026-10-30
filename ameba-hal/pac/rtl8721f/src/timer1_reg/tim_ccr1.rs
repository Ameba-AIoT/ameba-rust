#[doc = "Register `TIM_CCR1` reader"]
pub type R = crate::R<TimCcr1Spec>;
#[doc = "Register `TIM_CCR1` writer"]
pub type W = crate::W<TimCcr1Spec>;
#[doc = "Field `CCR1` reader - Capture/Compare 1 value CC1 channel configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register(preload value). It is loaded permanently if the preload feature is not selected in the OC1PE bit.Else the preload value is copied in the active capture/compare 1 register when an UEV occurs.The active capture/compare 1 register contains the value to be compared to the counter TIM_CNT and signalled on OC1 output. CC1 channel configured as pulse mode 1: CCR1 is the counter value transferred by the last input capture event(TRGI)"]
pub type Ccr1R = crate::FieldReader<u16>;
#[doc = "Field `CCR1` writer - Capture/Compare 1 value CC1 channel configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register(preload value). It is loaded permanently if the preload feature is not selected in the OC1PE bit.Else the preload value is copied in the active capture/compare 1 register when an UEV occurs.The active capture/compare 1 register contains the value to be compared to the counter TIM_CNT and signalled on OC1 output. CC1 channel configured as pulse mode 1: CCR1 is the counter value transferred by the last input capture event(TRGI)"]
pub type Ccr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CC1E` reader - enable CC1"]
pub type Cc1eR = crate::BitReader;
#[doc = "Field `CC1E` writer - enable CC1"]
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - enable output capture 1 preload * 0: disable preload register on CCR1. CCR1 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR1. Read/Write operations access the preload register. CCR1 preload value is loaded in the active register at each UEV."]
pub type Oc1peR = crate::BitReader;
#[doc = "Field `OC1PE` writer - enable output capture 1 preload * 0: disable preload register on CCR1. CCR1 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR1. Read/Write operations access the preload register. CCR1 preload value is loaded in the active register at each UEV."]
pub type Oc1peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - CC1 channel configured as output: * 0: OC1 active high * 1: OC1 ative low CC1 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
pub type Cc1pR = crate::BitReader;
#[doc = "Field `CC1P` writer - CC1 channel configured as output: * 0: OC1 active high * 1: OC1 ative low CC1 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1M` reader - CC1 working mode * 0: PWM mode * 1: Input capture mode"]
pub type Cc1mR = crate::BitReader;
#[doc = "Field `CC1M` writer - CC1 working mode * 0: PWM mode * 1: Input capture mode"]
pub type Cc1mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM_DL1` reader - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
pub type OpmDl1R = crate::BitReader;
#[doc = "Field `OPM_DL1` writer - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
pub type OpmDl1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 1 value CC1 channel configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register(preload value). It is loaded permanently if the preload feature is not selected in the OC1PE bit.Else the preload value is copied in the active capture/compare 1 register when an UEV occurs.The active capture/compare 1 register contains the value to be compared to the counter TIM_CNT and signalled on OC1 output. CC1 channel configured as pulse mode 1: CCR1 is the counter value transferred by the last input capture event(TRGI)"]
    #[inline(always)]
    pub fn ccr1(&self) -> Ccr1R {
        Ccr1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - enable CC1"]
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable output capture 1 preload * 0: disable preload register on CCR1. CCR1 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR1. Read/Write operations access the preload register. CCR1 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CC1 channel configured as output: * 0: OC1 active high * 1: OC1 ative low CC1 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CC1 working mode * 0: PWM mode * 1: Input capture mode"]
    #[inline(always)]
    pub fn cc1m(&self) -> Cc1mR {
        Cc1mR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
    #[inline(always)]
    pub fn opm_dl1(&self) -> OpmDl1R {
        OpmDl1R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 1 value CC1 channel configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register(preload value). It is loaded permanently if the preload feature is not selected in the OC1PE bit.Else the preload value is copied in the active capture/compare 1 register when an UEV occurs.The active capture/compare 1 register contains the value to be compared to the counter TIM_CNT and signalled on OC1 output. CC1 channel configured as pulse mode 1: CCR1 is the counter value transferred by the last input capture event(TRGI)"]
    #[inline(always)]
    pub fn ccr1(&mut self) -> Ccr1W<'_, TimCcr1Spec> {
        Ccr1W::new(self, 0)
    }
    #[doc = "Bit 24 - enable CC1"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> Cc1eW<'_, TimCcr1Spec> {
        Cc1eW::new(self, 24)
    }
    #[doc = "Bit 25 - enable output capture 1 preload * 0: disable preload register on CCR1. CCR1 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR1. Read/Write operations access the preload register. CCR1 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> Oc1peW<'_, TimCcr1Spec> {
        Oc1peW::new(self, 25)
    }
    #[doc = "Bit 26 - CC1 channel configured as output: * 0: OC1 active high * 1: OC1 ative low CC1 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> Cc1pW<'_, TimCcr1Spec> {
        Cc1pW::new(self, 26)
    }
    #[doc = "Bit 27 - CC1 working mode * 0: PWM mode * 1: Input capture mode"]
    #[inline(always)]
    pub fn cc1m(&mut self) -> Cc1mW<'_, TimCcr1Spec> {
        Cc1mW::new(self, 27)
    }
    #[doc = "Bit 29 - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
    #[inline(always)]
    pub fn opm_dl1(&mut self) -> OpmDl1W<'_, TimCcr1Spec> {
        OpmDl1W::new(self, 29)
    }
}
#[doc = "TIM Capture/Compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCcr1Spec;
impl crate::RegisterSpec for TimCcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_ccr1::R`](R) reader structure"]
impl crate::Readable for TimCcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_ccr1::W`](W) writer structure"]
impl crate::Writable for TimCcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_CCR1 to value 0"]
impl crate::Resettable for TimCcr1Spec {}
