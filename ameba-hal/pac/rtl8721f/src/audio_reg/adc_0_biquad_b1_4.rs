#[doc = "Register `adc_0_biquad_b1_4` reader"]
pub type R = crate::R<Adc0BiquadB1_4Spec>;
#[doc = "Register `adc_0_biquad_b1_4` writer"]
pub type W = crate::W<Adc0BiquadB1_4Spec>;
#[doc = "Field `adc_0_biquad_b1_4` reader - ADC channel 0 EQ 4-band coef. B1 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc0BiquadB1_4R = crate::FieldReader<u32>;
#[doc = "Field `adc_0_biquad_b1_4` writer - ADC channel 0 EQ 4-band coef. B1 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc0BiquadB1_4W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - ADC channel 0 EQ 4-band coef. B1 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_0_biquad_b1_4(&self) -> Adc0BiquadB1_4R {
        Adc0BiquadB1_4R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - ADC channel 0 EQ 4-band coef. B1 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_0_biquad_b1_4(&mut self) -> Adc0BiquadB1_4W<'_, Adc0BiquadB1_4Spec> {
        Adc0BiquadB1_4W::new(self, 0)
    }
}
#[doc = "ADC Channel 0 EQ Band 4 Biquad B1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_0_biquad_b1_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_0_biquad_b1_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0BiquadB1_4Spec;
impl crate::RegisterSpec for Adc0BiquadB1_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_0_biquad_b1_4::R`](R) reader structure"]
impl crate::Readable for Adc0BiquadB1_4Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_0_biquad_b1_4::W`](W) writer structure"]
impl crate::Writable for Adc0BiquadB1_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_0_biquad_b1_4 to value 0"]
impl crate::Resettable for Adc0BiquadB1_4Spec {}
