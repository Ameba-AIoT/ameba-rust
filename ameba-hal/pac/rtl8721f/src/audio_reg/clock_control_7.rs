#[doc = "Register `clock_control_7` reader"]
pub type R = crate::R<ClockControl7Spec>;
#[doc = "Register `clock_control_7` writer"]
pub type W = crate::W<ClockControl7Spec>;
#[doc = "Field `adc_0_dmic_lpf_clk_sel` reader - Channel 0 ADC path DMIC LPF clock 2'b00: 10M 2'b01: 5M 2'b10: 2.5M 2'b11: 769.2*2K"]
pub type Adc0DmicLpfClkSelR = crate::FieldReader;
#[doc = "Field `adc_0_dmic_lpf_clk_sel` writer - Channel 0 ADC path DMIC LPF clock 2'b00: 10M 2'b01: 5M 2'b10: 2.5M 2'b11: 769.2*2K"]
pub type Adc0DmicLpfClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_1_dmic_lpf_clk_sel` reader - Channel 1 ADC path DMIC LPF clock 2'b00: 10M 2'b01: 5M 2'b10: 2.5M 2'b11: 769.2*2K"]
pub type Adc1DmicLpfClkSelR = crate::FieldReader;
#[doc = "Field `adc_1_dmic_lpf_clk_sel` writer - Channel 1 ADC path DMIC LPF clock 2'b00: 10M 2'b01: 5M 2'b10: 2.5M 2'b11: 769.2*2K"]
pub type Adc1DmicLpfClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 7:8 - Channel 0 ADC path DMIC LPF clock 2'b00: 10M 2'b01: 5M 2'b10: 2.5M 2'b11: 769.2*2K"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf_clk_sel(&self) -> Adc0DmicLpfClkSelR {
        Adc0DmicLpfClkSelR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Channel 1 ADC path DMIC LPF clock 2'b00: 10M 2'b01: 5M 2'b10: 2.5M 2'b11: 769.2*2K"]
    #[inline(always)]
    pub fn adc_1_dmic_lpf_clk_sel(&self) -> Adc1DmicLpfClkSelR {
        Adc1DmicLpfClkSelR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 7:8 - Channel 0 ADC path DMIC LPF clock 2'b00: 10M 2'b01: 5M 2'b10: 2.5M 2'b11: 769.2*2K"]
    #[inline(always)]
    pub fn adc_0_dmic_lpf_clk_sel(&mut self) -> Adc0DmicLpfClkSelW<'_, ClockControl7Spec> {
        Adc0DmicLpfClkSelW::new(self, 7)
    }
    #[doc = "Bits 9:10 - Channel 1 ADC path DMIC LPF clock 2'b00: 10M 2'b01: 5M 2'b10: 2.5M 2'b11: 769.2*2K"]
    #[inline(always)]
    pub fn adc_1_dmic_lpf_clk_sel(&mut self) -> Adc1DmicLpfClkSelW<'_, ClockControl7Spec> {
        Adc1DmicLpfClkSelW::new(self, 9)
    }
}
#[doc = "Clock Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_control_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_control_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockControl7Spec;
impl crate::RegisterSpec for ClockControl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_control_7::R`](R) reader structure"]
impl crate::Readable for ClockControl7Spec {}
#[doc = "`write(|w| ..)` method takes [`clock_control_7::W`](W) writer structure"]
impl crate::Writable for ClockControl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clock_control_7 to value 0"]
impl crate::Resettable for ClockControl7Spec {}
