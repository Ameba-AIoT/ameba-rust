#[doc = "Register `REG_ADC_CLK_HCNT0_DIV` reader"]
pub type R = crate::R<RegAdcClkHcnt0DivSpec>;
#[doc = "Register `REG_ADC_CLK_HCNT0_DIV` writer"]
pub type W = crate::W<RegAdcClkHcnt0DivSpec>;
#[doc = "Field `bit_adc_clk_hcnt0` reader - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt0+1) * ip_clk period . hcnt11-13 is rsvd."]
pub type BitAdcClkHcnt0R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_clk_hcnt0` writer - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt0+1) * ip_clk period . hcnt11-13 is rsvd."]
pub type BitAdcClkHcnt0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt0+1) * ip_clk period . hcnt11-13 is rsvd."]
    #[inline(always)]
    pub fn bit_adc_clk_hcnt0(&self) -> BitAdcClkHcnt0R {
        BitAdcClkHcnt0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt0+1) * ip_clk period . hcnt11-13 is rsvd."]
    #[inline(always)]
    pub fn bit_adc_clk_hcnt0(&mut self) -> BitAdcClkHcnt0W<'_, RegAdcClkHcnt0DivSpec> {
        BitAdcClkHcnt0W::new(self, 0)
    }
}
#[doc = "ADC Channel Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_clk_hcnt0_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_clk_hcnt0_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcClkHcnt0DivSpec;
impl crate::RegisterSpec for RegAdcClkHcnt0DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_clk_hcnt0_div::R`](R) reader structure"]
impl crate::Readable for RegAdcClkHcnt0DivSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_clk_hcnt0_div::W`](W) writer structure"]
impl crate::Writable for RegAdcClkHcnt0DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_CLK_HCNT0_DIV to value 0x15"]
impl crate::Resettable for RegAdcClkHcnt0DivSpec {
    const RESET_VALUE: u32 = 0x15;
}
