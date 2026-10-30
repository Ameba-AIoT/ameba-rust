#[doc = "Register `Mutex_overwrite` reader"]
pub type R = crate::R<MutexOverwriteSpec>;
#[doc = "Register `Mutex_overwrite` writer"]
pub type W = crate::W<MutexOverwriteSpec>;
#[doc = "Field `Engine_mutex_overwrite` reader - Allows secure domain overwrite mutex 1: enable 0: disable"]
pub type EngineMutexOverwriteR = crate::BitReader;
#[doc = "Field `Engine_mutex_overwrite` writer - Allows secure domain overwrite mutex 1: enable 0: disable"]
pub type EngineMutexOverwriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Allows secure domain overwrite mutex 1: enable 0: disable"]
    #[inline(always)]
    pub fn engine_mutex_overwrite(&self) -> EngineMutexOverwriteR {
        EngineMutexOverwriteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Allows secure domain overwrite mutex 1: enable 0: disable"]
    #[inline(always)]
    pub fn engine_mutex_overwrite(&mut self) -> EngineMutexOverwriteW<'_, MutexOverwriteSpec> {
        EngineMutexOverwriteW::new(self, 0)
    }
}
#[doc = "Mutex overwrite register\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex_overwrite::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex_overwrite::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MutexOverwriteSpec;
impl crate::RegisterSpec for MutexOverwriteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mutex_overwrite::R`](R) reader structure"]
impl crate::Readable for MutexOverwriteSpec {}
#[doc = "`write(|w| ..)` method takes [`mutex_overwrite::W`](W) writer structure"]
impl crate::Writable for MutexOverwriteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Mutex_overwrite to value 0"]
impl crate::Resettable for MutexOverwriteSpec {}
