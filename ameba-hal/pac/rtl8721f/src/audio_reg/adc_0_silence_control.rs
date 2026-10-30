#[doc = "Register `adc_0_silence_control` reader"]
pub type R = crate::R<Adc0SilenceControlSpec>;
#[doc = "Register `adc_0_silence_control` writer"]
pub type W = crate::W<Adc0SilenceControlSpec>;
#[doc = "Field `adc_0_silence_det_en` reader - Channel 0 ADC path silence detection enable 0: Disable 1: Enable"]
pub type Adc0SilenceDetEnR = crate::BitReader;
#[doc = "Field `adc_0_silence_det_en` writer - Channel 0 ADC path silence detection enable 0: Disable 1: Enable"]
pub type Adc0SilenceDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_silence_level_sel` reader - Channel 0 ADC path silence detection threshold 3'b000: -54db 3'b001: -60db 3'b010: -66db 3'b011: -72db 3'b100: -78db 3'b101: -84db 3'b110: -90db 3'b111: -96db"]
pub type Adc0SilenceLevelSelR = crate::FieldReader;
#[doc = "Field `adc_0_silence_level_sel` writer - Channel 0 ADC path silence detection threshold 3'b000: -54db 3'b001: -60db 3'b010: -66db 3'b011: -72db 3'b100: -78db 3'b101: -84db 3'b110: -90db 3'b111: -96db"]
pub type Adc0SilenceLevelSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `adc_0_silence_debounce_sel` reader - Channel 0 ADC path silence detection debounce (48K) 3'b000: 80ms 3'b001: 160ms 3'b010: 320ms 3'b011: 640ms 3'b100: 1.28s 3'b101: 2.56s 3'b110: 5.12s 3'b111: 0.16ms"]
pub type Adc0SilenceDebounceSelR = crate::FieldReader;
#[doc = "Field `adc_0_silence_debounce_sel` writer - Channel 0 ADC path silence detection debounce (48K) 3'b000: 80ms 3'b001: 160ms 3'b010: 320ms 3'b011: 640ms 3'b100: 1.28s 3'b101: 2.56s 3'b110: 5.12s 3'b111: 0.16ms"]
pub type Adc0SilenceDebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Channel 0 ADC path silence detection enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn adc_0_silence_det_en(&self) -> Adc0SilenceDetEnR {
        Adc0SilenceDetEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Channel 0 ADC path silence detection threshold 3'b000: -54db 3'b001: -60db 3'b010: -66db 3'b011: -72db 3'b100: -78db 3'b101: -84db 3'b110: -90db 3'b111: -96db"]
    #[inline(always)]
    pub fn adc_0_silence_level_sel(&self) -> Adc0SilenceLevelSelR {
        Adc0SilenceLevelSelR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Channel 0 ADC path silence detection debounce (48K) 3'b000: 80ms 3'b001: 160ms 3'b010: 320ms 3'b011: 640ms 3'b100: 1.28s 3'b101: 2.56s 3'b110: 5.12s 3'b111: 0.16ms"]
    #[inline(always)]
    pub fn adc_0_silence_debounce_sel(&self) -> Adc0SilenceDebounceSelR {
        Adc0SilenceDebounceSelR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 ADC path silence detection enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn adc_0_silence_det_en(&mut self) -> Adc0SilenceDetEnW<'_, Adc0SilenceControlSpec> {
        Adc0SilenceDetEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Channel 0 ADC path silence detection threshold 3'b000: -54db 3'b001: -60db 3'b010: -66db 3'b011: -72db 3'b100: -78db 3'b101: -84db 3'b110: -90db 3'b111: -96db"]
    #[inline(always)]
    pub fn adc_0_silence_level_sel(&mut self) -> Adc0SilenceLevelSelW<'_, Adc0SilenceControlSpec> {
        Adc0SilenceLevelSelW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Channel 0 ADC path silence detection debounce (48K) 3'b000: 80ms 3'b001: 160ms 3'b010: 320ms 3'b011: 640ms 3'b100: 1.28s 3'b101: 2.56s 3'b110: 5.12s 3'b111: 0.16ms"]
    #[inline(always)]
    pub fn adc_0_silence_debounce_sel(
        &mut self,
    ) -> Adc0SilenceDebounceSelW<'_, Adc0SilenceControlSpec> {
        Adc0SilenceDebounceSelW::new(self, 4)
    }
}
#[doc = "ADC Channel 0 Silence Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_0_silence_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_0_silence_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0SilenceControlSpec;
impl crate::RegisterSpec for Adc0SilenceControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_0_silence_control::R`](R) reader structure"]
impl crate::Readable for Adc0SilenceControlSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_0_silence_control::W`](W) writer structure"]
impl crate::Writable for Adc0SilenceControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets adc_0_silence_control to value 0x32"]
impl crate::Resettable for Adc0SilenceControlSpec {
    const RESET_VALUE: u32 = 0x32;
}
