#[doc = "Register `TIMEOUT_CLEAN_N` reader"]
pub type R = crate::R<TimeoutCleanNSpec>;
#[doc = "Register `TIMEOUT_CLEAN_N` writer"]
pub type W = crate::W<TimeoutCleanNSpec>;
#[doc = "Field `TIMEOUT_CLEAR_N` reader - The register can only written to 0 when a time out occures, and automatically clear this register by hardware"]
pub type TimeoutClearNR = crate::BitReader;
#[doc = "Field `TIMEOUT_CLEAR_N` writer - The register can only written to 0 when a time out occures, and automatically clear this register by hardware"]
pub type TimeoutClearNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The register can only written to 0 when a time out occures, and automatically clear this register by hardware"]
    #[inline(always)]
    pub fn timeout_clear_n(&self) -> TimeoutClearNR {
        TimeoutClearNR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The register can only written to 0 when a time out occures, and automatically clear this register by hardware"]
    #[inline(always)]
    pub fn timeout_clear_n(&mut self) -> TimeoutClearNW<'_, TimeoutCleanNSpec> {
        TimeoutClearNW::new(self, 0)
    }
}
#[doc = "Timeout clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_clean_n::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_clean_n::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutCleanNSpec;
impl crate::RegisterSpec for TimeoutCleanNSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_clean_n::R`](R) reader structure"]
impl crate::Readable for TimeoutCleanNSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_clean_n::W`](W) writer structure"]
impl crate::Writable for TimeoutCleanNSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMEOUT_CLEAN_N to value 0x01"]
impl crate::Resettable for TimeoutCleanNSpec {
    const RESET_VALUE: u32 = 0x01;
}
