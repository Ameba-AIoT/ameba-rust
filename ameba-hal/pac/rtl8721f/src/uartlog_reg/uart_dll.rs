#[doc = "Register `UART_DLL` reader"]
pub type R = crate::R<UartDllSpec>;
#[doc = "Register `UART_DLL` writer"]
pub type W = crate::W<UartDllSpec>;
#[doc = "Field `baud` reader - The low 8 bits of divisor coefficient (not used)"]
pub type BaudR = crate::FieldReader;
#[doc = "Field `baud` writer - The low 8 bits of divisor coefficient (not used)"]
pub type BaudW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The low 8 bits of divisor coefficient (not used)"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The low 8 bits of divisor coefficient (not used)"]
    #[inline(always)]
    pub fn baud(&mut self) -> BaudW<'_, UartDllSpec> {
        BaudW::new(self, 0)
    }
}
#[doc = "UART divisor length register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_dll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_dll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartDllSpec;
impl crate::RegisterSpec for UartDllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_dll::R`](R) reader structure"]
impl crate::Readable for UartDllSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_dll::W`](W) writer structure"]
impl crate::Writable for UartDllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_DLL to value 0"]
impl crate::Resettable for UartDllSpec {}
