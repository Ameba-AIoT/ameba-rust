#[doc = "Register `REG_DARFRC1_H` reader"]
pub type R = crate::R<RegDarfrc1HSpec>;
#[doc = "Register `REG_DARFRC1_H` writer"]
pub type W = crate::W<RegDarfrc1HSpec>;
#[doc = "Field `BIT_DATA_FALLBACK1_RC5` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc5R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK1_RC5` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK1_RC6` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc6R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK1_RC6` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK1_RC7` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc7R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK1_RC7` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK1_RC8` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc8R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK1_RC8` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc5(&self) -> BitDataFallback1Rc5R {
        BitDataFallback1Rc5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc6(&self) -> BitDataFallback1Rc6R {
        BitDataFallback1Rc6R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc7(&self) -> BitDataFallback1Rc7R {
        BitDataFallback1Rc7R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc8(&self) -> BitDataFallback1Rc8R {
        BitDataFallback1Rc8R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc5(&mut self) -> BitDataFallback1Rc5W<'_, RegDarfrc1HSpec> {
        BitDataFallback1Rc5W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc6(&mut self) -> BitDataFallback1Rc6W<'_, RegDarfrc1HSpec> {
        BitDataFallback1Rc6W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc7(&mut self) -> BitDataFallback1Rc7W<'_, RegDarfrc1HSpec> {
        BitDataFallback1Rc7W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc8(&mut self) -> BitDataFallback1Rc8W<'_, RegDarfrc1HSpec> {
        BitDataFallback1Rc8W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_darfrc1_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_darfrc1_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDarfrc1HSpec;
impl crate::RegisterSpec for RegDarfrc1HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_darfrc1_h::R`](R) reader structure"]
impl crate::Readable for RegDarfrc1HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_darfrc1_h::W`](W) writer structure"]
impl crate::Writable for RegDarfrc1HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DARFRC1_H to value 0x0101_0101"]
impl crate::Resettable for RegDarfrc1HSpec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
