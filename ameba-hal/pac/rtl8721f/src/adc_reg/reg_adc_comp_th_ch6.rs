#[doc = "Register `REG_ADC_COMP_TH_CH6` reader"]
pub type R = crate::R<RegAdcCompThCh6Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH6` writer"]
pub type W = crate::W<RegAdcCompThCh6Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch6` reader - This field defines the lower threshold of channel 6 for ADC automatic comparison."]
pub type BitAdcCompThLCh6R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch6` writer - This field defines the lower threshold of channel 6 for ADC automatic comparison."]
pub type BitAdcCompThLCh6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch6` reader - This field defines the higher threshold of channel 6 for ADC automatic comparison."]
pub type BitAdcCompThHCh6R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch6` writer - This field defines the higher threshold of channel 6 for ADC automatic comparison."]
pub type BitAdcCompThHCh6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 6 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch6(&self) -> BitAdcCompThLCh6R {
        BitAdcCompThLCh6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 6 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch6(&self) -> BitAdcCompThHCh6R {
        BitAdcCompThHCh6R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 6 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch6(&mut self) -> BitAdcCompThLCh6W<'_, RegAdcCompThCh6Spec> {
        BitAdcCompThLCh6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 6 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch6(&mut self) -> BitAdcCompThHCh6W<'_, RegAdcCompThCh6Spec> {
        BitAdcCompThHCh6W::new(self, 16)
    }
}
#[doc = "ADC Channel 6 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh6Spec;
impl crate::RegisterSpec for RegAdcCompThCh6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch6::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh6Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch6::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH6 to value 0"]
impl crate::Resettable for RegAdcCompThCh6Spec {}
