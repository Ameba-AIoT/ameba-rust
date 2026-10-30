#[doc = "Register `HISR3` reader"]
pub type R = crate::R<Hisr3Spec>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `BIT_HS_NOA0_TSF_BIT32_INT` reader - NOA unit0 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa0TsfBit32IntR = crate::BitReader;
#[doc = "Field `BIT_HS_NOA1_TSFT_BIT32_INT` reader - NOA unit1 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa1TsftBit32IntR = crate::BitReader;
#[doc = "Field `BIT_HS_NOA2_TSFT_BIT32_INT` reader - NOA unit2 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa2TsftBit32IntR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFOFF0_INT` reader - P2P NoA RF off time indication 0 interrupt"]
pub type BitHsP2pRfoff0IntR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFON0_INT` reader - P2P NoA RF on time indication 0 interrupt"]
pub type BitHsP2pRfon0IntR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFOFF1_INT` reader - P2P NoA RF off time indication 1 interrupt"]
pub type BitHsP2pRfoff1IntR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFON1_INT` reader - P2P NoA RF on time indication 1 interrupt"]
pub type BitHsP2pRfon1IntR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFOFF2_INT` reader - P2P NoA RF off time indication 2 interrupt"]
pub type BitHsP2pRfoff2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFON2_INT` reader - P2P NoA RF on time indication 2 interrupt"]
pub type BitHsP2pRfon2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWEnd0_INT` reader - CTWindow End NOA0 interrupt."]
pub type BitHsCtwend0IntR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWEnd1_INT` reader - CTWindow End NOA1 interrupt."]
pub type BitHsCtwend1IntR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWEnd2_INT` reader - CTWindow End NOA2 interrupt."]
pub type BitHsCtwend2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_CSI_BUF_FULL_INT` reader - indicate channel info rxcsibuffer is full"]
pub type BitHsCsiBufFullIntR = crate::BitReader;
#[doc = "Field `BIT_HS_CHINFO_SEG_DONE_INT` reader - indicate one channel info segment is received ok"]
pub type BitHsChinfoSegDoneIntR = crate::BitReader;
#[doc = "Field `BIT_HS_RX_PAYLOAD_MATCH_INT` reader - indicate rx payload content match"]
pub type BitHsRxPayloadMatchIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXTM_INT` reader - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
pub type BitHsTxtmIntR = crate::BitReader;
#[doc = "Field `BIT_HS_RXTM_INT` reader - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
pub type BitHsRxtmIntR = crate::BitReader;
#[doc = "Field `BIT_HS_RXTMREQ_INT` reader - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
pub type BitHsRxtmreqIntR = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - NOA unit0 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa0_tsf_bit32_int(&self) -> BitHsNoa0TsfBit32IntR {
        BitHsNoa0TsfBit32IntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NOA unit1 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa1_tsft_bit32_int(&self) -> BitHsNoa1TsftBit32IntR {
        BitHsNoa1TsftBit32IntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NOA unit2 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa2_tsft_bit32_int(&self) -> BitHsNoa2TsftBit32IntR {
        BitHsNoa2TsftBit32IntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - P2P NoA RF off time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff0_int(&self) -> BitHsP2pRfoff0IntR {
        BitHsP2pRfoff0IntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - P2P NoA RF on time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon0_int(&self) -> BitHsP2pRfon0IntR {
        BitHsP2pRfon0IntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - P2P NoA RF off time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff1_int(&self) -> BitHsP2pRfoff1IntR {
        BitHsP2pRfoff1IntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - P2P NoA RF on time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon1_int(&self) -> BitHsP2pRfon1IntR {
        BitHsP2pRfon1IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - P2P NoA RF off time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff2_int(&self) -> BitHsP2pRfoff2IntR {
        BitHsP2pRfoff2IntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - P2P NoA RF on time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon2_int(&self) -> BitHsP2pRfon2IntR {
        BitHsP2pRfon2IntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CTWindow End NOA0 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend0_int(&self) -> BitHsCtwend0IntR {
        BitHsCtwend0IntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CTWindow End NOA1 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend1_int(&self) -> BitHsCtwend1IntR {
        BitHsCtwend1IntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CTWindow End NOA2 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend2_int(&self) -> BitHsCtwend2IntR {
        BitHsCtwend2IntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - indicate channel info rxcsibuffer is full"]
    #[inline(always)]
    pub fn bit_hs_csi_buf_full_int(&self) -> BitHsCsiBufFullIntR {
        BitHsCsiBufFullIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - indicate one channel info segment is received ok"]
    #[inline(always)]
    pub fn bit_hs_chinfo_seg_done_int(&self) -> BitHsChinfoSegDoneIntR {
        BitHsChinfoSegDoneIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - indicate rx payload content match"]
    #[inline(always)]
    pub fn bit_hs_rx_payload_match_int(&self) -> BitHsRxPayloadMatchIntR {
        BitHsRxPayloadMatchIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_txtm_int(&self) -> BitHsTxtmIntR {
        BitHsTxtmIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_rxtm_int(&self) -> BitHsRxtmIntR {
        BitHsRxtmIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_rxtmreq_int(&self) -> BitHsRxtmreqIntR {
        BitHsRxtmreqIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hisr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hisr3Spec;
impl crate::RegisterSpec for Hisr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hisr3::R`](R) reader structure"]
impl crate::Readable for Hisr3Spec {}
#[doc = "`reset()` method sets HISR3 to value 0"]
impl crate::Resettable for Hisr3Spec {}
