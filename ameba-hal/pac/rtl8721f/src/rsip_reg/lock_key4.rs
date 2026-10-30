#[doc = "Register `Lock_key4` writer"]
pub type W = crate::W<LockKey4Spec>;
#[doc = "Field `lock_key4` writer - When set DC to this field, key_x are not allowed to be written or read. Once the lock is set, the way to unlock is reset the flash SEC engine. If SW wants to modified the key registers, it should keep this lock unset."]
pub type LockKey4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - When set DC to this field, key_x are not allowed to be written or read. Once the lock is set, the way to unlock is reset the flash SEC engine. If SW wants to modified the key registers, it should keep this lock unset."]
    #[inline(always)]
    pub fn lock_key4(&mut self) -> LockKey4W<'_, LockKey4Spec> {
        LockKey4W::new(self, 0)
    }
}
#[doc = "Lock key 4 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock_key4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockKey4Spec;
impl crate::RegisterSpec for LockKey4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lock_key4::W`](W) writer structure"]
impl crate::Writable for LockKey4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Lock_key4 to value 0"]
impl crate::Resettable for LockKey4Spec {}
