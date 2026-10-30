#[doc = "Register `LCDC_IRQ_RAW` reader"]
pub type R = crate::R<LcdcIrqRawSpec>;
#[doc = "Field `dma_un_intrs` reader - DMA FIFO underflow raw interrupt status. When LCDC DMA FIFO under flow event happens, this bit is set. Write 1 to the DMAUNINTS field in register LCDC_IRQ_STATUS to clear this bit."]
pub type DmaUnIntrsR = crate::BitReader;
#[doc = "Field `lcd_frd_intrs` reader - LCD refresh frame done raw interrupt status. No matter the refresh done interrupt is enabled or disabled, this bit is set when refresh done interrupt happens. Writting 1 to the LCD_FRD_INTS field in register LCDC_IRQ_STATUS is to clear this bit."]
pub type LcdFrdIntrsR = crate::BitReader;
#[doc = "Field `lcd_lin_intrs` reader - Line Interrupt raw status. No matter the line interrupt is enabled or disabled, this bit is set when line interrupt happens. Writting 1 to the LCD_LIN_INTS field in register LCDC_IRQ_STATUS is to clear this bit."]
pub type LcdLinIntrsR = crate::BitReader;
#[doc = "Field `io_timeout_intrs` reader - Write or read timeout interrupt raw interrupt status in MCU I/F I/O mode. No matter the I/O write/read timeout interrupt is enabled or disabled, this bit is set when I/O write/read timeout interrupt happens. Write 1 to the IO_TIMEOUT_INTS field in register LCDC_IRQ_STATUS to clear this bit."]
pub type IoTimeoutIntrsR = crate::BitReader;
#[doc = "Field `frm_start_intrs` reader - DMA frame start raw interrupt status. No matter DMA frame start interrupt is enabled or disabled, this bit is set when DMA frame start interrupt happens. Writting 1 to the FRM_START_INTS field in register LCDC_IRQ_STATUS is to clear this bit."]
pub type FrmStartIntrsR = crate::BitReader;
#[doc = "Field `panel_te_intrs` reader - panel TE input raw interrupt status. No matter panel TE input interrupt is enabled or disabled, this bit is set when panel TE input interrupt happens. Writting 1 to the PANEL_TE_INTS field in register LCDC_IRQ_STATUS is to clear this bit."]
pub type PanelTeIntrsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA FIFO underflow raw interrupt status. When LCDC DMA FIFO under flow event happens, this bit is set. Write 1 to the DMAUNINTS field in register LCDC_IRQ_STATUS to clear this bit."]
    #[inline(always)]
    pub fn dma_un_intrs(&self) -> DmaUnIntrsR {
        DmaUnIntrsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - LCD refresh frame done raw interrupt status. No matter the refresh done interrupt is enabled or disabled, this bit is set when refresh done interrupt happens. Writting 1 to the LCD_FRD_INTS field in register LCDC_IRQ_STATUS is to clear this bit."]
    #[inline(always)]
    pub fn lcd_frd_intrs(&self) -> LcdFrdIntrsR {
        LcdFrdIntrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line Interrupt raw status. No matter the line interrupt is enabled or disabled, this bit is set when line interrupt happens. Writting 1 to the LCD_LIN_INTS field in register LCDC_IRQ_STATUS is to clear this bit."]
    #[inline(always)]
    pub fn lcd_lin_intrs(&self) -> LcdLinIntrsR {
        LcdLinIntrsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write or read timeout interrupt raw interrupt status in MCU I/F I/O mode. No matter the I/O write/read timeout interrupt is enabled or disabled, this bit is set when I/O write/read timeout interrupt happens. Write 1 to the IO_TIMEOUT_INTS field in register LCDC_IRQ_STATUS to clear this bit."]
    #[inline(always)]
    pub fn io_timeout_intrs(&self) -> IoTimeoutIntrsR {
        IoTimeoutIntrsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA frame start raw interrupt status. No matter DMA frame start interrupt is enabled or disabled, this bit is set when DMA frame start interrupt happens. Writting 1 to the FRM_START_INTS field in register LCDC_IRQ_STATUS is to clear this bit."]
    #[inline(always)]
    pub fn frm_start_intrs(&self) -> FrmStartIntrsR {
        FrmStartIntrsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - panel TE input raw interrupt status. No matter panel TE input interrupt is enabled or disabled, this bit is set when panel TE input interrupt happens. Writting 1 to the PANEL_TE_INTS field in register LCDC_IRQ_STATUS is to clear this bit."]
    #[inline(always)]
    pub fn panel_te_intrs(&self) -> PanelTeIntrsR {
        PanelTeIntrsR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "LCDC raw interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_irq_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcIrqRawSpec;
impl crate::RegisterSpec for LcdcIrqRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_irq_raw::R`](R) reader structure"]
impl crate::Readable for LcdcIrqRawSpec {}
#[doc = "`reset()` method sets LCDC_IRQ_RAW to value 0"]
impl crate::Resettable for LcdcIrqRawSpec {}
