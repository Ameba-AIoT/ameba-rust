#[doc = "Register `REG_ADC_COMP_TH_CH0` reader"]
pub type R = crate::R<RegAdcCompThCh0Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH0` writer"]
pub type W = crate::W<RegAdcCompThCh0Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch0` reader - This field defines the lower threshold of channel 0 for ADC automatic comparison."]
pub type BitAdcCompThLCh0R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch0` writer - This field defines the lower threshold of channel 0 for ADC automatic comparison."]
pub type BitAdcCompThLCh0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch0` reader - This field defines the higher threshold of channel 0 for ADC automatic comparison."]
pub type BitAdcCompThHCh0R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch0` writer - This field defines the higher threshold of channel 0 for ADC automatic comparison."]
pub type BitAdcCompThHCh0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 0 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch0(&self) -> BitAdcCompThLCh0R {
        BitAdcCompThLCh0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 0 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch0(&self) -> BitAdcCompThHCh0R {
        BitAdcCompThHCh0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 0 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch0(&mut self) -> BitAdcCompThLCh0W<'_, RegAdcCompThCh0Spec> {
        BitAdcCompThLCh0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 0 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch0(&mut self) -> BitAdcCompThHCh0W<'_, RegAdcCompThCh0Spec> {
        BitAdcCompThHCh0W::new(self, 16)
    }
}
#[doc = "ADC Channel 0 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh0Spec;
impl crate::RegisterSpec for RegAdcCompThCh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch0::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch0::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH0 to value 0"]
impl crate::Resettable for RegAdcCompThCh0Spec {}
