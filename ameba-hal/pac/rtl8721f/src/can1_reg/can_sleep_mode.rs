#[doc = "Register `CAN_SLEEP_MODE` reader"]
pub type R = crate::R<CanSleepModeSpec>;
#[doc = "Register `CAN_SLEEP_MODE` writer"]
pub type W = crate::W<CanSleepModeSpec>;
#[doc = "Field `can_wakepin_flt_length` reader - wakeup pin digital filter length"]
pub type CanWakepinFltLengthR = crate::FieldReader;
#[doc = "Field `can_wakepin_flt_length` writer - wakeup pin digital filter length"]
pub type CanWakepinFltLengthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_wakepin_flt_en` reader - wakeup pin digital filter enable"]
pub type CanWakepinFltEnR = crate::BitReader;
#[doc = "Field `can_wakepin_flt_en` writer - wakeup pin digital filter enable"]
pub type CanWakepinFltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_sleep_en` reader - can sleep enable 1:request can to sleep mode 0:wakeup can"]
pub type CanSleepEnR = crate::BitReader;
#[doc = "Field `can_sleep_en` writer - can sleep enable 1:request can to sleep mode 0:wakeup can"]
pub type CanSleepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_sleep_state` reader - can is sleep or not 1: is 0: isn't"]
pub type CanSleepStateR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - wakeup pin digital filter length"]
    #[inline(always)]
    pub fn can_wakepin_flt_length(&self) -> CanWakepinFltLengthR {
        CanWakepinFltLengthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - wakeup pin digital filter enable"]
    #[inline(always)]
    pub fn can_wakepin_flt_en(&self) -> CanWakepinFltEnR {
        CanWakepinFltEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - can sleep enable 1:request can to sleep mode 0:wakeup can"]
    #[inline(always)]
    pub fn can_sleep_en(&self) -> CanSleepEnR {
        CanSleepEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - can is sleep or not 1: is 0: isn't"]
    #[inline(always)]
    pub fn can_sleep_state(&self) -> CanSleepStateR {
        CanSleepStateR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - wakeup pin digital filter length"]
    #[inline(always)]
    pub fn can_wakepin_flt_length(&mut self) -> CanWakepinFltLengthW<'_, CanSleepModeSpec> {
        CanWakepinFltLengthW::new(self, 0)
    }
    #[doc = "Bit 8 - wakeup pin digital filter enable"]
    #[inline(always)]
    pub fn can_wakepin_flt_en(&mut self) -> CanWakepinFltEnW<'_, CanSleepModeSpec> {
        CanWakepinFltEnW::new(self, 8)
    }
    #[doc = "Bit 16 - can sleep enable 1:request can to sleep mode 0:wakeup can"]
    #[inline(always)]
    pub fn can_sleep_en(&mut self) -> CanSleepEnW<'_, CanSleepModeSpec> {
        CanSleepEnW::new(self, 16)
    }
}
#[doc = "sleep mode request and wakeup pin digital filter\n\nYou can [`read`](crate::Reg::read) this register and get [`can_sleep_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_sleep_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanSleepModeSpec;
impl crate::RegisterSpec for CanSleepModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_sleep_mode::R`](R) reader structure"]
impl crate::Readable for CanSleepModeSpec {}
#[doc = "`write(|w| ..)` method takes [`can_sleep_mode::W`](W) writer structure"]
impl crate::Writable for CanSleepModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_SLEEP_MODE to value 0x01"]
impl crate::Resettable for CanSleepModeSpec {
    const RESET_VALUE: u32 = 0x01;
}
