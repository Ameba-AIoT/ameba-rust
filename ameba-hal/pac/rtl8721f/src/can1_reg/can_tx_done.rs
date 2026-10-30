#[doc = "Register `CAN_TX_DONE` reader"]
pub type R = crate::R<CanTxDoneSpec>;
#[doc = "Field `can_tx_done` reader - tx done for each message buffer"]
pub type CanTxDoneR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - tx done for each message buffer"]
    #[inline(always)]
    pub fn can_tx_done(&self) -> CanTxDoneR {
        CanTxDoneR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CAN tx done for message buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tx_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanTxDoneSpec;
impl crate::RegisterSpec for CanTxDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_tx_done::R`](R) reader structure"]
impl crate::Readable for CanTxDoneSpec {}
#[doc = "`reset()` method sets CAN_TX_DONE to value 0"]
impl crate::Resettable for CanTxDoneSpec {}
