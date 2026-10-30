#[doc = "Register `CAN_MB9_STS` reader"]
pub type R = crate::R<CanMb9StsSpec>;
#[doc = "Field `can_msg9_tx_req` reader - tx message is pending for transmit"]
pub type CanMsg9TxReqR = crate::BitReader;
#[doc = "Field `can_msg9_tx_done` reader - tx message in the message buffer finish sending"]
pub type CanMsg9TxDoneR = crate::BitReader;
#[doc = "Field `can_msg9_rx_rdy` reader - the message buffer is ready for receiving a new message"]
pub type CanMsg9RxRdyR = crate::BitReader;
#[doc = "Field `can_msg9_rx_vld` reader - new message have been received in the message buffer"]
pub type CanMsg9RxVldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx message is pending for transmit"]
    #[inline(always)]
    pub fn can_msg9_tx_req(&self) -> CanMsg9TxReqR {
        CanMsg9TxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx message in the message buffer finish sending"]
    #[inline(always)]
    pub fn can_msg9_tx_done(&self) -> CanMsg9TxDoneR {
        CanMsg9TxDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the message buffer is ready for receiving a new message"]
    #[inline(always)]
    pub fn can_msg9_rx_rdy(&self) -> CanMsg9RxRdyR {
        CanMsg9RxRdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - new message have been received in the message buffer"]
    #[inline(always)]
    pub fn can_msg9_rx_vld(&self) -> CanMsg9RxVldR {
        CanMsg9RxVldR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb9_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb9StsSpec;
impl crate::RegisterSpec for CanMb9StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb9_sts::R`](R) reader structure"]
impl crate::Readable for CanMb9StsSpec {}
#[doc = "`reset()` method sets CAN_MB9_STS to value 0"]
impl crate::Resettable for CanMb9StsSpec {}
