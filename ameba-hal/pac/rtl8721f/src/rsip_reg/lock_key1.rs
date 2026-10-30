#[doc = "Register `Lock_key1` writer"]
pub type W = crate::W<LockKey1Spec>;
#[doc = "Field `lock_key1` writer - When set DC to this field, key_x are not allowed to be written or read. Once the lock is set, the way to unlock is reset the flash SEC engine. If SW wants to modified the key registers, it should keep this lock unset."]
pub type LockKey1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - When set DC to this field, key_x are not allowed to be written or read. Once the lock is set, the way to unlock is reset the flash SEC engine. If SW wants to modified the key registers, it should keep this lock unset."]
    #[inline(always)]
    pub fn lock_key1(&mut self) -> LockKey1W<'_, LockKey1Spec> {
        LockKey1W::new(self, 0)
    }
}
#[doc = "Lock key 1 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock_key1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockKey1Spec;
impl crate::RegisterSpec for LockKey1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lock_key1::W`](W) writer structure"]
impl crate::Writable for LockKey1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Lock_key1 to value 0"]
impl crate::Resettable for LockKey1Spec {}
