#[doc = "Register `REG_DARFRC0_L` reader"]
pub type R = crate::R<RegDarfrc0LSpec>;
#[doc = "Register `REG_DARFRC0_L` writer"]
pub type W = crate::W<RegDarfrc0LSpec>;
#[doc = "Field `BIT_DATA_FALLBACK_RC1` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc1R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK_RC1` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK_RC2` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc2R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK_RC2` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK_RC3` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc3R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK_RC3` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK_RC4` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc4R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK_RC4` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallbackRc4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc1(&self) -> BitDataFallbackRc1R {
        BitDataFallbackRc1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc2(&self) -> BitDataFallbackRc2R {
        BitDataFallbackRc2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc3(&self) -> BitDataFallbackRc3R {
        BitDataFallbackRc3R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc4(&self) -> BitDataFallbackRc4R {
        BitDataFallbackRc4R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc1(&mut self) -> BitDataFallbackRc1W<'_, RegDarfrc0LSpec> {
        BitDataFallbackRc1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc2(&mut self) -> BitDataFallbackRc2W<'_, RegDarfrc0LSpec> {
        BitDataFallbackRc2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc3(&mut self) -> BitDataFallbackRc3W<'_, RegDarfrc0LSpec> {
        BitDataFallbackRc3W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback_rc4(&mut self) -> BitDataFallbackRc4W<'_, RegDarfrc0LSpec> {
        BitDataFallbackRc4W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_darfrc0_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_darfrc0_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDarfrc0LSpec;
impl crate::RegisterSpec for RegDarfrc0LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_darfrc0_l::R`](R) reader structure"]
impl crate::Readable for RegDarfrc0LSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_darfrc0_l::W`](W) writer structure"]
impl crate::Writable for RegDarfrc0LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DARFRC0_L to value 0x0101_0101"]
impl crate::Resettable for RegDarfrc0LSpec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
