#[doc = "Register `REG_FT1IMR` reader"]
pub type R = crate::R<RegFt1imrSpec>;
#[doc = "Register `REG_FT1IMR` writer"]
pub type W = crate::W<RegFt1imrSpec>;
#[doc = "Field `BIT_FS_PORT1_TX_NULL0_DONE_INT_EN` reader - PORT1 TX NULL0 packet ok interrupt"]
pub type BitFsPort1TxNull0DoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_TX_NULL0_DONE_INT_EN` writer - PORT1 TX NULL0 packet ok interrupt"]
pub type BitFsPort1TxNull0DoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_TX_NULL1_DONE_INT_EN` reader - PORT1 TX NULL1 packet ok interrupt"]
pub type BitFsPort1TxNull1DoneIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_TX_NULL1_DONE_INT_EN` writer - PORT1 TX NULL1 packet ok interrupt"]
pub type BitFsPort1TxNull1DoneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_EOSP_INT_EN` reader - EOSP_INT interrupt mask"]
pub type BitFsEospIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_EOSP_INT_EN` writer - EOSP_INT interrupt mask"]
pub type BitFsEospIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TRIGGER_PKT_INT_EN` reader - tx TRIGGER_PKT interrupt"]
pub type BitFsTriggerPktIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TRIGGER_PKT_INT_EN` writer - tx TRIGGER_PKT interrupt"]
pub type BitFsTriggerPktIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_UAPSDMD0_INT_EN` reader - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=0, UAPSD=0 2.) NonQoS Unicast data frame with MD=0"]
pub type BitFsRxUapsdmd0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_UAPSDMD0_INT_EN` writer - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=0, UAPSD=0 2.) NonQoS Unicast data frame with MD=0"]
pub type BitFsRxUapsdmd0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RX_UAPSDMD1_INT_EN` reader - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=1, UAPSD=0 2.) NonQoS Unicast data frame with MD=1"]
pub type BitFsRxUapsdmd1IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RX_UAPSDMD1_INT_EN` writer - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=1, UAPSD=0 2.) NonQoS Unicast data frame with MD=1"]
pub type BitFsRxUapsdmd1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_11` reader - RSVD"]
pub type Rsvd11R = crate::BitReader;
#[doc = "Field `RSVD_11` writer - RSVD"]
pub type Rsvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_10` reader - RSVD"]
pub type Rsvd10R = crate::BitReader;
#[doc = "Field `RSVD_10` writer - RSVD"]
pub type Rsvd10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_9` reader - RSVD"]
pub type Rsvd9R = crate::BitReader;
#[doc = "Field `RSVD_9` writer - RSVD"]
pub type Rsvd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_8` reader - RSVD"]
pub type Rsvd8R = crate::BitReader;
#[doc = "Field `RSVD_8` writer - RSVD"]
pub type Rsvd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXPRPT_CNT_FULL_INT_EN` reader - txrpt buffer full interrupt"]
pub type BitFsTxprptCntFullIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXPRPT_CNT_FULL_INT_EN` writer - txrpt buffer full interrupt"]
pub type BitFsTxprptCntFullIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_7` reader - RSVD"]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `RSVD_7` writer - RSVD"]
pub type Rsvd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_6` reader - RSVD"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `RSVD_6` writer - RSVD"]
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TX_NULL0_P0_INT_EN` reader - TX null 0 packet done interrupt"]
pub type BitFsTxNull0P0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TX_NULL0_P0_INT_EN` writer - TX null 0 packet done interrupt"]
pub type BitFsTxNull0P0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TX_NULL1_P0_INT_EN` reader - TX null 1 packet done interrupt"]
pub type BitFsTxNull1P0IntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TX_NULL1_P0_INT_EN` writer - TX null 1 packet done interrupt"]
pub type BitFsTxNull1P0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_5` reader - RSVD"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `RSVD_5` writer - RSVD"]
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RX_EOSP_OK_INT_EN` reader - PORT1 RX packet with EOSP =1 ok interrupt"]
pub type BitFsPort1RxEospOkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RX_EOSP_OK_INT_EN` writer - PORT1 RX packet with EOSP =1 ok interrupt"]
pub type BitFsPort1RxEospOkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_TRIPKT_OK_INT_EN` reader - PORT1 trigger packet assign to queue or release interrupt"]
pub type BitFsPort1TripktOkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_TRIPKT_OK_INT_EN` writer - PORT1 trigger packet assign to queue or release interrupt"]
pub type BitFsPort1TripktOkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RX_UCMD0_UAPSD0_OK_INT_EN` reader - PORT1 rx unicast packet ok interrupt with more data =0 & UAPSD =0"]
pub type BitFsPort1RxUcmd0Uapsd0OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RX_UCMD0_UAPSD0_OK_INT_EN` writer - PORT1 rx unicast packet ok interrupt with more data =0 & UAPSD =0"]
pub type BitFsPort1RxUcmd0Uapsd0OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_PORT1_RX_UCMD1_UAPSD0_OK_INT_EN` reader - PORT1 rx unicast packet ok interrupt with more data =1 & UAPSD =0"]
pub type BitFsPort1RxUcmd1Uapsd0OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_PORT1_RX_UCMD1_UAPSD0_OK_INT_EN` writer - PORT1 rx unicast packet ok interrupt with more data =1 & UAPSD =0"]
pub type BitFsPort1RxUcmd1Uapsd0OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_4` reader - RSVD"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_4` writer - RSVD"]
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_3` reader - RSVD"]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD_3` writer - RSVD"]
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXTM_INT_EN` reader - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
pub type BitFsTxtmIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXTM_INT_EN` writer - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
pub type BitFsTxtmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXTM_INT_EN` reader - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
pub type BitFsRxtmIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXTM_INT_EN` writer - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
pub type BitFsRxtmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXTMREQ_INT_EN` reader - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
pub type BitFsRxtmreqIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXTMREQ_INT_EN` writer - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
pub type BitFsRxtmreqIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_TXFTM_INT_EN` reader - If BIT_TXFTM_INT_EN =1. When HW transmit FTM and receive ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
pub type BitFsTxftmIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_TXFTM_INT_EN` writer - If BIT_TXFTM_INT_EN =1. When HW transmit FTM and receive ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
pub type BitFsTxftmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXFTM_INT_EN` reader - If BIT_RXFTM_INT_EN =1. When HW receive FTM and transmit ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
pub type BitFsRxftmIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXFTM_INT_EN` writer - If BIT_RXFTM_INT_EN =1. When HW receive FTM and transmit ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
pub type BitFsRxftmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_RXFTMREQ_INT_EN` reader - If BIT_RXFTMREQ_INT_EN =1. When HW receive FTMREQ packet, it will generate interrupt to firmware."]
pub type BitFsRxftmreqIntEnR = crate::BitReader;
#[doc = "Field `BIT_FS_RXFTMREQ_INT_EN` writer - If BIT_RXFTMREQ_INT_EN =1. When HW receive FTMREQ packet, it will generate interrupt to firmware."]
pub type BitFsRxftmreqIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PORT1 TX NULL0 packet ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tx_null0_done_int_en(&self) -> BitFsPort1TxNull0DoneIntEnR {
        BitFsPort1TxNull0DoneIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PORT1 TX NULL1 packet ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tx_null1_done_int_en(&self) -> BitFsPort1TxNull1DoneIntEnR {
        BitFsPort1TxNull1DoneIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOSP_INT interrupt mask"]
    #[inline(always)]
    pub fn bit_fs_eosp_int_en(&self) -> BitFsEospIntEnR {
        BitFsEospIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - tx TRIGGER_PKT interrupt"]
    #[inline(always)]
    pub fn bit_fs_trigger_pkt_int_en(&self) -> BitFsTriggerPktIntEnR {
        BitFsTriggerPktIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=0, UAPSD=0 2.) NonQoS Unicast data frame with MD=0"]
    #[inline(always)]
    pub fn bit_fs_rx_uapsdmd0_int_en(&self) -> BitFsRxUapsdmd0IntEnR {
        BitFsRxUapsdmd0IntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=1, UAPSD=0 2.) NonQoS Unicast data frame with MD=1"]
    #[inline(always)]
    pub fn bit_fs_rx_uapsdmd1_int_en(&self) -> BitFsRxUapsdmd1IntEnR {
        BitFsRxUapsdmd1IntEnR::new(((self.bits >> 5) & 1) != 0)
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
    pub fn bit_fs_txprpt_cnt_full_int_en(&self) -> BitFsTxprptCntFullIntEnR {
        BitFsTxprptCntFullIntEnR::new(((self.bits >> 10) & 1) != 0)
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
    pub fn bit_fs_tx_null0_p0_int_en(&self) -> BitFsTxNull0P0IntEnR {
        BitFsTxNull0P0IntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TX null 1 packet done interrupt"]
    #[inline(always)]
    pub fn bit_fs_tx_null1_p0_int_en(&self) -> BitFsTxNull1P0IntEnR {
        BitFsTxNull1P0IntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PORT1 RX packet with EOSP =1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_eosp_ok_int_en(&self) -> BitFsPort1RxEospOkIntEnR {
        BitFsPort1RxEospOkIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PORT1 trigger packet assign to queue or release interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tripkt_ok_int_en(&self) -> BitFsPort1TripktOkIntEnR {
        BitFsPort1TripktOkIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PORT1 rx unicast packet ok interrupt with more data =0 & UAPSD =0"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_ucmd0_uapsd0_ok_int_en(&self) -> BitFsPort1RxUcmd0Uapsd0OkIntEnR {
        BitFsPort1RxUcmd0Uapsd0OkIntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PORT1 rx unicast packet ok interrupt with more data =1 & UAPSD =0"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_ucmd1_uapsd0_ok_int_en(&self) -> BitFsPort1RxUcmd1Uapsd0OkIntEnR {
        BitFsPort1RxUcmd1Uapsd0OkIntEnR::new(((self.bits >> 19) & 1) != 0)
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
    pub fn bit_fs_txtm_int_en(&self) -> BitFsTxtmIntEnR {
        BitFsTxtmIntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxtm_int_en(&self) -> BitFsRxtmIntEnR {
        BitFsRxtmIntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxtmreq_int_en(&self) -> BitFsRxtmreqIntEnR {
        BitFsRxtmreqIntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If BIT_TXFTM_INT_EN =1. When HW transmit FTM and receive ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_txftm_int_en(&self) -> BitFsTxftmIntEnR {
        BitFsTxftmIntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - If BIT_RXFTM_INT_EN =1. When HW receive FTM and transmit ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxftm_int_en(&self) -> BitFsRxftmIntEnR {
        BitFsRxftmIntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If BIT_RXFTMREQ_INT_EN =1. When HW receive FTMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxftmreq_int_en(&self) -> BitFsRxftmreqIntEnR {
        BitFsRxftmreqIntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORT1 TX NULL0 packet ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tx_null0_done_int_en(
        &mut self,
    ) -> BitFsPort1TxNull0DoneIntEnW<'_, RegFt1imrSpec> {
        BitFsPort1TxNull0DoneIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - PORT1 TX NULL1 packet ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tx_null1_done_int_en(
        &mut self,
    ) -> BitFsPort1TxNull1DoneIntEnW<'_, RegFt1imrSpec> {
        BitFsPort1TxNull1DoneIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - EOSP_INT interrupt mask"]
    #[inline(always)]
    pub fn bit_fs_eosp_int_en(&mut self) -> BitFsEospIntEnW<'_, RegFt1imrSpec> {
        BitFsEospIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - tx TRIGGER_PKT interrupt"]
    #[inline(always)]
    pub fn bit_fs_trigger_pkt_int_en(&mut self) -> BitFsTriggerPktIntEnW<'_, RegFt1imrSpec> {
        BitFsTriggerPktIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=0, UAPSD=0 2.) NonQoS Unicast data frame with MD=0"]
    #[inline(always)]
    pub fn bit_fs_rx_uapsdmd0_int_en(&mut self) -> BitFsRxUapsdmd0IntEnW<'_, RegFt1imrSpec> {
        BitFsRxUapsdmd0IntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Setting this bit indicates Rx pkt is matched with one of below: 1.) QoS Unicast data frame with MD=1, UAPSD=0 2.) NonQoS Unicast data frame with MD=1"]
    #[inline(always)]
    pub fn bit_fs_rx_uapsdmd1_int_en(&mut self) -> BitFsRxUapsdmd1IntEnW<'_, RegFt1imrSpec> {
        BitFsRxUapsdmd1IntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - RSVD"]
    #[inline(always)]
    pub fn rsvd_11(&mut self) -> Rsvd11W<'_, RegFt1imrSpec> {
        Rsvd11W::new(self, 6)
    }
    #[doc = "Bit 7 - RSVD"]
    #[inline(always)]
    pub fn rsvd_10(&mut self) -> Rsvd10W<'_, RegFt1imrSpec> {
        Rsvd10W::new(self, 7)
    }
    #[doc = "Bit 8 - RSVD"]
    #[inline(always)]
    pub fn rsvd_9(&mut self) -> Rsvd9W<'_, RegFt1imrSpec> {
        Rsvd9W::new(self, 8)
    }
    #[doc = "Bit 9 - RSVD"]
    #[inline(always)]
    pub fn rsvd_8(&mut self) -> Rsvd8W<'_, RegFt1imrSpec> {
        Rsvd8W::new(self, 9)
    }
    #[doc = "Bit 10 - txrpt buffer full interrupt"]
    #[inline(always)]
    pub fn bit_fs_txprpt_cnt_full_int_en(&mut self) -> BitFsTxprptCntFullIntEnW<'_, RegFt1imrSpec> {
        BitFsTxprptCntFullIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - RSVD"]
    #[inline(always)]
    pub fn rsvd_7(&mut self) -> Rsvd7W<'_, RegFt1imrSpec> {
        Rsvd7W::new(self, 11)
    }
    #[doc = "Bit 12 - RSVD"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> Rsvd6W<'_, RegFt1imrSpec> {
        Rsvd6W::new(self, 12)
    }
    #[doc = "Bit 13 - TX null 0 packet done interrupt"]
    #[inline(always)]
    pub fn bit_fs_tx_null0_p0_int_en(&mut self) -> BitFsTxNull0P0IntEnW<'_, RegFt1imrSpec> {
        BitFsTxNull0P0IntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - TX null 1 packet done interrupt"]
    #[inline(always)]
    pub fn bit_fs_tx_null1_p0_int_en(&mut self) -> BitFsTxNull1P0IntEnW<'_, RegFt1imrSpec> {
        BitFsTxNull1P0IntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&mut self) -> Rsvd5W<'_, RegFt1imrSpec> {
        Rsvd5W::new(self, 15)
    }
    #[doc = "Bit 16 - PORT1 RX packet with EOSP =1 ok interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_eosp_ok_int_en(
        &mut self,
    ) -> BitFsPort1RxEospOkIntEnW<'_, RegFt1imrSpec> {
        BitFsPort1RxEospOkIntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - PORT1 trigger packet assign to queue or release interrupt"]
    #[inline(always)]
    pub fn bit_fs_port1_tripkt_ok_int_en(&mut self) -> BitFsPort1TripktOkIntEnW<'_, RegFt1imrSpec> {
        BitFsPort1TripktOkIntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - PORT1 rx unicast packet ok interrupt with more data =0 & UAPSD =0"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_ucmd0_uapsd0_ok_int_en(
        &mut self,
    ) -> BitFsPort1RxUcmd0Uapsd0OkIntEnW<'_, RegFt1imrSpec> {
        BitFsPort1RxUcmd0Uapsd0OkIntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - PORT1 rx unicast packet ok interrupt with more data =1 & UAPSD =0"]
    #[inline(always)]
    pub fn bit_fs_port1_rx_ucmd1_uapsd0_ok_int_en(
        &mut self,
    ) -> BitFsPort1RxUcmd1Uapsd0OkIntEnW<'_, RegFt1imrSpec> {
        BitFsPort1RxUcmd1Uapsd0OkIntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&mut self) -> Rsvd4W<'_, RegFt1imrSpec> {
        Rsvd4W::new(self, 20)
    }
    #[doc = "Bit 21 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> Rsvd3W<'_, RegFt1imrSpec> {
        Rsvd3W::new(self, 21)
    }
    #[doc = "Bit 22 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegFt1imrSpec> {
        Rsvd2W::new(self, 22)
    }
    #[doc = "Bit 23 - If BIT_TXTM_INT_EN =1. When HW transmit TM and receive ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_txtm_int_en(&mut self) -> BitFsTxtmIntEnW<'_, RegFt1imrSpec> {
        BitFsTxtmIntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - If BIT_RXTM_INT_EN =1. When HW receive TM and transmit ACK frame successfully. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxtm_int_en(&mut self) -> BitFsRxtmIntEnW<'_, RegFt1imrSpec> {
        BitFsRxtmIntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - If BIT_RXTMREQ_INT_EN =1. When HW receive TMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxtmreq_int_en(&mut self) -> BitFsRxtmreqIntEnW<'_, RegFt1imrSpec> {
        BitFsRxtmreqIntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - If BIT_TXFTM_INT_EN =1. When HW transmit FTM and receive ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_txftm_int_en(&mut self) -> BitFsTxftmIntEnW<'_, RegFt1imrSpec> {
        BitFsTxftmIntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - If BIT_RXFTM_INT_EN =1. When HW receive FTM and transmit ACK frame successfully, the BB generate FTM report. It will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxftm_int_en(&mut self) -> BitFsRxftmIntEnW<'_, RegFt1imrSpec> {
        BitFsRxftmIntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - If BIT_RXFTMREQ_INT_EN =1. When HW receive FTMREQ packet, it will generate interrupt to firmware."]
    #[inline(always)]
    pub fn bit_fs_rxftmreq_int_en(&mut self) -> BitFsRxftmreqIntEnW<'_, RegFt1imrSpec> {
        BitFsRxftmreqIntEnW::new(self, 28)
    }
    #[doc = "Bit 31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFt1imrSpec> {
        RsvdW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ft1imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ft1imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFt1imrSpec;
impl crate::RegisterSpec for RegFt1imrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ft1imr::R`](R) reader structure"]
impl crate::Readable for RegFt1imrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ft1imr::W`](W) writer structure"]
impl crate::Writable for RegFt1imrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FT1IMR to value 0"]
impl crate::Resettable for RegFt1imrSpec {}
