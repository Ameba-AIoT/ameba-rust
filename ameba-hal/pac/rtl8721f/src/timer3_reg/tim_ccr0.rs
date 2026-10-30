#[doc = "Register `TIM_CCR0` reader"]
pub type R = crate::R<TimCcr0Spec>;
#[doc = "Register `TIM_CCR0` writer"]
pub type W = crate::W<TimCcr0Spec>;
#[doc = "Field `CCR0` reader - Capture/Compare 0 value CC0 channel configured as output: CCR0 is the value to be loaded in the actual capture/compare 0 register(preload value). It is loaded permanently if the preload feature is not selected in the OC0PE bit.Else the preload value is copied in the active capture/compare 0 register when an UEV occurs.The active capture/compare 0 register contains the value to be compared to the counter TIM_CNT and signalled on OC0 output. CC0 channel configured as pulse mode 1: CCR0 is the counter value transferred by the last input capture event(TRGI)"]
pub type Ccr0R = crate::FieldReader<u16>;
#[doc = "Field `CCR0` writer - Capture/Compare 0 value CC0 channel configured as output: CCR0 is the value to be loaded in the actual capture/compare 0 register(preload value). It is loaded permanently if the preload feature is not selected in the OC0PE bit.Else the preload value is copied in the active capture/compare 0 register when an UEV occurs.The active capture/compare 0 register contains the value to be compared to the counter TIM_CNT and signalled on OC0 output. CC0 channel configured as pulse mode 1: CCR0 is the counter value transferred by the last input capture event(TRGI)"]
pub type Ccr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CC0E` reader - enable CC0"]
pub type Cc0eR = crate::BitReader;
#[doc = "Field `CC0E` writer - enable CC0"]
pub type Cc0eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC0PE` reader - enable output capture 0 preload * 0: disable preload register on CCR0. CCR0 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR0. Read/Write operations access the preload register. CCR0 preload value is loaded in the active register at each UEV."]
pub type Oc0peR = crate::BitReader;
#[doc = "Field `OC0PE` writer - enable output capture 0 preload * 0: disable preload register on CCR0. CCR0 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR0. Read/Write operations access the preload register. CCR0 preload value is loaded in the active register at each UEV."]
pub type Oc0peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0P` reader - CC0 channel configured as output: * 0: OC0 active high * 1: OC0 ative low CC0 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
pub type Cc0pR = crate::BitReader;
#[doc = "Field `CC0P` writer - CC0 channel configured as output: * 0: OC0 active high * 1: OC0 ative low CC0 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
pub type Cc0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0M` reader - CC0 working mode * 0: PWM mode * 1: Input capture mode"]
pub type Cc0mR = crate::BitReader;
#[doc = "Field `CC0M` writer - CC0 working mode * 0: PWM mode * 1: Input capture mode"]
pub type Cc0mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM_DL0` reader - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
pub type OpmDl0R = crate::BitReader;
#[doc = "Field `OPM_DL0` writer - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
pub type OpmDl0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 0 value CC0 channel configured as output: CCR0 is the value to be loaded in the actual capture/compare 0 register(preload value). It is loaded permanently if the preload feature is not selected in the OC0PE bit.Else the preload value is copied in the active capture/compare 0 register when an UEV occurs.The active capture/compare 0 register contains the value to be compared to the counter TIM_CNT and signalled on OC0 output. CC0 channel configured as pulse mode 1: CCR0 is the counter value transferred by the last input capture event(TRGI)"]
    #[inline(always)]
    pub fn ccr0(&self) -> Ccr0R {
        Ccr0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - enable CC0"]
    #[inline(always)]
    pub fn cc0e(&self) -> Cc0eR {
        Cc0eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable output capture 0 preload * 0: disable preload register on CCR0. CCR0 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR0. Read/Write operations access the preload register. CCR0 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn oc0pe(&self) -> Oc0peR {
        Oc0peR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CC0 channel configured as output: * 0: OC0 active high * 1: OC0 ative low CC0 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
    #[inline(always)]
    pub fn cc0p(&self) -> Cc0pR {
        Cc0pR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CC0 working mode * 0: PWM mode * 1: Input capture mode"]
    #[inline(always)]
    pub fn cc0m(&self) -> Cc0mR {
        Cc0mR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
    #[inline(always)]
    pub fn opm_dl0(&self) -> OpmDl0R {
        OpmDl0R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 0 value CC0 channel configured as output: CCR0 is the value to be loaded in the actual capture/compare 0 register(preload value). It is loaded permanently if the preload feature is not selected in the OC0PE bit.Else the preload value is copied in the active capture/compare 0 register when an UEV occurs.The active capture/compare 0 register contains the value to be compared to the counter TIM_CNT and signalled on OC0 output. CC0 channel configured as pulse mode 1: CCR0 is the counter value transferred by the last input capture event(TRGI)"]
    #[inline(always)]
    pub fn ccr0(&mut self) -> Ccr0W<'_, TimCcr0Spec> {
        Ccr0W::new(self, 0)
    }
    #[doc = "Bit 24 - enable CC0"]
    #[inline(always)]
    pub fn cc0e(&mut self) -> Cc0eW<'_, TimCcr0Spec> {
        Cc0eW::new(self, 24)
    }
    #[doc = "Bit 25 - enable output capture 0 preload * 0: disable preload register on CCR0. CCR0 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR0. Read/Write operations access the preload register. CCR0 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn oc0pe(&mut self) -> Oc0peW<'_, TimCcr0Spec> {
        Oc0peW::new(self, 25)
    }
    #[doc = "Bit 26 - CC0 channel configured as output: * 0: OC0 active high * 1: OC0 ative low CC0 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
    #[inline(always)]
    pub fn cc0p(&mut self) -> Cc0pW<'_, TimCcr0Spec> {
        Cc0pW::new(self, 26)
    }
    #[doc = "Bit 27 - CC0 working mode * 0: PWM mode * 1: Input capture mode"]
    #[inline(always)]
    pub fn cc0m(&mut self) -> Cc0mW<'_, TimCcr0Spec> {
        Cc0mW::new(self, 27)
    }
    #[doc = "Bit 29 - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
    #[inline(always)]
    pub fn opm_dl0(&mut self) -> OpmDl0W<'_, TimCcr0Spec> {
        OpmDl0W::new(self, 29)
    }
}
#[doc = "TIM Capture/Compare register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_ccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCcr0Spec;
impl crate::RegisterSpec for TimCcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_ccr0::R`](R) reader structure"]
impl crate::Readable for TimCcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_ccr0::W`](W) writer structure"]
impl crate::Writable for TimCcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_CCR0 to value 0"]
impl crate::Resettable for TimCcr0Spec {}
