#[doc = "Register `REG_ADC_COMP_TH_CH2` reader"]
pub type R = crate::R<RegAdcCompThCh2Spec>;
#[doc = "Register `REG_ADC_COMP_TH_CH2` writer"]
pub type W = crate::W<RegAdcCompThCh2Spec>;
#[doc = "Field `bit_adc_comp_th_l_ch2` reader - This field defines the lower threshold of channel 2 for ADC automatic comparison."]
pub type BitAdcCompThLCh2R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_l_ch2` writer - This field defines the lower threshold of channel 2 for ADC automatic comparison."]
pub type BitAdcCompThLCh2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bit_adc_comp_th_h_ch2` reader - This field defines the higher threshold of channel 2 for ADC automatic comparison."]
pub type BitAdcCompThHCh2R = crate::FieldReader<u16>;
#[doc = "Field `bit_adc_comp_th_h_ch2` writer - This field defines the higher threshold of channel 2 for ADC automatic comparison."]
pub type BitAdcCompThHCh2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 2 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch2(&self) -> BitAdcCompThLCh2R {
        BitAdcCompThLCh2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 2 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch2(&self) -> BitAdcCompThHCh2R {
        BitAdcCompThHCh2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the lower threshold of channel 2 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_l_ch2(&mut self) -> BitAdcCompThLCh2W<'_, RegAdcCompThCh2Spec> {
        BitAdcCompThLCh2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This field defines the higher threshold of channel 2 for ADC automatic comparison."]
    #[inline(always)]
    pub fn bit_adc_comp_th_h_ch2(&mut self) -> BitAdcCompThHCh2W<'_, RegAdcCompThCh2Spec> {
        BitAdcCompThHCh2W::new(self, 16)
    }
}
#[doc = "ADC Channel 2 Comparison Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_comp_th_ch2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_comp_th_ch2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcCompThCh2Spec;
impl crate::RegisterSpec for RegAdcCompThCh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_comp_th_ch2::R`](R) reader structure"]
impl crate::Readable for RegAdcCompThCh2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_comp_th_ch2::W`](W) writer structure"]
impl crate::Writable for RegAdcCompThCh2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_COMP_TH_CH2 to value 0"]
impl crate::Resettable for RegAdcCompThCh2Spec {}
