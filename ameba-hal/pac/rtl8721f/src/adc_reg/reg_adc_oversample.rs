#[doc = "Register `REG_ADC_OVERSAMPLE` reader"]
pub type R = crate::R<RegAdcOversampleSpec>;
#[doc = "Register `REG_ADC_OVERSAMPLE` writer"]
pub type W = crate::W<RegAdcOversampleSpec>;
#[doc = "Field `bit_adc_os_en` reader - Over sample enable 1-enable; 0-disable"]
pub type BitAdcOsEnR = crate::BitReader;
#[doc = "Field `bit_adc_os_en` writer - Over sample enable 1-enable; 0-disable"]
pub type BitAdcOsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_ov_mode` reader - Oversample mode 1-All oversampling conversions done in regular sequence 0-All oversampling conversions done in staggered sequence"]
pub type BitOvModeR = crate::BitReader;
#[doc = "Field `bit_ov_mode` writer - Oversample mode 1-All oversampling conversions done in regular sequence 0-All oversampling conversions done in staggered sequence"]
pub type BitOvModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bit_adc_osr` reader - over sample ratio 000: 2x 001: 4x 010: 8x 011: 16x 100: 32x 101: 64x 110: 128x 111: 256x"]
pub type BitAdcOsrR = crate::FieldReader;
#[doc = "Field `bit_adc_osr` writer - over sample ratio 000: 2x 001: 4x 010: 8x 011: 16x 100: 32x 101: 64x 110: 128x 111: 256x"]
pub type BitAdcOsrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `bit_adc_osf` reader - oversampling right shift 000: no shift 001: shift 1-bit 010: shift 2-bit 011: shift 3-bit 100: shift 4-bit"]
pub type BitAdcOsfR = crate::FieldReader;
#[doc = "Field `bit_adc_osf` writer - oversampling right shift 000: no shift 001: shift 1-bit 010: shift 2-bit 011: shift 3-bit 100: shift 4-bit"]
pub type BitAdcOsfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Over sample enable 1-enable; 0-disable"]
    #[inline(always)]
    pub fn bit_adc_os_en(&self) -> BitAdcOsEnR {
        BitAdcOsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oversample mode 1-All oversampling conversions done in regular sequence 0-All oversampling conversions done in staggered sequence"]
    #[inline(always)]
    pub fn bit_ov_mode(&self) -> BitOvModeR {
        BitOvModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:5 - over sample ratio 000: 2x 001: 4x 010: 8x 011: 16x 100: 32x 101: 64x 110: 128x 111: 256x"]
    #[inline(always)]
    pub fn bit_adc_osr(&self) -> BitAdcOsrR {
        BitAdcOsrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - oversampling right shift 000: no shift 001: shift 1-bit 010: shift 2-bit 011: shift 3-bit 100: shift 4-bit"]
    #[inline(always)]
    pub fn bit_adc_osf(&self) -> BitAdcOsfR {
        BitAdcOsfR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Over sample enable 1-enable; 0-disable"]
    #[inline(always)]
    pub fn bit_adc_os_en(&mut self) -> BitAdcOsEnW<'_, RegAdcOversampleSpec> {
        BitAdcOsEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Oversample mode 1-All oversampling conversions done in regular sequence 0-All oversampling conversions done in staggered sequence"]
    #[inline(always)]
    pub fn bit_ov_mode(&mut self) -> BitOvModeW<'_, RegAdcOversampleSpec> {
        BitOvModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - over sample ratio 000: 2x 001: 4x 010: 8x 011: 16x 100: 32x 101: 64x 110: 128x 111: 256x"]
    #[inline(always)]
    pub fn bit_adc_osr(&mut self) -> BitAdcOsrW<'_, RegAdcOversampleSpec> {
        BitAdcOsrW::new(self, 3)
    }
    #[doc = "Bits 6:8 - oversampling right shift 000: no shift 001: shift 1-bit 010: shift 2-bit 011: shift 3-bit 100: shift 4-bit"]
    #[inline(always)]
    pub fn bit_adc_osf(&mut self) -> BitAdcOsfW<'_, RegAdcOversampleSpec> {
        BitAdcOsfW::new(self, 6)
    }
}
#[doc = "ADC Over Sample Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_adc_oversample::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_adc_oversample::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAdcOversampleSpec;
impl crate::RegisterSpec for RegAdcOversampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_adc_oversample::R`](R) reader structure"]
impl crate::Readable for RegAdcOversampleSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_adc_oversample::W`](W) writer structure"]
impl crate::Writable for RegAdcOversampleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ADC_OVERSAMPLE to value 0"]
impl crate::Resettable for RegAdcOversampleSpec {}
