#[doc = "Register `REG_ADC_EXT_TRIG_TIMER_SEL` reader"]
pub type R = crate::R<RegAdcExtTrigTimerSelSpec>;
#[doc = "Register `REG_ADC_EXT_TRIG_TIMER_SEL` writer"]
pub type W = crate::W<RegAdcExtTrigTimerSelSpec>;
#[doc = "Field `bit_adc_ext_trig_timer_sel` reader - These three bits defines which timer channel would be used to make ADC module do a conversion. 0: Timer module 0 is used as the comparator external trigger source. 1: Timer module 1 is used as the comparator external trigger source. .. 7: Timer module 7 is used as the comparator external trigger source."]
pub type BitAdcExtTrigTimerSelR = crate::FieldReader;
#[doc = "Field `bit_adc_ext_trig_timer_sel` writer - These three bits defines which timer channel would be used to make ADC module do a conversion. 0: Timer module 0 is used as the comparator external trigger source. 1: Timer module 1 is used as the comparator external trigger source. .. 7: Timer module 7 is used as the comparator external trigger source."]
pub type BitAdcExtTrigTimerSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - These three bits defines which timer channel would be used to make ADC module do a conversion. 0: Timer module 0 is used as the comparator external trigger source. 1: Timer module 1 is used as the comparator external trigger source. .. 7: Timer module 7 is used as the comparator external trigger source."]
    #[inline(always)]
    pub fn bit_adc_ext_trig_timer_sel(&self) -> BitAdcExtTrigTimerSelR {
        BitAdcExtTrigTimerSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - These three bits defines which timer channel would be used to make ADC module do a conversion. 0: Timer module 0 is used as the comparator external trigger source. 1: Timer module 1 is used as the comparator external trigger source. .. 7: Timer module 7 is used as the comparator external trigger source."]
    #[inline(always)]
    pub fn bit_adc_ext_trig_timer_sel(
        &mut self,
    ) -> BitAdcExtTrigTimerSelW<'_, RegAdcExtTrigTimerSelSpec> {
        BitAdcExtTrigTimerSelW::new(self, 0)
    }
}
#[doc = "ADC External Trigger Timer Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_ext_trig_timer_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_ext_trig_timer_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcExtTrigTimerSelSpec;
impl crate::RegisterSpec for RegAdcExtTrigTimerSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_ext_trig_timer_sel::R`](R) reader structure"]
impl crate::Readable for RegAdcExtTrigTimerSelSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_ext_trig_timer_sel::W`](W) writer structure"]
impl crate::Writable for RegAdcExtTrigTimerSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_EXT_TRIG_TIMER_SEL to value 0"]
impl crate::Resettable for RegAdcExtTrigTimerSelSpec {}
