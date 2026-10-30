#[doc = "Register `REG_ADC_CLK_LCNT0_DIV` reader"]
pub type R = crate::R<RegAdcClkLcnt0DivSpec>;
#[doc = "Register `REG_ADC_CLK_LCNT0_DIV` writer"]
pub type W = crate::W<RegAdcClkLcnt0DivSpec>;
#[doc = "Field `bit_adc_clk_lcnt` reader - This field defines clock driver lower level of chn in ADC module.when source clock is 4M, the duration of low level need >=2 for timing req.(>=0.5us) lower level =bit_adc_clk_lcnt* ip_clk period"]
pub type BitAdcClkLcntR = crate::FieldReader;
#[doc = "Field `bit_adc_clk_lcnt` writer - This field defines clock driver lower level of chn in ADC module.when source clock is 4M, the duration of low level need >=2 for timing req.(>=0.5us) lower level =bit_adc_clk_lcnt* ip_clk period"]
pub type BitAdcClkLcntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - This field defines clock driver lower level of chn in ADC module.when source clock is 4M, the duration of low level need >=2 for timing req.(>=0.5us) lower level =bit_adc_clk_lcnt* ip_clk period"]
    #[inline(always)]
    pub fn bit_adc_clk_lcnt(&self) -> BitAdcClkLcntR {
        BitAdcClkLcntR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This field defines clock driver lower level of chn in ADC module.when source clock is 4M, the duration of low level need >=2 for timing req.(>=0.5us) lower level =bit_adc_clk_lcnt* ip_clk period"]
    #[inline(always)]
    pub fn bit_adc_clk_lcnt(&mut self) -> BitAdcClkLcntW<'_, RegAdcClkLcnt0DivSpec> {
        BitAdcClkLcntW::new(self, 0)
    }
}
#[doc = "ADC Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_clk_lcnt0_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_clk_lcnt0_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcClkLcnt0DivSpec;
impl crate::RegisterSpec for RegAdcClkLcnt0DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_clk_lcnt0_div::R`](R) reader structure"]
impl crate::Readable for RegAdcClkLcnt0DivSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_clk_lcnt0_div::W`](W) writer structure"]
impl crate::Writable for RegAdcClkLcnt0DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_CLK_LCNT0_DIV to value 0x02"]
impl crate::Resettable for RegAdcClkLcnt0DivSpec {
    const RESET_VALUE: u32 = 0x02;
}
