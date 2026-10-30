#[doc = "Register `REG_CMDQ_INFO` reader"]
pub type R = crate::R<RegCmdqInfoSpec>;
#[doc = "Field `BIT_Head_PKT_CMDQ` reader - The head page of the first packet of Queue 4."]
pub type BitHeadPktCmdqR = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_CMDQ` reader - The total Queue 4 pkt number in TxPktbuffer"]
pub type BitPktNumCmdqR = crate::FieldReader;
#[doc = "Field `BIT_Tail_PKT_CMDQ` reader - The head page of the last packet of Queue 4."]
pub type BitTailPktCmdqR = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_CMDQ_V2` reader - The total CMD Queue pkt number in TxPktbuffer"]
pub type BitPktNumCmdqV2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Queue 4."]
    #[inline(always)]
    pub fn bit_head_pkt_cmdq(&self) -> BitHeadPktCmdqR {
        BitHeadPktCmdqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The total Queue 4 pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_cmdq(&self) -> BitPktNumCmdqR {
        BitPktNumCmdqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The head page of the last packet of Queue 4."]
    #[inline(always)]
    pub fn bit_tail_pkt_cmdq(&self) -> BitTailPktCmdqR {
        BitTailPktCmdqR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The total CMD Queue pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_cmdq_v2(&self) -> BitPktNumCmdqV2R {
        BitPktNumCmdqV2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmdq_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCmdqInfoSpec;
impl crate::RegisterSpec for RegCmdqInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cmdq_info::R`](R) reader structure"]
impl crate::Readable for RegCmdqInfoSpec {}
#[doc = "`reset()` method sets REG_CMDQ_INFO to value 0x00ff_00ff"]
impl crate::Resettable for RegCmdqInfoSpec {
    const RESET_VALUE: u32 = 0x00ff_00ff;
}
