#[doc = "Register `GPIO_INT_POLARITY` reader"]
pub type R = crate::R<GpioIntPolaritySpec>;
#[doc = "Register `GPIO_INT_POLARITY` writer"]
pub type W = crate::W<GpioIntPolaritySpec>;
#[doc = "Field `GPIO_INT_POLARITY` reader - Controls the polarity of edge or level sensitivity that can occur on input of Portx. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive. 0: Active-low (default) 1: Active-high"]
pub type GpioIntPolarityR = crate::FieldReader<u32>;
#[doc = "Field `GPIO_INT_POLARITY` writer - Controls the polarity of edge or level sensitivity that can occur on input of Portx. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive. 0: Active-low (default) 1: Active-high"]
pub type GpioIntPolarityW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Controls the polarity of edge or level sensitivity that can occur on input of Portx. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive. 0: Active-low (default) 1: Active-high"]
    #[inline(always)]
    pub fn gpio_int_polarity(&self) -> GpioIntPolarityR {
        GpioIntPolarityR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the polarity of edge or level sensitivity that can occur on input of Portx. Whenever a 0 is written to a bit of this register, it configures the interrupt type to falling-edge or active-low sensitive; otherwise, it is rising-edge or active-high sensitive. 0: Active-low (default) 1: Active-high"]
    #[inline(always)]
    pub fn gpio_int_polarity(&mut self) -> GpioIntPolarityW<'_, GpioIntPolaritySpec> {
        GpioIntPolarityW::new(self, 0)
    }
}
#[doc = "Port interrupt polarity register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntPolaritySpec;
impl crate::RegisterSpec for GpioIntPolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_polarity::R`](R) reader structure"]
impl crate::Readable for GpioIntPolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_polarity::W`](W) writer structure"]
impl crate::Writable for GpioIntPolaritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_POLARITY to value 0"]
impl crate::Resettable for GpioIntPolaritySpec {}
