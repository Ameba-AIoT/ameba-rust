#[doc = "Register `IDAU_LOCK` reader"]
pub type R = crate::R<IdauLockSpec>;
#[doc = "Register `IDAU_LOCK` writer"]
pub type W = crate::W<IdauLockSpec>;
#[doc = "Field `IDAU_LOCK` reader - Lock write transfer for all registers in IDAU (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
pub type IdauLockR = crate::BitReader;
#[doc = "Field `IDAU_LOCK` writer - Lock write transfer for all registers in IDAU (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
pub type IdauLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock write transfer for all registers in IDAU (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
    #[inline(always)]
    pub fn idau_lock(&self) -> IdauLockR {
        IdauLockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock write transfer for all registers in IDAU (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
    #[inline(always)]
    pub fn idau_lock(&mut self) -> IdauLockW<'_, IdauLockSpec> {
        IdauLockW::new(self, 0)
    }
}
#[doc = "Lock control register in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauLockSpec;
impl crate::RegisterSpec for IdauLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_lock::R`](R) reader structure"]
impl crate::Readable for IdauLockSpec {}
#[doc = "`write(|w| ..)` method takes [`idau_lock::W`](W) writer structure"]
impl crate::Writable for IdauLockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_LOCK to value 0"]
impl crate::Resettable for IdauLockSpec {}
