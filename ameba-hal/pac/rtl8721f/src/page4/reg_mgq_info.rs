#[doc = "Register `REG_MGQ_INFO` reader"]
pub type R = crate::R<RegMgqInfoSpec>;
#[doc = "Field `BIT_Head_PKT_MGQ` reader - The head page of the first packet of MGQ."]
pub type BitHeadPktMgqR = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_MGQ_V1` reader - The total MGQ pkt number in TxPktbuffer"]
pub type BitPktNumMgqV1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_MGQ_V1` reader - The head page of the last packet of MGQ."]
pub type BitTailPktMgqV1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_MGQ_V1` reader - AC type of management Queue."]
pub type BitQueueAcMgqV1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_MGQ_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdMgqV1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of MGQ."]
    #[inline(always)]
    pub fn bit_head_pkt_mgq(&self) -> BitHeadPktMgqR {
        BitHeadPktMgqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total MGQ pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_mgq_v1(&self) -> BitPktNumMgqV1R {
        BitPktNumMgqV1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of MGQ."]
    #[inline(always)]
    pub fn bit_tail_pkt_mgq_v1(&self) -> BitTailPktMgqV1R {
        BitTailPktMgqV1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of management Queue."]
    #[inline(always)]
    pub fn bit_queue_ac_mgq_v1(&self) -> BitQueueAcMgqV1R {
        BitQueueAcMgqV1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_mgq_v1(&self) -> BitQueueMacIdMgqV1R {
        BitQueueMacIdMgqV1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mgq_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMgqInfoSpec;
impl crate::RegisterSpec for RegMgqInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mgq_info::R`](R) reader structure"]
impl crate::Readable for RegMgqInfoSpec {}
#[doc = "`reset()` method sets REG_MGQ_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegMgqInfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
