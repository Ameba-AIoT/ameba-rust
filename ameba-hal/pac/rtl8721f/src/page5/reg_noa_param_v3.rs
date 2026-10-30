#[doc = "Register `REG_NOA_PARAM_V3` reader"]
pub type R = crate::R<RegNoaParamV3Spec>;
#[doc = "Register `REG_NOA_PARAM_V3` writer"]
pub type W = crate::W<RegNoaParamV3Spec>;
#[doc = "Field `BIT_NOA_START_TIME_V3` reader - The start time for the schedule expressed in terms of the lower 4 bytes of the TSF timer."]
pub type BitNoaStartTimeV3R = crate::FieldReader<u32>;
#[doc = "Field `BIT_NOA_START_TIME_V3` writer - The start time for the schedule expressed in terms of the lower 4 bytes of the TSF timer."]
pub type BitNoaStartTimeV3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start time for the schedule expressed in terms of the lower 4 bytes of the TSF timer."]
    #[inline(always)]
    pub fn bit_noa_start_time_v3(&self) -> BitNoaStartTimeV3R {
        BitNoaStartTimeV3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The start time for the schedule expressed in terms of the lower 4 bytes of the TSF timer."]
    #[inline(always)]
    pub fn bit_noa_start_time_v3(&mut self) -> BitNoaStartTimeV3W<'_, RegNoaParamV3Spec> {
        BitNoaStartTimeV3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa_param_v3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa_param_v3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoaParamV3Spec;
impl crate::RegisterSpec for RegNoaParamV3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa_param_v3::R`](R) reader structure"]
impl crate::Readable for RegNoaParamV3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa_param_v3::W`](W) writer structure"]
impl crate::Writable for RegNoaParamV3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA_PARAM_V3 to value 0x1000"]
impl crate::Resettable for RegNoaParamV3Spec {
    const RESET_VALUE: u32 = 0x1000;
}
