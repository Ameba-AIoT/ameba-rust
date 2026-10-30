#[doc = "Register `TIM_PHASECNT3` reader"]
pub type R = crate::R<TimPhasecnt3Spec>;
#[doc = "Field `CNT3` reader - phase counter value of channel3"]
pub type Cnt3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - phase counter value of channel3"]
    #[inline(always)]
    pub fn cnt3(&self) -> Cnt3R {
        Cnt3R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TIM phase counter register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_phasecnt3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPhasecnt3Spec;
impl crate::RegisterSpec for TimPhasecnt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_phasecnt3::R`](R) reader structure"]
impl crate::Readable for TimPhasecnt3Spec {}
#[doc = "`reset()` method sets TIM_PHASECNT3 to value 0"]
impl crate::Resettable for TimPhasecnt3Spec {}
