#[doc = "Register `LCDC_DMA_MODE_CFG` reader"]
pub type R = crate::R<LcdcDmaModeCfgSpec>;
#[doc = "Register `LCDC_DMA_MODE_CFG` writer"]
pub type W = crate::W<LcdcDmaModeCfgSpec>;
#[doc = "Field `dma_triger_mode` reader - DMA Trigger-mode enable. *0: Auto-mode, open, DMA refreshes automatically based on synchronous signal. *1: Trigger-mode open, DMA refreshes frame manually based on TRIGER_ONETIME bit of this register. Trigger mode is useful for LCD with internal GRAM."]
pub type DmaTrigerModeR = crate::BitReader;
#[doc = "Field `dma_triger_mode` writer - DMA Trigger-mode enable. *0: Auto-mode, open, DMA refreshes automatically based on synchronous signal. *1: Trigger-mode open, DMA refreshes frame manually based on TRIGER_ONETIME bit of this register. Trigger mode is useful for LCD with internal GRAM."]
pub type DmaTrigerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `triger_onetime` reader - Write '1', hardware DMA one frame from DMA buffer based on synchronous signal. After DMA completed,this bit is cleared automatically."]
pub type TrigerOnetimeR = crate::BitReader;
#[doc = "Field `triger_onetime` writer - Write '1', hardware DMA one frame from DMA buffer based on synchronous signal. After DMA completed,this bit is cleared automatically."]
pub type TrigerOnetimeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_otsd` reader - The value of read outstanding -1, for DMA burst size configuration"]
pub type RdOtsdR = crate::FieldReader;
#[doc = "Field `rd_otsd` writer - The value of read outstanding -1, for DMA burst size configuration"]
pub type RdOtsdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dma_te_mode` reader - 1: TE trigger-mode open, DMA refreshes frame manually based on panel TE input 0: use dma_triger_mode(bit 0 setting, auto or onetime)"]
pub type DmaTeModeR = crate::BitReader;
#[doc = "Field `dma_te_mode` writer - 1: TE trigger-mode open, DMA refreshes frame manually based on panel TE input 0: use dma_triger_mode(bit 0 setting, auto or onetime)"]
pub type DmaTeModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_fetched_thld` reader - threshold value for dma data_fetched indicator(start panelif FSM)"]
pub type DmaFetchedThldR = crate::FieldReader;
#[doc = "Field `dma_fetched_thld` writer - threshold value for dma data_fetched indicator(start panelif FSM)"]
pub type DmaFetchedThldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `dma_udf_timeout` reader - threshold value for dma underflow timeout counter"]
pub type DmaUdfTimeoutR = crate::FieldReader<u16>;
#[doc = "Field `dma_udf_timeout` writer - threshold value for dma underflow timeout counter"]
pub type DmaUdfTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `pif_hs_eol` reader - threshold value to check if panelif hs_cnt almost reaching end of line"]
pub type PifHsEolR = crate::FieldReader;
#[doc = "Field `pif_hs_eol` writer - threshold value to check if panelif hs_cnt almost reaching end of line"]
pub type PifHsEolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DMA Trigger-mode enable. *0: Auto-mode, open, DMA refreshes automatically based on synchronous signal. *1: Trigger-mode open, DMA refreshes frame manually based on TRIGER_ONETIME bit of this register. Trigger mode is useful for LCD with internal GRAM."]
    #[inline(always)]
    pub fn dma_triger_mode(&self) -> DmaTrigerModeR {
        DmaTrigerModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1', hardware DMA one frame from DMA buffer based on synchronous signal. After DMA completed,this bit is cleared automatically."]
    #[inline(always)]
    pub fn triger_onetime(&self) -> TrigerOnetimeR {
        TrigerOnetimeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - The value of read outstanding -1, for DMA burst size configuration"]
    #[inline(always)]
    pub fn rd_otsd(&self) -> RdOtsdR {
        RdOtsdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 1: TE trigger-mode open, DMA refreshes frame manually based on panel TE input 0: use dma_triger_mode(bit 0 setting, auto or onetime)"]
    #[inline(always)]
    pub fn dma_te_mode(&self) -> DmaTeModeR {
        DmaTeModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - threshold value for dma data_fetched indicator(start panelif FSM)"]
    #[inline(always)]
    pub fn dma_fetched_thld(&self) -> DmaFetchedThldR {
        DmaFetchedThldR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:27 - threshold value for dma underflow timeout counter"]
    #[inline(always)]
    pub fn dma_udf_timeout(&self) -> DmaUdfTimeoutR {
        DmaUdfTimeoutR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - threshold value to check if panelif hs_cnt almost reaching end of line"]
    #[inline(always)]
    pub fn pif_hs_eol(&self) -> PifHsEolR {
        PifHsEolR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Trigger-mode enable. *0: Auto-mode, open, DMA refreshes automatically based on synchronous signal. *1: Trigger-mode open, DMA refreshes frame manually based on TRIGER_ONETIME bit of this register. Trigger mode is useful for LCD with internal GRAM."]
    #[inline(always)]
    pub fn dma_triger_mode(&mut self) -> DmaTrigerModeW<'_, LcdcDmaModeCfgSpec> {
        DmaTrigerModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1', hardware DMA one frame from DMA buffer based on synchronous signal. After DMA completed,this bit is cleared automatically."]
    #[inline(always)]
    pub fn triger_onetime(&mut self) -> TrigerOnetimeW<'_, LcdcDmaModeCfgSpec> {
        TrigerOnetimeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - The value of read outstanding -1, for DMA burst size configuration"]
    #[inline(always)]
    pub fn rd_otsd(&mut self) -> RdOtsdW<'_, LcdcDmaModeCfgSpec> {
        RdOtsdW::new(self, 2)
    }
    #[doc = "Bit 4 - 1: TE trigger-mode open, DMA refreshes frame manually based on panel TE input 0: use dma_triger_mode(bit 0 setting, auto or onetime)"]
    #[inline(always)]
    pub fn dma_te_mode(&mut self) -> DmaTeModeW<'_, LcdcDmaModeCfgSpec> {
        DmaTeModeW::new(self, 4)
    }
    #[doc = "Bits 8:15 - threshold value for dma data_fetched indicator(start panelif FSM)"]
    #[inline(always)]
    pub fn dma_fetched_thld(&mut self) -> DmaFetchedThldW<'_, LcdcDmaModeCfgSpec> {
        DmaFetchedThldW::new(self, 8)
    }
    #[doc = "Bits 16:27 - threshold value for dma underflow timeout counter"]
    #[inline(always)]
    pub fn dma_udf_timeout(&mut self) -> DmaUdfTimeoutW<'_, LcdcDmaModeCfgSpec> {
        DmaUdfTimeoutW::new(self, 16)
    }
    #[doc = "Bits 28:31 - threshold value to check if panelif hs_cnt almost reaching end of line"]
    #[inline(always)]
    pub fn pif_hs_eol(&mut self) -> PifHsEolW<'_, LcdcDmaModeCfgSpec> {
        PifHsEolW::new(self, 28)
    }
}
#[doc = "LCDC DMA mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_dma_mode_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_dma_mode_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcDmaModeCfgSpec;
impl crate::RegisterSpec for LcdcDmaModeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_dma_mode_cfg::R`](R) reader structure"]
impl crate::Readable for LcdcDmaModeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_dma_mode_cfg::W`](W) writer structure"]
impl crate::Writable for LcdcDmaModeCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_DMA_MODE_CFG to value 0xf400_000c"]
impl crate::Resettable for LcdcDmaModeCfgSpec {
    const RESET_VALUE: u32 = 0xf400_000c;
}
