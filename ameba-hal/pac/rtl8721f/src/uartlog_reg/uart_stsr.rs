#[doc = "Register `UART_STSR` reader"]
pub type R = crate::R<UartStsrSpec>;
#[doc = "Register `UART_STSR` writer"]
pub type W = crate::W<UartStsrSpec>;
#[doc = "Field `xfactor` reader - Factor of Baud rate calculation for Tx path"]
pub type XfactorR = crate::FieldReader<u32>;
#[doc = "Field `xfactor` writer - Factor of Baud rate calculation for Tx path"]
pub type XfactorW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 4:23 - Factor of Baud rate calculation for Tx path"]
    #[inline(always)]
    pub fn xfactor(&self) -> XfactorR {
        XfactorR::new((self.bits >> 4) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 4:23 - Factor of Baud rate calculation for Tx path"]
    #[inline(always)]
    pub fn xfactor(&mut self) -> XfactorW<'_, UartStsrSpec> {
        XfactorW::new(self, 4)
    }
}
#[doc = "UART STS register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_stsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_stsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartStsrSpec;
impl crate::RegisterSpec for UartStsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_stsr::R`](R) reader structure"]
impl crate::Readable for UartStsrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_stsr::W`](W) writer structure"]
impl crate::Writable for UartStsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_STSR to value 0x0001_0470"]
impl crate::Resettable for UartStsrSpec {
    const RESET_VALUE: u32 = 0x0001_0470;
}
