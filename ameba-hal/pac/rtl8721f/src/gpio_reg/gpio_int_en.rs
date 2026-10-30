#[doc = "Register `GPIO_INT_EN` reader"]
pub type R = crate::R<GpioIntEnSpec>;
#[doc = "Register `GPIO_INT_EN` writer"]
pub type W = crate::W<GpioIntEnSpec>;
#[doc = "Field `GPIO_INT_EN` reader - Allows each bit of Portx to be configured for interrupts. By default, the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Portx to become an interrupt; otherwise, Portx operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Portx if the corresponding data direction register is set to Output or if Portx mode is set to Hardware. 0: Configure Portx bit as normal GPIO signal (default) 1: Configure Portx bit as interrupt"]
pub type GpioIntEnR = crate::FieldReader<u32>;
#[doc = "Field `GPIO_INT_EN` writer - Allows each bit of Portx to be configured for interrupts. By default, the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Portx to become an interrupt; otherwise, Portx operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Portx if the corresponding data direction register is set to Output or if Portx mode is set to Hardware. 0: Configure Portx bit as normal GPIO signal (default) 1: Configure Portx bit as interrupt"]
pub type GpioIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Allows each bit of Portx to be configured for interrupts. By default, the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Portx to become an interrupt; otherwise, Portx operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Portx if the corresponding data direction register is set to Output or if Portx mode is set to Hardware. 0: Configure Portx bit as normal GPIO signal (default) 1: Configure Portx bit as interrupt"]
    #[inline(always)]
    pub fn gpio_int_en(&self) -> GpioIntEnR {
        GpioIntEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Allows each bit of Portx to be configured for interrupts. By default, the generation of interrupts is disabled. Whenever a 1 is written to a bit of this register, it configures the corresponding bit on Portx to become an interrupt; otherwise, Portx operates as a normal GPIO signal. Interrupts are disabled on the corresponding bits of Portx if the corresponding data direction register is set to Output or if Portx mode is set to Hardware. 0: Configure Portx bit as normal GPIO signal (default) 1: Configure Portx bit as interrupt"]
    #[inline(always)]
    pub fn gpio_int_en(&mut self) -> GpioIntEnW<'_, GpioIntEnSpec> {
        GpioIntEnW::new(self, 0)
    }
}
#[doc = "Port interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntEnSpec;
impl crate::RegisterSpec for GpioIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_en::R`](R) reader structure"]
impl crate::Readable for GpioIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_en::W`](W) writer structure"]
impl crate::Writable for GpioIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_EN to value 0"]
impl crate::Resettable for GpioIntEnSpec {}
