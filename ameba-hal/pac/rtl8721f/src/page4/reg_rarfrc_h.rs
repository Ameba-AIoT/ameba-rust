#[doc = "Register `REG_RARFRC_H` reader"]
pub type R = crate::R<RegRarfrcHSpec>;
#[doc = "Register `REG_RARFRC_H` writer"]
pub type W = crate::W<RegRarfrcHSpec>;
#[doc = "Field `BIT_CTRL_FALLBACK_RC5` reader - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc5R = crate::FieldReader;
#[doc = "Field `BIT_CTRL_FALLBACK_RC5` writer - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_CTRL_FALLBACK_RC6` reader - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc6R = crate::FieldReader;
#[doc = "Field `BIT_CTRL_FALLBACK_RC6` writer - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_CTRL_FALLBACK_RC7` reader - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc7R = crate::FieldReader;
#[doc = "Field `BIT_CTRL_FALLBACK_RC7` writer - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_CTRL_FALLBACK_RC8` reader - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc8R = crate::FieldReader;
#[doc = "Field `BIT_CTRL_FALLBACK_RC8` writer - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
pub type BitCtrlFallbackRc8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc5(&self) -> BitCtrlFallbackRc5R {
        BitCtrlFallbackRc5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc6(&self) -> BitCtrlFallbackRc6R {
        BitCtrlFallbackRc6R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc7(&self) -> BitCtrlFallbackRc7R {
        BitCtrlFallbackRc7R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc8(&self) -> BitCtrlFallbackRc8R {
        BitCtrlFallbackRc8R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc5(&mut self) -> BitCtrlFallbackRc5W<'_, RegRarfrcHSpec> {
        BitCtrlFallbackRc5W::new(self, 0)
    }
    #[doc = "Bits 8:12 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc6(&mut self) -> BitCtrlFallbackRc6W<'_, RegRarfrcHSpec> {
        BitCtrlFallbackRc6W::new(self, 8)
    }
    #[doc = "Bits 16:20 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc7(&mut self) -> BitCtrlFallbackRc7W<'_, RegRarfrcHSpec> {
        BitCtrlFallbackRc7W::new(self, 16)
    }
    #[doc = "Bits 24:28 - RTS Auto Rate Fallback Retry Count After the number of retry, HW will fall back the rate to the next enabled MAC in ARFR."]
    #[inline(always)]
    pub fn bit_ctrl_fallback_rc8(&mut self) -> BitCtrlFallbackRc8W<'_, RegRarfrcHSpec> {
        BitCtrlFallbackRc8W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rarfrc_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rarfrc_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRarfrcHSpec;
impl crate::RegisterSpec for RegRarfrcHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rarfrc_h::R`](R) reader structure"]
impl crate::Readable for RegRarfrcHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rarfrc_h::W`](W) writer structure"]
impl crate::Writable for RegRarfrcHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RARFRC_H to value 0x0807_0605"]
impl crate::Resettable for RegRarfrcHSpec {
    const RESET_VALUE: u32 = 0x0807_0605;
}
