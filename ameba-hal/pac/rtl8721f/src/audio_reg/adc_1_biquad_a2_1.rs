#[doc = "Register `adc_1_biquad_a2_1` reader"]
pub type R = crate::R<Adc1BiquadA2_1Spec>;
#[doc = "Register `adc_1_biquad_a2_1` writer"]
pub type W = crate::W<Adc1BiquadA2_1Spec>;
#[doc = "Field `adc_1_biquad_a2_1` reader - ADC channel 1 EQ 1-band coef. a2 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc1BiquadA2_1R = crate::FieldReader<u32>;
#[doc = "Field `adc_1_biquad_a2_1` writer - ADC channel 1 EQ 1-band coef. a2 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc1BiquadA2_1W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - ADC channel 1 EQ 1-band coef. a2 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_1_biquad_a2_1(&self) -> Adc1BiquadA2_1R {
        Adc1BiquadA2_1R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - ADC channel 1 EQ 1-band coef. a2 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_1_biquad_a2_1(&mut self) -> Adc1BiquadA2_1W<'_, Adc1BiquadA2_1Spec> {
        Adc1BiquadA2_1W::new(self, 0)
    }
}
#[doc = "ADC Channel 1 EQ Band 1 Biquad A2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1_biquad_a2_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_1_biquad_a2_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1BiquadA2_1Spec;
impl crate::RegisterSpec for Adc1BiquadA2_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_1_biquad_a2_1::R`](R) reader structure"]
impl crate::Readable for Adc1BiquadA2_1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_1_biquad_a2_1::W`](W) writer structure"]
impl crate::Writable for Adc1BiquadA2_1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_1_biquad_a2_1 to value 0"]
impl crate::Resettable for Adc1BiquadA2_1Spec {}
