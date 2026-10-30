#[doc = "Register `REG_NOA0_PKT_LIFETIME_CTRL` reader"]
pub type R = crate::R<RegNoa0PktLifetimeCtrlSpec>;
#[doc = "Register `REG_NOA0_PKT_LIFETIME_CTRL` writer"]
pub type W = crate::W<RegNoa0PktLifetimeCtrlSpec>;
#[doc = "Field `BIT_EDCCA_THRS` reader - EDCCA threshold value, edcca over this threshold will cause bkf_rst Bit\\[7\\]: when PHY_EDCCA=1, block TX when the timer times out Bit\\[6:0\\]: timer (unit:32us)"]
pub type BitEdccaThrsR = crate::FieldReader;
#[doc = "Field `BIT_EDCCA_THRS` writer - EDCCA threshold value, edcca over this threshold will cause bkf_rst Bit\\[7\\]: when PHY_EDCCA=1, block TX when the timer times out Bit\\[6:0\\]: timer (unit:32us)"]
pub type BitEdccaThrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_CCA_FILTER_THRS` reader - If PHY_CCA signal width is bigger than this threshold, this PHY_CCA signal is valid for MAC. Unit: us"]
pub type BitCcaFilterThrsR = crate::FieldReader;
#[doc = "Field `BIT_CCA_FILTER_THRS` writer - If PHY_CCA signal width is bigger than this threshold, this PHY_CCA signal is valid for MAC. Unit: us"]
pub type BitCcaFilterThrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EN_FILTER_CCA` reader - Enable CCA filter threshold"]
pub type BitEnFilterCcaR = crate::BitReader;
#[doc = "Field `BIT_EN_FILTER_CCA` writer - Enable CCA filter threshold"]
pub type BitEnFilterCcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_NAVEND_RST_TXOP` reader - When set this bit, TXOP will be reset while NAV end."]
pub type BitEnNavendRstTxopR = crate::BitReader;
#[doc = "Field `BIT_EN_NAVEND_RST_TXOP` writer - When set this bit, TXOP will be reset while NAV end."]
pub type BitEnNavendRstTxopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_BCN_DIS_CTN` reader - Disable the Beacon queue disable contention,when set to 0 enable the force beacon function."]
pub type BitDisBcnDisCtnR = crate::BitReader;
#[doc = "Field `BIT_DIS_BCN_DIS_CTN` writer - Disable the Beacon queue disable contention,when set to 0 enable the force beacon function."]
pub type BitDisBcnDisCtnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_PKT_TX_ATIM` reader - 1: Disable AC Queue Packet Tx in ATIM (only valid at Nettype: Adhoc/AP) 0: Enable AC Queue packet Tx in ATIM"]
pub type BitDisPktTxAtimR = crate::BitReader;
#[doc = "Field `BIT_DIS_PKT_TX_ATIM` writer - 1: Disable AC Queue Packet Tx in ATIM (only valid at Nettype: Adhoc/AP) 0: Enable AC Queue packet Tx in ATIM"]
pub type BitDisPktTxAtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BCN_TX_BTCCA` reader - 1: Enable Beacon Tx at TBTT if BT_CCA is enabled 0: Disable"]
pub type BitEnBcnTxBtccaR = crate::BitReader;
#[doc = "Field `BIT_EN_BCN_TX_BTCCA` writer - 1: Enable Beacon Tx at TBTT if BT_CCA is enabled 0: Disable"]
pub type BitEnBcnTxBtccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BKF_CLR_TXREQ` reader - when gen tx command done, while will backoff is invalid, then tx command will be cleared"]
pub type BitEnBkfClrTxreqR = crate::BitReader;
#[doc = "Field `BIT_EN_BKF_CLR_TXREQ` writer - when gen tx command done, while will backoff is invalid, then tx command will be cleared"]
pub type BitEnBkfClrTxreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHANGE_POW_BCN_AREA` reader - select real_ctwindow & real_bcnq_area for noa_unit"]
pub type BitChangePowBcnAreaR = crate::BitReader;
#[doc = "Field `BIT_CHANGE_POW_BCN_AREA` writer - select real_ctwindow & real_bcnq_area for noa_unit"]
pub type BitChangePowBcnAreaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FORCE_DOZE0_0` reader - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
pub type BitForceDoze0_0R = crate::BitReader;
#[doc = "Field `BIT_NOA0_OFF_PERIOD_0` reader - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
pub type BitNoa0OffPeriod0R = crate::BitReader;
#[doc = "Field `BIT_FORCE_DOZE1_0` reader - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
pub type BitForceDoze1_0R = crate::BitReader;
#[doc = "Field `BIT_NOA1_OFF_PERIOD_0` reader - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
pub type BitNoa1OffPeriod0R = crate::BitReader;
#[doc = "Field `BIT_CTWIN_EARLY_DISTX_0` reader - ctwin_early_distx. (It is including early time. )"]
pub type BitCtwinEarlyDistx0R = crate::BitReader;
#[doc = "Field `BIT_BEACON_AREA_ON_0` reader - The time is from HOLD Time to BeaconTx or Timeout. (It is including early time. )"]
pub type BitBeaconAreaOn0R = crate::BitReader;
#[doc = "Field `BIT_CTWINDOW_ON_0` reader - Spec time and Beacon Early Time to TBTT Time. (It is including early time. )"]
pub type BitCtwindowOn0R = crate::BitReader;
#[doc = "Field `BIT_POWER_STATE_0` reader - 1: On, 0: Off. (It is including early time. )"]
pub type BitPowerState0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - EDCCA threshold value, edcca over this threshold will cause bkf_rst Bit\\[7\\]: when PHY_EDCCA=1, block TX when the timer times out Bit\\[6:0\\]: timer (unit:32us)"]
    #[inline(always)]
    pub fn bit_edcca_thrs(&self) -> BitEdccaThrsR {
        BitEdccaThrsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - If PHY_CCA signal width is bigger than this threshold, this PHY_CCA signal is valid for MAC. Unit: us"]
    #[inline(always)]
    pub fn bit_cca_filter_thrs(&self) -> BitCcaFilterThrsR {
        BitCcaFilterThrsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable CCA filter threshold"]
    #[inline(always)]
    pub fn bit_en_filter_cca(&self) -> BitEnFilterCcaR {
        BitEnFilterCcaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When set this bit, TXOP will be reset while NAV end."]
    #[inline(always)]
    pub fn bit_en_navend_rst_txop(&self) -> BitEnNavendRstTxopR {
        BitEnNavendRstTxopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable the Beacon queue disable contention,when set to 0 enable the force beacon function."]
    #[inline(always)]
    pub fn bit_dis_bcn_dis_ctn(&self) -> BitDisBcnDisCtnR {
        BitDisBcnDisCtnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Disable AC Queue Packet Tx in ATIM (only valid at Nettype: Adhoc/AP) 0: Enable AC Queue packet Tx in ATIM"]
    #[inline(always)]
    pub fn bit_dis_pkt_tx_atim(&self) -> BitDisPktTxAtimR {
        BitDisPktTxAtimR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Enable Beacon Tx at TBTT if BT_CCA is enabled 0: Disable"]
    #[inline(always)]
    pub fn bit_en_bcn_tx_btcca(&self) -> BitEnBcnTxBtccaR {
        BitEnBcnTxBtccaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - when gen tx command done, while will backoff is invalid, then tx command will be cleared"]
    #[inline(always)]
    pub fn bit_en_bkf_clr_txreq(&self) -> BitEnBkfClrTxreqR {
        BitEnBkfClrTxreqR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - select real_ctwindow & real_bcnq_area for noa_unit"]
    #[inline(always)]
    pub fn bit_change_pow_bcn_area(&self) -> BitChangePowBcnAreaR {
        BitChangePowBcnAreaR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_force_doze0_0(&self) -> BitForceDoze0_0R {
        BitForceDoze0_0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_noa0_off_period_0(&self) -> BitNoa0OffPeriod0R {
        BitNoa0OffPeriod0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - If NoA_Count == 1, this value is 1. else 0. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_force_doze1_0(&self) -> BitForceDoze1_0R {
        BitForceDoze1_0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Spec time but exclude BCNDMATIM time for wake. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_noa1_off_period_0(&self) -> BitNoa1OffPeriod0R {
        BitNoa1OffPeriod0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ctwin_early_distx. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_ctwin_early_distx_0(&self) -> BitCtwinEarlyDistx0R {
        BitCtwinEarlyDistx0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The time is from HOLD Time to BeaconTx or Timeout. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_beacon_area_on_0(&self) -> BitBeaconAreaOn0R {
        BitBeaconAreaOn0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Spec time and Beacon Early Time to TBTT Time. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_ctwindow_on_0(&self) -> BitCtwindowOn0R {
        BitCtwindowOn0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: On, 0: Off. (It is including early time. )"]
    #[inline(always)]
    pub fn bit_power_state_0(&self) -> BitPowerState0R {
        BitPowerState0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - EDCCA threshold value, edcca over this threshold will cause bkf_rst Bit\\[7\\]: when PHY_EDCCA=1, block TX when the timer times out Bit\\[6:0\\]: timer (unit:32us)"]
    #[inline(always)]
    pub fn bit_edcca_thrs(&mut self) -> BitEdccaThrsW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitEdccaThrsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - If PHY_CCA signal width is bigger than this threshold, this PHY_CCA signal is valid for MAC. Unit: us"]
    #[inline(always)]
    pub fn bit_cca_filter_thrs(&mut self) -> BitCcaFilterThrsW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitCcaFilterThrsW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable CCA filter threshold"]
    #[inline(always)]
    pub fn bit_en_filter_cca(&mut self) -> BitEnFilterCcaW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitEnFilterCcaW::new(self, 16)
    }
    #[doc = "Bit 17 - When set this bit, TXOP will be reset while NAV end."]
    #[inline(always)]
    pub fn bit_en_navend_rst_txop(
        &mut self,
    ) -> BitEnNavendRstTxopW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitEnNavendRstTxopW::new(self, 17)
    }
    #[doc = "Bit 18 - Disable the Beacon queue disable contention,when set to 0 enable the force beacon function."]
    #[inline(always)]
    pub fn bit_dis_bcn_dis_ctn(&mut self) -> BitDisBcnDisCtnW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitDisBcnDisCtnW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Disable AC Queue Packet Tx in ATIM (only valid at Nettype: Adhoc/AP) 0: Enable AC Queue packet Tx in ATIM"]
    #[inline(always)]
    pub fn bit_dis_pkt_tx_atim(&mut self) -> BitDisPktTxAtimW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitDisPktTxAtimW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Enable Beacon Tx at TBTT if BT_CCA is enabled 0: Disable"]
    #[inline(always)]
    pub fn bit_en_bcn_tx_btcca(&mut self) -> BitEnBcnTxBtccaW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitEnBcnTxBtccaW::new(self, 20)
    }
    #[doc = "Bit 22 - when gen tx command done, while will backoff is invalid, then tx command will be cleared"]
    #[inline(always)]
    pub fn bit_en_bkf_clr_txreq(&mut self) -> BitEnBkfClrTxreqW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitEnBkfClrTxreqW::new(self, 22)
    }
    #[doc = "Bit 23 - select real_ctwindow & real_bcnq_area for noa_unit"]
    #[inline(always)]
    pub fn bit_change_pow_bcn_area(
        &mut self,
    ) -> BitChangePowBcnAreaW<'_, RegNoa0PktLifetimeCtrlSpec> {
        BitChangePowBcnAreaW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_noa0_pkt_lifetime_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_noa0_pkt_lifetime_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNoa0PktLifetimeCtrlSpec;
impl crate::RegisterSpec for RegNoa0PktLifetimeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_noa0_pkt_lifetime_ctrl::R`](R) reader structure"]
impl crate::Readable for RegNoa0PktLifetimeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_noa0_pkt_lifetime_ctrl::W`](W) writer structure"]
impl crate::Writable for RegNoa0PktLifetimeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NOA0_PKT_LIFETIME_CTRL to value 0x004c_0a8f"]
impl crate::Resettable for RegNoa0PktLifetimeCtrlSpec {
    const RESET_VALUE: u32 = 0x004c_0a8f;
}
