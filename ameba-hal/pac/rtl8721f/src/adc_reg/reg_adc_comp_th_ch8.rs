#[doc = "Register `REG_ADC_COMP_TH_CH8` reader"]
pub type R = crate::R<RegAdcCompThCh8Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH8` writer"]
pub type W = crate::W<RegAdcCompThCh8Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch8` reader - This field defines the lower threshold of channel 8 for ADC automatic comparison."]
pub type BitAdcCompThLCh8R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch8` writer - This field defines the lower threshold of channel 8 for ADC automatic comparison."]
pub type BitAdcCompThLCh8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch8` reader - This field defines the higher threshold of channel 8 for ADC automatic comparison."]
pub type BitAdcCompThHCh8R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch8` writer - This field defines the higher threshold of channel 8 for ADC automatic comparison."]
pub type BitAdcCompThHCh8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 8 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch8(&self) -> BitAdcCompThLCh8R {
        BitAdcCompThLCh8R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 8 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch8(&self) -> BitAdcCompThHCh8R {
        BitAdcCompThHCh8R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 8 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch8(&mut self) -> BitAdcCompThLCh8W<'_, RegAdcCompThCh8Spec> {
        BitAdcCompThLCh8W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 8 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch8(&mut self) -> BitAdcCompThHCh8W<'_, RegAdcCompThCh8Spec> {
        BitAdcCompThHCh8W::new(self, 16)
    }
}
#[doc = "ADC Channel 8 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh8Spec;
impl crate::RegisterSpec for RegAdcCompThCh8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch8::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh8Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch8::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH8 to value 0"]
impl crate::Resettable for RegAdcCompThCh8Spec {}
