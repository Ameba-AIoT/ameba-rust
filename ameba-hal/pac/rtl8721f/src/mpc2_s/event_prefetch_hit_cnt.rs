#[doc = "Register `EVENT_PREFETCH_HIT_CNT` reader"]
pub type R = crate::R<EventPrefetchHitCntSpec>;
#[doc = "Field `PREFETCH_HIT` reader - The number of transfer data is not immediately returned after cache line hits in ther stream buffer"]
pub type PrefetchHitR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - The number of transfer data is not immediately returned after cache line hits in ther stream buffer"]
    #[inline(always)]
    pub fn prefetch_hit(&self) -> PrefetchHitR {
        PrefetchHitR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "The number of cache line hits after prefetch\n\nYou can [`read`](crate::Reg::read) this register and get [`event_prefetch_hit_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventPrefetchHitCntSpec;
impl crate::RegisterSpec for EventPrefetchHitCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_prefetch_hit_cnt::R`](R) reader structure"]
impl crate::Readable for EventPrefetchHitCntSpec {}
#[doc = "`reset()` method sets EVENT_PREFETCH_HIT_CNT to value 0"]
impl crate::Resettable for EventPrefetchHitCntSpec {}
