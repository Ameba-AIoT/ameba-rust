#[doc = "Register `adc_1_biquad_h0_4` reader"]
pub type R = crate::R<Adc1BiquadH0_4Spec>;
#[doc = "Register `adc_1_biquad_h0_4` writer"]
pub type W = crate::W<Adc1BiquadH0_4Spec>;
#[doc = "Field `adc_1_biquad_h0_4` reader - ADC channel 1 EQ 4-band coef. h0 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc1BiquadH0_4R = crate::FieldReader<u32>;
#[doc = "Field `adc_1_biquad_h0_4` writer - ADC channel 1 EQ 4-band coef. h0 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
pub type Adc1BiquadH0_4W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - ADC channel 1 EQ 4-band coef. h0 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_1_biquad_h0_4(&self) -> Adc1BiquadH0_4R {
        Adc1BiquadH0_4R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - ADC channel 1 EQ 4-band coef. h0 2's complement in 4.25 format, i.e. the range is from -8~7.99."]
    #[inline(always)]
    pub fn adc_1_biquad_h0_4(&mut self) -> Adc1BiquadH0_4W<'_, Adc1BiquadH0_4Spec> {
        Adc1BiquadH0_4W::new(self, 0)
    }
}
#[doc = "ADC Channel 1 EQ Band 4 Biquad H0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1_biquad_h0_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_1_biquad_h0_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1BiquadH0_4Spec;
impl crate::RegisterSpec for Adc1BiquadH0_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_1_biquad_h0_4::R`](R) reader structure"]
impl crate::Readable for Adc1BiquadH0_4Spec {}
#[doc = "`write(|w| ..)` method takes [`adc_1_biquad_h0_4::W`](W) writer structure"]
impl crate::Writable for Adc1BiquadH0_4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_1_biquad_h0_4 to value 0x0200_0000"]
impl crate::Resettable for Adc1BiquadH0_4Spec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
