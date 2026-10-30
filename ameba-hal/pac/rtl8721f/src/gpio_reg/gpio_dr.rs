#[doc = "Register `GPIO_DR` reader"]
pub type R = crate::R<GpioDrSpec>;
#[doc = "Register `GPIO_DR` writer"]
pub type W = crate::W<GpioDrSpec>;
#[doc = "Field `PORT_DATA` reader - Values written to this register are output on the I/O signals for Portx if the corresponding data direction bits for Portx are set to Output mode and the corresponding control bit for Portx is set to software mode. The value read back is equal to the last value written to this register."]
pub type PortDataR = crate::FieldReader<u32>;
#[doc = "Field `PORT_DATA` writer - Values written to this register are output on the I/O signals for Portx if the corresponding data direction bits for Portx are set to Output mode and the corresponding control bit for Portx is set to software mode. The value read back is equal to the last value written to this register."]
pub type PortDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Values written to this register are output on the I/O signals for Portx if the corresponding data direction bits for Portx are set to Output mode and the corresponding control bit for Portx is set to software mode. The value read back is equal to the last value written to this register."]
    #[inline(always)]
    pub fn port_data(&self) -> PortDataR {
        PortDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Values written to this register are output on the I/O signals for Portx if the corresponding data direction bits for Portx are set to Output mode and the corresponding control bit for Portx is set to software mode. The value read back is equal to the last value written to this register."]
    #[inline(always)]
    pub fn port_data(&mut self) -> PortDataW<'_, GpioDrSpec> {
        PortDataW::new(self, 0)
    }
}
#[doc = "Port data register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDrSpec;
impl crate::RegisterSpec for GpioDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_dr::R`](R) reader structure"]
impl crate::Readable for GpioDrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_dr::W`](W) writer structure"]
impl crate::Writable for GpioDrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_DR to value 0"]
impl crate::Resettable for GpioDrSpec {}
