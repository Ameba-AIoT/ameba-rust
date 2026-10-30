#[doc = "Register `REG_Q4_INFO` reader"]
pub type R = crate::R<RegQ4InfoSpec>;
#[doc = "Field `BIT_Head_PKT_Q4` reader - The head page of the first packet of Queue 4."]
pub type BitHeadPktQ4R = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_Q4_V1` reader - The total Queue 4 pkt number in TxPktbuffer"]
pub type BitPktNumQ4V1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_Q4_V1` reader - The head page of the last packet of Queue 4."]
pub type BitTailPktQ4V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_Q4_V1` reader - AC type of Queue 4."]
pub type BitQueueAcQ4V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_Q4_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdQ4V1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue 4."]
    #[inline(always)]
    pub fn bit_head_pkt_q4(&self) -> BitHeadPktQ4R {
        BitHeadPktQ4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total Queue 4 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_q4_v1(&self) -> BitPktNumQ4V1R {
        BitPktNumQ4V1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of Queue 4."]
    #[inline(always)]
    pub fn bit_tail_pkt_q4_v1(&self) -> BitTailPktQ4V1R {
        BitTailPktQ4V1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of Queue 4."]
    #[inline(always)]
    pub fn bit_queue_ac_q4_v1(&self) -> BitQueueAcQ4V1R {
        BitQueueAcQ4V1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_q4_v1(&self) -> BitQueueMacIdQ4V1R {
        BitQueueMacIdQ4V1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q4_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQ4InfoSpec;
impl crate::RegisterSpec for RegQ4InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q4_info::R`](R) reader structure"]
impl crate::Readable for RegQ4InfoSpec {}
#[doc = "`reset()` method sets REG_Q4_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegQ4InfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
