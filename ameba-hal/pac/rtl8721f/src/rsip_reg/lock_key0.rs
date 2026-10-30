#[doc = "Register `Lock_key0` writer"]
pub type W = crate::W<LockKey0Spec>;
#[doc = "Field `lock_key0` writer - When set DC to this field, key_x are not allowed to be written or read. Once the lock is set, the way to unlock is reset the flash SEC engine. If SW wants to modified the key registers, it should keep this lock unset."]
pub type LockKey0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - When set DC to this field, key_x are not allowed to be written or read. Once the lock is set, the way to unlock is reset the flash SEC engine. If SW wants to modified the key registers, it should keep this lock unset."]
    #[inline(always)]
    pub fn lock_key0(&mut self) -> LockKey0W<'_, LockKey0Spec> {
        LockKey0W::new(self, 0)
    }
}
#[doc = "Lock key 0 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock_key0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockKey0Spec;
impl crate::RegisterSpec for LockKey0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lock_key0::W`](W) writer structure"]
impl crate::Writable for LockKey0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Lock_key0 to value 0"]
impl crate::Resettable for LockKey0Spec {}
