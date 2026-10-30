#[doc = "Register `REG_ADC_SW_TRIG` reader"]
pub type R = crate::R<RegAdcSwTrigSpec>;
#[doc = "Register `REG_ADC_SW_TRIG` writer"]
pub type W = crate::W<RegAdcSwTrigSpec>;
#[doc = "Field `bit_adc_sw_trig` reader - This bit controls the ADC module to do a conversion. Every time this bit is set to 1, ADC module would switch to a new channel and do one conversion. Therefore, this bit could be used as a start-convert event which is controlled by software. Every time a conversion is done, software MUST clear this bit manually. The interval between clearing and restart this bit must exceed one sample clock period. 0: disable the analog module and analog mux. 1: enable the analog module and analog mux. And then start a new channel conversion."]
pub type BitAdcSwTrigR = crate::BitReader;
#[doc = "Field `bit_adc_sw_trig` writer - This bit controls the ADC module to do a conversion. Every time this bit is set to 1, ADC module would switch to a new channel and do one conversion. Therefore, this bit could be used as a start-convert event which is controlled by software. Every time a conversion is done, software MUST clear this bit manually. The interval between clearing and restart this bit must exceed one sample clock period. 0: disable the analog module and analog mux. 1: enable the analog module and analog mux. And then start a new channel conversion."]
pub type BitAdcSwTrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit controls the ADC module to do a conversion. Every time this bit is set to 1, ADC module would switch to a new channel and do one conversion. Therefore, this bit could be used as a start-convert event which is controlled by software. Every time a conversion is done, software MUST clear this bit manually. The interval between clearing and restart this bit must exceed one sample clock period. 0: disable the analog module and analog mux. 1: enable the analog module and analog mux. And then start a new channel conversion."]
    #[inline(always)]
    pub fn bit_adc_sw_trig(&self) -> BitAdcSwTrigR {
        BitAdcSwTrigR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls the ADC module to do a conversion. Every time this bit is set to 1, ADC module would switch to a new channel and do one conversion. Therefore, this bit could be used as a start-convert event which is controlled by software. Every time a conversion is done, software MUST clear this bit manually. The interval between clearing and restart this bit must exceed one sample clock period. 0: disable the analog module and analog mux. 1: enable the analog module and analog mux. And then start a new channel conversion."]
    #[inline(always)]
    pub fn bit_adc_sw_trig(&mut self) -> BitAdcSwTrigW<'_, RegAdcSwTrigSpec> {
        BitAdcSwTrigW::new(self, 0)
    }
}
#[doc = "ADC Software Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_sw_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_sw_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcSwTrigSpec;
impl crate::RegisterSpec for RegAdcSwTrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_sw_trig::R`](R) reader structure"]
impl crate::Readable for RegAdcSwTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_sw_trig::W`](W) writer structure"]
impl crate::Writable for RegAdcSwTrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_SW_TRIG to value 0"]
impl crate::Resettable for RegAdcSwTrigSpec {}
