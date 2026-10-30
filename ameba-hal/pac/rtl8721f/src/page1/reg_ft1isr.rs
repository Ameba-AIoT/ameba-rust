#[doc = "Register `REG_FT1ISR` reader"]
pub type R = crate::R<RegFt1isrSpec>;
#[doc = "Field `BIT_FS_PORT1_TX_NULL0_DONE_INT` reader - PORT1 TX NULL0 packet ok interrupt"]
pub type BitFsPort1TxNull0DoneIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_TX_NULL1_DONE_INT` reader - PORT1 TX NULL1 packet ok interrupt"]
pub type BitFsPort1TxNull1DoneIntR = crate::BitReader;
#[doc = "Field `BIT_FS_EOSP_INT` reader - EOSP_INT interrupt mask"]
pub type BitFsEospIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TRIGGER_PKT_INT` reader - tx TRIGGER_PKT interrupt"]
pub type BitFsTriggerPktIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_UAPSDMD0_INT` reader - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=0, UAPSD=0 2.) NonQoS Unicast data frame with MD=0"]
pub type BitFsRxUapsdmd0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_UAPSDMD1_INT` reader - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=1, UAPSD=0 2.) NonQoS Unicast data frame with MD=1"]
pub type BitFsRxUapsdmd1IntR = crate::BitReader;
#[doc = "Field `RSVD_11` reader - RSVD"]
pub type Rsvd11R = crate::BitReader;
#[doc = "Field `RSVD_10` reader - RSVD"]
pub type Rsvd10R = crate::BitReader;
#[doc = "Field `RSVD_9` reader - RSVD"]
pub type Rsvd9R = crate::BitReader;
#[doc = "Field `RSVD_8` reader - RSVD"]
pub type Rsvd8R = crate::BitReader;
#[doc = "Field `BIT_FS_TXPRPT_CNT_FULL_INT` reader - txrpt buffer full interrupt"]
pub type BitFsTxprptCntFullIntR = crate::BitReader;
#[doc = "Field `RSVD_7` reader - RSVD"]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `RSVD_6` reader - RSVD"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `BIT_FS_TX_NULL0_P0_INT` reader - TX null 0 packet done interrupt"]
pub type BitFsTxNull0P0IntR = crate::BitReader;
#[doc = "Field `BIT_FS_TX_NULL1_P0_INT` reader - TX null 1 packet done interrupt"]
pub type BitFsTxNull1P0IntR = crate::BitReader;
#[doc = "Field `RSVD_5` reader - RSVD"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RX_EOSP_OK_INT` reader - PORT1 RX packet with EOSP =1 ok interrupt"]
pub type BitFsPort1RxEospOkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_TRIPKT_OK_INT` reader - PORT1 trigger packet assign to queue or release interrupt"]
pub type BitFsPort1TripktOkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RX_UCMD0_UAPSD0_OK_INT` reader - PORT1 rx unicast packet ok interrupt with more data =0 & UAPSD =0"]
pub type BitFsPort1RxUcmd0Uapsd0OkIntR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RX_UCMD1_UAPSD0_OK_INT` reader - PORT1 rx unicast packet ok interrupt with more data =1 & UAPSD =0"]
pub type BitFsPort1RxUcmd1Uapsd0OkIntR = crate::BitReader;
#[doc = "Field `RSVD_4` reader - RSVD"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_3` reader - RSVD"]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `BIT_FS_TXTM_INT` reader - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
pub type BitFsTxtmIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXTM_INT` reader - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
pub type BitFsRxtmIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXTMREQ_INT` reader - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
pub type BitFsRxtmreqIntR = crate::BitReader;
#[doc = "Field `BIT_FS_TXFTM_INT` reader - If BIT_TXFTM_INT_EN =1. When HW transmit FTM and receive ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
pub type BitFsTxftmIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXFTM_INT` reader - If BIT_RXFTM_INT_EN =1. When HW receive FTM and transmit ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
pub type BitFsRxftmIntR = crate::BitReader;
#[doc = "Field `BIT_FS_RXFTMREQ_INT` reader - If BIT_RXFTMREQ_INT_EN =1. When HW receive FTMREQ packet, it will generate interrupt to firmware."]
pub type BitFsRxftmreqIntR = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PORT1 TX NULL0 packet ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tx_null0_done_int(&self) -> BitFsPort1TxNull0DoneIntR {
        BitFsPort1TxNull0DoneIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PORT1 TX NULL1 packet ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tx_null1_done_int(&self) -> BitFsPort1TxNull1DoneIntR {
        BitFsPort1TxNull1DoneIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOSP_INT interrupt mask"]
    #[inline(always)]
    pub fn bit_fs_eosp_int(&self) -> BitFsEospIntR {
        BitFsEospIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tx TRIGGER_PKT interrupt"]
    #[inline(always)]
    pub fn bit_fs_trigger_pkt_int(&self) -> BitFsTriggerPktIntR {
        BitFsTriggerPktIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=0, UAPSD=0 2.) NonQoS Unicast data frame with MD=0"]
    #[inline(always)]
    pub fn bit_fs_rx_uapsdmd0_int(&self) -> BitFsRxUapsdmd0IntR {
        BitFsRxUapsdmd0IntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=1, UAPSD=0 2.) NonQoS Unicast data frame with MD=1"]
    #[inline(always)]
    pub fn bit_fs_rx_uapsdmd1_int(&self) -> BitFsRxUapsdmd1IntR {
        BitFsRxUapsdmd1IntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RSVD"]
    #[inline(always)]
    pub fn rsvd_11(&self) -> Rsvd11R {
        Rsvd11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RSVD"]
    #[inline(always)]
    pub fn rsvd_10(&self) -> Rsvd10R {
        Rsvd10R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RSVD"]
    #[inline(always)]
    pub fn rsvd_9(&self) -> Rsvd9R {
        Rsvd9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RSVD"]
    #[inline(always)]
    pub fn rsvd_8(&self) -> Rsvd8R {
        Rsvd8R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - txrpt buffer full interrupt"]
    #[inline(always)]
    pub fn bit_fs_txprpt_cnt_full_int(&self) -> BitFsTxprptCntFullIntR {
        BitFsTxprptCntFullIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RSVD"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RSVD"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX null 0 packet done interrupt"]
    #[inline(always)]
    pub fn bit_fs_tx_null0_p0_int(&self) -> BitFsTxNull0P0IntR {
        BitFsTxNull0P0IntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TX null 1 packet done interrupt"]
    #[inline(always)]
    pub fn bit_fs_tx_null1_p0_int(&self) -> BitFsTxNull1P0IntR {
        BitFsTxNull1P0IntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PORT1 RX packet with EOSP =1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_eosp_ok_int(&self) -> BitFsPort1RxEospOkIntR {
        BitFsPort1RxEospOkIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PORT1 trigger packet assign to queue or release interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tripkt_ok_int(&self) -> BitFsPort1TripktOkIntR {
        BitFsPort1TripktOkIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PORT1 rx unicast packet ok interrupt with more data =0 & UAPSD =0"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_ucmd0_uapsd0_ok_int(&self) -> BitFsPort1RxUcmd0Uapsd0OkIntR {
        BitFsPort1RxUcmd0Uapsd0OkIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PORT1 rx unicast packet ok interrupt with more data =1 & UAPSD =0"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_ucmd1_uapsd0_ok_int(&self) -> BitFsPort1RxUcmd1Uapsd0OkIntR {
        BitFsPort1RxUcmd1Uapsd0OkIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_txtm_int(&self) -> BitFsTxtmIntR {
        BitFsTxtmIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxtm_int(&self) -> BitFsRxtmIntR {
        BitFsRxtmIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxtmreq_int(&self) -> BitFsRxtmreqIntR {
        BitFsRxtmreqIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If BIT_TXFTM_INT_EN =1. When HW transmit FTM and receive ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_txftm_int(&self) -> BitFsTxftmIntR {
        BitFsTxftmIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - If BIT_RXFTM_INT_EN =1. When HW receive FTM and transmit ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxftm_int(&self) -> BitFsRxftmIntR {
        BitFsRxftmIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If BIT_RXFTMREQ_INT_EN =1. When HW receive FTMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxftmreq_int(&self) -> BitFsRxftmreqIntR {
        BitFsRxftmreqIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ft1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFt1isrSpec;
impl crate::RegisterSpec for RegFt1isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ft1isr::R`](R) reader structure"]
impl crate::Readable for RegFt1isrSpec {}
#[doc = "`reset()` method sets REG_FT1ISR to value 0"]
impl crate::Resettable for RegFt1isrSpec {}
