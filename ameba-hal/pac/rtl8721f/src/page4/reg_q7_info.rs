#[doc = "Register `REG_Q7_INFO` reader"]
pub type R = crate::R<RegQ7InfoSpec>;
#[doc = "Field `BIT_Head_PKT_Q7` reader - The head page of the first packet of Queue 7."]
pub type BitHeadPktQ7R = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_Q7_V1` reader - The total Queue 7 pkt number in TxPktbuffer"]
pub type BitPktNumQ7V1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_Q7_V1` reader - The head page of the last packet of Queue 7."]
pub type BitTailPktQ7V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_Q7_V1` reader - AC type of Queue 7."]
pub type BitQueueAcQ7V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_Q7_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdQ7V1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue 7."]
    #[inline(always)]
    pub fn bit_head_pkt_q7(&self) -> BitHeadPktQ7R {
        BitHeadPktQ7R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total Queue 7 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_q7_v1(&self) -> BitPktNumQ7V1R {
        BitPktNumQ7V1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of Queue 7."]
    #[inline(always)]
    pub fn bit_tail_pkt_q7_v1(&self) -> BitTailPktQ7V1R {
        BitTailPktQ7V1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of Queue 7."]
    #[inline(always)]
    pub fn bit_queue_ac_q7_v1(&self) -> BitQueueAcQ7V1R {
        BitQueueAcQ7V1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_q7_v1(&self) -> BitQueueMacIdQ7V1R {
        BitQueueMacIdQ7V1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q7_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQ7InfoSpec;
impl crate::RegisterSpec for RegQ7InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q7_info::R`](R) reader structure"]
impl crate::Readable for RegQ7InfoSpec {}
#[doc = "`reset()` method sets REG_Q7_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegQ7InfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
