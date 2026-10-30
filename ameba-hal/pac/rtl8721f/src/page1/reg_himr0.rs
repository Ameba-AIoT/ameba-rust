#[doc = "Register `REG_HIMR0` reader"]
pub type R = crate::R<RegHimr0Spec>;
#[doc = "Register `REG_HIMR0` writer"]
pub type W = crate::W<RegHimr0Spec>;
#[doc = "Field `BIT_HS_TIMEOUT0_V1_INT_EN` reader - tsf Timer0 interrupt"]
pub type BitHsTimeout0V1IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TIMEOUT0_V1_INT_EN` writer - tsf Timer0 interrupt"]
pub type BitHsTimeout0V1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TIMEOUT1_V1_INT_EN` reader - tsf Timer1 interrupt"]
pub type BitHsTimeout1V1IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TIMEOUT1_V1_INT_EN` writer - tsf Timer1 interrupt"]
pub type BitHsTimeout1V1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TIMEOUT2_INT_EN` reader - tsf Timer2 interrupt"]
pub type BitHsTimeout2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TIMEOUT2_INT_EN` writer - tsf Timer2 interrupt"]
pub type BitHsTimeout2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_CTWE_INT2_EN` reader - port2 CTWindow end interrupt for p2p."]
pub type BitHsCtweInt2EnR = crate::BitReader;
#[doc = "Field `BIT_HS_CTWE_INT2_EN` writer - port2 CTWindow end interrupt for p2p."]
pub type BitHsCtweInt2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN1_OK_INT_EN` reader - port0 BSSID0 TX beacon ok interrupt"]
pub type BitHsTxbcn1OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN1_OK_INT_EN` writer - port0 BSSID0 TX beacon ok interrupt"]
pub type BitHsTxbcn1OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN2_OK_INT_EN` reader - port0 BSSID1 TX beacon ok interrupt"]
pub type BitHsTxbcn2OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN2_OK_INT_EN` writer - port0 BSSID1 TX beacon ok interrupt"]
pub type BitHsTxbcn2OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN3_OK_INT_EN` reader - port0 BSSID2 TX beacon ok interrupt"]
pub type BitHsTxbcn3OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN3_OK_INT_EN` writer - port0 BSSID2 TX beacon ok interrupt"]
pub type BitHsTxbcn3OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN4_OK_INT_EN` reader - port0 BSSID3 TX beacon ok interrupt"]
pub type BitHsTxbcn4OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN4_OK_INT_EN` writer - port0 BSSID3 TX beacon ok interrupt"]
pub type BitHsTxbcn4OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN5_OK_INT_EN` reader - port0 BSSID4 TX beacon ok interrupt"]
pub type BitHsTxbcn5OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN5_OK_INT_EN` writer - port0 BSSID4 TX beacon ok interrupt"]
pub type BitHsTxbcn5OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN6_OK_INT_EN` reader - port0 BSSID5 TX beacon ok interrupt"]
pub type BitHsTxbcn6OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN6_OK_INT_EN` writer - port0 BSSID5 TX beacon ok interrupt"]
pub type BitHsTxbcn6OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN7_OK_INT_EN` reader - port0 BSSID6 TX beacon ok interrupt"]
pub type BitHsTxbcn7OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN7_OK_INT_EN` writer - port0 BSSID6 TX beacon ok interrupt"]
pub type BitHsTxbcn7OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN8_OK_INT_EN` reader - port0 BSSID7 TX beacon ok interrupt"]
pub type BitHsTxbcn8OkIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN8_OK_INT_EN` writer - port0 BSSID7 TX beacon ok interrupt"]
pub type BitHsTxbcn8OkIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN1_ERR_INT_EN` reader - port0 BSSID0 TX beacon error interrupt"]
pub type BitHsTxbcn1ErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN1_ERR_INT_EN` writer - port0 BSSID0 TX beacon error interrupt"]
pub type BitHsTxbcn1ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN2_ERR_INT_EN` reader - port0 BSSID1 TX beacon error interrupt"]
pub type BitHsTxbcn2ErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN2_ERR_INT_EN` writer - port0 BSSID1 TX beacon error interrupt"]
pub type BitHsTxbcn2ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN3_ERR_INT_EN` reader - port0 BSSID2 TX beacon error interrupt"]
pub type BitHsTxbcn3ErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN3_ERR_INT_EN` writer - port0 BSSID2 TX beacon error interrupt"]
pub type BitHsTxbcn3ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN4_ERR_INT_EN` reader - port0 BSSID3 TX beacon error interrupt"]
pub type BitHsTxbcn4ErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN4_ERR_INT_EN` writer - port0 BSSID3 TX beacon error interrupt"]
pub type BitHsTxbcn4ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN5_ERR_INT_EN` reader - port0 BSSID4 TX beacon error interrupt"]
pub type BitHsTxbcn5ErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN5_ERR_INT_EN` writer - port0 BSSID4 TX beacon error interrupt"]
pub type BitHsTxbcn5ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN6_ERR_INT_EN` reader - port0 BSSID5 TX beacon error interrupt"]
pub type BitHsTxbcn6ErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN6_ERR_INT_EN` writer - port0 BSSID5 TX beacon error interrupt"]
pub type BitHsTxbcn6ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN7_ERR_INT_EN` reader - port0 BSSID6 TX beacon error interrupt"]
pub type BitHsTxbcn7ErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN7_ERR_INT_EN` writer - port0 BSSID6 TX beacon error interrupt"]
pub type BitHsTxbcn7ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXBCN8_ERR_INT_EN` reader - port0 BSSID7 TX beacon error interrupt"]
pub type BitHsTxbcn8ErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXBCN8_ERR_INT_EN` writer - port0 BSSID7 TX beacon error interrupt"]
pub type BitHsTxbcn8ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TXFF_FIFO_INT_EN` reader - txpktbuffer full interrupt"]
pub type BitHsTxffFifoIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TXFF_FIFO_INT_EN` writer - txpktbuffer full interrupt"]
pub type BitHsTxffFifoIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TDE_ERR_INT_EN` reader - Txdma error interrupt"]
pub type BitHsTdeErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TDE_ERR_INT_EN` writer - Txdma error interrupt"]
pub type BitHsTdeErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_RDE_ERR_INT_EN` reader - Rxdma error interrupt"]
pub type BitHsRdeErrIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_RDE_ERR_INT_EN` writer - Rxdma error interrupt"]
pub type BitHsRdeErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_RXFF_FULL_INT_EN` reader - Rxpktbuffer full interrupt"]
pub type BitHsRxffFullIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_RXFF_FULL_INT_EN` writer - Rxpktbuffer full interrupt"]
pub type BitHsRxffFullIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_HCPWM_INT_EN` reader - CPWM interrupt,CPU power Mode exchange INT Status, Write 1 clear"]
pub type BitHsHcpwmIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_HCPWM_INT_EN` writer - CPWM interrupt,CPU power Mode exchange INT Status, Write 1 clear"]
pub type BitHsHcpwmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TRIG_LAT_AUDIO_CNT_INT_EN` reader - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
pub type BitHsTrigLatAudioCntIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TRIG_LAT_AUDIO_CNT_INT_EN` writer - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
pub type BitHsTrigLatAudioCntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_FWMSG_INT_EN` reader - FW message interrupt,CPU to Host Command INT Status, Write 1 clear"]
pub type BitHsFwmsgIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_FWMSG_INT_EN` writer - FW message interrupt,CPU to Host Command INT Status, Write 1 clear"]
pub type BitHsFwmsgIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_TSF_BIT32_TOGGLE_INT_EN` reader - TSF Timer BIT32 toggle indication interrupt"]
pub type BitHsTsfBit32ToggleIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_TSF_BIT32_TOGGLE_INT_EN` writer - TSF Timer BIT32 toggle indication interrupt"]
pub type BitHsTsfBit32ToggleIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HISR3_INT_EN` reader - HISR3 Indicator (HIISR3 andHIMR3 are true, this bit is set to 1)"]
pub type BitHisr3IntEnR = crate::BitReader;
#[doc = "Field `BIT_HISR3_INT_EN` writer - HISR3 Indicator (HIISR3 andHIMR3 are true, this bit is set to 1)"]
pub type BitHisr3IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HISR2_INT_EN` reader - HISR2 Indicator (HIISR2 andHIMR2 are true, this bit is set to 1)"]
pub type BitHisr2IntEnR = crate::BitReader;
#[doc = "Field `BIT_HISR2_INT_EN` writer - HISR2 Indicator (HIISR2 andHIMR2 are true, this bit is set to 1)"]
pub type BitHisr2IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HS_RXDES_UNAVAIL_INT_EN` reader - Rx Descriptor Unavailable"]
pub type BitHsRxdesUnavailIntEnR = crate::BitReader;
#[doc = "Field `BIT_HS_RXDES_UNAVAIL_INT_EN` writer - Rx Descriptor Unavailable"]
pub type BitHsRxdesUnavailIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HISR1_INT_EN` reader - HISR1 Indicator (HIISR1 andHIMR1 are true, this bit is set to 1)"]
pub type BitHisr1IntEnR = crate::BitReader;
#[doc = "Field `BIT_HISR1_INT_EN` writer - HISR1 Indicator (HIISR1 andHIMR1 are true, this bit is set to 1)"]
pub type BitHisr1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - tsf Timer0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout0_v1_int_en(&self) -> BitHsTimeout0V1IntEnR {
        BitHsTimeout0V1IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tsf Timer1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout1_v1_int_en(&self) -> BitHsTimeout1V1IntEnR {
        BitHsTimeout1V1IntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tsf Timer2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout2_int_en(&self) -> BitHsTimeout2IntEnR {
        BitHsTimeout2IntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - port2 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_hs_ctwe_int2_en(&self) -> BitHsCtweInt2EnR {
        BitHsCtweInt2EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - port0 BSSID0 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn1_ok_int_en(&self) -> BitHsTxbcn1OkIntEnR {
        BitHsTxbcn1OkIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - port0 BSSID1 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn2_ok_int_en(&self) -> BitHsTxbcn2OkIntEnR {
        BitHsTxbcn2OkIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - port0 BSSID2 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn3_ok_int_en(&self) -> BitHsTxbcn3OkIntEnR {
        BitHsTxbcn3OkIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - port0 BSSID3 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn4_ok_int_en(&self) -> BitHsTxbcn4OkIntEnR {
        BitHsTxbcn4OkIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - port0 BSSID4 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn5_ok_int_en(&self) -> BitHsTxbcn5OkIntEnR {
        BitHsTxbcn5OkIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - port0 BSSID5 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn6_ok_int_en(&self) -> BitHsTxbcn6OkIntEnR {
        BitHsTxbcn6OkIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - port0 BSSID6 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn7_ok_int_en(&self) -> BitHsTxbcn7OkIntEnR {
        BitHsTxbcn7OkIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - port0 BSSID7 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn8_ok_int_en(&self) -> BitHsTxbcn8OkIntEnR {
        BitHsTxbcn8OkIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - port0 BSSID0 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn1_err_int_en(&self) -> BitHsTxbcn1ErrIntEnR {
        BitHsTxbcn1ErrIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - port0 BSSID1 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn2_err_int_en(&self) -> BitHsTxbcn2ErrIntEnR {
        BitHsTxbcn2ErrIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - port0 BSSID2 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn3_err_int_en(&self) -> BitHsTxbcn3ErrIntEnR {
        BitHsTxbcn3ErrIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - port0 BSSID3 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn4_err_int_en(&self) -> BitHsTxbcn4ErrIntEnR {
        BitHsTxbcn4ErrIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - port0 BSSID4 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn5_err_int_en(&self) -> BitHsTxbcn5ErrIntEnR {
        BitHsTxbcn5ErrIntEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - port0 BSSID5 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn6_err_int_en(&self) -> BitHsTxbcn6ErrIntEnR {
        BitHsTxbcn6ErrIntEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - port0 BSSID6 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn7_err_int_en(&self) -> BitHsTxbcn7ErrIntEnR {
        BitHsTxbcn7ErrIntEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - port0 BSSID7 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn8_err_int_en(&self) -> BitHsTxbcn8ErrIntEnR {
        BitHsTxbcn8ErrIntEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - txpktbuffer full interrupt"]
    #[inline(always)]
    pub fn bit_hs_txff_fifo_int_en(&self) -> BitHsTxffFifoIntEnR {
        BitHsTxffFifoIntEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Txdma error interrupt"]
    #[inline(always)]
    pub fn bit_hs_tde_err_int_en(&self) -> BitHsTdeErrIntEnR {
        BitHsTdeErrIntEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rxdma error interrupt"]
    #[inline(always)]
    pub fn bit_hs_rde_err_int_en(&self) -> BitHsRdeErrIntEnR {
        BitHsRdeErrIntEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rxpktbuffer full interrupt"]
    #[inline(always)]
    pub fn bit_hs_rxff_full_int_en(&self) -> BitHsRxffFullIntEnR {
        BitHsRxffFullIntEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPWM interrupt,CPU power Mode exchange INT Status, Write 1 clear"]
    #[inline(always)]
    pub fn bit_hs_hcpwm_int_en(&self) -> BitHsHcpwmIntEnR {
        BitHsHcpwmIntEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
    #[inline(always)]
    pub fn bit_hs_trig_lat_audio_cnt_int_en(&self) -> BitHsTrigLatAudioCntIntEnR {
        BitHsTrigLatAudioCntIntEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FW message interrupt,CPU to Host Command INT Status, Write 1 clear"]
    #[inline(always)]
    pub fn bit_hs_fwmsg_int_en(&self) -> BitHsFwmsgIntEnR {
        BitHsFwmsgIntEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TSF Timer BIT32 toggle indication interrupt"]
    #[inline(always)]
    pub fn bit_hs_tsf_bit32_toggle_int_en(&self) -> BitHsTsfBit32ToggleIntEnR {
        BitHsTsfBit32ToggleIntEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HISR3 Indicator (HIISR3 andHIMR3 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr3_int_en(&self) -> BitHisr3IntEnR {
        BitHisr3IntEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HISR2 Indicator (HIISR2 andHIMR2 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr2_int_en(&self) -> BitHisr2IntEnR {
        BitHisr2IntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rx Descriptor Unavailable"]
    #[inline(always)]
    pub fn bit_hs_rxdes_unavail_int_en(&self) -> BitHsRxdesUnavailIntEnR {
        BitHsRxdesUnavailIntEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - HISR1 Indicator (HIISR1 andHIMR1 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr1_int_en(&self) -> BitHisr1IntEnR {
        BitHisr1IntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tsf Timer0 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout0_v1_int_en(&mut self) -> BitHsTimeout0V1IntEnW<'_, RegHimr0Spec> {
        BitHsTimeout0V1IntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - tsf Timer1 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout1_v1_int_en(&mut self) -> BitHsTimeout1V1IntEnW<'_, RegHimr0Spec> {
        BitHsTimeout1V1IntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - tsf Timer2 interrupt"]
    #[inline(always)]
    pub fn bit_hs_timeout2_int_en(&mut self) -> BitHsTimeout2IntEnW<'_, RegHimr0Spec> {
        BitHsTimeout2IntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - port2 CTWindow end interrupt for p2p."]
    #[inline(always)]
    pub fn bit_hs_ctwe_int2_en(&mut self) -> BitHsCtweInt2EnW<'_, RegHimr0Spec> {
        BitHsCtweInt2EnW::new(self, 3)
    }
    #[doc = "Bit 4 - port0 BSSID0 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn1_ok_int_en(&mut self) -> BitHsTxbcn1OkIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn1OkIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - port0 BSSID1 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn2_ok_int_en(&mut self) -> BitHsTxbcn2OkIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn2OkIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - port0 BSSID2 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn3_ok_int_en(&mut self) -> BitHsTxbcn3OkIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn3OkIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - port0 BSSID3 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn4_ok_int_en(&mut self) -> BitHsTxbcn4OkIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn4OkIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - port0 BSSID4 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn5_ok_int_en(&mut self) -> BitHsTxbcn5OkIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn5OkIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - port0 BSSID5 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn6_ok_int_en(&mut self) -> BitHsTxbcn6OkIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn6OkIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - port0 BSSID6 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn7_ok_int_en(&mut self) -> BitHsTxbcn7OkIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn7OkIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - port0 BSSID7 TX beacon ok interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn8_ok_int_en(&mut self) -> BitHsTxbcn8OkIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn8OkIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - port0 BSSID0 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn1_err_int_en(&mut self) -> BitHsTxbcn1ErrIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn1ErrIntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - port0 BSSID1 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn2_err_int_en(&mut self) -> BitHsTxbcn2ErrIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn2ErrIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - port0 BSSID2 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn3_err_int_en(&mut self) -> BitHsTxbcn3ErrIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn3ErrIntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - port0 BSSID3 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn4_err_int_en(&mut self) -> BitHsTxbcn4ErrIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn4ErrIntEnW::new(self, 15)
    }
    #[doc = "Bit 16 - port0 BSSID4 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn5_err_int_en(&mut self) -> BitHsTxbcn5ErrIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn5ErrIntEnW::new(self, 16)
    }
    #[doc = "Bit 17 - port0 BSSID5 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn6_err_int_en(&mut self) -> BitHsTxbcn6ErrIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn6ErrIntEnW::new(self, 17)
    }
    #[doc = "Bit 18 - port0 BSSID6 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn7_err_int_en(&mut self) -> BitHsTxbcn7ErrIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn7ErrIntEnW::new(self, 18)
    }
    #[doc = "Bit 19 - port0 BSSID7 TX beacon error interrupt"]
    #[inline(always)]
    pub fn bit_hs_txbcn8_err_int_en(&mut self) -> BitHsTxbcn8ErrIntEnW<'_, RegHimr0Spec> {
        BitHsTxbcn8ErrIntEnW::new(self, 19)
    }
    #[doc = "Bit 20 - txpktbuffer full interrupt"]
    #[inline(always)]
    pub fn bit_hs_txff_fifo_int_en(&mut self) -> BitHsTxffFifoIntEnW<'_, RegHimr0Spec> {
        BitHsTxffFifoIntEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Txdma error interrupt"]
    #[inline(always)]
    pub fn bit_hs_tde_err_int_en(&mut self) -> BitHsTdeErrIntEnW<'_, RegHimr0Spec> {
        BitHsTdeErrIntEnW::new(self, 21)
    }
    #[doc = "Bit 22 - Rxdma error interrupt"]
    #[inline(always)]
    pub fn bit_hs_rde_err_int_en(&mut self) -> BitHsRdeErrIntEnW<'_, RegHimr0Spec> {
        BitHsRdeErrIntEnW::new(self, 22)
    }
    #[doc = "Bit 23 - Rxpktbuffer full interrupt"]
    #[inline(always)]
    pub fn bit_hs_rxff_full_int_en(&mut self) -> BitHsRxffFullIntEnW<'_, RegHimr0Spec> {
        BitHsRxffFullIntEnW::new(self, 23)
    }
    #[doc = "Bit 24 - CPWM interrupt,CPU power Mode exchange INT Status, Write 1 clear"]
    #[inline(always)]
    pub fn bit_hs_hcpwm_int_en(&mut self) -> BitHsHcpwmIntEnW<'_, RegHimr0Spec> {
        BitHsHcpwmIntEnW::new(self, 24)
    }
    #[doc = "Bit 25 - WLMAC trigger audio to latch counter periodically. Generate by P0/P1's TSFT."]
    #[inline(always)]
    pub fn bit_hs_trig_lat_audio_cnt_int_en(
        &mut self,
    ) -> BitHsTrigLatAudioCntIntEnW<'_, RegHimr0Spec> {
        BitHsTrigLatAudioCntIntEnW::new(self, 25)
    }
    #[doc = "Bit 26 - FW message interrupt,CPU to Host Command INT Status, Write 1 clear"]
    #[inline(always)]
    pub fn bit_hs_fwmsg_int_en(&mut self) -> BitHsFwmsgIntEnW<'_, RegHimr0Spec> {
        BitHsFwmsgIntEnW::new(self, 26)
    }
    #[doc = "Bit 27 - TSF Timer BIT32 toggle indication interrupt"]
    #[inline(always)]
    pub fn bit_hs_tsf_bit32_toggle_int_en(
        &mut self,
    ) -> BitHsTsfBit32ToggleIntEnW<'_, RegHimr0Spec> {
        BitHsTsfBit32ToggleIntEnW::new(self, 27)
    }
    #[doc = "Bit 28 - HISR3 Indicator (HIISR3 andHIMR3 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr3_int_en(&mut self) -> BitHisr3IntEnW<'_, RegHimr0Spec> {
        BitHisr3IntEnW::new(self, 28)
    }
    #[doc = "Bit 29 - HISR2 Indicator (HIISR2 andHIMR2 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr2_int_en(&mut self) -> BitHisr2IntEnW<'_, RegHimr0Spec> {
        BitHisr2IntEnW::new(self, 29)
    }
    #[doc = "Bit 30 - Rx Descriptor Unavailable"]
    #[inline(always)]
    pub fn bit_hs_rxdes_unavail_int_en(&mut self) -> BitHsRxdesUnavailIntEnW<'_, RegHimr0Spec> {
        BitHsRxdesUnavailIntEnW::new(self, 30)
    }
    #[doc = "Bit 31 - HISR1 Indicator (HIISR1 andHIMR1 are true, this bit is set to 1)"]
    #[inline(always)]
    pub fn bit_hisr1_int_en(&mut self) -> BitHisr1IntEnW<'_, RegHimr0Spec> {
        BitHisr1IntEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_himr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_himr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHimr0Spec;
impl crate::RegisterSpec for RegHimr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_himr0::R`](R) reader structure"]
impl crate::Readable for RegHimr0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_himr0::W`](W) writer structure"]
impl crate::Writable for RegHimr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HIMR0 to value 0"]
impl crate::Resettable for RegHimr0Spec {}
