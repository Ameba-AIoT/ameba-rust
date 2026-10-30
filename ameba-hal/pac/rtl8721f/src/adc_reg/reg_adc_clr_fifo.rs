#[doc = "Register `REG_ADC_CLR_FIFO` reader"]
pub type R = crate::R<RegAdcClrFifoSpec>;
#[doc = "Register `REG_ADC_CLR_FIFO` writer"]
pub type W = crate::W<RegAdcClrFifoSpec>;
#[doc = "Field `bit_adc_clr_fifo` reader - This bit is used to clear FIFO contents and makes it to the default status. Every time this bit is set to 1. FIFO would be cleared to the default status. Software should set this bit to 0 after FIFO is cleared. 0: No effect 1: Clear FIFO"]
pub type BitAdcClrFifoR = crate::BitReader;
#[doc = "Field `bit_adc_clr_fifo` writer - This bit is used to clear FIFO contents and makes it to the default status. Every time this bit is set to 1. FIFO would be cleared to the default status. Software should set this bit to 0 after FIFO is cleared. 0: No effect 1: Clear FIFO"]
pub type BitAdcClrFifoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to clear FIFO contents and makes it to the default status. Every time this bit is set to 1. FIFO would be cleared to the default status. Software should set this bit to 0 after FIFO is cleared. 0: No effect 1: Clear FIFO"]
    #[inline(always)]
    pub fn bit_adc_clr_fifo(&self) -> BitAdcClrFifoR {
        BitAdcClrFifoR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to clear FIFO contents and makes it to the default status. Every time this bit is set to 1. FIFO would be cleared to the default status. Software should set this bit to 0 after FIFO is cleared. 0: No effect 1: Clear FIFO"]
    #[inline(always)]
    pub fn bit_adc_clr_fifo(&mut self) -> BitAdcClrFifoW<'_, RegAdcClrFifoSpec> {
        BitAdcClrFifoW::new(self, 0)
    }
}
#[doc = "ADC Clear FIFO Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_clr_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_clr_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcClrFifoSpec;
impl crate::RegisterSpec for RegAdcClrFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_clr_fifo::R`](R) reader structure"]
impl crate::Readable for RegAdcClrFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_clr_fifo::W`](W) writer structure"]
impl crate::Writable for RegAdcClrFifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_CLR_FIFO to value 0"]
impl crate::Resettable for RegAdcClrFifoSpec {}
