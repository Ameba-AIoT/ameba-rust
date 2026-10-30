#[doc = "Register `REG_SW_AMPDU_BURST_MODE_CTRL` reader"]
pub type R = crate::R<RegSwAmpduBurstModeCtrlSpec>;
#[doc = "Register `REG_SW_AMPDU_BURST_MODE_CTRL` writer"]
pub type W = crate::W<RegSwAmpduBurstModeCtrlSpec>;
#[doc = "Field `BIT_vo_en` reader - AMPDU burst mode enable for AC_VO queue"]
pub type BitVoEnR = crate::BitReader;
#[doc = "Field `BIT_vo_en` writer - AMPDU burst mode enable for AC_VO queue"]
pub type BitVoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_vi_en` reader - AMPDU burst mode enable for AC_VI queue"]
pub type BitViEnR = crate::BitReader;
#[doc = "Field `BIT_vi_en` writer - AMPDU burst mode enable for AC_VI queue"]
pub type BitViEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_be_en` reader - AMPDU burst mode enable for AC_BE queue"]
pub type BitBeEnR = crate::BitReader;
#[doc = "Field `BIT_be_en` writer - AMPDU burst mode enable for AC_BE queue"]
pub type BitBeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_bk_en` reader - AMPDU burst mode enable for AC_BK queue"]
pub type BitBkEnR = crate::BitReader;
#[doc = "Field `BIT_bk_en` writer - AMPDU burst mode enable for AC_BK queue"]
pub type BitBkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_Num_scl_en` reader - SW MPDU Burst mode select 00: each length field for 1 MPDU 01: each length field is the summation of 2 MPDUs 10: each length field is the summation of 3 MPDUs 11: each length field is the summation of 4 MPDUs Note: the total MPDU number is no more than 15."]
pub type BitNumSclEnR = crate::FieldReader;
#[doc = "Field `BIT_Num_scl_en` writer - SW MPDU Burst mode select 00: each length field for 1 MPDU 01: each length field is the summation of 2 MPDUs 10: each length field is the summation of 3 MPDUs 11: each length field is the summation of 4 MPDUs Note: the total MPDU number is no more than 15."]
pub type BitNumSclEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_pre_tx_cmd` reader - enable pre tx cmd to use aggr more packet"]
pub type BitPreTxCmdR = crate::BitReader;
#[doc = "Field `BIT_pre_tx_cmd` writer - enable pre tx cmd to use aggr more packet"]
pub type BitPreTxCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_wep_pretx_en` reader - Enable pretx for WEP/TKIP, to allow a slower security clock"]
pub type BitWepPretxEnR = crate::BitReader;
#[doc = "Field `BIT_wep_pretx_en` writer - Enable pretx for WEP/TKIP, to allow a slower security clock"]
pub type BitWepPretxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TOTAL_LEN_THRS` reader - aggregation length over threshold then fetch pad packet in early mode"]
pub type BitTotalLenThrsR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TOTAL_LEN_THRS` writer - aggregation length over threshold then fetch pad packet in early mode"]
pub type BitTotalLenThrsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_DMA_THIS_QUEUE` reader - scheduler enable"]
pub type BitDmaThisQueueR = crate::FieldReader;
#[doc = "Field `BIT_DMA_THIS_QUEUE` writer - scheduler enable"]
pub type BitDmaThisQueueW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_EVTQ_HEAD` reader - TXDMA will pull eventq Pkt in this address. NAN"]
pub type BitEvtqHeadR = crate::FieldReader;
#[doc = "Field `BIT_EVTQ_HEAD` writer - TXDMA will pull eventq Pkt in this address. NAN"]
pub type BitEvtqHeadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - AMPDU burst mode enable for AC_VO queue"]
    #[inline(always)]
    pub fn bit_vo_en(&self) -> BitVoEnR {
        BitVoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AMPDU burst mode enable for AC_VI queue"]
    #[inline(always)]
    pub fn bit_vi_en(&self) -> BitViEnR {
        BitViEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AMPDU burst mode enable for AC_BE queue"]
    #[inline(always)]
    pub fn bit_be_en(&self) -> BitBeEnR {
        BitBeEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AMPDU burst mode enable for AC_BK queue"]
    #[inline(always)]
    pub fn bit_bk_en(&self) -> BitBkEnR {
        BitBkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SW MPDU Burst mode select 00: each length field for 1 MPDU 01: each length field is the summation of 2 MPDUs 10: each length field is the summation of 3 MPDUs 11: each length field is the summation of 4 MPDUs Note: the total MPDU number is no more than 15."]
    #[inline(always)]
    pub fn bit_num_scl_en(&self) -> BitNumSclEnR {
        BitNumSclEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - enable pre tx cmd to use aggr more packet"]
    #[inline(always)]
    pub fn bit_pre_tx_cmd(&self) -> BitPreTxCmdR {
        BitPreTxCmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable pretx for WEP/TKIP, to allow a slower security clock"]
    #[inline(always)]
    pub fn bit_wep_pretx_en(&self) -> BitWepPretxEnR {
        BitWepPretxEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:19 - aggregation length over threshold then fetch pad packet in early mode"]
    #[inline(always)]
    pub fn bit_total_len_thrs(&self) -> BitTotalLenThrsR {
        BitTotalLenThrsR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:23 - scheduler enable"]
    #[inline(always)]
    pub fn bit_dma_this_queue(&self) -> BitDmaThisQueueR {
        BitDmaThisQueueR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - TXDMA will pull eventq Pkt in this address. NAN"]
    #[inline(always)]
    pub fn bit_evtq_head(&self) -> BitEvtqHeadR {
        BitEvtqHeadR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AMPDU burst mode enable for AC_VO queue"]
    #[inline(always)]
    pub fn bit_vo_en(&mut self) -> BitVoEnW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitVoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - AMPDU burst mode enable for AC_VI queue"]
    #[inline(always)]
    pub fn bit_vi_en(&mut self) -> BitViEnW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitViEnW::new(self, 1)
    }
    #[doc = "Bit 2 - AMPDU burst mode enable for AC_BE queue"]
    #[inline(always)]
    pub fn bit_be_en(&mut self) -> BitBeEnW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitBeEnW::new(self, 2)
    }
    #[doc = "Bit 3 - AMPDU burst mode enable for AC_BK queue"]
    #[inline(always)]
    pub fn bit_bk_en(&mut self) -> BitBkEnW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitBkEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - SW MPDU Burst mode select 00: each length field for 1 MPDU 01: each length field is the summation of 2 MPDUs 10: each length field is the summation of 3 MPDUs 11: each length field is the summation of 4 MPDUs Note: the total MPDU number is no more than 15."]
    #[inline(always)]
    pub fn bit_num_scl_en(&mut self) -> BitNumSclEnW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitNumSclEnW::new(self, 4)
    }
    #[doc = "Bit 6 - enable pre tx cmd to use aggr more packet"]
    #[inline(always)]
    pub fn bit_pre_tx_cmd(&mut self) -> BitPreTxCmdW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitPreTxCmdW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable pretx for WEP/TKIP, to allow a slower security clock"]
    #[inline(always)]
    pub fn bit_wep_pretx_en(&mut self) -> BitWepPretxEnW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitWepPretxEnW::new(self, 7)
    }
    #[doc = "Bits 8:19 - aggregation length over threshold then fetch pad packet in early mode"]
    #[inline(always)]
    pub fn bit_total_len_thrs(&mut self) -> BitTotalLenThrsW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitTotalLenThrsW::new(self, 8)
    }
    #[doc = "Bits 20:23 - scheduler enable"]
    #[inline(always)]
    pub fn bit_dma_this_queue(&mut self) -> BitDmaThisQueueW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitDmaThisQueueW::new(self, 20)
    }
    #[doc = "Bits 24:31 - TXDMA will pull eventq Pkt in this address. NAN"]
    #[inline(always)]
    pub fn bit_evtq_head(&mut self) -> BitEvtqHeadW<'_, RegSwAmpduBurstModeCtrlSpec> {
        BitEvtqHeadW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sw_ampdu_burst_mode_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sw_ampdu_burst_mode_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwAmpduBurstModeCtrlSpec;
impl crate::RegisterSpec for RegSwAmpduBurstModeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sw_ampdu_burst_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSwAmpduBurstModeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sw_ampdu_burst_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSwAmpduBurstModeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SW_AMPDU_BURST_MODE_CTRL to value 0xf604_00c0"]
impl crate::Resettable for RegSwAmpduBurstModeCtrlSpec {
    const RESET_VALUE: u32 = 0xf604_00c0;
}
