#[doc = "Register `REG_NAN_SETTING` reader"]
pub type R = crate::R<RegNanSettingSpec>;
#[doc = "Register `REG_NAN_SETTING` writer"]
pub type W = crate::W<RegNanSettingSpec>;
#[doc = "Field `BIT_P2_ENP2P_BCNQ_AREA` reader - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
pub type BitP2Enp2pBcnqAreaR = crate::BitReader;
#[doc = "Field `BIT_P2_ENP2P_BCNQ_AREA` writer - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
pub type BitP2Enp2pBcnqAreaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2_ENP2P_CTWINDOW` reader - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
pub type BitP2Enp2pCtwindowR = crate::BitReader;
#[doc = "Field `BIT_P2_ENP2P_CTWINDOW` writer - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
pub type BitP2Enp2pCtwindowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2_EN_BCN_RPT` reader - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
pub type BitP2EnBcnRptR = crate::BitReader;
#[doc = "Field `BIT_P2_EN_BCN_RPT` writer - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
pub type BitP2EnBcnRptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2_EN_BCN_FUNCTION` reader - When this bit is set, TSF and other beacon related functions are then enabled."]
pub type BitP2EnBcnFunctionR = crate::BitReader;
#[doc = "Field `BIT_P2_EN_BCN_FUNCTION` writer - When this bit is set, TSF and other beacon related functions are then enabled."]
pub type BitP2EnBcnFunctionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2_DIS_TSF_UDT` reader - When set, disable clint 2 TSF update when beacon frames or probe response frames are received."]
pub type BitP2DisTsfUdtR = crate::BitReader;
#[doc = "Field `BIT_P2_DIS_TSF_UDT` writer - When set, disable clint 2 TSF update when beacon frames or probe response frames are received."]
pub type BitP2DisTsfUdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2_TXBCN_RPT_ENABLE` reader - When this bit asserted, then TXBCN ERROR report is issued"]
pub type BitP2TxbcnRptEnableR = crate::BitReader;
#[doc = "Field `BIT_P2_TXBCN_RPT_ENABLE` writer - When this bit asserted, then TXBCN ERROR report is issued"]
pub type BitP2TxbcnRptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2_DIS_RX_BSSID_FIT` reader - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
pub type BitP2DisRxBssidFitR = crate::BitReader;
#[doc = "Field `BIT_P2_DIS_RX_BSSID_FIT` writer - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
pub type BitP2DisRxBssidFitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_P2_RXBCN_RPT_ENABLE` reader - When this bit asserted, RXBCNOK report is issued"]
pub type BitP2RxbcnRptEnableR = crate::BitReader;
#[doc = "Field `BIT_P2_RXBCN_RPT_ENABLE` writer - When this bit asserted, RXBCNOK report is issued"]
pub type BitP2RxbcnRptEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DRVERLYITV_P1` reader - Time to send interrupt to notify driver to change beacon content. Time unit is TU. It is used to wake up early at DriverEarlyTime. For port1"]
pub type BitDrverlyitvP1R = crate::FieldReader;
#[doc = "Field `BIT_DRVERLYITV_P1` writer - Time to send interrupt to notify driver to change beacon content. Time unit is TU. It is used to wake up early at DriverEarlyTime. For port1"]
pub type BitDrverlyitvP1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_FCSOK_CHK_BSSID` reader - 1:HW will check bssid when fcsok"]
pub type BitFcsokChkBssidR = crate::BitReader;
#[doc = "Field `BIT_FCSOK_CHK_BSSID` writer - 1:HW will check bssid when fcsok"]
pub type BitFcsokChkBssidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TBTT_PROHIBIT_HOLD_P2` reader - the predicted hold time after tbtt for beacon tx of port2, unit is 32us"]
pub type BitTbttProhibitHoldP2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_TBTT_PROHIBIT_HOLD_P2` writer - the predicted hold time after tbtt for beacon tx of port2, unit is 32us"]
pub type BitTbttProhibitHoldP2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_ENP2P_DW_AREA` reader - DW condition for P2P,for port2"]
pub type BitEnp2pDwAreaR = crate::BitReader;
#[doc = "Field `BIT_ENP2P_DW_AREA` writer - DW condition for P2P,for port2"]
pub type BitEnp2pDwAreaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_MULTI_BCN` reader - enable multiple beacons of port0~port2"]
pub type BitEnMultiBcnR = crate::BitReader;
#[doc = "Field `BIT_EN_MULTI_BCN` writer - enable multiple beacons of port0~port2"]
pub type BitEnMultiBcnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
    #[inline(always)]
    pub fn bit_p2_enp2p_bcnq_area(&self) -> BitP2Enp2pBcnqAreaR {
        BitP2Enp2pBcnqAreaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
    #[inline(always)]
    pub fn bit_p2_enp2p_ctwindow(&self) -> BitP2Enp2pCtwindowR {
        BitP2Enp2pCtwindowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
    #[inline(always)]
    pub fn bit_p2_en_bcn_rpt(&self) -> BitP2EnBcnRptR {
        BitP2EnBcnRptR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set, TSF and other beacon related functions are then enabled."]
    #[inline(always)]
    pub fn bit_p2_en_bcn_function(&self) -> BitP2EnBcnFunctionR {
        BitP2EnBcnFunctionR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set, disable clint 2 TSF update when beacon frames or probe response frames are received."]
    #[inline(always)]
    pub fn bit_p2_dis_tsf_udt(&self) -> BitP2DisTsfUdtR {
        BitP2DisTsfUdtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When this bit asserted, then TXBCN ERROR report is issued"]
    #[inline(always)]
    pub fn bit_p2_txbcn_rpt_enable(&self) -> BitP2TxbcnRptEnableR {
        BitP2TxbcnRptEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
    #[inline(always)]
    pub fn bit_p2_dis_rx_bssid_fit(&self) -> BitP2DisRxBssidFitR {
        BitP2DisRxBssidFitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit asserted, RXBCNOK report is issued"]
    #[inline(always)]
    pub fn bit_p2_rxbcn_rpt_enable(&self) -> BitP2RxbcnRptEnableR {
        BitP2RxbcnRptEnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Time to send interrupt to notify driver to change beacon content. Time unit is TU. It is used to wake up early at DriverEarlyTime. For port1"]
    #[inline(always)]
    pub fn bit_drverlyitv_p1(&self) -> BitDrverlyitvP1R {
        BitDrverlyitvP1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1:HW will check bssid when fcsok"]
    #[inline(always)]
    pub fn bit_fcsok_chk_bssid(&self) -> BitFcsokChkBssidR {
        BitFcsokChkBssidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:29 - the predicted hold time after tbtt for beacon tx of port2, unit is 32us"]
    #[inline(always)]
    pub fn bit_tbtt_prohibit_hold_p2(&self) -> BitTbttProhibitHoldP2R {
        BitTbttProhibitHoldP2R::new(((self.bits >> 18) & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - DW condition for P2P,for port2"]
    #[inline(always)]
    pub fn bit_enp2p_dw_area(&self) -> BitEnp2pDwAreaR {
        BitEnp2pDwAreaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable multiple beacons of port0~port2"]
    #[inline(always)]
    pub fn bit_en_multi_bcn(&self) -> BitEnMultiBcnR {
        BitEnMultiBcnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable P2P BCN area on function. 0 : Disable P2P BCN area on function."]
    #[inline(always)]
    pub fn bit_p2_enp2p_bcnq_area(&mut self) -> BitP2Enp2pBcnqAreaW<'_, RegNanSettingSpec> {
        BitP2Enp2pBcnqAreaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable P2P CTN WINDOWS function. 0: Disable P2P CTN WINDOWS function."]
    #[inline(always)]
    pub fn bit_p2_enp2p_ctwindow(&mut self) -> BitP2Enp2pCtwindowW<'_, RegNanSettingSpec> {
        BitP2Enp2pCtwindowW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit asserted, txbcnok_int and txbcnerr_int will valid."]
    #[inline(always)]
    pub fn bit_p2_en_bcn_rpt(&mut self) -> BitP2EnBcnRptW<'_, RegNanSettingSpec> {
        BitP2EnBcnRptW::new(self, 2)
    }
    #[doc = "Bit 3 - When this bit is set, TSF and other beacon related functions are then enabled."]
    #[inline(always)]
    pub fn bit_p2_en_bcn_function(&mut self) -> BitP2EnBcnFunctionW<'_, RegNanSettingSpec> {
        BitP2EnBcnFunctionW::new(self, 3)
    }
    #[doc = "Bit 4 - When set, disable clint 2 TSF update when beacon frames or probe response frames are received."]
    #[inline(always)]
    pub fn bit_p2_dis_tsf_udt(&mut self) -> BitP2DisTsfUdtW<'_, RegNanSettingSpec> {
        BitP2DisTsfUdtW::new(self, 4)
    }
    #[doc = "Bit 5 - When this bit asserted, then TXBCN ERROR report is issued"]
    #[inline(always)]
    pub fn bit_p2_txbcn_rpt_enable(&mut self) -> BitP2TxbcnRptEnableW<'_, RegNanSettingSpec> {
        BitP2TxbcnRptEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - 0:HW will handle rx_beacon 1:HW will ignore rx_beacon_pk"]
    #[inline(always)]
    pub fn bit_p2_dis_rx_bssid_fit(&mut self) -> BitP2DisRxBssidFitW<'_, RegNanSettingSpec> {
        BitP2DisRxBssidFitW::new(self, 6)
    }
    #[doc = "Bit 7 - When this bit asserted, RXBCNOK report is issued"]
    #[inline(always)]
    pub fn bit_p2_rxbcn_rpt_enable(&mut self) -> BitP2RxbcnRptEnableW<'_, RegNanSettingSpec> {
        BitP2RxbcnRptEnableW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Time to send interrupt to notify driver to change beacon content. Time unit is TU. It is used to wake up early at DriverEarlyTime. For port1"]
    #[inline(always)]
    pub fn bit_drverlyitv_p1(&mut self) -> BitDrverlyitvP1W<'_, RegNanSettingSpec> {
        BitDrverlyitvP1W::new(self, 8)
    }
    #[doc = "Bit 16 - 1:HW will check bssid when fcsok"]
    #[inline(always)]
    pub fn bit_fcsok_chk_bssid(&mut self) -> BitFcsokChkBssidW<'_, RegNanSettingSpec> {
        BitFcsokChkBssidW::new(self, 16)
    }
    #[doc = "Bits 18:29 - the predicted hold time after tbtt for beacon tx of port2, unit is 32us"]
    #[inline(always)]
    pub fn bit_tbtt_prohibit_hold_p2(&mut self) -> BitTbttProhibitHoldP2W<'_, RegNanSettingSpec> {
        BitTbttProhibitHoldP2W::new(self, 18)
    }
    #[doc = "Bit 30 - DW condition for P2P,for port2"]
    #[inline(always)]
    pub fn bit_enp2p_dw_area(&mut self) -> BitEnp2pDwAreaW<'_, RegNanSettingSpec> {
        BitEnp2pDwAreaW::new(self, 30)
    }
    #[doc = "Bit 31 - enable multiple beacons of port0~port2"]
    #[inline(always)]
    pub fn bit_en_multi_bcn(&mut self) -> BitEnMultiBcnW<'_, RegNanSettingSpec> {
        BitEnMultiBcnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanSettingSpec;
impl crate::RegisterSpec for RegNanSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_setting::R`](R) reader structure"]
impl crate::Readable for RegNanSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_setting::W`](W) writer structure"]
impl crate::Writable for RegNanSettingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_SETTING to value 0x8401_0514"]
impl crate::Resettable for RegNanSettingSpec {
    const RESET_VALUE: u32 = 0x8401_0514;
}
