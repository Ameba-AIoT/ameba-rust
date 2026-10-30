#[doc = "Register `REG_ADC_CHSW_LIST_1` reader"]
pub type R = crate::R<RegAdcChswList1Spec>;
#[doc = "Register `REG_ADC_CHSW_LIST_1` writer"]
pub type W = crate::W<RegAdcChswList1Spec>;
#[doc = "Field `bit_adc_chsw_8` reader - This field defines the number 8 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw8R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_8` writer - This field defines the number 8 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_9` reader - This field defines the number 9 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw9R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_9` writer - This field defines the number 9 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_10` reader - This field defines the number 10 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw10R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_10` writer - This field defines the number 10 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_11` reader - This field defines the number 11 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw11R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_11` writer - This field defines the number 11 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_12` reader - This field defines the number 12 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw12R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_12` writer - This field defines the number 12 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_13` reader - This field defines the number 13 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw13R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_13` writer - This field defines the number 13 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_14` reader - This field defines the number 14 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw14R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_14` writer - This field defines the number 14 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bit_adc_chsw_15` reader - This field defines the number 15 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw15R = crate::FieldReader;
#[doc = "Field `bit_adc_chsw_15` writer - This field defines the number 15 channel to do a conversion when a conversion event takes place."]
pub type BitAdcChsw15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - This field defines the number 8 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_8(&self) -> BitAdcChsw8R {
        BitAdcChsw8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This field defines the number 9 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_9(&self) -> BitAdcChsw9R {
        BitAdcChsw9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field defines the number 10 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_10(&self) -> BitAdcChsw10R {
        BitAdcChsw10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - This field defines the number 11 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_11(&self) -> BitAdcChsw11R {
        BitAdcChsw11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - This field defines the number 12 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_12(&self) -> BitAdcChsw12R {
        BitAdcChsw12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - This field defines the number 13 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_13(&self) -> BitAdcChsw13R {
        BitAdcChsw13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - This field defines the number 14 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_14(&self) -> BitAdcChsw14R {
        BitAdcChsw14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - This field defines the number 15 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_15(&self) -> BitAdcChsw15R {
        BitAdcChsw15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field defines the number 8 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_8(&mut self) -> BitAdcChsw8W<'_, RegAdcChswList1Spec> {
        BitAdcChsw8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - This field defines the number 9 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_9(&mut self) -> BitAdcChsw9W<'_, RegAdcChswList1Spec> {
        BitAdcChsw9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - This field defines the number 10 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_10(&mut self) -> BitAdcChsw10W<'_, RegAdcChswList1Spec> {
        BitAdcChsw10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - This field defines the number 11 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_11(&mut self) -> BitAdcChsw11W<'_, RegAdcChswList1Spec> {
        BitAdcChsw11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - This field defines the number 12 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_12(&mut self) -> BitAdcChsw12W<'_, RegAdcChswList1Spec> {
        BitAdcChsw12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - This field defines the number 13 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_13(&mut self) -> BitAdcChsw13W<'_, RegAdcChswList1Spec> {
        BitAdcChsw13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - This field defines the number 14 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_14(&mut self) -> BitAdcChsw14W<'_, RegAdcChswList1Spec> {
        BitAdcChsw14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - This field defines the number 15 channel to do a conversion when a conversion event takes place."]
    #[inline(always)]
    pub fn bit_adc_chsw_15(&mut self) -> BitAdcChsw15W<'_, RegAdcChswList1Spec> {
        BitAdcChsw15W::new(self, 28)
    }
}
#[doc = "ADC Channel Switch List 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_chsw_list_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_chsw_list_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcChswList1Spec;
impl crate::RegisterSpec for RegAdcChswList1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_chsw_list_1::R`](R) reader structure"]
impl crate::Readable for RegAdcChswList1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_chsw_list_1::W`](W) writer structure"]
impl crate::Writable for RegAdcChswList1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_CHSW_LIST_1 to value 0"]
impl crate::Resettable for RegAdcChswList1Spec {}
