#[doc = "Register `LCDC_IRQ_EN` reader"]
pub type R = crate::R<LcdcIrqEnSpec>;
#[doc = "Register `LCDC_IRQ_EN` writer"]
pub type W = crate::W<LcdcIrqEnSpec>;
#[doc = "Field `dma_un_inten` reader - DMA FIFO underflow interrupt enable"]
pub type DmaUnIntenR = crate::BitReader;
#[doc = "Field `dma_un_inten` writer - DMA FIFO underflow interrupt enable"]
pub type DmaUnIntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lcd_frd_inten` reader - LCD refresh frame done interrupt enable"]
pub type LcdFrdIntenR = crate::BitReader;
#[doc = "Field `lcd_frd_inten` writer - LCD refresh frame done interrupt enable"]
pub type LcdFrdIntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lcd_lin_inten` reader - Line Interrupt Enable. This bit can be set and cleared by software. * 0: Line interrupt disable * 1: Line Interrupt enable"]
pub type LcdLinIntenR = crate::BitReader;
#[doc = "Field `lcd_lin_inten` writer - Line Interrupt Enable. This bit can be set and cleared by software. * 0: Line interrupt disable * 1: Line Interrupt enable"]
pub type LcdLinIntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `io_timeout_inten` reader - Write or read timeout interrupt Enable in MCU I/F I/O mode. This bit can be set and cleared by software. * 0: MCU I/O mode write/read timeout interrupt disable * 1: MCU I/O mode write/read timeout interrupt enable"]
pub type IoTimeoutIntenR = crate::BitReader;
#[doc = "Field `io_timeout_inten` writer - Write or read timeout interrupt Enable in MCU I/F I/O mode. This bit can be set and cleared by software. * 0: MCU I/O mode write/read timeout interrupt disable * 1: MCU I/O mode write/read timeout interrupt enable"]
pub type IoTimeoutIntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frm_start_inten` reader - DMA Frame start interrupt Enable. This bit can be set and cleared by software. * 0: DMA Frame start interrupt disable * 1: DMA Frame start interrupt enable"]
pub type FrmStartIntenR = crate::BitReader;
#[doc = "Field `frm_start_inten` writer - DMA Frame start interrupt Enable. This bit can be set and cleared by software. * 0: DMA Frame start interrupt disable * 1: DMA Frame start interrupt enable"]
pub type FrmStartIntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `panel_te_inten` reader - panel TE input interrupt Enable"]
pub type PanelTeIntenR = crate::BitReader;
#[doc = "Field `panel_te_inten` writer - panel TE input interrupt Enable"]
pub type PanelTeIntenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA FIFO underflow interrupt enable"]
    #[inline(always)]
    pub fn dma_un_inten(&self) -> DmaUnIntenR {
        DmaUnIntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - LCD refresh frame done interrupt enable"]
    #[inline(always)]
    pub fn lcd_frd_inten(&self) -> LcdFrdIntenR {
        LcdFrdIntenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line Interrupt Enable. This bit can be set and cleared by software. * 0: Line interrupt disable * 1: Line Interrupt enable"]
    #[inline(always)]
    pub fn lcd_lin_inten(&self) -> LcdLinIntenR {
        LcdLinIntenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write or read timeout interrupt Enable in MCU I/F I/O mode. This bit can be set and cleared by software. * 0: MCU I/O mode write/read timeout interrupt disable * 1: MCU I/O mode write/read timeout interrupt enable"]
    #[inline(always)]
    pub fn io_timeout_inten(&self) -> IoTimeoutIntenR {
        IoTimeoutIntenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Frame start interrupt Enable. This bit can be set and cleared by software. * 0: DMA Frame start interrupt disable * 1: DMA Frame start interrupt enable"]
    #[inline(always)]
    pub fn frm_start_inten(&self) -> FrmStartIntenR {
        FrmStartIntenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - panel TE input interrupt Enable"]
    #[inline(always)]
    pub fn panel_te_inten(&self) -> PanelTeIntenR {
        PanelTeIntenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA FIFO underflow interrupt enable"]
    #[inline(always)]
    pub fn dma_un_inten(&mut self) -> DmaUnIntenW<'_, LcdcIrqEnSpec> {
        DmaUnIntenW::new(self, 0)
    }
    #[doc = "Bit 2 - LCD refresh frame done interrupt enable"]
    #[inline(always)]
    pub fn lcd_frd_inten(&mut self) -> LcdFrdIntenW<'_, LcdcIrqEnSpec> {
        LcdFrdIntenW::new(self, 2)
    }
    #[doc = "Bit 3 - Line Interrupt Enable. This bit can be set and cleared by software. * 0: Line interrupt disable * 1: Line Interrupt enable"]
    #[inline(always)]
    pub fn lcd_lin_inten(&mut self) -> LcdLinIntenW<'_, LcdcIrqEnSpec> {
        LcdLinIntenW::new(self, 3)
    }
    #[doc = "Bit 4 - Write or read timeout interrupt Enable in MCU I/F I/O mode. This bit can be set and cleared by software. * 0: MCU I/O mode write/read timeout interrupt disable * 1: MCU I/O mode write/read timeout interrupt enable"]
    #[inline(always)]
    pub fn io_timeout_inten(&mut self) -> IoTimeoutIntenW<'_, LcdcIrqEnSpec> {
        IoTimeoutIntenW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Frame start interrupt Enable. This bit can be set and cleared by software. * 0: DMA Frame start interrupt disable * 1: DMA Frame start interrupt enable"]
    #[inline(always)]
    pub fn frm_start_inten(&mut self) -> FrmStartIntenW<'_, LcdcIrqEnSpec> {
        FrmStartIntenW::new(self, 5)
    }
    #[doc = "Bit 6 - panel TE input interrupt Enable"]
    #[inline(always)]
    pub fn panel_te_inten(&mut self) -> PanelTeIntenW<'_, LcdcIrqEnSpec> {
        PanelTeIntenW::new(self, 6)
    }
}
#[doc = "LCDC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_irq_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc_irq_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcIrqEnSpec;
impl crate::RegisterSpec for LcdcIrqEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_irq_en::R`](R) reader structure"]
impl crate::Readable for LcdcIrqEnSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdc_irq_en::W`](W) writer structure"]
impl crate::Writable for LcdcIrqEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDC_IRQ_EN to value 0"]
impl crate::Resettable for LcdcIrqEnSpec {}
