#[doc = "Register `MOR` writer"]
pub type W = crate::W<MorSpec>;
#[doc = "Field `dec_key_init` writer - Only used in MIX mode. Whenever the key is changed, write 1 to this bit to initialize the decryption key. When keys are update by external port, sec engine will trigger this bit automatically. The status of completion can be determined by reading bit 1 of the status register."]
pub type DecKeyInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clean_cache_line` writer - Manually clear the line buffer. Write 1 to clear all after flash data update."]
pub type CleanCacheLineW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Only used in MIX mode. Whenever the key is changed, write 1 to this bit to initialize the decryption key. When keys are update by external port, sec engine will trigger this bit automatically. The status of completion can be determined by reading bit 1 of the status register."]
    #[inline(always)]
    pub fn dec_key_init(&mut self) -> DecKeyInitW<'_, MorSpec> {
        DecKeyInitW::new(self, 1)
    }
    #[doc = "Bit 2 - Manually clear the line buffer. Write 1 to clear all after flash data update."]
    #[inline(always)]
    pub fn clean_cache_line(&mut self) -> CleanCacheLineW<'_, MorSpec> {
        CleanCacheLineW::new(self, 2)
    }
}
#[doc = "Maintenance Operation Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mor::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MorSpec;
impl crate::RegisterSpec for MorSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mor::W`](W) writer structure"]
impl crate::Writable for MorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOR to value 0"]
impl crate::Resettable for MorSpec {}
