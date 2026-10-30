#[doc = "Register `REG_RARFRC_L` reader"]
pub type R = crate::R<RegRarfrcLSpec>;
#[doc = "Register `REG_RARFRC_L` writer"]
pub type W = crate::W<RegRarfrcLSpec>;
#[doc = "Field `BIT_CTRL_FALLBACK_RC1` reader - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc1R = crate::FieldReader;
#[doc = "Field `BIT_CTRL_FALLBACK_RC1` writer - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_CTRL_FALLBACK_RC2` reader - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc2R = crate::FieldReader;
#[doc = "Field `BIT_CTRL_FALLBACK_RC2` writer - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_CTRL_FALLBACK_RC3` reader - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc3R = crate::FieldReader;
#[doc = "Field `BIT_CTRL_FALLBACK_RC3` writer - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_CTRL_FALLBACK_RC4` reader - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc4R = crate::FieldReader;
#[doc = "Field `BIT_CTRL_FALLBACK_RC4` writer - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc1(&self) -> BitCtrlFallbackRc1R {
        BitCtrlFallbackRc1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc2(&self) -> BitCtrlFallbackRc2R {
        BitCtrlFallbackRc2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc3(&self) -> BitCtrlFallbackRc3R {
        BitCtrlFallbackRc3R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc4(&self) -> BitCtrlFallbackRc4R {
        BitCtrlFallbackRc4R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc1(&mut self) -> BitCtrlFallbackRc1W<'_, RegRarfrcLSpec> {
        BitCtrlFallbackRc1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc2(&mut self) -> BitCtrlFallbackRc2W<'_, RegRarfrcLSpec> {
        BitCtrlFallbackRc2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc3(&mut self) -> BitCtrlFallbackRc3W<'_, RegRarfrcLSpec> {
        BitCtrlFallbackRc3W::new(self, 16)
    }
    #[doc = "Bits 24:28 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc4(&mut self) -> BitCtrlFallbackRc4W<'_, RegRarfrcLSpec> {
        BitCtrlFallbackRc4W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rarfrc_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rarfrc_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRarfrcLSpec;
impl crate::RegisterSpec for RegRarfrcLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rarfrc_l::R`](R) reader structure"]
impl crate::Readable for RegRarfrcLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rarfrc_l::W`](W) writer structure"]
impl crate::Writable for RegRarfrcLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RARFRC_L to value 0x0403_0201"]
impl crate::Resettable for RegRarfrcLSpec {
    const RESET_VALUE: u32 = 0x0403_0201;
}
