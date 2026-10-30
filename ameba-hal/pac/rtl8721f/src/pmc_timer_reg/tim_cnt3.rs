#[doc = "Register `TIM_CNT3` reader"]
pub type R = crate::R<TimCnt3Spec>;
#[doc = "Field `cnt3` reader - counter value, counter will stop automatically when it counts down to zero."]
pub type Cnt3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - counter value, counter will stop automatically when it counts down to zero."]
    #[inline(always)]
    pub fn cnt3(&self) -> Cnt3R {
        Cnt3R::new(self.bits)
    }
}
#[doc = "TIM counter3\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_cnt3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCnt3Spec;
impl crate::RegisterSpec for TimCnt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_cnt3::R`](R) reader structure"]
impl crate::Readable for TimCnt3Spec {}
#[doc = "`reset()` method sets TIM_CNT3 to value 0xffff_ffff"]
impl crate::Resettable for TimCnt3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
