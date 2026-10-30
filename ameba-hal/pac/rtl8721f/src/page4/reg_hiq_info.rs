#[doc = "Register `REG_HIQ_INFO` reader"]
pub type R = crate::R<RegHiqInfoSpec>;
#[doc = "Field `BIT_Head_PKT_HIQ` reader - The head page of the first packet of HIQ."]
pub type BitHeadPktHiqR = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_HIQ_V1` reader - The total HIQ pkt number in TxPktbuffer"]
pub type BitPktNumHiqV1R = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_HIQ_V1` reader - The head page of the last packet of HIQ."]
pub type BitTailPktHiqV1R = crate::FieldReader;
#[doc = "Field `BIT_QueueAC_HIQ_V1` reader - AC type of High Queue."]
pub type BitQueueAcHiqV1R = crate::FieldReader;
#[doc = "Field `BIT_QueueMacID_HIQ_V1` reader - MacID of head pkt"]
pub type BitQueueMacIdHiqV1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of HIQ."]
    #[inline(always)]
    pub fn bit_head_pkt_hiq(&self) -> BitHeadPktHiqR {
        BitHeadPktHiqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - The total HIQ pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_hiq_v1(&self) -> BitPktNumHiqV1R {
        BitPktNumHiqV1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:22 - The head page of the last packet of HIQ."]
    #[inline(always)]
    pub fn bit_tail_pkt_hiq_v1(&self) -> BitTailPktHiqV1R {
        BitTailPktHiqV1R::new(((self.bits >> 15) & 0xff) as u8)
    }
    #[doc = "Bits 23:24 - AC type of High Queue."]
    #[inline(always)]
    pub fn bit_queue_ac_hiq_v1(&self) -> BitQueueAcHiqV1R {
        BitQueueAcHiqV1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:31 - MacID of head pkt"]
    #[inline(always)]
    pub fn bit_queue_mac_id_hiq_v1(&self) -> BitQueueMacIdHiqV1R {
        BitQueueMacIdHiqV1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hiq_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHiqInfoSpec;
impl crate::RegisterSpec for RegHiqInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hiq_info::R`](R) reader structure"]
impl crate::Readable for RegHiqInfoSpec {}
#[doc = "`reset()` method sets REG_HIQ_INFO to value 0x007f_80ff"]
impl crate::Resettable for RegHiqInfoSpec {
    const RESET_VALUE: u32 = 0x007f_80ff;
}
