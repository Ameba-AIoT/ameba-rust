#[doc = "Register `EVENT_PREFETCH_CNT` reader"]
pub type R = crate::R<EventPrefetchCntSpec>;
#[doc = "Field `PREFETCH_NUM` reader - The number of prefetch"]
pub type PrefetchNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - The number of prefetch"]
    #[inline(always)]
    pub fn prefetch_num(&self) -> PrefetchNumR {
        PrefetchNumR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "The number of prefetch\n\nYou can [`read`](crate::Reg::read) this register and get [`event_prefetch_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventPrefetchCntSpec;
impl crate::RegisterSpec for EventPrefetchCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_prefetch_cnt::R`](R) reader structure"]
impl crate::Readable for EventPrefetchCntSpec {}
#[doc = "`reset()` method sets EVENT_PREFETCH_CNT to value 0"]
impl crate::Resettable for EventPrefetchCntSpec {}
