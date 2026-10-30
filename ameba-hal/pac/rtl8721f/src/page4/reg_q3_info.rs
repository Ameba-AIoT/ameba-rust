#[doc = "Register `REG_Q3_INFO` reader"]
pub type R = crate::R<RegQ3InfoSpec>;
#[doc = "Field `BIT_Head_PKT_Q3` reader - The head page of the first packet of Queue 3."]
pub type BitHeadPktQ3R = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_Q3_V1` reader - The total Queue 3 pkt number in TxPktbuffer"]
pub type BitPktNumQ3V1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_Q3_V1` reader - The head page of the last packet of Queue 3."]
pub type BitTailPktQ3V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_Q3_V1` reader - AC type of Queue 3."]
pub type BitQueueAcQ3V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_Q3_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdQ3V1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue 3."]
    #[inline(always)]
    pub fn bit_head_pkt_q3(&self) -> BitHeadPktQ3R {
        BitHeadPktQ3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total Queue 3 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_q3_v1(&self) -> BitPktNumQ3V1R {
        BitPktNumQ3V1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of Queue 3."]
    #[inline(always)]
    pub fn bit_tail_pkt_q3_v1(&self) -> BitTailPktQ3V1R {
        BitTailPktQ3V1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of Queue 3."]
    #[inline(always)]
    pub fn bit_queue_ac_q3_v1(&self) -> BitQueueAcQ3V1R {
        BitQueueAcQ3V1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_q3_v1(&self) -> BitQueueMacIdQ3V1R {
        BitQueueMacIdQ3V1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q3_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQ3InfoSpec;
impl crate::RegisterSpec for RegQ3InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q3_info::R`](R) reader structure"]
impl crate::Readable for RegQ3InfoSpec {}
#[doc = "`reset()` method sets REG_Q3_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegQ3InfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
