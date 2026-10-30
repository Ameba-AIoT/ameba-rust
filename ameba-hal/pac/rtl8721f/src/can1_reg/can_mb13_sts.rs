#[doc = "Register `CAN_MB13_STS` reader"]
pub type R = crate::R<CanMb13StsSpec>;
#[doc = "Field `can_msg13_tx_req` reader - tx message is pending for transmit"]
pub type CanMsg13TxReqR = crate::BitReader;
#[doc = "Field `can_msg13_tx_done` reader - tx message in the message buffer finish sending"]
pub type CanMsg13TxDoneR = crate::BitReader;
#[doc = "Field `can_msg13_rx_rdy` reader - the message buffer is ready for receiving a new message"]
pub type CanMsg13RxRdyR = crate::BitReader;
#[doc = "Field `can_msg13_rx_vld` reader - new message have been received in the message buffer"]
pub type CanMsg13RxVldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx message is pending for transmit"]
    #[inline(always)]
    pub fn can_msg13_tx_req(&self) -> CanMsg13TxReqR {
        CanMsg13TxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx message in the message buffer finish sending"]
    #[inline(always)]
    pub fn can_msg13_tx_done(&self) -> CanMsg13TxDoneR {
        CanMsg13TxDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the message buffer is ready for receiving a new message"]
    #[inline(always)]
    pub fn can_msg13_rx_rdy(&self) -> CanMsg13RxRdyR {
        CanMsg13RxRdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - new message have been received in the message buffer"]
    #[inline(always)]
    pub fn can_msg13_rx_vld(&self) -> CanMsg13RxVldR {
        CanMsg13RxVldR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb13_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb13StsSpec;
impl crate::RegisterSpec for CanMb13StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb13_sts::R`](R) reader structure"]
impl crate::Readable for CanMb13StsSpec {}
#[doc = "`reset()` method sets CAN_MB13_STS to value 0"]
impl crate::Resettable for CanMb13StsSpec {}
