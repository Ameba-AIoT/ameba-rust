#[doc = "Register `GPIO_EXT_PORT` reader"]
pub type R = crate::R<GpioExtPortSpec>;
#[doc = "Field `GPIO_EXT_PORTA` reader - When Portx is configured as Input, then reading this location reads the values on the signal. When the data direction of Portx is set as Output, reading this location reads the data register for Portx."]
pub type GpioExtPortaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - When Portx is configured as Input, then reading this location reads the values on the signal. When the data direction of Portx is set as Output, reading this location reads the data register for Portx."]
    #[inline(always)]
    pub fn gpio_ext_porta(&self) -> GpioExtPortaR {
        GpioExtPortaR::new(self.bits)
    }
}
#[doc = "External Port data register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_port::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioExtPortSpec;
impl crate::RegisterSpec for GpioExtPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_port::R`](R) reader structure"]
impl crate::Readable for GpioExtPortSpec {}
#[doc = "`reset()` method sets GPIO_EXT_PORT to value 0"]
impl crate::Resettable for GpioExtPortSpec {}
