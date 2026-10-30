#[doc = "Register `TIM_PHASECNT1` reader"]
pub type R = crate::R<TimPhasecnt1Spec>;
#[doc = "Field `CNT1` reader - phase counter value of channel1"]
pub type Cnt1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - phase counter value of channel1"]
    #[inline(always)]
    pub fn cnt1(&self) -> Cnt1R {
        Cnt1R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TIM phase counter register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_phasecnt1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimPhasecnt1Spec;
impl crate::RegisterSpec for TimPhasecnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_phasecnt1::R`](R) reader structure"]
impl crate::Readable for TimPhasecnt1Spec {}
#[doc = "`reset()` method sets TIM_PHASECNT1 to value 0"]
impl crate::Resettable for TimPhasecnt1Spec {}
