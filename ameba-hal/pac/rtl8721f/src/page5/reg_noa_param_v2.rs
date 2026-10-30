#[doc = "Register `REG_NOA_PARAM_V2` reader"]
pub type R = crate::R<RegNoaParamV2Spec>;
#[doc = "Register `REG_NOA_PARAM_V2` writer"]
pub type W = crate::W<RegNoaParamV2Spec>;
#[doc = "Field `BIT_NOA_INTERVAL_V2` reader - The length of the Notice of Absence interval in units of microseconds."]
pub type BitNoaIntervalV2R = crate::FieldReader<u32>;
#[doc = "Field `BIT_NOA_INTERVAL_V2` writer - The length of the Notice of Absence interval in units of microseconds."]
pub type BitNoaIntervalV2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The length of the Notice of Absence interval in units of microseconds."]
    #[inline(always)]
    pub fn bit_noa_interval_v2(&self) -> BitNoaIntervalV2R {
        BitNoaIntervalV2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The length of the Notice of Absence interval in units of microseconds."]
    #[inline(always)]
    pub fn bit_noa_interval_v2(&mut self) -> BitNoaIntervalV2W<'_, RegNoaParamV2Spec> {
        BitNoaIntervalV2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa_param_v2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa_param_v2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoaParamV2Spec;
impl crate::RegisterSpec for RegNoaParamV2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa_param_v2::R`](R) reader structure"]
impl crate::Readable for RegNoaParamV2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa_param_v2::W`](W) writer structure"]
impl crate::Writable for RegNoaParamV2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA_PARAM_V2 to value 0x3000"]
impl crate::Resettable for RegNoaParamV2Spec {
    const RESET_VALUE: u32 = 0x3000;
}
