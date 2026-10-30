#[doc = "Register `adc_1_eq_ctrl` reader"]
pub type R = crate::R<Adc1EqCtrlSpec>;
#[doc = "Register `adc_1_eq_ctrl` writer"]
pub type W = crate::W<Adc1EqCtrlSpec>;
#[doc = "Field `adc_1_biquad_en_0` reader - ADC channel 1 EQ 0-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn0R = crate::BitReader;
#[doc = "Field `adc_1_biquad_en_0` writer - ADC channel 1 EQ 0-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_biquad_en_1` reader - ADC channel 1 EQ 1-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn1R = crate::BitReader;
#[doc = "Field `adc_1_biquad_en_1` writer - ADC channel 1 EQ 1-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_biquad_en_2` reader - ADC channel 1 EQ 2-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn2R = crate::BitReader;
#[doc = "Field `adc_1_biquad_en_2` writer - ADC channel 1 EQ 2-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_biquad_en_3` reader - ADC channel 1 EQ 3-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn3R = crate::BitReader;
#[doc = "Field `adc_1_biquad_en_3` writer - ADC channel 1 EQ 3-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_biquad_en_4` reader - ADC channel 1 EQ 4-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn4R = crate::BitReader;
#[doc = "Field `adc_1_biquad_en_4` writer - ADC channel 1 EQ 4-band biquad enable 0: disable 1: enable"]
pub type Adc1BiquadEn4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC channel 1 EQ 0-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_0(&self) -> Adc1BiquadEn0R {
        Adc1BiquadEn0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC channel 1 EQ 1-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_1(&self) -> Adc1BiquadEn1R {
        Adc1BiquadEn1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC channel 1 EQ 2-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_2(&self) -> Adc1BiquadEn2R {
        Adc1BiquadEn2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC channel 1 EQ 3-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_3(&self) -> Adc1BiquadEn3R {
        Adc1BiquadEn3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC channel 1 EQ 4-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_4(&self) -> Adc1BiquadEn4R {
        Adc1BiquadEn4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC channel 1 EQ 0-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_0(&mut self) -> Adc1BiquadEn0W<'_, Adc1EqCtrlSpec> {
        Adc1BiquadEn0W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC channel 1 EQ 1-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_1(&mut self) -> Adc1BiquadEn1W<'_, Adc1EqCtrlSpec> {
        Adc1BiquadEn1W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC channel 1 EQ 2-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_2(&mut self) -> Adc1BiquadEn2W<'_, Adc1EqCtrlSpec> {
        Adc1BiquadEn2W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC channel 1 EQ 3-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_3(&mut self) -> Adc1BiquadEn3W<'_, Adc1EqCtrlSpec> {
        Adc1BiquadEn3W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC channel 1 EQ 4-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_1_biquad_en_4(&mut self) -> Adc1BiquadEn4W<'_, Adc1EqCtrlSpec> {
        Adc1BiquadEn4W::new(self, 4)
    }
}
#[doc = "ADC Channel 1 EQ Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_1_eq_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_1_eq_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1EqCtrlSpec;
impl crate::RegisterSpec for Adc1EqCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_1_eq_ctrl::R`](R) reader structure"]
impl crate::Readable for Adc1EqCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_1_eq_ctrl::W`](W) writer structure"]
impl crate::Writable for Adc1EqCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_1_eq_ctrl to value 0"]
impl crate::Resettable for Adc1EqCtrlSpec {}
