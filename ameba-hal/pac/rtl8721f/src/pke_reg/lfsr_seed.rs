#[doc = "Register `LFSR_Seed` writer"]
pub type W = crate::W<LfsrSeedSpec>;
#[doc = "Field `SEED` writer - Seed of internal LFSRs"]
pub type SeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Seed of internal LFSRs"]
    #[inline(always)]
    pub fn seed(&mut self) -> SeedW<'_, LfsrSeedSpec> {
        SeedW::new(self, 0)
    }
}
#[doc = "PKESeeding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfsr_seed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfsrSeedSpec;
impl crate::RegisterSpec for LfsrSeedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lfsr_seed::W`](W) writer structure"]
impl crate::Writable for LfsrSeedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSR_Seed to value 0"]
impl crate::Resettable for LfsrSeedSpec {}
