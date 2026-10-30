#[doc = "Register `REG_Q2_INFO` reader"]
pub type R = crate::R<RegQ2InfoSpec>;
#[doc = "Field `BIT_Head_PKT_Q2` reader - The head page of the first packet of Queue 2."]
pub type BitHeadPktQ2R = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_Q2_V1` reader - The total Queue 2 pkt number in TxPktbuffer"]
pub type BitPktNumQ2V1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_Q2_V1` reader - The head page of the last packet of Queue 2."]
pub type BitTailPktQ2V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_Q2_V1` reader - AC type of Queue 2."]
pub type BitQueueAcQ2V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_Q2_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdQ2V1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue 2."]
    #[inline(always)]
    pub fn bit_head_pkt_q2(&self) -> BitHeadPktQ2R {
        BitHeadPktQ2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total Queue 2 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_q2_v1(&self) -> BitPktNumQ2V1R {
        BitPktNumQ2V1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of Queue 2."]
    #[inline(always)]
    pub fn bit_tail_pkt_q2_v1(&self) -> BitTailPktQ2V1R {
        BitTailPktQ2V1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of Queue 2."]
    #[inline(always)]
    pub fn bit_queue_ac_q2_v1(&self) -> BitQueueAcQ2V1R {
        BitQueueAcQ2V1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_q2_v1(&self) -> BitQueueMacIdQ2V1R {
        BitQueueMacIdQ2V1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q2_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQ2InfoSpec;
impl crate::RegisterSpec for RegQ2InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q2_info::R`](R) reader structure"]
impl crate::Readable for RegQ2InfoSpec {}
#[doc = "`reset()` method sets REG_Q2_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegQ2InfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
