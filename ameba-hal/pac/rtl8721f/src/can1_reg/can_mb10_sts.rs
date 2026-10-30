#[doc = "Register `CAN_MB10_STS` reader"]
pub type R = crate::R<CanMb10StsSpec>;
#[doc = "Field `can_msg10_tx_req` reader - tx message is pending for transmit"]
pub type CanMsg10TxReqR = crate::BitReader;
#[doc = "Field `can_msg10_tx_done` reader - tx message in the message buffer finish sending"]
pub type CanMsg10TxDoneR = crate::BitReader;
#[doc = "Field `can_msg10_rx_rdy` reader - the message buffer is ready for receiving a new message"]
pub type CanMsg10RxRdyR = crate::BitReader;
#[doc = "Field `can_msg10_rx_vld` reader - new message have been received in the message buffer"]
pub type CanMsg10RxVldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx message is pending for transmit"]
    #[inline(always)]
    pub fn can_msg10_tx_req(&self) -> CanMsg10TxReqR {
        CanMsg10TxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx message in the message buffer finish sending"]
    #[inline(always)]
    pub fn can_msg10_tx_done(&self) -> CanMsg10TxDoneR {
        CanMsg10TxDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the message buffer is ready for receiving a new message"]
    #[inline(always)]
    pub fn can_msg10_rx_rdy(&self) -> CanMsg10RxRdyR {
        CanMsg10RxRdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - new message have been received in the message buffer"]
    #[inline(always)]
    pub fn can_msg10_rx_vld(&self) -> CanMsg10RxVldR {
        CanMsg10RxVldR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb10_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMb10StsSpec;
impl crate::RegisterSpec for CanMb10StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb10_sts::R`](R) reader structure"]
impl crate::Readable for CanMb10StsSpec {}
#[doc = "`reset()` method sets CAN_MB10_STS to value 0"]
impl crate::Resettable for CanMb10StsSpec {}
