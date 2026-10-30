#[doc = "Register `REG_DARFRC0_H` reader"]
pub type R = crate::R<RegDarfrc0HSpec>;
#[doc = "Register `REG_DARFRC0_H` writer"]
pub type W = crate::W<RegDarfrc0HSpec>;
#[doc = "Field `BIT_DATA_FALLBACK_RC5` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc5R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK_RC5` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK_RC6` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc6R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK_RC6` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK_RC7` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc7R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK_RC7` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK_RC8` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc8R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK_RC8` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc5(&self) -> BitDataFallbackRc5R {
        BitDataFallbackRc5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc6(&self) -> BitDataFallbackRc6R {
        BitDataFallbackRc6R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc7(&self) -> BitDataFallbackRc7R {
        BitDataFallbackRc7R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc8(&self) -> BitDataFallbackRc8R {
        BitDataFallbackRc8R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc5(&mut self) -> BitDataFallbackRc5W<'_, RegDarfrc0HSpec> {
        BitDataFallbackRc5W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc6(&mut self) -> BitDataFallbackRc6W<'_, RegDarfrc0HSpec> {
        BitDataFallbackRc6W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc7(&mut self) -> BitDataFallbackRc7W<'_, RegDarfrc0HSpec> {
        BitDataFallbackRc7W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc8(&mut self) -> BitDataFallbackRc8W<'_, RegDarfrc0HSpec> {
        BitDataFallbackRc8W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_darfrc0_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_darfrc0_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDarfrc0HSpec;
impl crate::RegisterSpec for RegDarfrc0HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_darfrc0_h::R`](R) reader structure"]
impl crate::Readable for RegDarfrc0HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_darfrc0_h::W`](W) writer structure"]
impl crate::Writable for RegDarfrc0HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DARFRC0_H to value 0x0101_0101"]
impl crate::Resettable for RegDarfrc0HSpec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
