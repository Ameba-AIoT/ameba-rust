#[doc = "Register `HIMR3` reader"]
pub type R = crate::R<Himr3Spec>;
#[doc = "Register `HIMR3` writer"]
pub type W = crate::W<Himr3Spec>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `BIT_HS_NOA0_TSF_BIT32_INT_EN` reader - NOA unit0 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa0TsfBit32IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_NOA0_TSF_BIT32_INT_EN` writer - NOA unit0 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa0TsfBit32IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_NOA1_TSFT_BIT32_INT_EN` reader - NOA unit1 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa1TsftBit32IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_NOA1_TSFT_BIT32_INT_EN` writer - NOA unit1 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa1TsftBit32IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_NOA2_TSFT_BIT32_INT_EN` reader - NOA unit2 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa2TsftBit32IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_NOA2_TSFT_BIT32_INT_EN` writer - NOA unit2 TSFT bit 32 toggle interrupt"]
pub type BitHsNoa2TsftBit32IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_P2P_RFOFF0_INT_EN` reader - P2P NoA RF off time indication 0 interrupt"]
pub type BitHsP2pRfoff0IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFOFF0_INT_EN` writer - P2P NoA RF off time indication 0 interrupt"]
pub type BitHsP2pRfoff0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_P2P_RFON0_INT_EN` reader - P2P NoA RF on time indication 0 interrupt"]
pub type BitHsP2pRfon0IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFON0_INT_EN` writer - P2P NoA RF on time indication 0 interrupt"]
pub type BitHsP2pRfon0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_P2P_RFOFF1_INT_EN` reader - P2P NoA RF off time indication 1 interrupt"]
pub type BitHsP2pRfoff1IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFOFF1_INT_EN` writer - P2P NoA RF off time indication 1 interrupt"]
pub type BitHsP2pRfoff1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_P2P_RFON1_INT_EN` reader - P2P NoA RF on time indication 1 interrupt"]
pub type BitHsP2pRfon1IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFON1_INT_EN` writer - P2P NoA RF on time indication 1 interrupt"]
pub type BitHsP2pRfon1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_P2P_RFOFF2_INT_EN` reader - P2P NoA RF off time indication 2 interrupt"]
pub type BitHsP2pRfoff2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFOFF2_INT_EN` writer - P2P NoA RF off time indication 2 interrupt"]
pub type BitHsP2pRfoff2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_P2P_RFON2_INT_EN` reader - P2P NoA RF on time indication 2 interrupt"]
pub type BitHsP2pRfon2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_P2P_RFON2_INT_EN` writer - P2P NoA RF on time indication 2 interrupt"]
pub type BitHsP2pRfon2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CTWEnd0_INT_EN` reader - CTWindow End NOA0 interrupt."]
pub type BitHsCtwend0IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWEnd0_INT_EN` writer - CTWindow End NOA0 interrupt."]
pub type BitHsCtwend0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CTWEnd1_INT_EN` reader - CTWindow End NOA1 interrupt."]
pub type BitHsCtwend1IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWEnd1_INT_EN` writer - CTWindow End NOA1 interrupt."]
pub type BitHsCtwend1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CTWEnd2_INT_EN` reader - CTWindow End NOA2 interrupt."]
pub type BitHsCtwend2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWEnd2_INT_EN` writer - CTWindow End NOA2 interrupt."]
pub type BitHsCtwend2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CSI_BUF_FULL_INT_EN` reader - indicate channel info rxcsibuffer is full"]
pub type BitHsCsiBufFullIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_CSI_BUF_FULL_INT_EN` writer - indicate channel info rxcsibuffer is full"]
pub type BitHsCsiBufFullIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CHINFO_SEG_DONE_INT_EN` reader - indicate one channel info segment is received ok"]
pub type BitHsChinfoSegDoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_CHINFO_SEG_DONE_INT_EN` writer - indicate one channel info segment is received ok"]
pub type BitHsChinfoSegDoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_RX_PAYLOAD_MATCH_INT_EN` reader - indicate rx payload content match"]
pub type BitHsRxPayloadMatchIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_RX_PAYLOAD_MATCH_INT_EN` writer - indicate rx payload content match"]
pub type BitHsRxPayloadMatchIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXTM_INT_EN` reader - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
pub type BitHsTxtmIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXTM_INT_EN` writer - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
pub type BitHsTxtmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_RXTM_INT_EN` reader - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
pub type BitHsRxtmIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_RXTM_INT_EN` writer - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
pub type BitHsRxtmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_RXTMREQ_INT_EN` reader - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
pub type BitHsRxtmreqIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_RXTMREQ_INT_EN` writer - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
pub type BitHsRxtmreqIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - NOA unit0 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa0_tsf_bit32_int_en(&self) -> BitHsNoa0TsfBit32IntEnR {
        BitHsNoa0TsfBit32IntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NOA unit1 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa1_tsft_bit32_int_en(&self) -> BitHsNoa1TsftBit32IntEnR {
        BitHsNoa1TsftBit32IntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NOA unit2 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa2_tsft_bit32_int_en(&self) -> BitHsNoa2TsftBit32IntEnR {
        BitHsNoa2TsftBit32IntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - P2P NoA RF off time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff0_int_en(&self) -> BitHsP2pRfoff0IntEnR {
        BitHsP2pRfoff0IntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - P2P NoA RF on time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon0_int_en(&self) -> BitHsP2pRfon0IntEnR {
        BitHsP2pRfon0IntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - P2P NoA RF off time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff1_int_en(&self) -> BitHsP2pRfoff1IntEnR {
        BitHsP2pRfoff1IntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - P2P NoA RF on time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon1_int_en(&self) -> BitHsP2pRfon1IntEnR {
        BitHsP2pRfon1IntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - P2P NoA RF off time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff2_int_en(&self) -> BitHsP2pRfoff2IntEnR {
        BitHsP2pRfoff2IntEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - P2P NoA RF on time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon2_int_en(&self) -> BitHsP2pRfon2IntEnR {
        BitHsP2pRfon2IntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CTWindow End NOA0 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend0_int_en(&self) -> BitHsCtwend0IntEnR {
        BitHsCtwend0IntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CTWindow End NOA1 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend1_int_en(&self) -> BitHsCtwend1IntEnR {
        BitHsCtwend1IntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CTWindow End NOA2 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend2_int_en(&self) -> BitHsCtwend2IntEnR {
        BitHsCtwend2IntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - indicate channel info rxcsibuffer is full"]
    #[inline(always)]
    pub fn bit_hs_csi_buf_full_int_en(&self) -> BitHsCsiBufFullIntEnR {
        BitHsCsiBufFullIntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - indicate one channel info segment is received ok"]
    #[inline(always)]
    pub fn bit_hs_chinfo_seg_done_int_en(&self) -> BitHsChinfoSegDoneIntEnR {
        BitHsChinfoSegDoneIntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - indicate rx payload content match"]
    #[inline(always)]
    pub fn bit_hs_rx_payload_match_int_en(&self) -> BitHsRxPayloadMatchIntEnR {
        BitHsRxPayloadMatchIntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_txtm_int_en(&self) -> BitHsTxtmIntEnR {
        BitHsTxtmIntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_rxtm_int_en(&self) -> BitHsRxtmIntEnR {
        BitHsRxtmIntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_rxtmreq_int_en(&self) -> BitHsRxtmreqIntEnR {
        BitHsRxtmreqIntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, Himr3Spec> {
        RsvdW::new(self, 0)
    }
    #[doc = "Bit 14 - NOA unit0 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa0_tsf_bit32_int_en(&mut self) -> BitHsNoa0TsfBit32IntEnW<'_, Himr3Spec> {
        BitHsNoa0TsfBit32IntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - NOA unit1 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa1_tsft_bit32_int_en(&mut self) -> BitHsNoa1TsftBit32IntEnW<'_, Himr3Spec> {
        BitHsNoa1TsftBit32IntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - NOA unit2 TSFT bit 32 toggle interrupt"]
    #[inline(always)]
    pub fn bit_hs_noa2_tsft_bit32_int_en(&mut self) -> BitHsNoa2TsftBit32IntEnW<'_, Himr3Spec> {
        BitHsNoa2TsftBit32IntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - P2P NoA RF off time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff0_int_en(&mut self) -> BitHsP2pRfoff0IntEnW<'_, Himr3Spec> {
        BitHsP2pRfoff0IntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - P2P NoA RF on time indication 0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon0_int_en(&mut self) -> BitHsP2pRfon0IntEnW<'_, Himr3Spec> {
        BitHsP2pRfon0IntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - P2P NoA RF off time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff1_int_en(&mut self) -> BitHsP2pRfoff1IntEnW<'_, Himr3Spec> {
        BitHsP2pRfoff1IntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - P2P NoA RF on time indication 1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon1_int_en(&mut self) -> BitHsP2pRfon1IntEnW<'_, Himr3Spec> {
        BitHsP2pRfon1IntEnW::new(self, 20)
    }
    #[doc = "Bit 21 - P2P NoA RF off time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfoff2_int_en(&mut self) -> BitHsP2pRfoff2IntEnW<'_, Himr3Spec> {
        BitHsP2pRfoff2IntEnW::new(self, 21)
    }
    #[doc = "Bit 22 - P2P NoA RF on time indication 2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_p2p_rfon2_int_en(&mut self) -> BitHsP2pRfon2IntEnW<'_, Himr3Spec> {
        BitHsP2pRfon2IntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - CTWindow End NOA0 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend0_int_en(&mut self) -> BitHsCtwend0IntEnW<'_, Himr3Spec> {
        BitHsCtwend0IntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - CTWindow End NOA1 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend1_int_en(&mut self) -> BitHsCtwend1IntEnW<'_, Himr3Spec> {
        BitHsCtwend1IntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - CTWindow End NOA2 interrupt."]
    #[inline(always)]
    pub fn bit_hs_ctwend2_int_en(&mut self) -> BitHsCtwend2IntEnW<'_, Himr3Spec> {
        BitHsCtwend2IntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - indicate channel info rxcsibuffer is full"]
    #[inline(always)]
    pub fn bit_hs_csi_buf_full_int_en(&mut self) -> BitHsCsiBufFullIntEnW<'_, Himr3Spec> {
        BitHsCsiBufFullIntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - indicate one channel info segment is received ok"]
    #[inline(always)]
    pub fn bit_hs_chinfo_seg_done_int_en(&mut self) -> BitHsChinfoSegDoneIntEnW<'_, Himr3Spec> {
        BitHsChinfoSegDoneIntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - indicate rx payload content match"]
    #[inline(always)]
    pub fn bit_hs_rx_payload_match_int_en(&mut self) -> BitHsRxPayloadMatchIntEnW<'_, Himr3Spec> {
        BitHsRxPayloadMatchIntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_txtm_int_en(&mut self) -> BitHsTxtmIntEnW<'_, Himr3Spec> {
        BitHsTxtmIntEnW::new(self, 29)
    }
    #[doc = "Bit 30 - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_rxtm_int_en(&mut self) -> BitHsRxtmIntEnW<'_, Himr3Spec> {
        BitHsRxtmIntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_hs_rxtmreq_int_en(&mut self) -> BitHsRxtmreqIntEnW<'_, Himr3Spec> {
        BitHsRxtmreqIntEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`himr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`himr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Himr3Spec;
impl crate::RegisterSpec for Himr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`himr3::R`](R) reader structure"]
impl crate::Readable for Himr3Spec {}
#[doc = "`write(|w| ..)` method takes [`himr3::W`](W) writer structure"]
impl crate::Writable for Himr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIMR3 to value 0"]
impl crate::Resettable for Himr3Spec {}
