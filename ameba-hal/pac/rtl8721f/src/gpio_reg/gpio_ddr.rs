#[doc = "Register `GPIO_DDR` reader"]
pub type R = crate::R<GpioDdrSpec>;
#[doc = "Register `GPIO_DDR` writer"]
pub type W = crate::W<GpioDdrSpec>;
#[doc = "Field `PORT_DATA_DIRECTION` reader - Values written to this register independently control the direction of the corresponding data bit in Portx. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_x parameter. 0: Input (default) 1: Output"]
pub type PortDataDirectionR = crate::FieldReader<u32>;
#[doc = "Field `PORT_DATA_DIRECTION` writer - Values written to this register independently control the direction of the corresponding data bit in Portx. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_x parameter. 0: Input (default) 1: Output"]
pub type PortDataDirectionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Values written to this register independently control the direction of the corresponding data bit in Portx. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_x parameter. 0: Input (default) 1: Output"]
    #[inline(always)]
    pub fn port_data_direction(&self) -> PortDataDirectionR {
        PortDataDirectionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Values written to this register independently control the direction of the corresponding data bit in Portx. The default direction can be configured as input or output after system reset through the GPIO_DFLT_DIR_x parameter. 0: Input (default) 1: Output"]
    #[inline(always)]
    pub fn port_data_direction(&mut self) -> PortDataDirectionW<'_, GpioDdrSpec> {
        PortDataDirectionW::new(self, 0)
    }
}
#[doc = "Port data direction register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDdrSpec;
impl crate::RegisterSpec for GpioDdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ddr::R`](R) reader structure"]
impl crate::Readable for GpioDdrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_ddr::W`](W) writer structure"]
impl crate::Writable for GpioDdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_DDR to value 0"]
impl crate::Resettable for GpioDdrSpec {}
