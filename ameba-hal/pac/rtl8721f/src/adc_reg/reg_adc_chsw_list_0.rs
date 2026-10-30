#[doc = "Register `REG_ADC_CHSW_LIST_0` reader"]
pub type R = crate::R<RegAdcChswList0Spec>;
#[doc = "Register `REG_ADC_CHSW_LIST_0` writer"]
pub type W = crate::W<RegAdcChswList0Spec>;
#[doc = "Field `bit_adc_chsw_0` reader - This field defines the number 0 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw0R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_0` writer - This field defines the number 0 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_1` reader - This field defines the number 1 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw1R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_1` writer - This field defines the number 1 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_2` reader - This field defines the number 2 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw2R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_2` writer - This field defines the number 2 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_3` reader - This field defines the number 3 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw3R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_3` writer - This field defines the number 3 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_4` reader - This field defines the number 4 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw4R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_4` writer - This field defines the number 4 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_5` reader - This field defines the number 5 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw5R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_5` writer - This field defines the number 5 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_6` reader - This field defines the number 6 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw6R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_6` writer - This field defines the number 6 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_7` reader - This field defines the number 7 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw7R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_7` writer - This field defines the number 7 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - This field defines the number 0 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_0(&self) -> BitAdcChsw0R {
        BitAdcChsw0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This field defines the number 1 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_1(&self) -> BitAdcChsw1R {
        BitAdcChsw1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field defines the number 2 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_2(&self) -> BitAdcChsw2R {
        BitAdcChsw2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - This field defines the number 3 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_3(&self) -> BitAdcChsw3R {
        BitAdcChsw3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - This field defines the number 4 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_4(&self) -> BitAdcChsw4R {
        BitAdcChsw4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - This field defines the number 5 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_5(&self) -> BitAdcChsw5R {
        BitAdcChsw5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - This field defines the number 6 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_6(&self) -> BitAdcChsw6R {
        BitAdcChsw6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - This field defines the number 7 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_7(&self) -> BitAdcChsw7R {
        BitAdcChsw7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field defines the number 0 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_0(&mut self) -> BitAdcChsw0W<'_, RegAdcChswList0Spec> {
        BitAdcChsw0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - This field defines the number 1 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_1(&mut self) -> BitAdcChsw1W<'_, RegAdcChswList0Spec> {
        BitAdcChsw1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - This field defines the number 2 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_2(&mut self) -> BitAdcChsw2W<'_, RegAdcChswList0Spec> {
        BitAdcChsw2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - This field defines the number 3 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_3(&mut self) -> BitAdcChsw3W<'_, RegAdcChswList0Spec> {
        BitAdcChsw3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - This field defines the number 4 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_4(&mut self) -> BitAdcChsw4W<'_, RegAdcChswList0Spec> {
        BitAdcChsw4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - This field defines the number 5 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_5(&mut self) -> BitAdcChsw5W<'_, RegAdcChswList0Spec> {
        BitAdcChsw5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - This field defines the number 6 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_6(&mut self) -> BitAdcChsw6W<'_, RegAdcChswList0Spec> {
        BitAdcChsw6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - This field defines the number 7 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_7(&mut self) -> BitAdcChsw7W<'_, RegAdcChswList0Spec> {
        BitAdcChsw7W::new(self, 28)
    }
}
#[doc = "ADC Channel Switch List 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_chsw_list_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_chsw_list_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcChswList0Spec;
impl crate::RegisterSpec for RegAdcChswList0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_chsw_list_0::R`](R) reader structure"]
impl crate::Readable for RegAdcChswList0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_chsw_list_0::W`](W) writer structure"]
impl crate::Writable for RegAdcChswList0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_CHSW_LIST_0 to value 0"]
impl crate::Resettable for RegAdcChswList0Spec {}
