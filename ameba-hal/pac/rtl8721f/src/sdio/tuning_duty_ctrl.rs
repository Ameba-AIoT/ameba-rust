#[doc = "Register `TUNING_DUTY_CTRL` reader"]
pub type R = crate::R<TuningDutyCtrlSpec>;
#[doc = "Field `use_tx_duty_ctrl` reader - Use TX Duty Control"]
pub type UseTxDutyCtrlR = crate::BitReader;
#[doc = "Field `use_rx_duty_ctrl` reader - Use RX Duty Control"]
pub type UseRxDutyCtrlR = crate::BitReader;
#[doc = "Field `rxwait_afifo_rst_en` reader - Rx wait AFIFO Reset enable"]
pub type RxwaitAfifoRstEnR = crate::BitReader;
#[doc = "Field `clr_inter_clk_stable` reader - Clear Internal CLK Stable"]
pub type ClrInterClkStableR = crate::BitReader;
#[doc = "Field `sdio_stop_clk_en` reader - Stop SDCLK enable"]
pub type SdioStopClkEnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Use TX Duty Control"]
    #[inline(always)]
    pub fn use_tx_duty_ctrl(&self) -> UseTxDutyCtrlR {
        UseTxDutyCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use RX Duty Control"]
    #[inline(always)]
    pub fn use_rx_duty_ctrl(&self) -> UseRxDutyCtrlR {
        UseRxDutyCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx wait AFIFO Reset enable"]
    #[inline(always)]
    pub fn rxwait_afifo_rst_en(&self) -> RxwaitAfifoRstEnR {
        RxwaitAfifoRstEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Internal CLK Stable"]
    #[inline(always)]
    pub fn clr_inter_clk_stable(&self) -> ClrInterClkStableR {
        ClrInterClkStableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop SDCLK enable"]
    #[inline(always)]
    pub fn sdio_stop_clk_en(&self) -> SdioStopClkEnR {
        SdioStopClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tuning_duty_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TuningDutyCtrlSpec;
impl crate::RegisterSpec for TuningDutyCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tuning_duty_ctrl::R`](R) reader structure"]
impl crate::Readable for TuningDutyCtrlSpec {}
#[doc = "`reset()` method sets TUNING_DUTY_CTRL to value 0"]
impl crate::Resettable for TuningDutyCtrlSpec {}
