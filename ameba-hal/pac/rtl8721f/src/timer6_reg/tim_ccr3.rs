#[doc = "Register `TIM_CCR3` reader"]
pub type R = crate::R<TimCcr3Spec>;
#[doc = "Register `TIM_CCR3` writer"]
pub type W = crate::W<TimCcr3Spec>;
#[doc = "Field `CCR3` reader - Capture/Compare 3 value CC3 channel configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register(preload value). It is loaded permanently if the preload feature is not selected in the OC3PE bit.Else the preload value is copied in the active capture/compare 3 register when an UEV occurs.The active capture/compare 3 register contains the value to be compared to the counter TIM_CNT and signalled on OC3 output. CC3 channel configured as pulse mode 1: CCR3 is the counter value transferred by the last input capture event(TRGI)"]
pub type Ccr3R = crate::FieldReader<u16>;
#[doc = "Field `CCR3` writer - Capture/Compare 3 value CC3 channel configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register(preload value). It is loaded permanently if the preload feature is not selected in the OC3PE bit.Else the preload value is copied in the active capture/compare 3 register when an UEV occurs.The active capture/compare 3 register contains the value to be compared to the counter TIM_CNT and signalled on OC3 output. CC3 channel configured as pulse mode 1: CCR3 is the counter value transferred by the last input capture event(TRGI)"]
pub type Ccr3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CC3E` reader - enable CC3"]
pub type Cc3eR = crate::BitReader;
#[doc = "Field `CC3E` writer - enable CC3"]
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - enable output capture 3 preload * 0: disable preload register on CCR3. CCR3 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR3. Read/Write operations access the preload register. CCR3 preload value is loaded in the active register at each UEV."]
pub type Oc3peR = crate::BitReader;
#[doc = "Field `OC3PE` writer - enable output capture 3 preload * 0: disable preload register on CCR3. CCR3 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR3. Read/Write operations access the preload register. CCR3 preload value is loaded in the active register at each UEV."]
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - CC3 channel configured as output: * 0: OC3 active high * 1: OC3 ative low CC3 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
pub type Cc3pR = crate::BitReader;
#[doc = "Field `CC3P` writer - CC3 channel configured as output: * 0: OC3 active high * 1: OC3 ative low CC3 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
pub type Cc3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3M` reader - CC3 working mode * 0: PWM mode * 1: Input capture mode"]
pub type Cc3mR = crate::BitReader;
#[doc = "Field `CC3M` writer - CC3 working mode * 0: PWM mode * 1: Input capture mode"]
pub type Cc3mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM_DL3` reader - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
pub type OpmDl3R = crate::BitReader;
#[doc = "Field `OPM_DL3` writer - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
pub type OpmDl3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 3 value CC3 channel configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register(preload value). It is loaded permanently if the preload feature is not selected in the OC3PE bit.Else the preload value is copied in the active capture/compare 3 register when an UEV occurs.The active capture/compare 3 register contains the value to be compared to the counter TIM_CNT and signalled on OC3 output. CC3 channel configured as pulse mode 1: CCR3 is the counter value transferred by the last input capture event(TRGI)"]
    #[inline(always)]
    pub fn ccr3(&self) -> Ccr3R {
        Ccr3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - enable CC3"]
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable output capture 3 preload * 0: disable preload register on CCR3. CCR3 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR3. Read/Write operations access the preload register. CCR3 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CC3 channel configured as output: * 0: OC3 active high * 1: OC3 ative low CC3 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CC3 working mode * 0: PWM mode * 1: Input capture mode"]
    #[inline(always)]
    pub fn cc3m(&self) -> Cc3mR {
        Cc3mR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
    #[inline(always)]
    pub fn opm_dl3(&self) -> OpmDl3R {
        OpmDl3R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 3 value CC3 channel configured as output: CCR3 is the value to be loaded in the actual capture/compare 3 register(preload value). It is loaded permanently if the preload feature is not selected in the OC3PE bit.Else the preload value is copied in the active capture/compare 3 register when an UEV occurs.The active capture/compare 3 register contains the value to be compared to the counter TIM_CNT and signalled on OC3 output. CC3 channel configured as pulse mode 1: CCR3 is the counter value transferred by the last input capture event(TRGI)"]
    #[inline(always)]
    pub fn ccr3(&mut self) -> Ccr3W<'_, TimCcr3Spec> {
        Ccr3W::new(self, 0)
    }
    #[doc = "Bit 24 - enable CC3"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> Cc3eW<'_, TimCcr3Spec> {
        Cc3eW::new(self, 24)
    }
    #[doc = "Bit 25 - enable output capture 3 preload * 0: disable preload register on CCR3. CCR3 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR3. Read/Write operations access the preload register. CCR3 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> Oc3peW<'_, TimCcr3Spec> {
        Oc3peW::new(self, 25)
    }
    #[doc = "Bit 26 - CC3 channel configured as output: * 0: OC3 active high * 1: OC3 ative low CC3 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> Cc3pW<'_, TimCcr3Spec> {
        Cc3pW::new(self, 26)
    }
    #[doc = "Bit 27 - CC3 working mode * 0: PWM mode * 1: Input capture mode"]
    #[inline(always)]
    pub fn cc3m(&mut self) -> Cc3mW<'_, TimCcr3Spec> {
        Cc3mW::new(self, 27)
    }
    #[doc = "Bit 29 - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
    #[inline(always)]
    pub fn opm_dl3(&mut self) -> OpmDl3W<'_, TimCcr3Spec> {
        OpmDl3W::new(self, 29)
    }
}
#[doc = "TIM Capture/Compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCcr3Spec;
impl crate::RegisterSpec for TimCcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_ccr3::R`](R) reader structure"]
impl crate::Readable for TimCcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_ccr3::W`](W) writer structure"]
impl crate::Writable for TimCcr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_CCR3 to value 0"]
impl crate::Resettable for TimCcr3Spec {}
