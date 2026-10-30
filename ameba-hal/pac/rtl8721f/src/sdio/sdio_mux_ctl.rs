#[doc = "Register `SDIO_MUX_CTL` reader"]
pub type R = crate::R<SdioMuxCtlSpec>;
#[doc = "Field `read_wait_no_stop_sdclk_en` reader - 1:no stop SD clock when read wait 0:stop SD clock when read wait"]
pub type ReadWaitNoStopSdclkEnR = crate::BitReader;
#[doc = "Field `card_int_period_en` reader - Card int period enable"]
pub type CardIntPeriodEnR = crate::BitReader;
#[doc = "Field `dat_tout_clk_opt` reader - Data Tout clk option"]
pub type DatToutClkOptR = crate::BitReader;
#[doc = "Field `v_tuning_err` reader - U Tuning error"]
pub type VTuningErrR = crate::BitReader;
#[doc = "Field `cmd_use_afifo_sync` reader - CMD use AFIFO sync"]
pub type CmdUseAfifoSyncR = crate::BitReader;
impl R {
    #[doc = "Bit 26 - 1:no stop SD clock when read wait 0:stop SD clock when read wait"]
    #[inline(always)]
    pub fn read_wait_no_stop_sdclk_en(&self) -> ReadWaitNoStopSdclkEnR {
        ReadWaitNoStopSdclkEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Card int period enable"]
    #[inline(always)]
    pub fn card_int_period_en(&self) -> CardIntPeriodEnR {
        CardIntPeriodEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Data Tout clk option"]
    #[inline(always)]
    pub fn dat_tout_clk_opt(&self) -> DatToutClkOptR {
        DatToutClkOptR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - U Tuning error"]
    #[inline(always)]
    pub fn v_tuning_err(&self) -> VTuningErrR {
        VTuningErrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CMD use AFIFO sync"]
    #[inline(always)]
    pub fn cmd_use_afifo_sync(&self) -> CmdUseAfifoSyncR {
        CmdUseAfifoSyncR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_mux_ctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioMuxCtlSpec;
impl crate::RegisterSpec for SdioMuxCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_mux_ctl::R`](R) reader structure"]
impl crate::Readable for SdioMuxCtlSpec {}
#[doc = "`reset()` method sets SDIO_MUX_CTL to value 0x4800_0000"]
impl crate::Resettable for SdioMuxCtlSpec {
    const RESET_VALUE: u32 = 0x4800_0000;
}
