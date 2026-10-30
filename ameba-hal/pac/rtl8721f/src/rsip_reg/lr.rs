#[doc = "Register `LR` writer"]
pub type W = crate::W<LrSpec>;
#[doc = "Field `lock_control` writer - When set DC to this field, all registers except Clr_intr, Maintenance and keys are not allowed to be written. Once the lock is set, the way to unlock is reset the flash SEC engine. If SW wants to modified the setting of control registers, it should keep this lock unset."]
pub type LockControlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - When set DC to this field, all registers except Clr_intr, Maintenance and keys are not allowed to be written. Once the lock is set, the way to unlock is reset the flash SEC engine. If SW wants to modified the setting of control registers, it should keep this lock unset."]
    #[inline(always)]
    pub fn lock_control(&mut self) -> LockControlW<'_, LrSpec> {
        LockControlW::new(self, 0)
    }
}
#[doc = "Lock Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LrSpec;
impl crate::RegisterSpec for LrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lr::W`](W) writer structure"]
impl crate::Writable for LrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LR to value 0"]
impl crate::Resettable for LrSpec {}
