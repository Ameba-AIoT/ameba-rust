#[doc = "Register `SDM_TIMEOUT` reader"]
pub type R = crate::R<SdmTimeoutSpec>;
#[doc = "Register `SDM_TIMEOUT` writer"]
pub type W = crate::W<SdmTimeoutSpec>;
#[doc = "Field `TIMEOUT_CNT` reader - Set SDM timer trigger calibration timeout value"]
pub type TimeoutCntR = crate::FieldReader<u32>;
#[doc = "Field `TIMEOUT_CNT` writer - Set SDM timer trigger calibration timeout value"]
pub type TimeoutCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set SDM timer trigger calibration timeout value"]
    #[inline(always)]
    pub fn timeout_cnt(&self) -> TimeoutCntR {
        TimeoutCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set SDM timer trigger calibration timeout value"]
    #[inline(always)]
    pub fn timeout_cnt(&mut self) -> TimeoutCntW<'_, SdmTimeoutSpec> {
        TimeoutCntW::new(self, 0)
    }
}
#[doc = "Set Timer Trigger Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmTimeoutSpec;
impl crate::RegisterSpec for SdmTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_timeout::R`](R) reader structure"]
impl crate::Readable for SdmTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`sdm_timeout::W`](W) writer structure"]
impl crate::Writable for SdmTimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDM_TIMEOUT to value 0x0200_0000"]
impl crate::Resettable for SdmTimeoutSpec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
