#[doc = "Register `EVENT_DATA_DELAY_CNT` reader"]
pub type R = crate::R<EventDataDelayCntSpec>;
#[doc = "Field `DATA_DELAY` reader - The number of cache line hits after prefetch"]
pub type DataDelayR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - The number of cache line hits after prefetch"]
    #[inline(always)]
    pub fn data_delay(&self) -> DataDelayR {
        DataDelayR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "The number of transfer data is not immediately returned after cache line hits in the stream buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`event_data_delay_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventDataDelayCntSpec;
impl crate::RegisterSpec for EventDataDelayCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_data_delay_cnt::R`](R) reader structure"]
impl crate::Readable for EventDataDelayCntSpec {}
#[doc = "`reset()` method sets EVENT_DATA_DELAY_CNT to value 0"]
impl crate::Resettable for EventDataDelayCntSpec {}
