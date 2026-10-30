#[doc = "Register `Ring_Sample_CLK_CTRL` reader"]
pub type R = crate::R<RingSampleClkCtrlSpec>;
#[doc = "Register `Ring_Sample_CLK_CTRL` writer"]
pub type W = crate::W<RingSampleClkCtrlSpec>;
#[doc = "Field `CLOCK_SAMPLE_RATE_CONTROL` reader - For fine-tune rocs sample clock ROSC sample clock frequency = TRNG APB clock frequency/(Clock sample rate control + 1)"]
pub type ClockSampleRateControlR = crate::FieldReader;
#[doc = "Field `CLOCK_SAMPLE_RATE_CONTROL` writer - For fine-tune rocs sample clock ROSC sample clock frequency = TRNG APB clock frequency/(Clock sample rate control + 1)"]
pub type ClockSampleRateControlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11 - For fine-tune rocs sample clock ROSC sample clock frequency = TRNG APB clock frequency/(Clock sample rate control + 1)"]
    #[inline(always)]
    pub fn clock_sample_rate_control(&self) -> ClockSampleRateControlR {
        ClockSampleRateControlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - For fine-tune rocs sample clock ROSC sample clock frequency = TRNG APB clock frequency/(Clock sample rate control + 1)"]
    #[inline(always)]
    pub fn clock_sample_rate_control(
        &mut self,
    ) -> ClockSampleRateControlW<'_, RingSampleClkCtrlSpec> {
        ClockSampleRateControlW::new(self, 8)
    }
}
#[doc = "Sample clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ring_sample_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ring_sample_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingSampleClkCtrlSpec;
impl crate::RegisterSpec for RingSampleClkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ring_sample_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for RingSampleClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ring_sample_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for RingSampleClkCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Ring_Sample_CLK_CTRL to value 0"]
impl crate::Resettable for RingSampleClkCtrlSpec {}
