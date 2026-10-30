#[doc = "Register `REG_DARFRC1_L` reader"]
pub type R = crate::R<RegDarfrc1LSpec>;
#[doc = "Register `REG_DARFRC1_L` writer"]
pub type W = crate::W<RegDarfrc1LSpec>;
#[doc = "Field `BIT_DATA_FALLBACK1_RC1` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc1R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK1_RC1` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK1_RC2` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc2R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK1_RC2` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK1_RC3` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc3R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK1_RC3` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DATA_FALLBACK1_RC4` reader - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc4R = crate::FieldReader;
#[doc = "Field `BIT_DATA_FALLBACK1_RC4` writer - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitDataFallback1Rc4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc1(&self) -> BitDataFallback1Rc1R {
        BitDataFallback1Rc1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc2(&self) -> BitDataFallback1Rc2R {
        BitDataFallback1Rc2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc3(&self) -> BitDataFallback1Rc3R {
        BitDataFallback1Rc3R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc4(&self) -> BitDataFallback1Rc4R {
        BitDataFallback1Rc4R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc1(&mut self) -> BitDataFallback1Rc1W<'_, RegDarfrc1LSpec> {
        BitDataFallback1Rc1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc2(&mut self) -> BitDataFallback1Rc2W<'_, RegDarfrc1LSpec> {
        BitDataFallback1Rc2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc3(&mut self) -> BitDataFallback1Rc3W<'_, RegDarfrc1LSpec> {
        BitDataFallback1Rc3W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Data Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_data_fallback1_rc4(&mut self) -> BitDataFallback1Rc4W<'_, RegDarfrc1LSpec> {
        BitDataFallback1Rc4W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_darfrc1_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_darfrc1_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDarfrc1LSpec;
impl crate::RegisterSpec for RegDarfrc1LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_darfrc1_l::R`](R) reader structure"]
impl crate::Readable for RegDarfrc1LSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_darfrc1_l::W`](W) writer structure"]
impl crate::Writable for RegDarfrc1LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DARFRC1_L to value 0x0101_0101"]
impl crate::Resettable for RegDarfrc1LSpec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
