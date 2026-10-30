#[doc = "Register `adc_1_biquad_b1_3` reader"]
pub type R = crate::R<Adc1BiquadB1_3Spec>;
#[doc = "Register `adc_1_biquad_b1_3` writer"]
pub type W = crate::W<Adc1BiquadB1_3Spec>;
#[doc = "Field `adc_1_biquad_b1_3` reader - ADC channel 1 EQ 3-band coef. b1 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc1BiquadB1_3R = crate::FieldReader<u32>;
#[doc = "Field `adc_1_biquad_b1_3` writer - ADC channel 1 EQ 3-band coef. b1 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc1BiquadB1_3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - ADC channel 1 EQ 3-band coef. b1 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_1_biquad_b1_3(&self) -> Adc1BiquadB1_3R {
        Adc1BiquadB1_3R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - ADC channel 1 EQ 3-band coef. b1 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_1_biquad_b1_3(&mut self) -> Adc1BiquadB1_3W<'_, Adc1BiquadB1_3Spec> {
        Adc1BiquadB1_3W::new(self, 0)
    }
}
#[doc = "ADC Channel 1 EQ Band 3 Biquad B1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1_biquad_b1_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_1_biquad_b1_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1BiquadB1_3Spec;
impl crate::RegisterSpec for Adc1BiquadB1_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_1_biquad_b1_3::R`](R) reader structure"]
impl crate::Readable for Adc1BiquadB1_3Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_1_biquad_b1_3::W`](W) writer structure"]
impl crate::Writable for Adc1BiquadB1_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_1_biquad_b1_3 to value 0"]
impl crate::Resettable for Adc1BiquadB1_3Spec {}
