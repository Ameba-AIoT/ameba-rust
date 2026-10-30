#[doc = "Register `REG_ADC_CLK_HCNT3_DIV` reader"]
pub type R = crate::R<RegAdcClkHcnt3DivSpec>;
#[doc = "Register `REG_ADC_CLK_HCNT3_DIV` writer"]
pub type W = crate::W<RegAdcClkHcnt3DivSpec>;
#[doc = "Field `bit_adc_clk_hcnt3` reader - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt3+1) * ip_clk period . hcnt11-13 is rsvd."]
pub type BitAdcClkHcnt3R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_clk_hcnt3` writer - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt3+1) * ip_clk period . hcnt11-13 is rsvd."]
pub type BitAdcClkHcnt3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt3+1) * ip_clk period . hcnt11-13 is rsvd."]
    #[inline(always)]
    pub fn bit_adc_clk_hcnt3(&self) -> BitAdcClkHcnt3R {
        BitAdcClkHcnt3R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field defines clock driver high level of chn in ADC module. high level =( bit_adc_clk_hcnt3+1) * ip_clk period . hcnt11-13 is rsvd."]
    #[inline(always)]
    pub fn bit_adc_clk_hcnt3(&mut self) -> BitAdcClkHcnt3W<'_, RegAdcClkHcnt3DivSpec> {
        BitAdcClkHcnt3W::new(self, 0)
    }
}
#[doc = "ADC Channel Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_clk_hcnt3_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_clk_hcnt3_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcClkHcnt3DivSpec;
impl crate::RegisterSpec for RegAdcClkHcnt3DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_clk_hcnt3_div::R`](R) reader structure"]
impl crate::Readable for RegAdcClkHcnt3DivSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_clk_hcnt3_div::W`](W) writer structure"]
impl crate::Writable for RegAdcClkHcnt3DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_CLK_HCNT3_DIV to value 0x15"]
impl crate::Resettable for RegAdcClkHcnt3DivSpec {
    const RESET_VALUE: u32 = 0x15;
}
