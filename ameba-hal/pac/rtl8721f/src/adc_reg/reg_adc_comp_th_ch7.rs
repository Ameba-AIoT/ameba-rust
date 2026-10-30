#[doc = "Register `REG_ADC_COMP_TH_CH7` reader"]
pub type R = crate::R<RegAdcCompThCh7Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH7` writer"]
pub type W = crate::W<RegAdcCompThCh7Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch7` reader - This field defines the lower threshold of channel 7 for ADC automatic comparison."]
pub type BitAdcCompThLCh7R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch7` writer - This field defines the lower threshold of channel 7 for ADC automatic comparison."]
pub type BitAdcCompThLCh7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch7` reader - This field defines the higher threshold of channel 7 for ADC automatic comparison."]
pub type BitAdcCompThHCh7R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch7` writer - This field defines the higher threshold of channel 7 for ADC automatic comparison."]
pub type BitAdcCompThHCh7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 7 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch7(&self) -> BitAdcCompThLCh7R {
        BitAdcCompThLCh7R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 7 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch7(&self) -> BitAdcCompThHCh7R {
        BitAdcCompThHCh7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 7 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch7(&mut self) -> BitAdcCompThLCh7W<'_, RegAdcCompThCh7Spec> {
        BitAdcCompThLCh7W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 7 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch7(&mut self) -> BitAdcCompThHCh7W<'_, RegAdcCompThCh7Spec> {
        BitAdcCompThHCh7W::new(self, 16)
    }
}
#[doc = "ADC Channel 7 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh7Spec;
impl crate::RegisterSpec for RegAdcCompThCh7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch7::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch7::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH7 to value 0"]
impl crate::Resettable for RegAdcCompThCh7Spec {}
