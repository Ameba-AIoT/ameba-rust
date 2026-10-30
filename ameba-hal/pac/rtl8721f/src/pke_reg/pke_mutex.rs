#[doc = "Register `PKE_MUTEX` reader"]
pub type R = crate::R<PkeMutexSpec>;
#[doc = "Register `PKE_MUTEX` writer"]
pub type W = crate::W<PkeMutexSpec>;
#[doc = "Field `MUTEX_STATUS` reader - If read 0, lock mutex. If not 0, the engine is being used. 2'b01: secure domain lock mutex 2'b10: non-secure domain lock mutex Write 0 to unlock mutex"]
pub type MutexStatusR = crate::FieldReader;
#[doc = "Field `MUTEX_STATUS` writer - If read 0, lock mutex. If not 0, the engine is being used. 2'b01: secure domain lock mutex 2'b10: non-secure domain lock mutex Write 0 to unlock mutex"]
pub type MutexStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - If read 0, lock mutex. If not 0, the engine is being used. 2'b01: secure domain lock mutex 2'b10: non-secure domain lock mutex Write 0 to unlock mutex"]
    #[inline(always)]
    pub fn mutex_status(&self) -> MutexStatusR {
        MutexStatusR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - If read 0, lock mutex. If not 0, the engine is being used. 2'b01: secure domain lock mutex 2'b10: non-secure domain lock mutex Write 0 to unlock mutex"]
    #[inline(always)]
    pub fn mutex_status(&mut self) -> MutexStatusW<'_, PkeMutexSpec> {
        MutexStatusW::new(self, 0)
    }
}
#[doc = "PKE Mutex Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_mutex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pke_mutex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeMutexSpec;
impl crate::RegisterSpec for PkeMutexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_mutex::R`](R) reader structure"]
impl crate::Readable for PkeMutexSpec {}
#[doc = "`write(|w| ..)` method takes [`pke_mutex::W`](W) writer structure"]
impl crate::Writable for PkeMutexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKE_MUTEX to value 0"]
impl crate::Resettable for PkeMutexSpec {}
