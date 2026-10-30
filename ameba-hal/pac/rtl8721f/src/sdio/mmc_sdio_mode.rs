#[doc = "Register `MMC_SDIO_MODE` reader"]
pub type R = crate::R<MmcSdioModeSpec>;
#[doc = "Register `MMC_SDIO_MODE` writer"]
pub type W = crate::W<MmcSdioModeSpec>;
#[doc = "Field `use_tx_tune` reader - Tuning"]
pub type UseTxTuneR = crate::BitReader;
#[doc = "Field `use_tx_tune` writer - Tuning"]
pub type UseTxTuneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `use_ddr_tune` reader - Use DDR Tuning"]
pub type UseDdrTuneR = crate::BitReader;
#[doc = "Field `use_ddr_tune` writer - Use DDR Tuning"]
pub type UseDdrTuneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_support` reader - SDIO Support"]
pub type SdioSupportR = crate::BitReader;
#[doc = "Field `mmc_mode` reader - MMC Mode"]
pub type MmcModeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Tuning"]
    #[inline(always)]
    pub fn use_tx_tune(&self) -> UseTxTuneR {
        UseTxTuneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use DDR Tuning"]
    #[inline(always)]
    pub fn use_ddr_tune(&self) -> UseDdrTuneR {
        UseDdrTuneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDIO Support"]
    #[inline(always)]
    pub fn sdio_support(&self) -> SdioSupportR {
        SdioSupportR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Mode"]
    #[inline(always)]
    pub fn mmc_mode(&self) -> MmcModeR {
        MmcModeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tuning"]
    #[inline(always)]
    pub fn use_tx_tune(&mut self) -> UseTxTuneW<'_, MmcSdioModeSpec> {
        UseTxTuneW::new(self, 0)
    }
    #[doc = "Bit 1 - Use DDR Tuning"]
    #[inline(always)]
    pub fn use_ddr_tune(&mut self) -> UseDdrTuneW<'_, MmcSdioModeSpec> {
        UseDdrTuneW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_sdio_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_sdio_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcSdioModeSpec;
impl crate::RegisterSpec for MmcSdioModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mmc_sdio_mode::R`](R) reader structure"]
impl crate::Readable for MmcSdioModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_sdio_mode::W`](W) writer structure"]
impl crate::Writable for MmcSdioModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMC_SDIO_MODE to value 0x01"]
impl crate::Resettable for MmcSdioModeSpec {
    const RESET_VALUE: u8 = 0x01;
}
