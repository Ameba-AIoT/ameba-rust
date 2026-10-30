#[doc = "Register `REG_ADC_COMP_TH_CH5` reader"]
pub type R = crate::R<RegAdcCompThCh5Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH5` writer"]
pub type W = crate::W<RegAdcCompThCh5Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch5` reader - This field defines the lower threshold of channel 5 for ADC automatic comparison."]
pub type BitAdcCompThLCh5R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch5` writer - This field defines the lower threshold of channel 5 for ADC automatic comparison."]
pub type BitAdcCompThLCh5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch5` reader - This field defines the higher threshold of channel 5 for ADC automatic comparison."]
pub type BitAdcCompThHCh5R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch5` writer - This field defines the higher threshold of channel 5 for ADC automatic comparison."]
pub type BitAdcCompThHCh5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 5 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch5(&self) -> BitAdcCompThLCh5R {
        BitAdcCompThLCh5R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 5 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch5(&self) -> BitAdcCompThHCh5R {
        BitAdcCompThHCh5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 5 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch5(&mut self) -> BitAdcCompThLCh5W<'_, RegAdcCompThCh5Spec> {
        BitAdcCompThLCh5W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 5 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch5(&mut self) -> BitAdcCompThHCh5W<'_, RegAdcCompThCh5Spec> {
        BitAdcCompThHCh5W::new(self, 16)
    }
}
#[doc = "ADC Channel 5 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh5Spec;
impl crate::RegisterSpec for RegAdcCompThCh5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch5::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch5::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH5 to value 0"]
impl crate::Resettable for RegAdcCompThCh5Spec {}
