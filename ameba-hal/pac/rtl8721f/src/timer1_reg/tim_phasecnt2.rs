#[doc = "Register `TIM_PHASECNT2` reader"]
pub type R = crate::R<TimPhasecnt2Spec>;
#[doc = "Field `CNT2` reader - phase counter value of channel2"]
pub type Cnt2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - phase counter value of channel2"]
    #[inline(always)]
    pub fn cnt2(&self) -> Cnt2R {
        Cnt2R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TIM phase counter register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_phasecnt2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPhasecnt2Spec;
impl crate::RegisterSpec for TimPhasecnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_phasecnt2::R`](R) reader structure"]
impl crate::Readable for TimPhasecnt2Spec {}
#[doc = "`reset()` method sets TIM_PHASECNT2 to value 0"]
impl crate::Resettable for TimPhasecnt2Spec {}
