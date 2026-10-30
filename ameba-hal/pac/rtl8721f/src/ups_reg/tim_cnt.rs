#[doc = "Register `TIM_CNT` reader"]
pub type R = crate::R<TimCntSpec>;
#[doc = "Field `CNT` reader - counter value"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TIM counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimCntSpec;
impl crate::RegisterSpec for TimCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_cnt::R`](R) reader structure"]
impl crate::Readable for TimCntSpec {}
#[doc = "`reset()` method sets TIM_CNT to value 0"]
impl crate::Resettable for TimCntSpec {}
