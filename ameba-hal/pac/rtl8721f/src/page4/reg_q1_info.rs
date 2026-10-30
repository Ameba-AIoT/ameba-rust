#[doc = "Register `REG_Q1_INFO` reader"]
pub type R = crate::R<RegQ1InfoSpec>;
#[doc = "Field `BIT_Head_PKT_Q1` reader - The head page of the first packet of Queue 1."]
pub type BitHeadPktQ1R = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_Q1_V1` reader - The total Queue 1 pkt number in TxPktbuffer"]
pub type BitPktNumQ1V1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_Q1_V1` reader - The head page of the last packet of Queue 1."]
pub type BitTailPktQ1V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_Q1_V1` reader - AC type of Queue 1."]
pub type BitQueueAcQ1V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_Q1_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdQ1V1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue 1."]
    #[inline(always)]
    pub fn bit_head_pkt_q1(&self) -> BitHeadPktQ1R {
        BitHeadPktQ1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total Queue 1 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_q1_v1(&self) -> BitPktNumQ1V1R {
        BitPktNumQ1V1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of Queue 1."]
    #[inline(always)]
    pub fn bit_tail_pkt_q1_v1(&self) -> BitTailPktQ1V1R {
        BitTailPktQ1V1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of Queue 1."]
    #[inline(always)]
    pub fn bit_queue_ac_q1_v1(&self) -> BitQueueAcQ1V1R {
        BitQueueAcQ1V1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_q1_v1(&self) -> BitQueueMacIdQ1V1R {
        BitQueueMacIdQ1V1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q1_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQ1InfoSpec;
impl crate::RegisterSpec for RegQ1InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q1_info::R`](R) reader structure"]
impl crate::Readable for RegQ1InfoSpec {}
#[doc = "`reset()` method sets REG_Q1_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegQ1InfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
