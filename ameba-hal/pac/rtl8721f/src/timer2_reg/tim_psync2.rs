#[doc = "Register `TIM_PSYNC2` reader"]
pub type R = crate::R<TimPsync2Spec>;
#[doc = "Register `TIM_PSYNC2` writer"]
pub type W = crate::W<TimPsync2Spec>;
#[doc = "Field `SYNCPHASE2` reader - the deviation value of channel2 to the reference counter"]
pub type Syncphase2R = crate::FieldReader<u16>;
#[doc = "Field `SYNCPHASE2` writer - the deviation value of channel2 to the reference counter"]
pub type Syncphase2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SYNCPE2` reader - enable sync phase parameter preload * 0: disable preload register on PSYNC2. PSYNC2 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC2. Read/Write operations access the preload register. PSYNC2 preload value is loaded in the active register at each UEV."]
pub type Syncpe2R = crate::BitReader;
#[doc = "Field `SYNCPE2` writer - enable sync phase parameter preload * 0: disable preload register on PSYNC2. PSYNC2 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC2. Read/Write operations access the preload register. PSYNC2 preload value is loaded in the active register at each UEV."]
pub type Syncpe2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDIR2` reader - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
pub type Syncdir2R = crate::BitReader;
#[doc = "Field `SYNCDIR2` writer - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
pub type Syncdir2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN2` reader - enable phase sync function in channel 2. Phase sync takes effect only in output PWM mode."]
pub type Syncen2R = crate::BitReader;
#[doc = "Field `SYNCEN2` writer - enable phase sync function in channel 2. Phase sync takes effect only in output PWM mode."]
pub type Syncen2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - the deviation value of channel2 to the reference counter"]
    #[inline(always)]
    pub fn syncphase2(&self) -> Syncphase2R {
        Syncphase2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 25 - enable sync phase parameter preload * 0: disable preload register on PSYNC2. PSYNC2 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC2. Read/Write operations access the preload register. PSYNC2 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn syncpe2(&self) -> Syncpe2R {
        Syncpe2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
    #[inline(always)]
    pub fn syncdir2(&self) -> Syncdir2R {
        Syncdir2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable phase sync function in channel 2. Phase sync takes effect only in output PWM mode."]
    #[inline(always)]
    pub fn syncen2(&self) -> Syncen2R {
        Syncen2R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the deviation value of channel2 to the reference counter"]
    #[inline(always)]
    pub fn syncphase2(&mut self) -> Syncphase2W<'_, TimPsync2Spec> {
        Syncphase2W::new(self, 0)
    }
    #[doc = "Bit 25 - enable sync phase parameter preload * 0: disable preload register on PSYNC2. PSYNC2 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC2. Read/Write operations access the preload register. PSYNC2 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn syncpe2(&mut self) -> Syncpe2W<'_, TimPsync2Spec> {
        Syncpe2W::new(self, 25)
    }
    #[doc = "Bit 26 - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
    #[inline(always)]
    pub fn syncdir2(&mut self) -> Syncdir2W<'_, TimPsync2Spec> {
        Syncdir2W::new(self, 26)
    }
    #[doc = "Bit 27 - enable phase sync function in channel 2. Phase sync takes effect only in output PWM mode."]
    #[inline(always)]
    pub fn syncen2(&mut self) -> Syncen2W<'_, TimPsync2Spec> {
        Syncen2W::new(self, 27)
    }
}
#[doc = "TIM Phase Synchronization register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_psync2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_psync2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPsync2Spec;
impl crate::RegisterSpec for TimPsync2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_psync2::R`](R) reader structure"]
impl crate::Readable for TimPsync2Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_psync2::W`](W) writer structure"]
impl crate::Writable for TimPsync2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_PSYNC2 to value 0"]
impl crate::Resettable for TimPsync2Spec {}
