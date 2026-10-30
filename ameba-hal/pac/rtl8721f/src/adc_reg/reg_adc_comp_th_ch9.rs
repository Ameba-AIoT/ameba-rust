#[doc = "Register `REG_ADC_COMP_TH_CH9` reader"]
pub type R = crate::R<RegAdcCompThCh9Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH9` writer"]
pub type W = crate::W<RegAdcCompThCh9Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch9` reader - This field defines the lower threshold of channel 9 for ADC automatic comparison."]
pub type BitAdcCompThLCh9R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch9` writer - This field defines the lower threshold of channel 9 for ADC automatic comparison."]
pub type BitAdcCompThLCh9W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch9` reader - This field defines the higher threshold of channel 9 for ADC automatic comparison."]
pub type BitAdcCompThHCh9R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch9` writer - This field defines the higher threshold of channel 9 for ADC automatic comparison."]
pub type BitAdcCompThHCh9W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 9 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch9(&self) -> BitAdcCompThLCh9R {
        BitAdcCompThLCh9R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 9 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch9(&self) -> BitAdcCompThHCh9R {
        BitAdcCompThHCh9R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 9 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch9(&mut self) -> BitAdcCompThLCh9W<'_, RegAdcCompThCh9Spec> {
        BitAdcCompThLCh9W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 9 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch9(&mut self) -> BitAdcCompThHCh9W<'_, RegAdcCompThCh9Spec> {
        BitAdcCompThHCh9W::new(self, 16)
    }
}
#[doc = "ADC Channel 9 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh9Spec;
impl crate::RegisterSpec for RegAdcCompThCh9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch9::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh9Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch9::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH9 to value 0"]
impl crate::Resettable for RegAdcCompThCh9Spec {}
