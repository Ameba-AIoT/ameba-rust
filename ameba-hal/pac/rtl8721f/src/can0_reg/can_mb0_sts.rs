#[doc = "Register `CAN_MB0_STS` reader"]
pub type R = crate::R<CanMb0StsSpec>;
#[doc = "Field `can_msg0_tx_req` reader - tx message is pending for transmit"]
pub type CanMsg0TxReqR = crate::BitReader;
#[doc = "Field `can_msg0_tx_done` reader - tx message in the message buffer finish sending"]
pub type CanMsg0TxDoneR = crate::BitReader;
#[doc = "Field `can_msg0_rx_rdy` reader - the message buffer is ready for receiving a new message"]
pub type CanMsg0RxRdyR = crate::BitReader;
#[doc = "Field `can_msg0_rx_vld` reader - new message have been received in the message buffer"]
pub type CanMsg0RxVldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx message is pending for transmit"]
    #[inline(always)]
    pub fn can_msg0_tx_req(&self) -> CanMsg0TxReqR {
        CanMsg0TxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx message in the message buffer finish sending"]
    #[inline(always)]
    pub fn can_msg0_tx_done(&self) -> CanMsg0TxDoneR {
        CanMsg0TxDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the message buffer is ready for receiving a new message"]
    #[inline(always)]
    pub fn can_msg0_rx_rdy(&self) -> CanMsg0RxRdyR {
        CanMsg0RxRdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - new message have been received in the message buffer"]
    #[inline(always)]
    pub fn can_msg0_rx_vld(&self) -> CanMsg0RxVldR {
        CanMsg0RxVldR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb0_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb0StsSpec;
impl crate::RegisterSpec for CanMb0StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb0_sts::R`](R) reader structure"]
impl crate::Readable for CanMb0StsSpec {}
#[doc = "`reset()` method sets CAN_MB0_STS to value 0"]
impl crate::Resettable for CanMb0StsSpec {}
