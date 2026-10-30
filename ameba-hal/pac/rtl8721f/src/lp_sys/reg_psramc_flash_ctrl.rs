#[doc = "Register `REG_PSRAMC_FLASH_CTRL` reader"]
pub type R = crate::R<RegPsramcFlashCtrlSpec>;
#[doc = "Register `REG_PSRAMC_FLASH_CTRL` writer"]
pub type W = crate::W<RegPsramcFlashCtrlSpec>;
#[doc = "Field `flash_pdex_req` reader - write 1 to trigger flash_spic excuting pdex_cmd, auto clear 0 when pdex_cmd is done"]
pub type FlashPdexReqR = crate::BitReader;
#[doc = "Field `flash_pdex_req` writer - write 1 to trigger flash_spic excuting pdex_cmd, auto clear 0 when pdex_cmd is done"]
pub type FlashPdexReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `flash_pdex_cmd` reader - flash_spic excute this cmd when receiving pdex_req 1: enter low power state 0: exit low power state"]
pub type FlashPdexCmdR = crate::BitReader;
#[doc = "Field `flash_pdex_cmd` writer - flash_spic excute this cmd when receiving pdex_req 1: enter low power state 0: exit low power state"]
pub type FlashPdexCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAMC_FLASH_CAL_EN` reader - Flash calibration enable 1: delay flash sout for calibration 0: bypass flash sout to spic"]
pub type PsramcFlashCalEnR = crate::BitReader;
#[doc = "Field `PSRAMC_FLASH_CAL_EN` writer - Flash calibration enable 1: delay flash sout for calibration 0: bypass flash sout to spic"]
pub type PsramcFlashCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAMC_DPHY_EN` reader - when set, enable spic_dphy function to read data from flash"]
pub type PsramcDphyEnR = crate::BitReader;
#[doc = "Field `PSRAMC_DPHY_EN` writer - when set, enable spic_dphy function to read data from flash"]
pub type PsramcDphyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAMC_DPHY_RD_DELAY_CNT` reader - spic_dphy output dealy cnt, when set < 3, output data will occur at fourth or fifth axi clock from the rising edge of spi_rx_data_en; when set >= 3, data will occur after DPHY_RD_DELAY_CNT + 2 axi clock from the rising edge of spi_rx_data_en"]
pub type PsramcDphyRdDelayCntR = crate::FieldReader;
#[doc = "Field `PSRAMC_DPHY_RD_DELAY_CNT` writer - spic_dphy output dealy cnt, when set < 3, output data will occur at fourth or fifth axi clock from the rising edge of spi_rx_data_en; when set >= 3, data will occur after DPHY_RD_DELAY_CNT + 2 axi clock from the rising edge of spi_rx_data_en"]
pub type PsramcDphyRdDelayCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `psram_frqc_grp` reader - select psram auto freq change group"]
pub type PsramFrqcGrpR = crate::FieldReader;
#[doc = "Field `psram_frqc_grp` writer - select psram auto freq change group"]
pub type PsramFrqcGrpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `psram_frqc_done` reader - indicate psram auto freq change is done"]
pub type PsramFrqcDoneR = crate::BitReader;
#[doc = "Field `psram_frqc_en` reader - enable psram auto freq change control flow"]
pub type PsramFrqcEnR = crate::BitReader;
#[doc = "Field `psram_frqc_en` writer - enable psram auto freq change control flow"]
pub type PsramFrqcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `psram_pdex_req` reader - write 1 to trigger psram_spic excuting pdex_cmd, auto clear 0 when pdex_cmd is done"]
pub type PsramPdexReqR = crate::BitReader;
#[doc = "Field `psram_pdex_req` writer - write 1 to trigger psram_spic excuting pdex_cmd, auto clear 0 when pdex_cmd is done"]
pub type PsramPdexReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `psram_pdex_cmd` reader - psram_spic excute this cmd when receiving pdex_req 1: enter low power state 0: exit low power state"]
pub type PsramPdexCmdR = crate::BitReader;
#[doc = "Field `psram_pdex_cmd` writer - psram_spic excute this cmd when receiving pdex_req 1: enter low power state 0: exit low power state"]
pub type PsramPdexCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `psramc_flash_en` reader - use psram_spic as flash controller, disable psram_phy"]
pub type PsramcFlashEnR = crate::BitReader;
#[doc = "Field `psramc_flash_en` writer - use psram_spic as flash controller, disable psram_phy"]
pub type PsramcFlashEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `flash_feedback_loc` reader - 0: pinmux S0 -> flash_spic, pinmux S1 -> psram_spic(if psramc_flash_en) 1: pinmux S1 -> flash_spic, pinmux S0 -> psram_spic(if psramc_flash_en)"]
pub type FlashFeedbackLocR = crate::BitReader;
#[doc = "Field `flash_feedback_loc` writer - 0: pinmux S0 -> flash_spic, pinmux S1 -> psram_spic(if psramc_flash_en) 1: pinmux S1 -> flash_spic, pinmux S0 -> psram_spic(if psramc_flash_en)"]
pub type FlashFeedbackLocW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to trigger flash_spic excuting pdex_cmd, auto clear 0 when pdex_cmd is done"]
    #[inline(always)]
    pub fn flash_pdex_req(&self) -> FlashPdexReqR {
        FlashPdexReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - flash_spic excute this cmd when receiving pdex_req 1: enter low power state 0: exit low power state"]
    #[inline(always)]
    pub fn flash_pdex_cmd(&self) -> FlashPdexCmdR {
        FlashPdexCmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash calibration enable 1: delay flash sout for calibration 0: bypass flash sout to spic"]
    #[inline(always)]
    pub fn psramc_flash_cal_en(&self) -> PsramcFlashCalEnR {
        PsramcFlashCalEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - when set, enable spic_dphy function to read data from flash"]
    #[inline(always)]
    pub fn psramc_dphy_en(&self) -> PsramcDphyEnR {
        PsramcDphyEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - spic_dphy output dealy cnt, when set < 3, output data will occur at fourth or fifth axi clock from the rising edge of spi_rx_data_en; when set >= 3, data will occur after DPHY_RD_DELAY_CNT + 2 axi clock from the rising edge of spi_rx_data_en"]
    #[inline(always)]
    pub fn psramc_dphy_rd_delay_cnt(&self) -> PsramcDphyRdDelayCntR {
        PsramcDphyRdDelayCntR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - select psram auto freq change group"]
    #[inline(always)]
    pub fn psram_frqc_grp(&self) -> PsramFrqcGrpR {
        PsramFrqcGrpR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - indicate psram auto freq change is done"]
    #[inline(always)]
    pub fn psram_frqc_done(&self) -> PsramFrqcDoneR {
        PsramFrqcDoneR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable psram auto freq change control flow"]
    #[inline(always)]
    pub fn psram_frqc_en(&self) -> PsramFrqcEnR {
        PsramFrqcEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - write 1 to trigger psram_spic excuting pdex_cmd, auto clear 0 when pdex_cmd is done"]
    #[inline(always)]
    pub fn psram_pdex_req(&self) -> PsramPdexReqR {
        PsramPdexReqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - psram_spic excute this cmd when receiving pdex_req 1: enter low power state 0: exit low power state"]
    #[inline(always)]
    pub fn psram_pdex_cmd(&self) -> PsramPdexCmdR {
        PsramPdexCmdR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - use psram_spic as flash controller, disable psram_phy"]
    #[inline(always)]
    pub fn psramc_flash_en(&self) -> PsramcFlashEnR {
        PsramcFlashEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: pinmux S0 -> flash_spic, pinmux S1 -> psram_spic(if psramc_flash_en) 1: pinmux S1 -> flash_spic, pinmux S0 -> psram_spic(if psramc_flash_en)"]
    #[inline(always)]
    pub fn flash_feedback_loc(&self) -> FlashFeedbackLocR {
        FlashFeedbackLocR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to trigger flash_spic excuting pdex_cmd, auto clear 0 when pdex_cmd is done"]
    #[inline(always)]
    pub fn flash_pdex_req(&mut self) -> FlashPdexReqW<'_, RegPsramcFlashCtrlSpec> {
        FlashPdexReqW::new(self, 0)
    }
    #[doc = "Bit 1 - flash_spic excute this cmd when receiving pdex_req 1: enter low power state 0: exit low power state"]
    #[inline(always)]
    pub fn flash_pdex_cmd(&mut self) -> FlashPdexCmdW<'_, RegPsramcFlashCtrlSpec> {
        FlashPdexCmdW::new(self, 1)
    }
    #[doc = "Bit 15 - Flash calibration enable 1: delay flash sout for calibration 0: bypass flash sout to spic"]
    #[inline(always)]
    pub fn psramc_flash_cal_en(&mut self) -> PsramcFlashCalEnW<'_, RegPsramcFlashCtrlSpec> {
        PsramcFlashCalEnW::new(self, 15)
    }
    #[doc = "Bit 16 - when set, enable spic_dphy function to read data from flash"]
    #[inline(always)]
    pub fn psramc_dphy_en(&mut self) -> PsramcDphyEnW<'_, RegPsramcFlashCtrlSpec> {
        PsramcDphyEnW::new(self, 16)
    }
    #[doc = "Bits 17:20 - spic_dphy output dealy cnt, when set < 3, output data will occur at fourth or fifth axi clock from the rising edge of spi_rx_data_en; when set >= 3, data will occur after DPHY_RD_DELAY_CNT + 2 axi clock from the rising edge of spi_rx_data_en"]
    #[inline(always)]
    pub fn psramc_dphy_rd_delay_cnt(
        &mut self,
    ) -> PsramcDphyRdDelayCntW<'_, RegPsramcFlashCtrlSpec> {
        PsramcDphyRdDelayCntW::new(self, 17)
    }
    #[doc = "Bits 24:25 - select psram auto freq change group"]
    #[inline(always)]
    pub fn psram_frqc_grp(&mut self) -> PsramFrqcGrpW<'_, RegPsramcFlashCtrlSpec> {
        PsramFrqcGrpW::new(self, 24)
    }
    #[doc = "Bit 27 - enable psram auto freq change control flow"]
    #[inline(always)]
    pub fn psram_frqc_en(&mut self) -> PsramFrqcEnW<'_, RegPsramcFlashCtrlSpec> {
        PsramFrqcEnW::new(self, 27)
    }
    #[doc = "Bit 28 - write 1 to trigger psram_spic excuting pdex_cmd, auto clear 0 when pdex_cmd is done"]
    #[inline(always)]
    pub fn psram_pdex_req(&mut self) -> PsramPdexReqW<'_, RegPsramcFlashCtrlSpec> {
        PsramPdexReqW::new(self, 28)
    }
    #[doc = "Bit 29 - psram_spic excute this cmd when receiving pdex_req 1: enter low power state 0: exit low power state"]
    #[inline(always)]
    pub fn psram_pdex_cmd(&mut self) -> PsramPdexCmdW<'_, RegPsramcFlashCtrlSpec> {
        PsramPdexCmdW::new(self, 29)
    }
    #[doc = "Bit 30 - use psram_spic as flash controller, disable psram_phy"]
    #[inline(always)]
    pub fn psramc_flash_en(&mut self) -> PsramcFlashEnW<'_, RegPsramcFlashCtrlSpec> {
        PsramcFlashEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 0: pinmux S0 -> flash_spic, pinmux S1 -> psram_spic(if psramc_flash_en) 1: pinmux S1 -> flash_spic, pinmux S0 -> psram_spic(if psramc_flash_en)"]
    #[inline(always)]
    pub fn flash_feedback_loc(&mut self) -> FlashFeedbackLocW<'_, RegPsramcFlashCtrlSpec> {
        FlashFeedbackLocW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_psramc_flash_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_psramc_flash_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsramcFlashCtrlSpec;
impl crate::RegisterSpec for RegPsramcFlashCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_psramc_flash_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPsramcFlashCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_psramc_flash_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPsramcFlashCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PSRAMC_FLASH_CTRL to value 0"]
impl crate::Resettable for RegPsramcFlashCtrlSpec {}
