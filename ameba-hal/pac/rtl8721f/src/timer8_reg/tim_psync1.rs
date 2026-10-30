#[doc = "Register `TIM_PSYNC1` reader"]
pub type R = crate::R<TimPsync1Spec>;
#[doc = "Register `TIM_PSYNC1` writer"]
pub type W = crate::W<TimPsync1Spec>;
#[doc = "Field `SYNCPHASE1` reader - the deviation value of channel1 to the reference counter"]
pub type Syncphase1R = crate::FieldReader<u16>;
#[doc = "Field `SYNCPHASE1` writer - the deviation value of channel1 to the reference counter"]
pub type Syncphase1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SYNCPE1` reader - enable sync phase parameter preload * 0: disable preload register on PSYNC1. PSYNC1 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC1. Read/Write operations access the preload register. PSYNC1 preload value is loaded in the active register at each UEV."]
pub type Syncpe1R = crate::BitReader;
#[doc = "Field `SYNCPE1` writer - enable sync phase parameter preload * 0: disable preload register on PSYNC1. PSYNC1 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC1. Read/Write operations access the preload register. PSYNC1 preload value is loaded in the active register at each UEV."]
pub type Syncpe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDIR1` reader - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
pub type Syncdir1R = crate::BitReader;
#[doc = "Field `SYNCDIR1` writer - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
pub type Syncdir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN1` reader - enable phase sync function in channel 1. Phase sync takes effect only in output PWM mode."]
pub type Syncen1R = crate::BitReader;
#[doc = "Field `SYNCEN1` writer - enable phase sync function in channel 1. Phase sync takes effect only in output PWM mode."]
pub type Syncen1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - the deviation value of channel1 to the reference counter"]
    #[inline(always)]
    pub fn syncphase1(&self) -> Syncphase1R {
        Syncphase1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 25 - enable sync phase parameter preload * 0: disable preload register on PSYNC1. PSYNC1 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC1. Read/Write operations access the preload register. PSYNC1 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn syncpe1(&self) -> Syncpe1R {
        Syncpe1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
    #[inline(always)]
    pub fn syncdir1(&self) -> Syncdir1R {
        Syncdir1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable phase sync function in channel 1. Phase sync takes effect only in output PWM mode."]
    #[inline(always)]
    pub fn syncen1(&self) -> Syncen1R {
        Syncen1R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the deviation value of channel1 to the reference counter"]
    #[inline(always)]
    pub fn syncphase1(&mut self) -> Syncphase1W<'_, TimPsync1Spec> {
        Syncphase1W::new(self, 0)
    }
    #[doc = "Bit 25 - enable sync phase parameter preload * 0: disable preload register on PSYNC1. PSYNC1 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC1. Read/Write operations access the preload register. PSYNC1 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn syncpe1(&mut self) -> Syncpe1W<'_, TimPsync1Spec> {
        Syncpe1W::new(self, 25)
    }
    #[doc = "Bit 26 - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
    #[inline(always)]
    pub fn syncdir1(&mut self) -> Syncdir1W<'_, TimPsync1Spec> {
        Syncdir1W::new(self, 26)
    }
    #[doc = "Bit 27 - enable phase sync function in channel 1. Phase sync takes effect only in output PWM mode."]
    #[inline(always)]
    pub fn syncen1(&mut self) -> Syncen1W<'_, TimPsync1Spec> {
        Syncen1W::new(self, 27)
    }
}
#[doc = "TIM Phase Synchronization register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_psync1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_psync1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPsync1Spec;
impl crate::RegisterSpec for TimPsync1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_psync1::R`](R) reader structure"]
impl crate::Readable for TimPsync1Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_psync1::W`](W) writer structure"]
impl crate::Writable for TimPsync1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_PSYNC1 to value 0"]
impl crate::Resettable for TimPsync1Spec {}
