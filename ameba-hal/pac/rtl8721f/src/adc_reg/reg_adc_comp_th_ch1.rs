#[doc = "Register `REG_ADC_COMP_TH_CH1` reader"]
pub type R = crate::R<RegAdcCompThCh1Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH1` writer"]
pub type W = crate::W<RegAdcCompThCh1Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch1` reader - This field defines the lower threshold of channel 1 for ADC automatic comparison."]
pub type BitAdcCompThLCh1R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch1` writer - This field defines the lower threshold of channel 1 for ADC automatic comparison."]
pub type BitAdcCompThLCh1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch1` reader - This field defines the higher threshold of channel 1 for ADC automatic comparison."]
pub type BitAdcCompThHCh1R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch1` writer - This field defines the higher threshold of channel 1 for ADC automatic comparison."]
pub type BitAdcCompThHCh1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 1 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch1(&self) -> BitAdcCompThLCh1R {
        BitAdcCompThLCh1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 1 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch1(&self) -> BitAdcCompThHCh1R {
        BitAdcCompThHCh1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 1 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch1(&mut self) -> BitAdcCompThLCh1W<'_, RegAdcCompThCh1Spec> {
        BitAdcCompThLCh1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 1 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch1(&mut self) -> BitAdcCompThHCh1W<'_, RegAdcCompThCh1Spec> {
        BitAdcCompThHCh1W::new(self, 16)
    }
}
#[doc = "ADC Channel 1 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh1Spec;
impl crate::RegisterSpec for RegAdcCompThCh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch1::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch1::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH1 to value 0"]
impl crate::Resettable for RegAdcCompThCh1Spec {}
