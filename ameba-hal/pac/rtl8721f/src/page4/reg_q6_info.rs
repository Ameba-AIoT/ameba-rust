#[doc = "Register `REG_Q6_INFO` reader"]
pub type R = crate::R<RegQ6InfoSpec>;
#[doc = "Field `BIT_Head_PKT_Q6` reader - The head page of the first packet of Queue 6."]
pub type BitHeadPktQ6R = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_Q6_V1` reader - The total Queue 6 pkt number in TxPktbuffer"]
pub type BitPktNumQ6V1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_Q6_V1` reader - The head page of the last packet of Queue 6."]
pub type BitTailPktQ6V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_Q6_V1` reader - AC type of Queue 6."]
pub type BitQueueAcQ6V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_Q6_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdQ6V1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue 6."]
    #[inline(always)]
    pub fn bit_head_pkt_q6(&self) -> BitHeadPktQ6R {
        BitHeadPktQ6R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total Queue 6 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_q6_v1(&self) -> BitPktNumQ6V1R {
        BitPktNumQ6V1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of Queue 6."]
    #[inline(always)]
    pub fn bit_tail_pkt_q6_v1(&self) -> BitTailPktQ6V1R {
        BitTailPktQ6V1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of Queue 6."]
    #[inline(always)]
    pub fn bit_queue_ac_q6_v1(&self) -> BitQueueAcQ6V1R {
        BitQueueAcQ6V1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_q6_v1(&self) -> BitQueueMacIdQ6V1R {
        BitQueueMacIdQ6V1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q6_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQ6InfoSpec;
impl crate::RegisterSpec for RegQ6InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q6_info::R`](R) reader structure"]
impl crate::Readable for RegQ6InfoSpec {}
#[doc = "`reset()` method sets REG_Q6_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegQ6InfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
