#[doc = "Register `GPIO_INT_BOTHEDGE` reader"]
pub type R = crate::R<GpioIntBothedgeSpec>;
#[doc = "Register `GPIO_INT_BOTHEDGE` writer"]
pub type W = crate::W<GpioIntBothedgeSpec>;
#[doc = "Field `GPIO_INT_BOTHEDGE` reader - Controls the edge types of interrupt that can occur on Portx. Whenever a particular bit is programmed to 1, it enables the generation of interrupt on both the rising edge and falling edge of an external input signal corresponding to that bit on Port. The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers. 0: Disabled. Single edge sensitive 1: Enabled. Both edge sensitive"]
pub type GpioIntBothedgeR = crate::FieldReader<u32>;
#[doc = "Field `GPIO_INT_BOTHEDGE` writer - Controls the edge types of interrupt that can occur on Portx. Whenever a particular bit is programmed to 1, it enables the generation of interrupt on both the rising edge and falling edge of an external input signal corresponding to that bit on Port. The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers. 0: Disabled. Single edge sensitive 1: Enabled. Both edge sensitive"]
pub type GpioIntBothedgeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Controls the edge types of interrupt that can occur on Portx. Whenever a particular bit is programmed to 1, it enables the generation of interrupt on both the rising edge and falling edge of an external input signal corresponding to that bit on Port. The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers. 0: Disabled. Single edge sensitive 1: Enabled. Both edge sensitive"]
    #[inline(always)]
    pub fn gpio_int_bothedge(&self) -> GpioIntBothedgeR {
        GpioIntBothedgeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls the edge types of interrupt that can occur on Portx. Whenever a particular bit is programmed to 1, it enables the generation of interrupt on both the rising edge and falling edge of an external input signal corresponding to that bit on Port. The values programmed in the registers gpio_intype_level and gpio_int_polarity for this particular bit are not considered when the corresponding bit of this register is set to 1. Whenever a particular bit is programmed to 0, the interrupt type depends on the value of the corresponding bits in the gpio_inttype_level and gpio_int_polarity registers. 0: Disabled. Single edge sensitive 1: Enabled. Both edge sensitive"]
    #[inline(always)]
    pub fn gpio_int_bothedge(&mut self) -> GpioIntBothedgeW<'_, GpioIntBothedgeSpec> {
        GpioIntBothedgeW::new(self, 0)
    }
}
#[doc = "Port both edge interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_bothedge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_bothedge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntBothedgeSpec;
impl crate::RegisterSpec for GpioIntBothedgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_bothedge::R`](R) reader structure"]
impl crate::Readable for GpioIntBothedgeSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_bothedge::W`](W) writer structure"]
impl crate::Writable for GpioIntBothedgeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_BOTHEDGE to value 0"]
impl crate::Resettable for GpioIntBothedgeSpec {}
