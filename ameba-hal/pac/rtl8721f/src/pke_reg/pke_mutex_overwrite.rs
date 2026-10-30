#[doc = "Register `PKE_MUTEX_OVERWRITE` reader"]
pub type R = crate::R<PkeMutexOverwriteSpec>;
#[doc = "Register `PKE_MUTEX_OVERWRITE` writer"]
pub type W = crate::W<PkeMutexOverwriteSpec>;
#[doc = "Field `MUTEX_OVERWRITE` reader - Allows secure domain overwrite mutex 1: Enable 0: Disable"]
pub type MutexOverwriteR = crate::BitReader;
#[doc = "Field `MUTEX_OVERWRITE` writer - Allows secure domain overwrite mutex 1: Enable 0: Disable"]
pub type MutexOverwriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Allows secure domain overwrite mutex 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn mutex_overwrite(&self) -> MutexOverwriteR {
        MutexOverwriteR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allows secure domain overwrite mutex 1: Enable 0: Disable"]
    #[inline(always)]
    pub fn mutex_overwrite(&mut self) -> MutexOverwriteW<'_, PkeMutexOverwriteSpec> {
        MutexOverwriteW::new(self, 0)
    }
}
#[doc = "PKE Mutex Overwrite Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_mutex_overwrite::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pke_mutex_overwrite::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeMutexOverwriteSpec;
impl crate::RegisterSpec for PkeMutexOverwriteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_mutex_overwrite::R`](R) reader structure"]
impl crate::Readable for PkeMutexOverwriteSpec {}
#[doc = "`write(|w| ..)` method takes [`pke_mutex_overwrite::W`](W) writer structure"]
impl crate::Writable for PkeMutexOverwriteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKE_MUTEX_OVERWRITE to value 0"]
impl crate::Resettable for PkeMutexOverwriteSpec {}
