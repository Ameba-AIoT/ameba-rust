#[doc = "Register `CAN_MB4_STS` reader"]
pub type R = crate::R<CanMb4StsSpec>;
#[doc = "Field `can_msg4_tx_req` reader - tx message is pending for transmit"]
pub type CanMsg4TxReqR = crate::BitReader;
#[doc = "Field `can_msg4_tx_done` reader - tx message in the message buffer finish sending"]
pub type CanMsg4TxDoneR = crate::BitReader;
#[doc = "Field `can_msg4_rx_rdy` reader - the message buffer is ready for receiving a new message"]
pub type CanMsg4RxRdyR = crate::BitReader;
#[doc = "Field `can_msg4_rx_vld` reader - new message have been received in the message buffer"]
pub type CanMsg4RxVldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx message is pending for transmit"]
    #[inline(always)]
    pub fn can_msg4_tx_req(&self) -> CanMsg4TxReqR {
        CanMsg4TxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx message in the message buffer finish sending"]
    #[inline(always)]
    pub fn can_msg4_tx_done(&self) -> CanMsg4TxDoneR {
        CanMsg4TxDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the message buffer is ready for receiving a new message"]
    #[inline(always)]
    pub fn can_msg4_rx_rdy(&self) -> CanMsg4RxRdyR {
        CanMsg4RxRdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - new message have been received in the message buffer"]
    #[inline(always)]
    pub fn can_msg4_rx_vld(&self) -> CanMsg4RxVldR {
        CanMsg4RxVldR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb4_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb4StsSpec;
impl crate::RegisterSpec for CanMb4StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb4_sts::R`](R) reader structure"]
impl crate::Readable for CanMb4StsSpec {}
#[doc = "`reset()` method sets CAN_MB4_STS to value 0"]
impl crate::Resettable for CanMb4StsSpec {}
