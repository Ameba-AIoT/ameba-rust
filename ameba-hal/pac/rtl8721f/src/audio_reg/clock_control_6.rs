#[doc = "Register `clock_control_6` reader"]
pub type R = crate::R<ClockControl6Spec>;
#[doc = "Register `clock_control_6` writer"]
pub type W = crate::W<ClockControl6Spec>;
#[doc = "Field `adc_0_asrc_en` reader - Channel 0 ADC path ASRC enable 0: Disable 1: Enable If ASRC is enabled, sample_rate becomes useless."]
pub type Adc0AsrcEnR = crate::BitReader;
#[doc = "Field `adc_0_asrc_en` writer - Channel 0 ADC path ASRC enable 0: Disable 1: Enable If ASRC is enabled, sample_rate becomes useless."]
pub type Adc0AsrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_asrc_en` reader - Channel 1 ADC path ASRC enable 0: Disable 1: Enable If ASRC is enabled, sample_rate becomes useless."]
pub type Adc1AsrcEnR = crate::BitReader;
#[doc = "Field `adc_1_asrc_en` writer - Channel 1 ADC path ASRC enable 0: Disable 1: Enable If ASRC is enabled, sample_rate becomes useless."]
pub type Adc1AsrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 ADC path ASRC enable 0: Disable 1: Enable If ASRC is enabled, sample_rate becomes useless."]
    #[inline(always)]
    pub fn adc_0_asrc_en(&self) -> Adc0AsrcEnR {
        Adc0AsrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 ADC path ASRC enable 0: Disable 1: Enable If ASRC is enabled, sample_rate becomes useless."]
    #[inline(always)]
    pub fn adc_1_asrc_en(&self) -> Adc1AsrcEnR {
        Adc1AsrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 ADC path ASRC enable 0: Disable 1: Enable If ASRC is enabled, sample_rate becomes useless."]
    #[inline(always)]
    pub fn adc_0_asrc_en(&mut self) -> Adc0AsrcEnW<'_, ClockControl6Spec> {
        Adc0AsrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 ADC path ASRC enable 0: Disable 1: Enable If ASRC is enabled, sample_rate becomes useless."]
    #[inline(always)]
    pub fn adc_1_asrc_en(&mut self) -> Adc1AsrcEnW<'_, ClockControl6Spec> {
        Adc1AsrcEnW::new(self, 1)
    }
}
#[doc = "Clock Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_control_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_control_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockControl6Spec;
impl crate::RegisterSpec for ClockControl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_control_6::R`](R) reader structure"]
impl crate::Readable for ClockControl6Spec {}
#[doc = "`write(|w| ..)` method takes [`clock_control_6::W`](W) writer structure"]
impl crate::Writable for ClockControl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clock_control_6 to value 0"]
impl crate::Resettable for ClockControl6Spec {}
