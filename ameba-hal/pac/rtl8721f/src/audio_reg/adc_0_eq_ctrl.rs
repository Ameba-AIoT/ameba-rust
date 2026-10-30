#[doc = "Register `adc_0_eq_ctrl` reader"]
pub type R = crate::R<Adc0EqCtrlSpec>;
#[doc = "Register `adc_0_eq_ctrl` writer"]
pub type W = crate::W<Adc0EqCtrlSpec>;
#[doc = "Field `adc_0_biquad_en_0` reader - ADC channel 0 EQ 0-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn0R = crate::BitReader;
#[doc = "Field `adc_0_biquad_en_0` writer - ADC channel 0 EQ 0-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_biquad_en_1` reader - ADC channel 0 EQ 1-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn1R = crate::BitReader;
#[doc = "Field `adc_0_biquad_en_1` writer - ADC channel 0 EQ 1-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_biquad_en_2` reader - ADC channel 0 EQ 2-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn2R = crate::BitReader;
#[doc = "Field `adc_0_biquad_en_2` writer - ADC channel 0 EQ 2-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_biquad_en_3` reader - ADC channel 0 EQ 3-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn3R = crate::BitReader;
#[doc = "Field `adc_0_biquad_en_3` writer - ADC channel 0 EQ 3-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_biquad_en_4` reader - ADC channel 0 EQ 4-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn4R = crate::BitReader;
#[doc = "Field `adc_0_biquad_en_4` writer - ADC channel 0 EQ 4-band biquad enable 0: disable 1: enable"]
pub type Adc0BiquadEn4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC channel 0 EQ 0-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_0(&self) -> Adc0BiquadEn0R {
        Adc0BiquadEn0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC channel 0 EQ 1-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_1(&self) -> Adc0BiquadEn1R {
        Adc0BiquadEn1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC channel 0 EQ 2-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_2(&self) -> Adc0BiquadEn2R {
        Adc0BiquadEn2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC channel 0 EQ 3-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_3(&self) -> Adc0BiquadEn3R {
        Adc0BiquadEn3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC channel 0 EQ 4-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_4(&self) -> Adc0BiquadEn4R {
        Adc0BiquadEn4R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC channel 0 EQ 0-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_0(&mut self) -> Adc0BiquadEn0W<'_, Adc0EqCtrlSpec> {
        Adc0BiquadEn0W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC channel 0 EQ 1-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_1(&mut self) -> Adc0BiquadEn1W<'_, Adc0EqCtrlSpec> {
        Adc0BiquadEn1W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC channel 0 EQ 2-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_2(&mut self) -> Adc0BiquadEn2W<'_, Adc0EqCtrlSpec> {
        Adc0BiquadEn2W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC channel 0 EQ 3-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_3(&mut self) -> Adc0BiquadEn3W<'_, Adc0EqCtrlSpec> {
        Adc0BiquadEn3W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC channel 0 EQ 4-band biquad enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn adc_0_biquad_en_4(&mut self) -> Adc0BiquadEn4W<'_, Adc0EqCtrlSpec> {
        Adc0BiquadEn4W::new(self, 4)
    }
}
#[doc = "ADC Channel 0 EQ Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_0_eq_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_0_eq_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0EqCtrlSpec;
impl crate::RegisterSpec for Adc0EqCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_0_eq_ctrl::R`](R) reader structure"]
impl crate::Readable for Adc0EqCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_0_eq_ctrl::W`](W) writer structure"]
impl crate::Writable for Adc0EqCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_0_eq_ctrl to value 0"]
impl crate::Resettable for Adc0EqCtrlSpec {}
