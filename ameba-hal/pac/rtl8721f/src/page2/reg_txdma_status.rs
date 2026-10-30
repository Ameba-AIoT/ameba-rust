#[doc = "Register `REG_TXDMA_STATUS` reader"]
pub type R = crate::R<RegTxdmaStatusSpec>;
#[doc = "Register `REG_TXDMA_STATUS` writer"]
pub type W = crate::W<RegTxdmaStatusSpec>;
#[doc = "Field `BIT_PG_OVF` reader - TXFF page overflow in TDE page controller.. Clear by SW."]
pub type BitPgOvfR = crate::BitReader;
#[doc = "Field `BIT_PG_UDN` reader - TXFF page underflow in TDE page controller. Clear by SW."]
pub type BitPgUdnR = crate::BitReader;
#[doc = "Field `BIT_PAGE_OVER` reader - Total pages included PTCL un-return pages are over the total reserved pages. Clear by SW."]
pub type BitPageOverR = crate::BitReader;
#[doc = "Field `BIT_PAGE_UNDER` reader - Total pages included PTCL un-return pages are under the total reserved pages minus the page threshold. Clear by SW."]
pub type BitPageUnderR = crate::BitReader;
#[doc = "Field `BIT_FREE_PG_FF` reader - TXDMA reads NULL page. Clear by SW."]
pub type BitFreePgFfR = crate::BitReader;
#[doc = "Field `BIT_SFF_OVF` reader - TXDMA Sync FIFO Overflow. Clear by SW."]
pub type BitSffOvfR = crate::BitReader;
#[doc = "Field `BIT_SFF_UDN` reader - TXDMA Sync FIFO Underflow. Clear by SW."]
pub type BitSffUdnR = crate::BitReader;
#[doc = "Field `BIT_TXOQT_OVF` reader - TXOQT Overflow. Clear by SW."]
pub type BitTxoqtOvfR = crate::BitReader;
#[doc = "Field `BIT_TXOQT_UDN` reader - TXOQT Underflow. Clear by SW."]
pub type BitTxoqtUdnR = crate::BitReader;
#[doc = "Field `BIT_TX_OFFS_UNMATCH` reader - Tx offset is unmatched. Clear by SW."]
pub type BitTxOffsUnmatchR = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `BIT_EP_QSEL_DIFF` reader - Tx Endpoint is unmatched with the QSEL of descriptor. Clear by SW."]
pub type BitEpQselDiffR = crate::BitReader;
#[doc = "Field `BIT_UNKNOWN_QSEL` reader - An unknown QSEL of Tx descriptor is detected. Clear by SW."]
pub type BitUnknownQselR = crate::BitReader;
#[doc = "Field `BIT_DSC_CHKSUM_FAIL` reader - Tx descriptor checksum error. Clear by SW."]
pub type BitDscChksumFailR = crate::BitReader;
#[doc = "Field `BIT_PAYLOAD_OVF` reader - Payload is over the packet length of Tx descriptor. Clear by SW."]
pub type BitPayloadOvfR = crate::BitReader;
#[doc = "Field `BIT_PAYLOAD_UDN` reader - Payload is under the packet length of Tx descriptor. Clear by SW."]
pub type BitPayloadUdnR = crate::BitReader;
#[doc = "Field `BIT_PAYLOAD_CHKSUM_ERR` reader - Payload checksum error"]
pub type BitPayloadChksumErrR = crate::BitReader;
#[doc = "Field `BIT_HI_OQT_OVF` reader - High Priority Queue_Tx OQT overflow"]
pub type BitHiOqtOvfR = crate::BitReader;
#[doc = "Field `BIT_HI_OQT_UDN` reader - High Priority Queue_Tx OQT underflow"]
pub type BitHiOqtUdnR = crate::BitReader;
#[doc = "Field `BIT_TDE_TRIG_STOP_SEL` reader - select txdma engine trigger stop signal for LA dump"]
pub type BitTdeTrigStopSelR = crate::FieldReader;
#[doc = "Field `BIT_TDE_TRIG_STOP_SEL` writer - select txdma engine trigger stop signal for LA dump"]
pub type BitTdeTrigStopSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - TXFF page overflow in TDE page controller.. Clear by SW."]
    #[inline(always)]
    pub fn bit_pg_ovf(&self) -> BitPgOvfR {
        BitPgOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFF page underflow in TDE page controller. Clear by SW."]
    #[inline(always)]
    pub fn bit_pg_udn(&self) -> BitPgUdnR {
        BitPgUdnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Total pages included PTCL un-return pages are over the total reserved pages. Clear by SW."]
    #[inline(always)]
    pub fn bit_page_over(&self) -> BitPageOverR {
        BitPageOverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Total pages included PTCL un-return pages are under the total reserved pages minus the page threshold. Clear by SW."]
    #[inline(always)]
    pub fn bit_page_under(&self) -> BitPageUnderR {
        BitPageUnderR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXDMA reads NULL page. Clear by SW."]
    #[inline(always)]
    pub fn bit_free_pg_ff(&self) -> BitFreePgFfR {
        BitFreePgFfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXDMA Sync FIFO Overflow. Clear by SW."]
    #[inline(always)]
    pub fn bit_sff_ovf(&self) -> BitSffOvfR {
        BitSffOvfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXDMA Sync FIFO Underflow. Clear by SW."]
    #[inline(always)]
    pub fn bit_sff_udn(&self) -> BitSffUdnR {
        BitSffUdnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXOQT Overflow. Clear by SW."]
    #[inline(always)]
    pub fn bit_txoqt_ovf(&self) -> BitTxoqtOvfR {
        BitTxoqtOvfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXOQT Underflow. Clear by SW."]
    #[inline(always)]
    pub fn bit_txoqt_udn(&self) -> BitTxoqtUdnR {
        BitTxoqtUdnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx offset is unmatched. Clear by SW."]
    #[inline(always)]
    pub fn bit_tx_offs_unmatch(&self) -> BitTxOffsUnmatchR {
        BitTxOffsUnmatchR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 10) & 0x003f_ffff)
    }
    #[doc = "Bit 10 - Tx Endpoint is unmatched with the QSEL of descriptor. Clear by SW."]
    #[inline(always)]
    pub fn bit_ep_qsel_diff(&self) -> BitEpQselDiffR {
        BitEpQselDiffR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - An unknown QSEL of Tx descriptor is detected. Clear by SW."]
    #[inline(always)]
    pub fn bit_unknown_qsel(&self) -> BitUnknownQselR {
        BitUnknownQselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx descriptor checksum error. Clear by SW."]
    #[inline(always)]
    pub fn bit_dsc_chksum_fail(&self) -> BitDscChksumFailR {
        BitDscChksumFailR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Payload is over the packet length of Tx descriptor. Clear by SW."]
    #[inline(always)]
    pub fn bit_payload_ovf(&self) -> BitPayloadOvfR {
        BitPayloadOvfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Payload is under the packet length of Tx descriptor. Clear by SW."]
    #[inline(always)]
    pub fn bit_payload_udn(&self) -> BitPayloadUdnR {
        BitPayloadUdnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Payload checksum error"]
    #[inline(always)]
    pub fn bit_payload_chksum_err(&self) -> BitPayloadChksumErrR {
        BitPayloadChksumErrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - High Priority Queue_Tx OQT overflow"]
    #[inline(always)]
    pub fn bit_hi_oqt_ovf(&self) -> BitHiOqtOvfR {
        BitHiOqtOvfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High Priority Queue_Tx OQT underflow"]
    #[inline(always)]
    pub fn bit_hi_oqt_udn(&self) -> BitHiOqtUdnR {
        BitHiOqtUdnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - select txdma engine trigger stop signal for LA dump"]
    #[inline(always)]
    pub fn bit_tde_trig_stop_sel(&self) -> BitTdeTrigStopSelR {
        BitTdeTrigStopSelR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTxdmaStatusSpec> {
        RsvdW::new(self, 10)
    }
    #[doc = "Bits 18:19 - select txdma engine trigger stop signal for LA dump"]
    #[inline(always)]
    pub fn bit_tde_trig_stop_sel(&mut self) -> BitTdeTrigStopSelW<'_, RegTxdmaStatusSpec> {
        BitTdeTrigStopSelW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txdma_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txdma_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxdmaStatusSpec;
impl crate::RegisterSpec for RegTxdmaStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txdma_status::R`](R) reader structure"]
impl crate::Readable for RegTxdmaStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txdma_status::W`](W) writer structure"]
impl crate::Writable for RegTxdmaStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXDMA_STATUS to value 0"]
impl crate::Resettable for RegTxdmaStatusSpec {}
