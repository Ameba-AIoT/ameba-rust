#[doc = "Register `TIM_PSYNC0` reader"]
pub type R = crate::R<TimPsync0Spec>;
#[doc = "Register `TIM_PSYNC0` writer"]
pub type W = crate::W<TimPsync0Spec>;
#[doc = "Field `SYNCPHASE0` reader - the deviation value of channel0 to the reference counter"]
pub type Syncphase0R = crate::FieldReader<u16>;
#[doc = "Field `SYNCPHASE0` writer - the deviation value of channel0 to the reference counter"]
pub type Syncphase0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SYNCPE0` reader - enable sync phase parameter preload * 0: disable preload register on PSYNC0. PSYNC0 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC0. Read/Write operations access the preload register. PSYNC0 preload value is loaded in the active register at each UEV."]
pub type Syncpe0R = crate::BitReader;
#[doc = "Field `SYNCPE0` writer - enable sync phase parameter preload * 0: disable preload register on PSYNC0. PSYNC0 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC0. Read/Write operations access the preload register. PSYNC0 preload value is loaded in the active register at each UEV."]
pub type Syncpe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDIR0` reader - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
pub type Syncdir0R = crate::BitReader;
#[doc = "Field `SYNCDIR0` writer - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
pub type Syncdir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN0` reader - enable phase sync function in channel 0. Phase sync takes effect only in output PWM mode."]
pub type Syncen0R = crate::BitReader;
#[doc = "Field `SYNCEN0` writer - enable phase sync function in channel 0. Phase sync takes effect only in output PWM mode."]
pub type Syncen0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - the deviation value of channel0 to the reference counter"]
    #[inline(always)]
    pub fn syncphase0(&self) -> Syncphase0R {
        Syncphase0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 25 - enable sync phase parameter preload * 0: disable preload register on PSYNC0. PSYNC0 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC0. Read/Write operations access the preload register. PSYNC0 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn syncpe0(&self) -> Syncpe0R {
        Syncpe0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
    #[inline(always)]
    pub fn syncdir0(&self) -> Syncdir0R {
        Syncdir0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable phase sync function in channel 0. Phase sync takes effect only in output PWM mode."]
    #[inline(always)]
    pub fn syncen0(&self) -> Syncen0R {
        Syncen0R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the deviation value of channel0 to the reference counter"]
    #[inline(always)]
    pub fn syncphase0(&mut self) -> Syncphase0W<'_, TimPsync0Spec> {
        Syncphase0W::new(self, 0)
    }
    #[doc = "Bit 25 - enable sync phase parameter preload * 0: disable preload register on PSYNC0. PSYNC0 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC0. Read/Write operations access the preload register. PSYNC0 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn syncpe0(&mut self) -> Syncpe0W<'_, TimPsync0Spec> {
        Syncpe0W::new(self, 25)
    }
    #[doc = "Bit 26 - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
    #[inline(always)]
    pub fn syncdir0(&mut self) -> Syncdir0W<'_, TimPsync0Spec> {
        Syncdir0W::new(self, 26)
    }
    #[doc = "Bit 27 - enable phase sync function in channel 0. Phase sync takes effect only in output PWM mode."]
    #[inline(always)]
    pub fn syncen0(&mut self) -> Syncen0W<'_, TimPsync0Spec> {
        Syncen0W::new(self, 27)
    }
}
#[doc = "TIM Phase Synchronization register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_psync0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_psync0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPsync0Spec;
impl crate::RegisterSpec for TimPsync0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_psync0::R`](R) reader structure"]
impl crate::Readable for TimPsync0Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_psync0::W`](W) writer structure"]
impl crate::Writable for TimPsync0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_PSYNC0 to value 0"]
impl crate::Resettable for TimPsync0Spec {}
