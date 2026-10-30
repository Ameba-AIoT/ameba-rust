#[doc = "Register `GPIO_INT_EOI` writer"]
pub type W = crate::W<GpioIntEoiSpec>;
#[doc = "Field `GPIO_INT_EOI` writer - Controls the clearing of edge type interrupts from Portx. When a 1 is written into a corresponding bit of this register, the interrupt is cleared. All interrupts are cleared when Portx is not configured for interrupts. 0: No interrupt clear (default) 1: Clear interrupt"]
pub type GpioIntEoiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Controls the clearing of edge type interrupts from Portx. When a 1 is written into a corresponding bit of this register, the interrupt is cleared. All interrupts are cleared when Portx is not configured for interrupts. 0: No interrupt clear (default) 1: Clear interrupt"]
    #[inline(always)]
    pub fn gpio_int_eoi(&mut self) -> GpioIntEoiW<'_, GpioIntEoiSpec> {
        GpioIntEoiW::new(self, 0)
    }
}
#[doc = "Port edge type interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_eoi::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntEoiSpec;
impl crate::RegisterSpec for GpioIntEoiSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_int_eoi::W`](W) writer structure"]
impl crate::Writable for GpioIntEoiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_EOI to value 0"]
impl crate::Resettable for GpioIntEoiSpec {}
