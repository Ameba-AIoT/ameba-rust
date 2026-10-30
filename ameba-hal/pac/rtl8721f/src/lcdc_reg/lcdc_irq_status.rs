#[doc = "Register `LCDC_IRQ_STATUS` reader"]
pub type R = crate::R<LcdcIrqStatusSpec>;
#[doc = "Field `dma_un_ints` reader - DMA FIFO underflow interrupt status. Write 1 to clear it. * 0: No DMA FIFO underflow interrupt generated * 1: Interrupt generated when DMA FIFO underflow happens"]
pub type DmaUnIntsR = crate::BitReader;
#[doc = "Field `lcd_frd_ints` reader - LCD refresh frame done interrupt status. Write 1 to clear it. * 0: No LCD refresh frame done interrupt generated. * 1: This interrupt generated, when sending LCD frame done. Note: When this bit is set and DMA fetch the last pixel data from frame buffer, the After LCD refresh frame done, interrupt happens."]
pub type LcdFrdIntsR = crate::BitReader;
#[doc = "Field `lcd_lin_ints` reader - Line Interrupt status. Write 1 to clear it. * 0: No Line interrupt generated * 1: A Line interrupt is generated, when a programmed line is reached Note: When this bit is set and DMA fetches the first pixel data of the specified line number from frame buffer, the line interrupt happens. The line number depends on LCDC_LINE_INT_POS register value."]
pub type LcdLinIntsR = crate::BitReader;
#[doc = "Field `io_timeout_ints` reader - Write or read timeout interrupt status in MCU I/F I/O mode. Write 1 to clear it. * 0: No write/read timeout interrupt generated. * 1: A write/read timeout interrupt is generated."]
pub type IoTimeoutIntsR = crate::BitReader;
#[doc = "Field `frm_start_ints` reader - DMA frame start interrupt status. Write 1 to clear it. * 0: No DMA frame start interrupt generated * 1: A DMA frame start interrupt is generated"]
pub type FrmStartIntsR = crate::BitReader;
#[doc = "Field `panel_te_ints` reader - panel TE input interrupt status. Write 1 to clear it. * 0: No panel TE input interrupt generated * 1: A panel TE input interrupt is generated"]
pub type PanelTeIntsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA FIFO underflow interrupt status. Write 1 to clear it. * 0: No DMA FIFO underflow interrupt generated * 1: Interrupt generated when DMA FIFO underflow happens"]
    #[inline(always)]
    pub fn dma_un_ints(&self) -> DmaUnIntsR {
        DmaUnIntsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - LCD refresh frame done interrupt status. Write 1 to clear it. * 0: No LCD refresh frame done interrupt generated. * 1: This interrupt generated, when sending LCD frame done. Note: When this bit is set and DMA fetch the last pixel data from frame buffer, the After LCD refresh frame done, interrupt happens."]
    #[inline(always)]
    pub fn lcd_frd_ints(&self) -> LcdFrdIntsR {
        LcdFrdIntsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line Interrupt status. Write 1 to clear it. * 0: No Line interrupt generated * 1: A Line interrupt is generated, when a programmed line is reached Note: When this bit is set and DMA fetches the first pixel data of the specified line number from frame buffer, the line interrupt happens. The line number depends on LCDC_LINE_INT_POS register value."]
    #[inline(always)]
    pub fn lcd_lin_ints(&self) -> LcdLinIntsR {
        LcdLinIntsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write or read timeout interrupt status in MCU I/F I/O mode. Write 1 to clear it. * 0: No write/read timeout interrupt generated. * 1: A write/read timeout interrupt is generated."]
    #[inline(always)]
    pub fn io_timeout_ints(&self) -> IoTimeoutIntsR {
        IoTimeoutIntsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA frame start interrupt status. Write 1 to clear it. * 0: No DMA frame start interrupt generated * 1: A DMA frame start interrupt is generated"]
    #[inline(always)]
    pub fn frm_start_ints(&self) -> FrmStartIntsR {
        FrmStartIntsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - panel TE input interrupt status. Write 1 to clear it. * 0: No panel TE input interrupt generated * 1: A panel TE input interrupt is generated"]
    #[inline(always)]
    pub fn panel_te_ints(&self) -> PanelTeIntsR {
        PanelTeIntsR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "LCDC interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_irq_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcIrqStatusSpec;
impl crate::RegisterSpec for LcdcIrqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_irq_status::R`](R) reader structure"]
impl crate::Readable for LcdcIrqStatusSpec {}
#[doc = "`reset()` method sets LCDC_IRQ_STATUS to value 0"]
impl crate::Resettable for LcdcIrqStatusSpec {}
