#[doc = "Register `REG_HISR0` reader"]
pub type R = crate::R<RegHisr0Spec>;
#[doc = "Field `BIT_HS_TIMEOUT0_INT_V1` reader - tsf Timer0 interrupt"]
pub type BitHsTimeout0IntV1R = crate::BitReader;
#[doc = "Field `BIT_HS_TIMEOUT1_INT_V1` reader - tsf Timer1 interrupt"]
pub type BitHsTimeout1IntV1R = crate::BitReader;
#[doc = "Field `BIT_HS_TIMEOUT2_INT` reader - tsf Timer2 interrupt"]
pub type BitHsTimeout2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWE_INT2` reader - port2 CTWindow end interrupt for p2p."]
pub type BitHsCtweInt2R = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN1_OK_INT` reader - port0 BSSID0 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn1OkIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN2_OK_INT` reader - port0 BSSID1 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn2OkIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN3_OK_INT` reader - port0 BSSID2 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn3OkIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN4_OK_INT` reader - port0 BSSID3 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn4OkIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN5_OK_INT` reader - port0 BSSID4 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn5OkIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN6_OK_INT` reader - port0 BSSID5 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn6OkIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN7_OK_INT` reader - port0 BSSID6 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn7OkIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN8_OK_INT` reader - port0 BSSID7 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn8OkIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN1_ERR_INT` reader - port0 BSSID0 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn1ErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN2_ERR_INT` reader - port0 BSSID1 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn2ErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN3_ERR_INT` reader - port0 BSSID2 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn3ErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN4_ERR_INT` reader - port0 BSSID3 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn4ErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN5_ERR_INT` reader - port0 BSSID4 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn5ErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN6_ERR_INT` reader - port0 BSSID5 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn6ErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN7_ERR_INT` reader - port0 BSSID6 TRX beacon error interrupt (ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn7ErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN8_ERR_INT` reader - port0 BSSID7 TRX beacon error interrupt (ap is txbcn, sta is rxbcn)"]
pub type BitHsTxbcn8ErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TXFF_FIFO_INT` reader - txpktbuffer full interrupt"]
pub type BitHsTxffFifoIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TDE_ERR_INT` reader - Txdma error interrupt"]
pub type BitHsTdeErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_RDE_ERR_INT` reader - Rxdma error interrupt"]
pub type BitHsRdeErrIntR = crate::BitReader;
#[doc = "Field `BIT_HS_RXFF_FULL_INT` reader - Rxpktbuffer full interrupt"]
pub type BitHsRxffFullIntR = crate::BitReader;
#[doc = "Field `BIT_HS_HCPWM_INT` reader - CPWM interrupt,CPU power Mode exchange INT Status, Write 1 clear"]
pub type BitHsHcpwmIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TRIG_LAT_AUDIO_CNT_INT` reader - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
pub type BitHsTrigLatAudioCntIntR = crate::BitReader;
#[doc = "Field `BIT_HS_FWMSG_INT` reader - FW message interrupt,CPU to Host Command INT Status, Write 1 clear"]
pub type BitHsFwmsgIntR = crate::BitReader;
#[doc = "Field `BIT_HS_TSF_BIT32_TOGGLE_INT` reader - TSF Timer BIT32 toggle indication interrupt"]
pub type BitHsTsfBit32ToggleIntR = crate::BitReader;
#[doc = "Field `BIT_HISR3_INT` reader - HISR3 Indicator (HIISR3 andHIMR3 are true, this bit is set to 1)"]
pub type BitHisr3IntR = crate::BitReader;
#[doc = "Field `BIT_HISR2_INT` reader - HISR2 Indicator (HIISR2 andHIMR2 are true, this bit is set to 1)"]
pub type BitHisr2IntR = crate::BitReader;
#[doc = "Field `BIT_HS_RXDES_UNAVAIL_INT` reader - Rx Descriptor Unavailable"]
pub type BitHsRxdesUnavailIntR = crate::BitReader;
#[doc = "Field `BIT_HISR1_INT` reader - HISR1 Indicator (HIISR1 andHIMR1 are true, this bit is set to 1)"]
pub type BitHisr1IntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tsf Timer0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout0_int_v1(&self) -> BitHsTimeout0IntV1R {
        BitHsTimeout0IntV1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tsf Timer1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout1_int_v1(&self) -> BitHsTimeout1IntV1R {
        BitHsTimeout1IntV1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tsf Timer2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout2_int(&self) -> BitHsTimeout2IntR {
        BitHsTimeout2IntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - port2 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_hs_ctwe_int2(&self) -> BitHsCtweInt2R {
        BitHsCtweInt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - port0 BSSID0 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn1_ok_int(&self) -> BitHsTxbcn1OkIntR {
        BitHsTxbcn1OkIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - port0 BSSID1 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn2_ok_int(&self) -> BitHsTxbcn2OkIntR {
        BitHsTxbcn2OkIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - port0 BSSID2 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn3_ok_int(&self) -> BitHsTxbcn3OkIntR {
        BitHsTxbcn3OkIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - port0 BSSID3 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn4_ok_int(&self) -> BitHsTxbcn4OkIntR {
        BitHsTxbcn4OkIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - port0 BSSID4 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn5_ok_int(&self) -> BitHsTxbcn5OkIntR {
        BitHsTxbcn5OkIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - port0 BSSID5 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn6_ok_int(&self) -> BitHsTxbcn6OkIntR {
        BitHsTxbcn6OkIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - port0 BSSID6 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn7_ok_int(&self) -> BitHsTxbcn7OkIntR {
        BitHsTxbcn7OkIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - port0 BSSID7 TRX beacon ok interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn8_ok_int(&self) -> BitHsTxbcn8OkIntR {
        BitHsTxbcn8OkIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - port0 BSSID0 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn1_err_int(&self) -> BitHsTxbcn1ErrIntR {
        BitHsTxbcn1ErrIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - port0 BSSID1 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn2_err_int(&self) -> BitHsTxbcn2ErrIntR {
        BitHsTxbcn2ErrIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - port0 BSSID2 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn3_err_int(&self) -> BitHsTxbcn3ErrIntR {
        BitHsTxbcn3ErrIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - port0 BSSID3 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn4_err_int(&self) -> BitHsTxbcn4ErrIntR {
        BitHsTxbcn4ErrIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - port0 BSSID4 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn5_err_int(&self) -> BitHsTxbcn5ErrIntR {
        BitHsTxbcn5ErrIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - port0 BSSID5 TRX beacon error interrupt(ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn6_err_int(&self) -> BitHsTxbcn6ErrIntR {
        BitHsTxbcn6ErrIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - port0 BSSID6 TRX beacon error interrupt (ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn7_err_int(&self) -> BitHsTxbcn7ErrIntR {
        BitHsTxbcn7ErrIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - port0 BSSID7 TRX beacon error interrupt (ap is txbcn, sta is rxbcn)"]
    #[inline(always)]
    pub fn bit_hs_txbcn8_err_int(&self) -> BitHsTxbcn8ErrIntR {
        BitHsTxbcn8ErrIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - txpktbuffer full interrupt"]
    #[inline(always)]
    pub fn bit_hs_txff_fifo_int(&self) -> BitHsTxffFifoIntR {
        BitHsTxffFifoIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Txdma error interrupt"]
    #[inline(always)]
    pub fn bit_hs_tde_err_int(&self) -> BitHsTdeErrIntR {
        BitHsTdeErrIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rxdma error interrupt"]
    #[inline(always)]
    pub fn bit_hs_rde_err_int(&self) -> BitHsRdeErrIntR {
        BitHsRdeErrIntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rxpktbuffer full interrupt"]
    #[inline(always)]
    pub fn bit_hs_rxff_full_int(&self) -> BitHsRxffFullIntR {
        BitHsRxffFullIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPWM interrupt,CPU power Mode exchange INT Status, Write 1 clear"]
    #[inline(always)]
    pub fn bit_hs_hcpwm_int(&self) -> BitHsHcpwmIntR {
        BitHsHcpwmIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
    #[inline(always)]
    pub fn bit_hs_trig_lat_audio_cnt_int(&self) -> BitHsTrigLatAudioCntIntR {
        BitHsTrigLatAudioCntIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FW message interrupt,CPU to Host Command INT Status, Write 1 clear"]
    #[inline(always)]
    pub fn bit_hs_fwmsg_int(&self) -> BitHsFwmsgIntR {
        BitHsFwmsgIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TSF Timer BIT32 toggle indication interrupt"]
    #[inline(always)]
    pub fn bit_hs_tsf_bit32_toggle_int(&self) -> BitHsTsfBit32ToggleIntR {
        BitHsTsfBit32ToggleIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HISR3 Indicator (HIISR3 andHIMR3 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr3_int(&self) -> BitHisr3IntR {
        BitHisr3IntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HISR2 Indicator (HIISR2 andHIMR2 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr2_int(&self) -> BitHisr2IntR {
        BitHisr2IntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rx Descriptor Unavailable"]
    #[inline(always)]
    pub fn bit_hs_rxdes_unavail_int(&self) -> BitHsRxdesUnavailIntR {
        BitHsRxdesUnavailIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - HISR1 Indicator (HIISR1 andHIMR1 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr1_int(&self) -> BitHisr1IntR {
        BitHisr1IntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hisr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHisr0Spec;
impl crate::RegisterSpec for RegHisr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hisr0::R`](R) reader structure"]
impl crate::Readable for RegHisr0Spec {}
#[doc = "`reset()` method sets REG_HISR0 to value 0"]
impl crate::Resettable for RegHisr0Spec {}
