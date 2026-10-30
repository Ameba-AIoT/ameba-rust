#[doc = "Register `TIM_CCR2` reader"]
pub type R = crate::R<TimCcr2Spec>;
#[doc = "Register `TIM_CCR2` writer"]
pub type W = crate::W<TimCcr2Spec>;
#[doc = "Field `CCR2` reader - Capture/Compare 2 value CC2 channel configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register(preload value). It is loaded permanently if the preload feature is not selected in the OC2PE bit.Else the preload value is copied in the active capture/compare 2 register when an UEV occurs.The active capture/compare 2 register contains the value to be compared to the counter TIM_CNT and signalled on OC2 output. CC2 channel configured as pulse mode 1: CCR2 is the counter value transferred by the last input capture event(TRGI)"]
pub type Ccr2R = crate::FieldReader<u16>;
#[doc = "Field `CCR2` writer - Capture/Compare 2 value CC2 channel configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register(preload value). It is loaded permanently if the preload feature is not selected in the OC2PE bit.Else the preload value is copied in the active capture/compare 2 register when an UEV occurs.The active capture/compare 2 register contains the value to be compared to the counter TIM_CNT and signalled on OC2 output. CC2 channel configured as pulse mode 1: CCR2 is the counter value transferred by the last input capture event(TRGI)"]
pub type Ccr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CC2E` reader - enable CC2"]
pub type Cc2eR = crate::BitReader;
#[doc = "Field `CC2E` writer - enable CC2"]
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - enable output capture 2 preload * 0: disable preload register on CCR2. CCR2 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR2. Read/Write operations access the preload register. CCR2 preload value is loaded in the active register at each UEV."]
pub type Oc2peR = crate::BitReader;
#[doc = "Field `OC2PE` writer - enable output capture 2 preload * 0: disable preload register on CCR2. CCR2 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR2. Read/Write operations access the preload register. CCR2 preload value is loaded in the active register at each UEV."]
pub type Oc2peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - CC2 channel configured as output: * 0: OC2 active high * 1: OC2 ative low CC2 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
pub type Cc2pR = crate::BitReader;
#[doc = "Field `CC2P` writer - CC2 channel configured as output: * 0: OC2 active high * 1: OC2 ative low CC2 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2M` reader - CC2 working mode * 0: PWM mode * 1: Input capture mode"]
pub type Cc2mR = crate::BitReader;
#[doc = "Field `CC2M` writer - CC2 working mode * 0: PWM mode * 1: Input capture mode"]
pub type Cc2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM_DL2` reader - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
pub type OpmDl2R = crate::BitReader;
#[doc = "Field `OPM_DL2` writer - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
pub type OpmDl2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 2 value CC2 channel configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register(preload value). It is loaded permanently if the preload feature is not selected in the OC2PE bit.Else the preload value is copied in the active capture/compare 2 register when an UEV occurs.The active capture/compare 2 register contains the value to be compared to the counter TIM_CNT and signalled on OC2 output. CC2 channel configured as pulse mode 1: CCR2 is the counter value transferred by the last input capture event(TRGI)"]
    #[inline(always)]
    pub fn ccr2(&self) -> Ccr2R {
        Ccr2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - enable CC2"]
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable output capture 2 preload * 0: disable preload register on CCR2. CCR2 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR2. Read/Write operations access the preload register. CCR2 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn oc2pe(&self) -> Oc2peR {
        Oc2peR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CC2 channel configured as output: * 0: OC2 active high * 1: OC2 ative low CC2 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CC2 working mode * 0: PWM mode * 1: Input capture mode"]
    #[inline(always)]
    pub fn cc2m(&self) -> Cc2mR {
        Cc2mR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
    #[inline(always)]
    pub fn opm_dl2(&self) -> OpmDl2R {
        OpmDl2R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 2 value CC2 channel configured as output: CCR2 is the value to be loaded in the actual capture/compare 2 register(preload value). It is loaded permanently if the preload feature is not selected in the OC2PE bit.Else the preload value is copied in the active capture/compare 2 register when an UEV occurs.The active capture/compare 2 register contains the value to be compared to the counter TIM_CNT and signalled on OC2 output. CC2 channel configured as pulse mode 1: CCR2 is the counter value transferred by the last input capture event(TRGI)"]
    #[inline(always)]
    pub fn ccr2(&mut self) -> Ccr2W<'_, TimCcr2Spec> {
        Ccr2W::new(self, 0)
    }
    #[doc = "Bit 24 - enable CC2"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> Cc2eW<'_, TimCcr2Spec> {
        Cc2eW::new(self, 24)
    }
    #[doc = "Bit 25 - enable output capture 2 preload * 0: disable preload register on CCR2. CCR2 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on CCR2. Read/Write operations access the preload register. CCR2 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn oc2pe(&mut self) -> Oc2peW<'_, TimCcr2Spec> {
        Oc2peW::new(self, 25)
    }
    #[doc = "Bit 26 - CC2 channel configured as output: * 0: OC2 active high * 1: OC2 ative low CC2 channel configured as input: * 0: positive edge of TRGI is active for capture * 1: negative edge of TRGI is active for capture"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> Cc2pW<'_, TimCcr2Spec> {
        Cc2pW::new(self, 26)
    }
    #[doc = "Bit 27 - CC2 working mode * 0: PWM mode * 1: Input capture mode"]
    #[inline(always)]
    pub fn cc2m(&mut self) -> Cc2mW<'_, TimCcr2Spec> {
        Cc2mW::new(self, 27)
    }
    #[doc = "Bit 29 - One Pulse Mode default level 0: default level is 0 1: default level is 1 this bit takes effect only in PWM output mode's One-Pulse-Mode"]
    #[inline(always)]
    pub fn opm_dl2(&mut self) -> OpmDl2W<'_, TimCcr2Spec> {
        OpmDl2W::new(self, 29)
    }
}
#[doc = "TIM Capture/Compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCcr2Spec;
impl crate::RegisterSpec for TimCcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_ccr2::R`](R) reader structure"]
impl crate::Readable for TimCcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_ccr2::W`](W) writer structure"]
impl crate::Writable for TimCcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_CCR2 to value 0"]
impl crate::Resettable for TimCcr2Spec {}
