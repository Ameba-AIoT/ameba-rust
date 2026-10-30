#[doc = "Register `IDAU2_LOCK` reader"]
pub type R = crate::R<Idau2LockSpec>;
#[doc = "Register `IDAU2_LOCK` writer"]
pub type W = crate::W<Idau2LockSpec>;
#[doc = "Field `IDAU2_LOCK` reader - Lock write transfer for all registers in IDAU2 (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
pub type Idau2LockR = crate::BitReader;
#[doc = "Field `IDAU2_LOCK` writer - Lock write transfer for all registers in IDAU2 (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
pub type Idau2LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock write transfer for all registers in IDAU2 (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
    #[inline(always)]
    pub fn idau2_lock(&self) -> Idau2LockR {
        Idau2LockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock write transfer for all registers in IDAU2 (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
    #[inline(always)]
    pub fn idau2_lock(&mut self) -> Idau2LockW<'_, Idau2LockSpec> {
        Idau2LockW::new(self, 0)
    }
}
#[doc = "Lock control register in IDAU2\n\nYou can [`read`](crate::Reg::read) this register and get [`idau2_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau2_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idau2LockSpec;
impl crate::RegisterSpec for Idau2LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau2_lock::R`](R) reader structure"]
impl crate::Readable for Idau2LockSpec {}
#[doc = "`write(|w| ..)` method takes [`idau2_lock::W`](W) writer structure"]
impl crate::Writable for Idau2LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU2_LOCK to value 0"]
impl crate::Resettable for Idau2LockSpec {}
