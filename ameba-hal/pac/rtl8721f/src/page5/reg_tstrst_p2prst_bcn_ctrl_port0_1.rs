#[doc = "Register `REG_TSTRST_P2PRST_BCN_CTRL_PORT0_1` reader"]
pub type R = crate::R<RegTstrstP2prstBcnCtrlPort0_1Spec>;
#[doc = "Register `REG_TSTRST_P2PRST_BCN_CTRL_PORT0_1` writer"]
pub type W = crate::W<RegTstrstP2prstBcnCtrlPort0_1Spec>;
#[doc = "Field `BIT_P0_EN_P2P_BCNQ_AREA` reader - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
pub type BitP0EnP2pBcnqAreaR = crate::BitReader;
#[doc = "Field `BIT_P0_EN_P2P_BCNQ_AREA` writer - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
pub type BitP0EnP2pBcnqAreaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P0_EN_P2P_CTWINDOW` reader - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
pub type BitP0EnP2pCtwindowR = crate::BitReader;
#[doc = "Field `BIT_P0_EN_P2P_CTWINDOW` writer - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
pub type BitP0EnP2pCtwindowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P0_EN_BCN_RPT` reader - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
pub type BitP0EnBcnRptR = crate::BitReader;
#[doc = "Field `BIT_P0_EN_BCN_RPT` writer - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
pub type BitP0EnBcnRptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P0_EN_BCN_FUNCTION` reader - When this bit is set, TSF and other beacon related functions are then enabled. Enable switch for 0x610(MACID)"]
pub type BitP0EnBcnFunctionR = crate::BitReader;
#[doc = "Field `BIT_P0_EN_BCN_FUNCTION` writer - When this bit is set, TSF and other beacon related functions are then enabled. Enable switch for 0x610(MACID)"]
pub type BitP0EnBcnFunctionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P0_DIS_TSF_UDT` reader - When set, disable port 0 TSF update when beacon frames or probe response frames are received."]
pub type BitP0DisTsfUdtR = crate::BitReader;
#[doc = "Field `BIT_P0_DIS_TSF_UDT` writer - When set, disable port 0 TSF update when beacon frames or probe response frames are received."]
pub type BitP0DisTsfUdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P0_TXBCN_RPT_ENABLE` reader - When this bit and BIT_P0_EN_BCN_RPT are set, then TXBCN ERROR report is issued"]
pub type BitP0TxbcnRptEnableR = crate::BitReader;
#[doc = "Field `BIT_P0_TXBCN_RPT_ENABLE` writer - When this bit and BIT_P0_EN_BCN_RPT are set, then TXBCN ERROR report is issued"]
pub type BitP0TxbcnRptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P0_DIS_RX_BSSID_FIT` reader - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
pub type BitP0DisRxBssidFitR = crate::BitReader;
#[doc = "Field `BIT_P0_DIS_RX_BSSID_FIT` writer - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
pub type BitP0DisRxBssidFitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P0_RXBCN_RPT_ENABLE` reader - When this bit asserted, RXBCNOK report is issued"]
pub type BitP0RxbcnRptEnableR = crate::BitReader;
#[doc = "Field `BIT_P0_RXBCN_RPT_ENABLE` writer - When this bit asserted, RXBCNOK report is issued"]
pub type BitP0RxbcnRptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P1_ENP2P_BCNQ_AREA` reader - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
pub type BitP1Enp2pBcnqAreaR = crate::BitReader;
#[doc = "Field `BIT_P1_ENP2P_BCNQ_AREA` writer - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
pub type BitP1Enp2pBcnqAreaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P1_ENP2P_CTWINDOW` reader - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
pub type BitP1Enp2pCtwindowR = crate::BitReader;
#[doc = "Field `BIT_P1_ENP2P_CTWINDOW` writer - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
pub type BitP1Enp2pCtwindowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P1_EN_BCN_RPT` reader - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
pub type BitP1EnBcnRptR = crate::BitReader;
#[doc = "Field `BIT_P1_EN_BCN_RPT` writer - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
pub type BitP1EnBcnRptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P1_EN_BCN_FUNCTION` reader - When this bit is set, TSF and other beacon related functions are then enabled. Enable switch for 0x700(MACID1)"]
pub type BitP1EnBcnFunctionR = crate::BitReader;
#[doc = "Field `BIT_P1_EN_BCN_FUNCTION` writer - When this bit is set, TSF and other beacon related functions are then enabled. Enable switch for 0x700(MACID1)"]
pub type BitP1EnBcnFunctionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P1_DIS_TSF_UDT` reader - When set, disable clint 0 TSF update when beacon frames or probe response frames are received."]
pub type BitP1DisTsfUdtR = crate::BitReader;
#[doc = "Field `BIT_P1_DIS_TSF_UDT` writer - When set, disable clint 0 TSF update when beacon frames or probe response frames are received."]
pub type BitP1DisTsfUdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P1_TXBCN_RPT_ENABLE` reader - When this bit and BIT_P1_EN_BCN_RPT are set, then TXBCN ERROR report is issued"]
pub type BitP1TxbcnRptEnableR = crate::BitReader;
#[doc = "Field `BIT_P1_TXBCN_RPT_ENABLE` writer - When this bit and BIT_P1_EN_BCN_RPT are set, then TXBCN ERROR report is issued"]
pub type BitP1TxbcnRptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P1_DIS_RX_BSSID_FIT` reader - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
pub type BitP1DisRxBssidFitR = crate::BitReader;
#[doc = "Field `BIT_P1_DIS_RX_BSSID_FIT` writer - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
pub type BitP1DisRxBssidFitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P1_RXBCN_RPT_ENABLE` reader - When this bit asserted, RXBCNOK report is issued"]
pub type BitP1RxbcnRptEnableR = crate::BitReader;
#[doc = "Field `BIT_P1_RXBCN_RPT_ENABLE` writer - When this bit asserted, RXBCNOK report is issued"]
pub type BitP1RxbcnRptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBID_BCN_NUM` reader - The number of virtual interface number excluding the root number."]
pub type BitMbidBcnNumR = crate::FieldReader;
#[doc = "Field `BIT_MBID_BCN_NUM` writer - The number of virtual interface number excluding the root number."]
pub type BitMbidBcnNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_EN_PRE_DL_BEACON` reader - If set 1, hw will enable pre_download beacon"]
pub type BitEnPreDlBeaconR = crate::BitReader;
#[doc = "Field `BIT_EN_PRE_DL_BEACON` writer - If set 1, hw will enable pre_download beacon"]
pub type BitEnPreDlBeaconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_PWR_RST2` writer - rst p2p circuit NOA 2, noa_unit1"]
pub type BitP2pPwrRst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_PWR_RST3` writer - rst p2p circuit NOA 3, noa_unit1"]
pub type BitP2pPwrRst3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_PWR_RST4` writer - rst p2p circuit NOA 4, noa_unit2"]
pub type BitP2pPwrRst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_PWR_RST5` writer - rst p2p circuit NOA 5, noa_unit2"]
pub type BitP2pPwrRst5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TSFTR_RST` writer - write this bit , bcn0 tsft will auto reset"]
pub type BitTsftrRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TSFTR1_RST` writer - write this bit , bcn1 tsft will auto reset"]
pub type BitTsftr1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TSFTR_SYNC_EN` writer - write this bit, bcn0 will sync to bcn1 with TSFTF_SYN_OFFSET"]
pub type BitTsftrSyncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TSFTR1_SYNC_EN` writer - write this bit, bcn1 will sync to bcn0 with TSFTF_SYN_OFFSET"]
pub type BitTsftr1SyncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_PWR_RST0` writer - rst p2p circuit NOA 0, noa_unit0"]
pub type BitP2pPwrRst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_SCHEDULER_RST` writer - Write this bit reset scheduler, ioreg_txfail"]
pub type BitSchedulerRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2P_PWR_RST1` writer - rst p2p circuit NOA 1, noa_unit0"]
pub type BitP2pPwrRst1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
    #[inline(always)]
    pub fn bit_p0_en_p2p_bcnq_area(&self) -> BitP0EnP2pBcnqAreaR {
        BitP0EnP2pBcnqAreaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
    #[inline(always)]
    pub fn bit_p0_en_p2p_ctwindow(&self) -> BitP0EnP2pCtwindowR {
        BitP0EnP2pCtwindowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
    #[inline(always)]
    pub fn bit_p0_en_bcn_rpt(&self) -> BitP0EnBcnRptR {
        BitP0EnBcnRptR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set, TSF and other beacon related functions are then enabled. Enable switch for 0x610(MACID)"]
    #[inline(always)]
    pub fn bit_p0_en_bcn_function(&self) -> BitP0EnBcnFunctionR {
        BitP0EnBcnFunctionR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set, disable port 0 TSF update when beacon frames or probe response frames are received."]
    #[inline(always)]
    pub fn bit_p0_dis_tsf_udt(&self) -> BitP0DisTsfUdtR {
        BitP0DisTsfUdtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When this bit and BIT_P0_EN_BCN_RPT are set, then TXBCN ERROR report is issued"]
    #[inline(always)]
    pub fn bit_p0_txbcn_rpt_enable(&self) -> BitP0TxbcnRptEnableR {
        BitP0TxbcnRptEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
    #[inline(always)]
    pub fn bit_p0_dis_rx_bssid_fit(&self) -> BitP0DisRxBssidFitR {
        BitP0DisRxBssidFitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit asserted, RXBCNOK report is issued"]
    #[inline(always)]
    pub fn bit_p0_rxbcn_rpt_enable(&self) -> BitP0RxbcnRptEnableR {
        BitP0RxbcnRptEnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
    #[inline(always)]
    pub fn bit_p1_enp2p_bcnq_area(&self) -> BitP1Enp2pBcnqAreaR {
        BitP1Enp2pBcnqAreaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
    #[inline(always)]
    pub fn bit_p1_enp2p_ctwindow(&self) -> BitP1Enp2pCtwindowR {
        BitP1Enp2pCtwindowR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
    #[inline(always)]
    pub fn bit_p1_en_bcn_rpt(&self) -> BitP1EnBcnRptR {
        BitP1EnBcnRptR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When this bit is set, TSF and other beacon related functions are then enabled. Enable switch for 0x700(MACID1)"]
    #[inline(always)]
    pub fn bit_p1_en_bcn_function(&self) -> BitP1EnBcnFunctionR {
        BitP1EnBcnFunctionR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When set, disable clint 0 TSF update when beacon frames or probe response frames are received."]
    #[inline(always)]
    pub fn bit_p1_dis_tsf_udt(&self) -> BitP1DisTsfUdtR {
        BitP1DisTsfUdtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When this bit and BIT_P1_EN_BCN_RPT are set, then TXBCN ERROR report is issued"]
    #[inline(always)]
    pub fn bit_p1_txbcn_rpt_enable(&self) -> BitP1TxbcnRptEnableR {
        BitP1TxbcnRptEnableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
    #[inline(always)]
    pub fn bit_p1_dis_rx_bssid_fit(&self) -> BitP1DisRxBssidFitR {
        BitP1DisRxBssidFitR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When this bit asserted, RXBCNOK report is issued"]
    #[inline(always)]
    pub fn bit_p1_rxbcn_rpt_enable(&self) -> BitP1RxbcnRptEnableR {
        BitP1RxbcnRptEnableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - The number of virtual interface number excluding the root number."]
    #[inline(always)]
    pub fn bit_mbid_bcn_num(&self) -> BitMbidBcnNumR {
        BitMbidBcnNumR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - If set 1, hw will enable pre_download beacon"]
    #[inline(always)]
    pub fn bit_en_pre_dl_beacon(&self) -> BitEnPreDlBeaconR {
        BitEnPreDlBeaconR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
    #[inline(always)]
    pub fn bit_p0_en_p2p_bcnq_area(
        &mut self,
    ) -> BitP0EnP2pBcnqAreaW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP0EnP2pBcnqAreaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
    #[inline(always)]
    pub fn bit_p0_en_p2p_ctwindow(
        &mut self,
    ) -> BitP0EnP2pCtwindowW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP0EnP2pCtwindowW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
    #[inline(always)]
    pub fn bit_p0_en_bcn_rpt(&mut self) -> BitP0EnBcnRptW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP0EnBcnRptW::new(self, 2)
    }
    #[doc = "Bit 3 - When this bit is set, TSF and other beacon related functions are then enabled. Enable switch for 0x610(MACID)"]
    #[inline(always)]
    pub fn bit_p0_en_bcn_function(
        &mut self,
    ) -> BitP0EnBcnFunctionW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP0EnBcnFunctionW::new(self, 3)
    }
    #[doc = "Bit 4 - When set, disable port 0 TSF update when beacon frames or probe response frames are received."]
    #[inline(always)]
    pub fn bit_p0_dis_tsf_udt(&mut self) -> BitP0DisTsfUdtW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP0DisTsfUdtW::new(self, 4)
    }
    #[doc = "Bit 5 - When this bit and BIT_P0_EN_BCN_RPT are set, then TXBCN ERROR report is issued"]
    #[inline(always)]
    pub fn bit_p0_txbcn_rpt_enable(
        &mut self,
    ) -> BitP0TxbcnRptEnableW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP0TxbcnRptEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
    #[inline(always)]
    pub fn bit_p0_dis_rx_bssid_fit(
        &mut self,
    ) -> BitP0DisRxBssidFitW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP0DisRxBssidFitW::new(self, 6)
    }
    #[doc = "Bit 7 - When this bit asserted, RXBCNOK report is issued"]
    #[inline(always)]
    pub fn bit_p0_rxbcn_rpt_enable(
        &mut self,
    ) -> BitP0RxbcnRptEnableW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP0RxbcnRptEnableW::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
    #[inline(always)]
    pub fn bit_p1_enp2p_bcnq_area(
        &mut self,
    ) -> BitP1Enp2pBcnqAreaW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP1Enp2pBcnqAreaW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
    #[inline(always)]
    pub fn bit_p1_enp2p_ctwindow(
        &mut self,
    ) -> BitP1Enp2pCtwindowW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP1Enp2pCtwindowW::new(self, 9)
    }
    #[doc = "Bit 10 - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
    #[inline(always)]
    pub fn bit_p1_en_bcn_rpt(&mut self) -> BitP1EnBcnRptW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP1EnBcnRptW::new(self, 10)
    }
    #[doc = "Bit 11 - When this bit is set, TSF and other beacon related functions are then enabled. Enable switch for 0x700(MACID1)"]
    #[inline(always)]
    pub fn bit_p1_en_bcn_function(
        &mut self,
    ) -> BitP1EnBcnFunctionW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP1EnBcnFunctionW::new(self, 11)
    }
    #[doc = "Bit 12 - When set, disable clint 0 TSF update when beacon frames or probe response frames are received."]
    #[inline(always)]
    pub fn bit_p1_dis_tsf_udt(&mut self) -> BitP1DisTsfUdtW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP1DisTsfUdtW::new(self, 12)
    }
    #[doc = "Bit 13 - When this bit and BIT_P1_EN_BCN_RPT are set, then TXBCN ERROR report is issued"]
    #[inline(always)]
    pub fn bit_p1_txbcn_rpt_enable(
        &mut self,
    ) -> BitP1TxbcnRptEnableW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP1TxbcnRptEnableW::new(self, 13)
    }
    #[doc = "Bit 14 - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
    #[inline(always)]
    pub fn bit_p1_dis_rx_bssid_fit(
        &mut self,
    ) -> BitP1DisRxBssidFitW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP1DisRxBssidFitW::new(self, 14)
    }
    #[doc = "Bit 15 - When this bit asserted, RXBCNOK report is issued"]
    #[inline(always)]
    pub fn bit_p1_rxbcn_rpt_enable(
        &mut self,
    ) -> BitP1RxbcnRptEnableW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP1RxbcnRptEnableW::new(self, 15)
    }
    #[doc = "Bits 16:18 - The number of virtual interface number excluding the root number."]
    #[inline(always)]
    pub fn bit_mbid_bcn_num(&mut self) -> BitMbidBcnNumW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitMbidBcnNumW::new(self, 16)
    }
    #[doc = "Bit 19 - If set 1, hw will enable pre_download beacon"]
    #[inline(always)]
    pub fn bit_en_pre_dl_beacon(
        &mut self,
    ) -> BitEnPreDlBeaconW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitEnPreDlBeaconW::new(self, 19)
    }
    #[doc = "Bit 20 - rst p2p circuit NOA 2, noa_unit1"]
    #[inline(always)]
    pub fn bit_p2p_pwr_rst2(&mut self) -> BitP2pPwrRst2W<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP2pPwrRst2W::new(self, 20)
    }
    #[doc = "Bit 21 - rst p2p circuit NOA 3, noa_unit1"]
    #[inline(always)]
    pub fn bit_p2p_pwr_rst3(&mut self) -> BitP2pPwrRst3W<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP2pPwrRst3W::new(self, 21)
    }
    #[doc = "Bit 22 - rst p2p circuit NOA 4, noa_unit2"]
    #[inline(always)]
    pub fn bit_p2p_pwr_rst4(&mut self) -> BitP2pPwrRst4W<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP2pPwrRst4W::new(self, 22)
    }
    #[doc = "Bit 23 - rst p2p circuit NOA 5, noa_unit2"]
    #[inline(always)]
    pub fn bit_p2p_pwr_rst5(&mut self) -> BitP2pPwrRst5W<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP2pPwrRst5W::new(self, 23)
    }
    #[doc = "Bit 24 - write this bit , bcn0 tsft will auto reset"]
    #[inline(always)]
    pub fn bit_tsftr_rst(&mut self) -> BitTsftrRstW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitTsftrRstW::new(self, 24)
    }
    #[doc = "Bit 25 - write this bit , bcn1 tsft will auto reset"]
    #[inline(always)]
    pub fn bit_tsftr1_rst(&mut self) -> BitTsftr1RstW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitTsftr1RstW::new(self, 25)
    }
    #[doc = "Bit 26 - write this bit, bcn0 will sync to bcn1 with TSFTF_SYN_OFFSET"]
    #[inline(always)]
    pub fn bit_tsftr_sync_en(&mut self) -> BitTsftrSyncEnW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitTsftrSyncEnW::new(self, 26)
    }
    #[doc = "Bit 27 - write this bit, bcn1 will sync to bcn0 with TSFTF_SYN_OFFSET"]
    #[inline(always)]
    pub fn bit_tsftr1_sync_en(
        &mut self,
    ) -> BitTsftr1SyncEnW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitTsftr1SyncEnW::new(self, 27)
    }
    #[doc = "Bit 28 - rst p2p circuit NOA 0, noa_unit0"]
    #[inline(always)]
    pub fn bit_p2p_pwr_rst0(&mut self) -> BitP2pPwrRst0W<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP2pPwrRst0W::new(self, 28)
    }
    #[doc = "Bit 29 - Write this bit reset scheduler, ioreg_txfail"]
    #[inline(always)]
    pub fn bit_scheduler_rst(&mut self) -> BitSchedulerRstW<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitSchedulerRstW::new(self, 29)
    }
    #[doc = "Bit 30 - rst p2p circuit NOA 1, noa_unit0"]
    #[inline(always)]
    pub fn bit_p2p_pwr_rst1(&mut self) -> BitP2pPwrRst1W<'_, RegTstrstP2prstBcnCtrlPort0_1Spec> {
        BitP2pPwrRst1W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tstrst_p2prst_bcn_ctrl_port0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tstrst_p2prst_bcn_ctrl_port0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTstrstP2prstBcnCtrlPort0_1Spec;
impl crate::RegisterSpec for RegTstrstP2prstBcnCtrlPort0_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tstrst_p2prst_bcn_ctrl_port0_1::R`](R) reader structure"]
impl crate::Readable for RegTstrstP2prstBcnCtrlPort0_1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tstrst_p2prst_bcn_ctrl_port0_1::W`](W) writer structure"]
impl crate::Writable for RegTstrstP2prstBcnCtrlPort0_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TSTRST_P2PRST_BCN_CTRL_PORT0_1 to value 0x1414"]
impl crate::Resettable for RegTstrstP2prstBcnCtrlPort0_1Spec {
    const RESET_VALUE: u32 = 0x1414;
}
