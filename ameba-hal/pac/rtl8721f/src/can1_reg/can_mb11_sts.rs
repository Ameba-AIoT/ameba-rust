#[doc = "Register `CAN_MB11_STS` reader"]
pub type R = crate::R<CanMb11StsSpec>;
#[doc = "Field `can_msg11_tx_req` reader - tx message is pending for transmit"]
pub type CanMsg11TxReqR = crate::BitReader;
#[doc = "Field `can_msg11_tx_done` reader - tx message in the message buffer finish sending"]
pub type CanMsg11TxDoneR = crate::BitReader;
#[doc = "Field `can_msg11_rx_rdy` reader - the message buffer is ready for receiving a new message"]
pub type CanMsg11RxRdyR = crate::BitReader;
#[doc = "Field `can_msg11_rx_vld` reader - new message have been received in the message buffer"]
pub type CanMsg11RxVldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx message is pending for transmit"]
    #[inline(always)]
    pub fn can_msg11_tx_req(&self) -> CanMsg11TxReqR {
        CanMsg11TxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx message in the message buffer finish sending"]
    #[inline(always)]
    pub fn can_msg11_tx_done(&self) -> CanMsg11TxDoneR {
        CanMsg11TxDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the message buffer is ready for receiving a new message"]
    #[inline(always)]
    pub fn can_msg11_rx_rdy(&self) -> CanMsg11RxRdyR {
        CanMsg11RxRdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - new message have been received in the message buffer"]
    #[inline(always)]
    pub fn can_msg11_rx_vld(&self) -> CanMsg11RxVldR {
        CanMsg11RxVldR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb11_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb11StsSpec;
impl crate::RegisterSpec for CanMb11StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb11_sts::R`](R) reader structure"]
impl crate::Readable for CanMb11StsSpec {}
#[doc = "`reset()` method sets CAN_MB11_STS to value 0"]
impl crate::Resettable for CanMb11StsSpec {}
