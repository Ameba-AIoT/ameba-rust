#[doc = "Register `TIMEOUT_GUARD_EN` reader"]
pub type R = crate::R<TimeoutGuardEnSpec>;
#[doc = "Register `TIMEOUT_GUARD_EN` writer"]
pub type W = crate::W<TimeoutGuardEnSpec>;
#[doc = "Field `GUARDIAN_MODE_EN` reader - The enable of timeout monitor group 0x0: Disable timeout check feature 0x1: Enable timeoute check feature"]
pub type GuardianModeEnR = crate::FieldReader;
#[doc = "Field `GUARDIAN_MODE_EN` writer - The enable of timeout monitor group 0x0: Disable timeout check feature 0x1: Enable timeoute check feature"]
pub type GuardianModeEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The enable of timeout monitor group 0x0: Disable timeout check feature 0x1: Enable timeoute check feature"]
    #[inline(always)]
    pub fn guardian_mode_en(&self) -> GuardianModeEnR {
        GuardianModeEnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The enable of timeout monitor group 0x0: Disable timeout check feature 0x1: Enable timeoute check feature"]
    #[inline(always)]
    pub fn guardian_mode_en(&mut self) -> GuardianModeEnW<'_, TimeoutGuardEnSpec> {
        GuardianModeEnW::new(self, 0)
    }
}
#[doc = "Timeout guardian enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_guard_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_guard_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutGuardEnSpec;
impl crate::RegisterSpec for TimeoutGuardEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_guard_en::R`](R) reader structure"]
impl crate::Readable for TimeoutGuardEnSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_guard_en::W`](W) writer structure"]
impl crate::Writable for TimeoutGuardEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMEOUT_GUARD_EN to value 0"]
impl crate::Resettable for TimeoutGuardEnSpec {}
