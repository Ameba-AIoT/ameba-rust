#[doc = "Register `REG_ADC_IT_CHNO_CON` reader"]
pub type R = crate::R<RegAdcItChnoConSpec>;
#[doc = "Register `REG_ADC_IT_CHNO_CON` writer"]
pub type W = crate::W<RegAdcItChnoConSpec>;
#[doc = "Field `bit_adc_it_chno_con` reader - This field defines that ADC module should send interrupt signal to system when a conversion which of channel number is the same as this field. This register is only valid when bit_adc_it_chcv_end_en is set. bit_adc_it_chno_con: channel number"]
pub type BitAdcItChnoConR = crate::FieldReader;
#[doc = "Field `bit_adc_it_chno_con` writer - This field defines that ADC module should send interrupt signal to system when a conversion which of channel number is the same as this field. This register is only valid when bit_adc_it_chcv_end_en is set. bit_adc_it_chno_con: channel number"]
pub type BitAdcItChnoConW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - This field defines that ADC module should send interrupt signal to system when a conversion which of channel number is the same as this field. This register is only valid when bit_adc_it_chcv_end_en is set. bit_adc_it_chno_con: channel number"]
    #[inline(always)]
    pub fn bit_adc_it_chno_con(&self) -> BitAdcItChnoConR {
        BitAdcItChnoConR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field defines that ADC module should send interrupt signal to system when a conversion which of channel number is the same as this field. This register is only valid when bit_adc_it_chcv_end_en is set. bit_adc_it_chno_con: channel number"]
    #[inline(always)]
    pub fn bit_adc_it_chno_con(&mut self) -> BitAdcItChnoConW<'_, RegAdcItChnoConSpec> {
        BitAdcItChnoConW::new(self, 0)
    }
}
#[doc = "ADC Interrupt Channel Number Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_it_chno_con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_it_chno_con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcItChnoConSpec;
impl crate::RegisterSpec for RegAdcItChnoConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_it_chno_con::R`](R) reader structure"]
impl crate::Readable for RegAdcItChnoConSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_it_chno_con::W`](W) writer structure"]
impl crate::Writable for RegAdcItChnoConSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_IT_CHNO_CON to value 0"]
impl crate::Resettable for RegAdcItChnoConSpec {}
