#[doc = "Register `GPIO_INT_MASK` reader"]
pub type R = crate::R<GpioIntMaskSpec>;
#[doc = "Register `GPIO_INT_MASK` writer"]
pub type W = crate::W<GpioIntMaskSpec>;
#[doc = "Field `GPIO_INT_MASK` reader - Controls whether an interrupt on Portx can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking. 0: Interrupt bits are unmasked (default) 1: Mask interrupt"]
pub type GpioIntMaskR = crate::FieldReader<u32>;
#[doc = "Field `GPIO_INT_MASK` writer - Controls whether an interrupt on Portx can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking. 0: Interrupt bits are unmasked (default) 1: Mask interrupt"]
pub type GpioIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Controls whether an interrupt on Portx can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking. 0: Interrupt bits are unmasked (default) 1: Mask interrupt"]
    #[inline(always)]
    pub fn gpio_int_mask(&self) -> GpioIntMaskR {
        GpioIntMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Controls whether an interrupt on Portx can create an interrupt for the interrupt controller by not masking it. By default, all interrupts bits are unmasked. Whenever a 1 is written to a bit in this register, it masks the interrupt generation capability for this signal; otherwise interrupts are allowed through. The unmasked status can be read as well as the resultant status after masking. 0: Interrupt bits are unmasked (default) 1: Mask interrupt"]
    #[inline(always)]
    pub fn gpio_int_mask(&mut self) -> GpioIntMaskW<'_, GpioIntMaskSpec> {
        GpioIntMaskW::new(self, 0)
    }
}
#[doc = "Port interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntMaskSpec;
impl crate::RegisterSpec for GpioIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_mask::R`](R) reader structure"]
impl crate::Readable for GpioIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_int_mask::W`](W) writer structure"]
impl crate::Writable for GpioIntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INT_MASK to value 0"]
impl crate::Resettable for GpioIntMaskSpec {}
