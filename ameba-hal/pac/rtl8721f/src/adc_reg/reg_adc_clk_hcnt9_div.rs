#[doc = "Register `REG_ADC_CLK_HCNT9_DIV` reader"]
pub type R = crate::R<RegAdcClkHcnt9DivSpec>;
#[doc = "Register `REG_ADC_CLK_HCNT9_DIV` writer"]
pub type W = crate::W<RegAdcClkHcnt9DivSpec>;
#[doc = "Field `bit_adc_clk_hcnt9` reader - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt9+1) * ip_clk period . hcnt11-13 is rsvd."]
pub type BitAdcClkHcnt9R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_clk_hcnt9` writer - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt9+1) * ip_clk period . hcnt11-13 is rsvd."]
pub type BitAdcClkHcnt9W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt9+1) * ip_clk period . hcnt11-13 is rsvd."]
    #[inline(always)]
    pub fn bit_adc_clk_hcnt9(&self) -> BitAdcClkHcnt9R {
        BitAdcClkHcnt9R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt9+1) * ip_clk period . hcnt11-13 is rsvd."]
    #[inline(always)]
    pub fn bit_adc_clk_hcnt9(&mut self) -> BitAdcClkHcnt9W<'_, RegAdcClkHcnt9DivSpec> {
        BitAdcClkHcnt9W::new(self, 0)
    }
}
#[doc = "ADC Channel Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_clk_hcnt9_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_clk_hcnt9_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcClkHcnt9DivSpec;
impl crate::RegisterSpec for RegAdcClkHcnt9DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_clk_hcnt9_div::R`](R) reader structure"]
impl crate::Readable for RegAdcClkHcnt9DivSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_clk_hcnt9_div::W`](W) writer structure"]
impl crate::Writable for RegAdcClkHcnt9DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_CLK_HCNT9_DIV to value 0x15"]
impl crate::Resettable for RegAdcClkHcnt9DivSpec {
    const RESET_VALUE: u32 = 0x15;
}
