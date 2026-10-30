#[doc = "Register `clock_control_5` reader"]
pub type R = crate::R<ClockControl5Spec>;
#[doc = "Register `clock_control_5` writer"]
pub type W = crate::W<ClockControl5Spec>;
#[doc = "Field `adc_0_fs_src_sel` reader - Channel 0 ADC path sample rate source selection 2'b00: Source 0 2'b01: Source 1 2'b10: Source 2"]
pub type Adc0FsSrcSelR = crate::FieldReader;
#[doc = "Field `adc_0_fs_src_sel` writer - Channel 0 ADC path sample rate source selection 2'b00: Source 0 2'b01: Source 1 2'b10: Source 2"]
pub type Adc0FsSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `adc_1_fs_src_sel` reader - Channel 1 ADC path sample rate source selection 2'b00: Source 0 2'b01: Source 1 2'b10: Source 2"]
pub type Adc1FsSrcSelR = crate::FieldReader;
#[doc = "Field `adc_1_fs_src_sel` writer - Channel 1 ADC path sample rate source selection 2'b00: Source 0 2'b01: Source 1 2'b10: Source 2"]
pub type Adc1FsSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Channel 0 ADC path sample rate source selection 2'b00: Source 0 2'b01: Source 1 2'b10: Source 2"]
    #[inline(always)]
    pub fn adc_0_fs_src_sel(&self) -> Adc0FsSrcSelR {
        Adc0FsSrcSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 ADC path sample rate source selection 2'b00: Source 0 2'b01: Source 1 2'b10: Source 2"]
    #[inline(always)]
    pub fn adc_1_fs_src_sel(&self) -> Adc1FsSrcSelR {
        Adc1FsSrcSelR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 ADC path sample rate source selection 2'b00: Source 0 2'b01: Source 1 2'b10: Source 2"]
    #[inline(always)]
    pub fn adc_0_fs_src_sel(&mut self) -> Adc0FsSrcSelW<'_, ClockControl5Spec> {
        Adc0FsSrcSelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 1 ADC path sample rate source selection 2'b00: Source 0 2'b01: Source 1 2'b10: Source 2"]
    #[inline(always)]
    pub fn adc_1_fs_src_sel(&mut self) -> Adc1FsSrcSelW<'_, ClockControl5Spec> {
        Adc1FsSrcSelW::new(self, 2)
    }
}
#[doc = "Clock Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_control_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_control_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockControl5Spec;
impl crate::RegisterSpec for ClockControl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_control_5::R`](R) reader structure"]
impl crate::Readable for ClockControl5Spec {}
#[doc = "`write(|w| ..)` method takes [`clock_control_5::W`](W) writer structure"]
impl crate::Writable for ClockControl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets clock_control_5 to value 0"]
impl crate::Resettable for ClockControl5Spec {}
