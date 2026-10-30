#[doc = "Register `REG_Q5_INFO` reader"]
pub type R = crate::R<RegQ5InfoSpec>;
#[doc = "Field `BIT_Head_PKT_Q5` reader - The head page of the first packet of Queue5."]
pub type BitHeadPktQ5R = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_Q5_V1` reader - The total Queue 5 pkt number in TxPktbuffer"]
pub type BitPktNumQ5V1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_Q5_V1` reader - The head page of the last packet of Queue 5."]
pub type BitTailPktQ5V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_Q5_V1` reader - AC type of Queue 5."]
pub type BitQueueAcQ5V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_Q5_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdQ5V1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue5."]
    #[inline(always)]
    pub fn bit_head_pkt_q5(&self) -> BitHeadPktQ5R {
        BitHeadPktQ5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total Queue 5 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_q5_v1(&self) -> BitPktNumQ5V1R {
        BitPktNumQ5V1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of Queue 5."]
    #[inline(always)]
    pub fn bit_tail_pkt_q5_v1(&self) -> BitTailPktQ5V1R {
        BitTailPktQ5V1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of Queue 5."]
    #[inline(always)]
    pub fn bit_queue_ac_q5_v1(&self) -> BitQueueAcQ5V1R {
        BitQueueAcQ5V1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_q5_v1(&self) -> BitQueueMacIdQ5V1R {
        BitQueueMacIdQ5V1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q5_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQ5InfoSpec;
impl crate::RegisterSpec for RegQ5InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q5_info::R`](R) reader structure"]
impl crate::Readable for RegQ5InfoSpec {}
#[doc = "`reset()` method sets REG_Q5_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegQ5InfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
