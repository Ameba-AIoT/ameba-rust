#[doc = "Register `IDAU1_LOCK` reader"]
pub type R = crate::R<Idau1LockSpec>;
#[doc = "Register `IDAU1_LOCK` writer"]
pub type W = crate::W<Idau1LockSpec>;
#[doc = "Field `IDAU1_LOCK` reader - Lock write transfer for all registers in IDAU1 (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
pub type Idau1LockR = crate::BitReader;
#[doc = "Field `IDAU1_LOCK` writer - Lock write transfer for all registers in IDAU1 (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
pub type Idau1LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock write transfer for all registers in IDAU1 (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
    #[inline(always)]
    pub fn idau1_lock(&self) -> Idau1LockR {
        Idau1LockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock write transfer for all registers in IDAU1 (including lock register itself) until system reset if it is ebabled. 0: disable lock 1: enable lock"]
    #[inline(always)]
    pub fn idau1_lock(&mut self) -> Idau1LockW<'_, Idau1LockSpec> {
        Idau1LockW::new(self, 0)
    }
}
#[doc = "Lock control register in IDAU1\n\nYou can [`read`](crate::Reg::read) this register and get [`idau1_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau1_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idau1LockSpec;
impl crate::RegisterSpec for Idau1LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau1_lock::R`](R) reader structure"]
impl crate::Readable for Idau1LockSpec {}
#[doc = "`write(|w| ..)` method takes [`idau1_lock::W`](W) writer structure"]
impl crate::Writable for Idau1LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU1_LOCK to value 0"]
impl crate::Resettable for Idau1LockSpec {}
