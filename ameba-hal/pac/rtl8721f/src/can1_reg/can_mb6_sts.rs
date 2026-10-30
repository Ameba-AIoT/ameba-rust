#[doc = "Register `CAN_MB6_STS` reader"]
pub type R = crate::R<CanMb6StsSpec>;
#[doc = "Field `can_msg6_tx_req` reader - tx message is pending for transmit"]
pub type CanMsg6TxReqR = crate::BitReader;
#[doc = "Field `can_msg6_tx_done` reader - tx message in the message buffer finish sending"]
pub type CanMsg6TxDoneR = crate::BitReader;
#[doc = "Field `can_msg6_rx_rdy` reader - the message buffer is ready for receiving a new message"]
pub type CanMsg6RxRdyR = crate::BitReader;
#[doc = "Field `can_msg6_rx_vld` reader - new message have been received in the message buffer"]
pub type CanMsg6RxVldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx message is pending for transmit"]
    #[inline(always)]
    pub fn can_msg6_tx_req(&self) -> CanMsg6TxReqR {
        CanMsg6TxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx message in the message buffer finish sending"]
    #[inline(always)]
    pub fn can_msg6_tx_done(&self) -> CanMsg6TxDoneR {
        CanMsg6TxDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the message buffer is ready for receiving a new message"]
    #[inline(always)]
    pub fn can_msg6_rx_rdy(&self) -> CanMsg6RxRdyR {
        CanMsg6RxRdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - new message have been received in the message buffer"]
    #[inline(always)]
    pub fn can_msg6_rx_vld(&self) -> CanMsg6RxVldR {
        CanMsg6RxVldR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb6_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb6StsSpec;
impl crate::RegisterSpec for CanMb6StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb6_sts::R`](R) reader structure"]
impl crate::Readable for CanMb6StsSpec {}
#[doc = "`reset()` method sets CAN_MB6_STS to value 0"]
impl crate::Resettable for CanMb6StsSpec {}
