#[doc = "Register `TIM_PSYNC3` reader"]
pub type R = crate::R<TimPsync3Spec>;
#[doc = "Register `TIM_PSYNC3` writer"]
pub type W = crate::W<TimPsync3Spec>;
#[doc = "Field `SYNCPHASE3` reader - the deviation value of channel3 to the reference counter"]
pub type Syncphase3R = crate::FieldReader<u16>;
#[doc = "Field `SYNCPHASE3` writer - the deviation value of channel3 to the reference counter"]
pub type Syncphase3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SYNCPE3` reader - enable sync phase parameter preload * 0: disable preload register on PSYNC3. PSYNC3 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC3. Read/Write operations access the preload register. PSYNC3 preload value is loaded in the active register at each UEV."]
pub type Syncpe3R = crate::BitReader;
#[doc = "Field `SYNCPE3` writer - enable sync phase parameter preload * 0: disable preload register on PSYNC3. PSYNC3 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC3. Read/Write operations access the preload register. PSYNC3 preload value is loaded in the active register at each UEV."]
pub type Syncpe3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDIR3` reader - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
pub type Syncdir3R = crate::BitReader;
#[doc = "Field `SYNCDIR3` writer - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
pub type Syncdir3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN3` reader - enable phase sync function in channel 3. Phase sync takes effect only in output PWM mode."]
pub type Syncen3R = crate::BitReader;
#[doc = "Field `SYNCEN3` writer - enable phase sync function in channel 3. Phase sync takes effect only in output PWM mode."]
pub type Syncen3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - the deviation value of channel3 to the reference counter"]
    #[inline(always)]
    pub fn syncphase3(&self) -> Syncphase3R {
        Syncphase3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 25 - enable sync phase parameter preload * 0: disable preload register on PSYNC3. PSYNC3 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC3. Read/Write operations access the preload register. PSYNC3 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn syncpe3(&self) -> Syncpe3R {
        Syncpe3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
    #[inline(always)]
    pub fn syncdir3(&self) -> Syncdir3R {
        Syncdir3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable phase sync function in channel 3. Phase sync takes effect only in output PWM mode."]
    #[inline(always)]
    pub fn syncen3(&self) -> Syncen3R {
        Syncen3R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the deviation value of channel3 to the reference counter"]
    #[inline(always)]
    pub fn syncphase3(&mut self) -> Syncphase3W<'_, TimPsync3Spec> {
        Syncphase3W::new(self, 0)
    }
    #[doc = "Bit 25 - enable sync phase parameter preload * 0: disable preload register on PSYNC3. PSYNC3 can be written at any time, the new value is taken into account immediately. * 1: enable preload register on PSYNC3. Read/Write operations access the preload register. PSYNC3 preload value is loaded in the active register at each UEV."]
    #[inline(always)]
    pub fn syncpe3(&mut self) -> Syncpe3W<'_, TimPsync3Spec> {
        Syncpe3W::new(self, 25)
    }
    #[doc = "Bit 26 - phase sync direction to the reference counter: 0:delay to the reference counter 1:ahead to the reference counter"]
    #[inline(always)]
    pub fn syncdir3(&mut self) -> Syncdir3W<'_, TimPsync3Spec> {
        Syncdir3W::new(self, 26)
    }
    #[doc = "Bit 27 - enable phase sync function in channel 3. Phase sync takes effect only in output PWM mode."]
    #[inline(always)]
    pub fn syncen3(&mut self) -> Syncen3W<'_, TimPsync3Spec> {
        Syncen3W::new(self, 27)
    }
}
#[doc = "TIM Phase Synchronization register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_psync3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_psync3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPsync3Spec;
impl crate::RegisterSpec for TimPsync3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_psync3::R`](R) reader structure"]
impl crate::Readable for TimPsync3Spec {}
#[doc = "`write(|w| ..)` method takes [`tim_psync3::W`](W) writer structure"]
impl crate::Writable for TimPsync3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_PSYNC3 to value 0"]
impl crate::Resettable for TimPsync3Spec {}
