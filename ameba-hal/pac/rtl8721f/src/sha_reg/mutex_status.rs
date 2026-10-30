#[doc = "Register `Mutex_status` reader"]
pub type R = crate::R<MutexStatusSpec>;
#[doc = "Register `Mutex_status` writer"]
pub type W = crate::W<MutexStatusSpec>;
#[doc = "Field `Engine_mutex_status` reader - If read 0, lock mutex. If not 0, the engine is being used. 2'b01: secure domain lock mutex 2'b10: non-secure domain lock mutex Write 0 to unlock mutex"]
pub type EngineMutexStatusR = crate::FieldReader;
#[doc = "Field `Engine_mutex_status` writer - If read 0, lock mutex. If not 0, the engine is being used. 2'b01: secure domain lock mutex 2'b10: non-secure domain lock mutex Write 0 to unlock mutex"]
pub type EngineMutexStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - If read 0, lock mutex. If not 0, the engine is being used. 2'b01: secure domain lock mutex 2'b10: non-secure domain lock mutex Write 0 to unlock mutex"]
    #[inline(always)]
    pub fn engine_mutex_status(&self) -> EngineMutexStatusR {
        EngineMutexStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - If read 0, lock mutex. If not 0, the engine is being used. 2'b01: secure domain lock mutex 2'b10: non-secure domain lock mutex Write 0 to unlock mutex"]
    #[inline(always)]
    pub fn engine_mutex_status(&mut self) -> EngineMutexStatusW<'_, MutexStatusSpec> {
        EngineMutexStatusW::new(self, 0)
    }
}
#[doc = "Mutex status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MutexStatusSpec;
impl crate::RegisterSpec for MutexStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mutex_status::R`](R) reader structure"]
impl crate::Readable for MutexStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mutex_status::W`](W) writer structure"]
impl crate::Writable for MutexStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Mutex_status to value 0"]
impl crate::Resettable for MutexStatusSpec {}
