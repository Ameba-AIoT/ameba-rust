#[doc = "Register `TIM_CNT1` reader"]
pub type R = crate::R<TimCnt1Spec>;
#[doc = "Field `cnt1` reader - counter value, counter will stop automatically when it counts down to zero."]
pub type Cnt1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - counter value, counter will stop automatically when it counts down to zero."]
    #[inline(always)]
    pub fn cnt1(&self) -> Cnt1R {
        Cnt1R::new(self.bits)
    }
}
#[doc = "TIM counter1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_cnt1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCnt1Spec;
impl crate::RegisterSpec for TimCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_cnt1::R`](R) reader structure"]
impl crate::Readable for TimCnt1Spec {}
#[doc = "`reset()` method sets TIM_CNT1 to value 0xffff_ffff"]
impl crate::Resettable for TimCnt1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
