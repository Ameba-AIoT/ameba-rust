#[doc = "Register `REG_NOA_PARAM_V1` reader"]
pub type R = crate::R<RegNoaParamV1Spec>;
#[doc = "Register `REG_NOA_PARAM_V1` writer"]
pub type W = crate::W<RegNoaParamV1Spec>;
#[doc = "Field `BIT_NOA_DURATION_V1` reader - The maximum duration in units of microseconds that the P2P Group Owner shall remain absent following the start of a Notice of Absence interval."]
pub type BitNoaDurationV1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_NOA_DURATION_V1` writer - The maximum duration in units of microseconds that the P2P Group Owner shall remain absent following the start of a Notice of Absence interval."]
pub type BitNoaDurationV1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The maximum duration in units of microseconds that the P2P Group Owner shall remain absent following the start of a Notice of Absence interval."]
    #[inline(always)]
    pub fn bit_noa_duration_v1(&self) -> BitNoaDurationV1R {
        BitNoaDurationV1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The maximum duration in units of microseconds that the P2P Group Owner shall remain absent following the start of a Notice of Absence interval."]
    #[inline(always)]
    pub fn bit_noa_duration_v1(&mut self) -> BitNoaDurationV1W<'_, RegNoaParamV1Spec> {
        BitNoaDurationV1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa_param_v1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa_param_v1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoaParamV1Spec;
impl crate::RegisterSpec for RegNoaParamV1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa_param_v1::R`](R) reader structure"]
impl crate::Readable for RegNoaParamV1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa_param_v1::W`](W) writer structure"]
impl crate::Writable for RegNoaParamV1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA_PARAM_V1 to value 0x1000"]
impl crate::Resettable for RegNoaParamV1Spec {
    const RESET_VALUE: u32 = 0x1000;
}
