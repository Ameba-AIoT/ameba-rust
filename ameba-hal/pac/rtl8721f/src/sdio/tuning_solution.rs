#[doc = "Register `TUNING_SOLUTION` reader"]
pub type R = crate::R<TuningSolutionSpec>;
#[doc = "Field `sdio_sd20_push_point` reader - SDIO SD20 Push Point 0: output at falling edge of SDCLK(set-up time: 1/2 SDCLK cycle; hold time: 1/2 SDCLK cycle), sd20_cmdo/sd20_dato (trigged by crc_clk rising edge) 1: output is ahead by 1/4 SDCLK period(set-up time: 3/4 SDCLK cycle; hold time: 1/4 SDCLK cycle), sd30_cmdo/sd30_dato (trigged by crc_clk falling edge)"]
pub type SdioSd20PushPointR = crate::BitReader;
#[doc = "Field `sdio_ddr_cmd_push_point` reader - SDIO DDR CMD Push Point 0: output at falling edge of SDCLK(set-up time: 1/2 SDCLK cycle; hold time: 1/2 SDCLK cycle), sd30_cmdo (trigged by crc_clk falling edge) 1: output is ahead by 1/4 SDCLK period(set-up time: 3/4 SDCLK cycle; hold time: 1/4 SDCLK cycle), sd20_cmdo (trigged by crc_clk rising edge)"]
pub type SdioDdrCmdPushPointR = crate::BitReader;
#[doc = "Field `sdio_ddr_dat_push_point` reader - SDIO DDR DAT Push Point 0: set-up time: 1/4 SDCLK cycle; hold time: 1/4 SDCLK cycle, sd20_dato (trigged by crc_clk rising edge) 1: set-up time: 1/2 SDCLK cycle; hold time: 0 SDCLK cycle, sd30_dato (trigged by crc_clk falling edge)"]
pub type SdioDdrDatPushPointR = crate::BitReader;
#[doc = "Field `sdio_ddr_push_type` reader - SDIO DDR Push Type"]
pub type SdioDdrPushTypeR = crate::BitReader;
#[doc = "Field `sdio_ddr_dig_solution` reader - SDIO DDR FW solution"]
pub type SdioDdrDigSolutionR = crate::BitReader;
#[doc = "Field `sdio_ddr_fw_solution` reader - SDIO DDR Digital solution"]
pub type SdioDdrFwSolutionR = crate::BitReader;
#[doc = "Field `sdio_sdclk_polar_swap` reader - SDIO SDCLK polarity swap"]
pub type SdioSdclkPolarSwapR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SDIO SD20 Push Point 0: output at falling edge of SDCLK(set-up time: 1/2 SDCLK cycle; hold time: 1/2 SDCLK cycle), sd20_cmdo/sd20_dato (trigged by crc_clk rising edge) 1: output is ahead by 1/4 SDCLK period(set-up time: 3/4 SDCLK cycle; hold time: 1/4 SDCLK cycle), sd30_cmdo/sd30_dato (trigged by crc_clk falling edge)"]
    #[inline(always)]
    pub fn sdio_sd20_push_point(&self) -> SdioSd20PushPointR {
        SdioSd20PushPointR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDIO DDR CMD Push Point 0: output at falling edge of SDCLK(set-up time: 1/2 SDCLK cycle; hold time: 1/2 SDCLK cycle), sd30_cmdo (trigged by crc_clk falling edge) 1: output is ahead by 1/4 SDCLK period(set-up time: 3/4 SDCLK cycle; hold time: 1/4 SDCLK cycle), sd20_cmdo (trigged by crc_clk rising edge)"]
    #[inline(always)]
    pub fn sdio_ddr_cmd_push_point(&self) -> SdioDdrCmdPushPointR {
        SdioDdrCmdPushPointR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO DDR DAT Push Point 0: set-up time: 1/4 SDCLK cycle; hold time: 1/4 SDCLK cycle, sd20_dato (trigged by crc_clk rising edge) 1: set-up time: 1/2 SDCLK cycle; hold time: 0 SDCLK cycle, sd30_dato (trigged by crc_clk falling edge)"]
    #[inline(always)]
    pub fn sdio_ddr_dat_push_point(&self) -> SdioDdrDatPushPointR {
        SdioDdrDatPushPointR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SDIO DDR Push Type"]
    #[inline(always)]
    pub fn sdio_ddr_push_type(&self) -> SdioDdrPushTypeR {
        SdioDdrPushTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDIO DDR FW solution"]
    #[inline(always)]
    pub fn sdio_ddr_dig_solution(&self) -> SdioDdrDigSolutionR {
        SdioDdrDigSolutionR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SDIO DDR Digital solution"]
    #[inline(always)]
    pub fn sdio_ddr_fw_solution(&self) -> SdioDdrFwSolutionR {
        SdioDdrFwSolutionR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SDIO SDCLK polarity swap"]
    #[inline(always)]
    pub fn sdio_sdclk_polar_swap(&self) -> SdioSdclkPolarSwapR {
        SdioSdclkPolarSwapR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tuning_solution::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TuningSolutionSpec;
impl crate::RegisterSpec for TuningSolutionSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tuning_solution::R`](R) reader structure"]
impl crate::Readable for TuningSolutionSpec {}
#[doc = "`reset()` method sets TUNING_SOLUTION to value 0x0a"]
impl crate::Resettable for TuningSolutionSpec {
    const RESET_VALUE: u8 = 0x0a;
}
