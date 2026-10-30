#[doc = "Register `TIM_CNT2` reader"]
pub type R = crate::R<TimCnt2Spec>;
#[doc = "Field `cnt2` reader - counter value, counter will stop automatically when it counts down to zero."]
pub type Cnt2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - counter value, counter will stop automatically when it counts down to zero."]
    #[inline(always)]
    pub fn cnt2(&self) -> Cnt2R {
        Cnt2R::new(self.bits)
    }
}
#[doc = "TIM counter2\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_cnt2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCnt2Spec;
impl crate::RegisterSpec for TimCnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_cnt2::R`](R) reader structure"]
impl crate::Readable for TimCnt2Spec {}
#[doc = "`reset()` method sets TIM_CNT2 to value 0xffff_ffff"]
impl crate::Resettable for TimCnt2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
