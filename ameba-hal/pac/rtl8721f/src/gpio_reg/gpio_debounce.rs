#[doc = "Register `GPIO_DEBOUNCE` reader"]
pub type R = crate::R<GpioDebounceSpec>;
#[doc = "Register `GPIO_DEBOUNCE` writer"]
pub type W = crate::W<GpioDebounceSpec>;
#[doc = "Field `GPIO_DEBOUNCE_EN` reader - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed. 0: No debounce (default) 1: Enable debounce This register is available only if Portx is configured to generate interrupts (GPIO_PORTA_INTR = Include (1)) and when the debounce logic is included (GPIO_DEBOUNCE = Include (1))."]
pub type GpioDebounceEnR = crate::FieldReader<u32>;
#[doc = "Field `GPIO_DEBOUNCE_EN` writer - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed. 0: No debounce (default) 1: Enable debounce This register is available only if Portx is configured to generate interrupts (GPIO_PORTA_INTR = Include (1)) and when the debounce logic is included (GPIO_DEBOUNCE = Include (1))."]
pub type GpioDebounceEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed. 0: No debounce (default) 1: Enable debounce This register is available only if Portx is configured to generate interrupts (GPIO_PORTA_INTR = Include (1)) and when the debounce logic is included (GPIO_DEBOUNCE = Include (1))."]
    #[inline(always)]
    pub fn gpio_debounce_en(&self) -> GpioDebounceEnR {
        GpioDebounceEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls whether an external signal that is the source of an interrupt needs to be debounced to remove any spurious glitches. Writing a 1 to a bit in this register enables the debouncing circuitry. A signal must be valid for two periods of an external clock before it is internally processed. 0: No debounce (default) 1: Enable debounce This register is available only if Portx is configured to generate interrupts (GPIO_PORTA_INTR = Include (1)) and when the debounce logic is included (GPIO_DEBOUNCE = Include (1))."]
    #[inline(always)]
    pub fn gpio_debounce_en(&mut self) -> GpioDebounceEnW<'_, GpioDebounceSpec> {
        GpioDebounceEnW::new(self, 0)
    }
}
#[doc = "Port interrupt debounce register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_debounce::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_debounce::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDebounceSpec;
impl crate::RegisterSpec for GpioDebounceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_debounce::R`](R) reader structure"]
impl crate::Readable for GpioDebounceSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_debounce::W`](W) writer structure"]
impl crate::Writable for GpioDebounceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_DEBOUNCE to value 0"]
impl crate::Resettable for GpioDebounceSpec {}
