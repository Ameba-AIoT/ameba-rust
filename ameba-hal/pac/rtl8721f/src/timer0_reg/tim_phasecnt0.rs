#[doc = "Register `TIM_PHASECNT0` reader"]
pub type R = crate::R<TimPhasecnt0Spec>;
#[doc = "Field `CNT0` reader - phase counter value of channel0"]
pub type Cnt0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - phase counter value of channel0"]
    #[inline(always)]
    pub fn cnt0(&self) -> Cnt0R {
        Cnt0R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TIM phase counter register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_phasecnt0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPhasecnt0Spec;
impl crate::RegisterSpec for TimPhasecnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_phasecnt0::R`](R) reader structure"]
impl crate::Readable for TimPhasecnt0Spec {}
#[doc = "`reset()` method sets TIM_PHASECNT0 to value 0"]
impl crate::Resettable for TimPhasecnt0Spec {}
