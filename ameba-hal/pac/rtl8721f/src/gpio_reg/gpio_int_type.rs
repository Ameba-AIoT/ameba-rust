#[doc = "Register `GPIO_INT_TYPE` reader"]
pub type R = crate::R<GpioIntTypeSpec>;
#[doc = "Register `GPIO_INT_TYPE` writer"]
pub type W = crate::W<GpioIntTypeSpec>;
#[doc = "Field `GPIO_INT_TYPE` reader - Controls the type of interrupt that can occur on Portx. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level-sensitive; otherwise, it is edge-sensitive. 0: Level-sensitive (default) 1: Edge-sensitive"]
pub type GpioIntTypeR = crate::FieldReader<u32>;
#[doc = "Field `GPIO_INT_TYPE` writer - Controls the type of interrupt that can occur on Portx. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level-sensitive; otherwise, it is edge-sensitive. 0: Level-sensitive (default) 1: Edge-sensitive"]
pub type GpioIntTypeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Controls the type of interrupt that can occur on Portx. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level-sensitive; otherwise, it is edge-sensitive. 0: Level-sensitive (default) 1: Edge-sensitive"]
    #[inline(always)]
    pub fn gpio_int_type(&self) -> GpioIntTypeR {
        GpioIntTypeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the type of interrupt that can occur on Portx. Whenever a 0 is written to a bit of this register, it configures the interrupt type to be level-sensitive; otherwise, it is edge-sensitive. 0: Level-sensitive (default) 1: Edge-sensitive"]
    #[inline(always)]
    pub fn gpio_int_type(&mut self) -> GpioIntTypeW<'_, GpioIntTypeSpec> {
        GpioIntTypeW::new(self, 0)
    }
}
#[doc = "Port interrupt trigger type register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntTypeSpec;
impl crate::RegisterSpec for GpioIntTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_type::R`](R) reader structure"]
impl crate::Readable for GpioIntTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_type::W`](W) writer structure"]
impl crate::Writable for GpioIntTypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_TYPE to value 0"]
impl crate::Resettable for GpioIntTypeSpec {}
