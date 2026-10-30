#[doc = "Register `REG_ADC_COMP_TH_CH3` reader"]
pub type R = crate::R<RegAdcCompThCh3Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH3` writer"]
pub type W = crate::W<RegAdcCompThCh3Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch3` reader - This field defines the lower threshold of channel 3 for ADC automatic comparison."]
pub type BitAdcCompThLCh3R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch3` writer - This field defines the lower threshold of channel 3 for ADC automatic comparison."]
pub type BitAdcCompThLCh3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch3` reader - This field defines the higher threshold of channel 3 for ADC automatic comparison."]
pub type BitAdcCompThHCh3R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch3` writer - This field defines the higher threshold of channel 3 for ADC automatic comparison."]
pub type BitAdcCompThHCh3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 3 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch3(&self) -> BitAdcCompThLCh3R {
        BitAdcCompThLCh3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 3 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch3(&self) -> BitAdcCompThHCh3R {
        BitAdcCompThHCh3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 3 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch3(&mut self) -> BitAdcCompThLCh3W<'_, RegAdcCompThCh3Spec> {
        BitAdcCompThLCh3W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 3 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch3(&mut self) -> BitAdcCompThHCh3W<'_, RegAdcCompThCh3Spec> {
        BitAdcCompThHCh3W::new(self, 16)
    }
}
#[doc = "ADC Channel 3 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh3Spec;
impl crate::RegisterSpec for RegAdcCompThCh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch3::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch3::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH3 to value 0"]
impl crate::Resettable for RegAdcCompThCh3Spec {}
