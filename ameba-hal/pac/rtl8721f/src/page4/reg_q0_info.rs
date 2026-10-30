#[doc = "Register `REG_Q0_INFO` reader"]
pub type R = crate::R<RegQ0InfoSpec>;
#[doc = "Field `BIT_Head_PKT_Q0` reader - The head page of the first packet of Queue 0."]
pub type BitHeadPktQ0R = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_Q0_V1` reader - The total Queue 0 pkt number in TxPktbuffer"]
pub type BitPktNumQ0V1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_Q0_V1` reader - The head page of the last packet of Queue 0."]
pub type BitTailPktQ0V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_Q0_V1` reader - AC type of Queue 0."]
pub type BitQueueAcQ0V1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_Q0_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdQ0V1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue 0."]
    #[inline(always)]
    pub fn bit_head_pkt_q0(&self) -> BitHeadPktQ0R {
        BitHeadPktQ0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total Queue 0 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_q0_v1(&self) -> BitPktNumQ0V1R {
        BitPktNumQ0V1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of Queue 0."]
    #[inline(always)]
    pub fn bit_tail_pkt_q0_v1(&self) -> BitTailPktQ0V1R {
        BitTailPktQ0V1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of Queue 0."]
    #[inline(always)]
    pub fn bit_queue_ac_q0_v1(&self) -> BitQueueAcQ0V1R {
        BitQueueAcQ0V1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_q0_v1(&self) -> BitQueueMacIdQ0V1R {
        BitQueueMacIdQ0V1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q0_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQ0InfoSpec;
impl crate::RegisterSpec for RegQ0InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q0_info::R`](R) reader structure"]
impl crate::Readable for RegQ0InfoSpec {}
#[doc = "`reset()` method sets REG_Q0_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegQ0InfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
