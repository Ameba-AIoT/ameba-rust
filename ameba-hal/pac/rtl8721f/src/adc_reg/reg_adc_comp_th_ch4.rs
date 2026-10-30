#[doc = "Register `REG_ADC_COMP_TH_CH4` reader"]
pub type R = crate::R<RegAdcCompThCh4Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH4` writer"]
pub type W = crate::W<RegAdcCompThCh4Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch4` reader - This field defines the lower threshold of channel 4 for ADC automatic comparison."]
pub type BitAdcCompThLCh4R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch4` writer - This field defines the lower threshold of channel 4 for ADC automatic comparison."]
pub type BitAdcCompThLCh4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch4` reader - This field defines the higher threshold of channel 4 for ADC automatic comparison."]
pub type BitAdcCompThHCh4R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch4` writer - This field defines the higher threshold of channel 4 for ADC automatic comparison."]
pub type BitAdcCompThHCh4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 4 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch4(&self) -> BitAdcCompThLCh4R {
        BitAdcCompThLCh4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 4 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch4(&self) -> BitAdcCompThHCh4R {
        BitAdcCompThHCh4R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 4 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch4(&mut self) -> BitAdcCompThLCh4W<'_, RegAdcCompThCh4Spec> {
        BitAdcCompThLCh4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 4 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch4(&mut self) -> BitAdcCompThHCh4W<'_, RegAdcCompThCh4Spec> {
        BitAdcCompThHCh4W::new(self, 16)
    }
}
#[doc = "ADC Channel 4 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh4Spec;
impl crate::RegisterSpec for RegAdcCompThCh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch4::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch4::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH4 to value 0"]
impl crate::Resettable for RegAdcCompThCh4Spec {}
