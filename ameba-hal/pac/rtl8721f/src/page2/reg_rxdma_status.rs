#[doc = "Register `REG_RXDMA_STATUS` reader"]
pub type R = crate::R<RegRxdmaStatusSpec>;
#[doc = "Register `REG_RXDMA_STATUS` writer"]
pub type W = crate::W<RegRxdmaStatusSpec>;
#[doc = "Field `BIT_RXPKT_OVF` reader - When RX Packet is more than 255 packets remaining in FF. Clear by SW."]
pub type BitRxpktOvfR = crate::BitReader;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RX_SFF_OVF` reader - RX Sync FIFO Overflow. Clear by SW."]
pub type BitRxSffOvfR = crate::BitReader;
#[doc = "Field `BIT_RX_SFF_UDN` reader - RX Sync FIFO Underflow. Clear by SW."]
pub type BitRxSffUdnR = crate::BitReader;
#[doc = "Field `BIT_RX_DATA_UDN` reader - FW Release Poll Error. Clear by SW."]
pub type BitRxDataUdnR = crate::BitReader;
#[doc = "Field `BIT_FW_POLL_ISSUE` reader - FW Release Poll Error. Clear by SW."]
pub type BitFwPollIssueR = crate::BitReader;
#[doc = "Field `BIT_AGG_CONFGI_ISSUE` reader - rxdma agg config error. Clr by SW"]
pub type BitAggConfgiIssueR = crate::BitReader;
#[doc = "Field `BIT_C2H_PKT_OVF` reader - CH2 pkt overflow. Clr by SW"]
pub type BitC2hPktOvfR = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - When RX Packet is more than 255 packets remaining in FF. Clear by SW."]
    #[inline(always)]
    pub fn bit_rxpkt_ovf(&self) -> BitRxpktOvfR {
        BitRxpktOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Sync FIFO Overflow. Clear by SW."]
    #[inline(always)]
    pub fn bit_rx_sff_ovf(&self) -> BitRxSffOvfR {
        BitRxSffOvfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Sync FIFO Underflow. Clear by SW."]
    #[inline(always)]
    pub fn bit_rx_sff_udn(&self) -> BitRxSffUdnR {
        BitRxSffUdnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FW Release Poll Error. Clear by SW."]
    #[inline(always)]
    pub fn bit_rx_data_udn(&self) -> BitRxDataUdnR {
        BitRxDataUdnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FW Release Poll Error. Clear by SW."]
    #[inline(always)]
    pub fn bit_fw_poll_issue(&self) -> BitFwPollIssueR {
        BitFwPollIssueR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - rxdma agg config error. Clr by SW"]
    #[inline(always)]
    pub fn bit_agg_confgi_issue(&self) -> BitAggConfgiIssueR {
        BitAggConfgiIssueR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH2 pkt overflow. Clr by SW"]
    #[inline(always)]
    pub fn bit_c2h_pkt_ovf(&self) -> BitC2hPktOvfR {
        BitC2hPktOvfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegRxdmaStatusSpec> {
        Rsvd2W::new(self, 1)
    }
    #[doc = "Bits 8:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegRxdmaStatusSpec> {
        RsvdW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxdma_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxdma_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxdmaStatusSpec;
impl crate::RegisterSpec for RegRxdmaStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxdma_status::R`](R) reader structure"]
impl crate::Readable for RegRxdmaStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxdma_status::W`](W) writer structure"]
impl crate::Writable for RegRxdmaStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXDMA_STATUS to value 0"]
impl crate::Resettable for RegRxdmaStatusSpec {}
