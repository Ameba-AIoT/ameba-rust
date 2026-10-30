#[doc = "Register `REG_ADC_COMP_TH_CH10` reader"]
pub type R = crate::R<RegAdcCompThCh10Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH10` writer"]
pub type W = crate::W<RegAdcCompThCh10Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch10` reader - This field defines the lower threshold of channel 10 for ADC automatic comparison."]
pub type BitAdcCompThLCh10R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch10` writer - This field defines the lower threshold of channel 10 for ADC automatic comparison."]
pub type BitAdcCompThLCh10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch10` reader - This field defines the higher threshold of channel 10 for ADC automatic comparison."]
pub type BitAdcCompThHCh10R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch10` writer - This field defines the higher threshold of channel 10 for ADC automatic comparison."]
pub type BitAdcCompThHCh10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 10 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch10(&self) -> BitAdcCompThLCh10R {
        BitAdcCompThLCh10R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 10 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch10(&self) -> BitAdcCompThHCh10R {
        BitAdcCompThHCh10R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 10 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch10(&mut self) -> BitAdcCompThLCh10W<'_, RegAdcCompThCh10Spec> {
        BitAdcCompThLCh10W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 10 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch10(&mut self) -> BitAdcCompThHCh10W<'_, RegAdcCompThCh10Spec> {
        BitAdcCompThHCh10W::new(self, 16)
    }
}
#[doc = "ADC Channel 10 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh10Spec;
impl crate::RegisterSpec for RegAdcCompThCh10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch10::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch10::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH10 to value 0"]
impl crate::Resettable for RegAdcCompThCh10Spec {}
