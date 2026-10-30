#[doc = "Register `adc_1_biquad_b2_2` reader"]
pub type R = crate::R<Adc1BiquadB2_2Spec>;
#[doc = "Register `adc_1_biquad_b2_2` writer"]
pub type W = crate::W<Adc1BiquadB2_2Spec>;
#[doc = "Field `adc_1_biquad_b2_2` reader - ADC channel 1 EQ 2-band coef. b2 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc1BiquadB2_2R = crate::FieldReader<u32>;
#[doc = "Field `adc_1_biquad_b2_2` writer - ADC channel 1 EQ 2-band coef. b2 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc1BiquadB2_2W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - ADC channel 1 EQ 2-band coef. b2 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_1_biquad_b2_2(&self) -> Adc1BiquadB2_2R {
        Adc1BiquadB2_2R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - ADC channel 1 EQ 2-band coef. b2 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_1_biquad_b2_2(&mut self) -> Adc1BiquadB2_2W<'_, Adc1BiquadB2_2Spec> {
        Adc1BiquadB2_2W::new(self, 0)
    }
}
#[doc = "ADC Channel 1 EQ Band 2 Biquad B2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1_biquad_b2_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_1_biquad_b2_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1BiquadB2_2Spec;
impl crate::RegisterSpec for Adc1BiquadB2_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_1_biquad_b2_2::R`](R) reader structure"]
impl crate::Readable for Adc1BiquadB2_2Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_1_biquad_b2_2::W`](W) writer structure"]
impl crate::Writable for Adc1BiquadB2_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_1_biquad_b2_2 to value 0"]
impl crate::Resettable for Adc1BiquadB2_2Spec {}
