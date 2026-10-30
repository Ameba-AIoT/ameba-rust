#[doc = "Register `SW_RST` writer"]
pub type W = crate::W<SwRstSpec>;
#[doc = "Field `SW_RESET` writer - Write 1 to reset TRNG. It triggers a reset generator to reset all the componets of TRNG except control register. The register settings remain unchange, but other function blocks incuding noise collector(ROSC sampler), debias modules and health tests will be reset. When SW changes the setting of RCT/APT, it's recommanded to clear error first, and then wirte this register to trigger a reset after apply new setting."]
pub type SwResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to reset TRNG. It triggers a reset generator to reset all the componets of TRNG except control register. The register settings remain unchange, but other function blocks incuding noise collector(ROSC sampler), debias modules and health tests will be reset. When SW changes the setting of RCT/APT, it's recommanded to clear error first, and then wirte this register to trigger a reset after apply new setting."]
    #[inline(always)]
    pub fn sw_reset(&mut self) -> SwResetW<'_, SwRstSpec> {
        SwResetW::new(self, 0)
    }
}
#[doc = "Software Reset Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwRstSpec;
impl crate::RegisterSpec for SwRstSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sw_rst::W`](W) writer structure"]
impl crate::Writable for SwRstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_RST to value 0"]
impl crate::Resettable for SwRstSpec {}
