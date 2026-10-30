#[doc = "Register `CAN_TX_ERROR_FLAG` reader"]
pub type R = crate::R<CanTxErrorFlagSpec>;
#[doc = "Field `can_tx_error_flag` reader - indicate which message tx buffer have error detected, cpu can write 1 to clear"]
pub type CanTxErrorFlagR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - indicate which message tx buffer have error detected, cpu can write 1 to clear"]
    #[inline(always)]
    pub fn can_tx_error_flag(&self) -> CanTxErrorFlagR {
        CanTxErrorFlagR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CAN tx error flag for each message buffe\n\nYou can [`read`](crate::Reg::read) this register and get [`can_tx_error_flag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanTxErrorFlagSpec;
impl crate::RegisterSpec for CanTxErrorFlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_tx_error_flag::R`](R) reader structure"]
impl crate::Readable for CanTxErrorFlagSpec {}
#[doc = "`reset()` method sets CAN_TX_ERROR_FLAG to value 0"]
impl crate::Resettable for CanTxErrorFlagSpec {}
