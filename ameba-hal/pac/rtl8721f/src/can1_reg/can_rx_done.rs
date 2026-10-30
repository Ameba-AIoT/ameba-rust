#[doc = "Register `CAN_RX_DONE` reader"]
pub type R = crate::R<CanRxDoneSpec>;
#[doc = "Field `can_rx_done` reader - rx done for each message buffer"]
pub type CanRxDoneR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - rx done for each message buffer"]
    #[inline(always)]
    pub fn can_rx_done(&self) -> CanRxDoneR {
        CanRxDoneR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CAN rx done for message buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rx_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRxDoneSpec;
impl crate::RegisterSpec for CanRxDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rx_done::R`](R) reader structure"]
impl crate::Readable for CanRxDoneSpec {}
#[doc = "`reset()` method sets CAN_RX_DONE to value 0"]
impl crate::Resettable for CanRxDoneSpec {}
