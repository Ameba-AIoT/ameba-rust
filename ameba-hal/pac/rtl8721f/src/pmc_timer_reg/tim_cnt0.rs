#[doc = "Register `TIM_CNT0` reader"]
pub type R = crate::R<TimCnt0Spec>;
#[doc = "Field `cnt0` reader - counter value, counter will stop automatically when it counts down to zero."]
pub type Cnt0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - counter value, counter will stop automatically when it counts down to zero."]
    #[inline(always)]
    pub fn cnt0(&self) -> Cnt0R {
        Cnt0R::new(self.bits)
    }
}
#[doc = "TIM counter0\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_cnt0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCnt0Spec;
impl crate::RegisterSpec for TimCnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_cnt0::R`](R) reader structure"]
impl crate::Readable for TimCnt0Spec {}
#[doc = "`reset()` method sets TIM_CNT0 to value 0xffff_ffff"]
impl crate::Resettable for TimCnt0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
