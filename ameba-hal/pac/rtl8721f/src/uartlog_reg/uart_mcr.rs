#[doc = "Register `UART_MCR` reader"]
pub type R = crate::R<UartMcrSpec>;
#[doc = "Register `UART_MCR` writer"]
pub type W = crate::W<UartMcrSpec>;
#[doc = "Field `dtr` reader - Data Terminal Ready (DTR) signal control * 0: DTR is logic 1. * 1: DTR is logic 0. This bit controls the DTR output."]
pub type DtrR = crate::BitReader;
#[doc = "Field `dtr` writer - Data Terminal Ready (DTR) signal control * 0: DTR is logic 1. * 1: DTR is logic 0. This bit controls the DTR output."]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rts` reader - Request to Send (RTS) signal control * 0: RTS is logic 1. * 1: RTS is logic 0. This bit controls the RTS_ output, Bit 1 affects the RTS_ output in a manner identical to that described above set to 0."]
pub type RtsR = crate::BitReader;
#[doc = "Field `rts` writer - Request to Send (RTS) signal control * 0: RTS is logic 1. * 1: RTS is logic 0. This bit controls the RTS_ output, Bit 1 affects the RTS_ output in a manner identical to that described above set to 0."]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `out1` reader - Auxiliary output 1 Bit3 affects the OUT1_ output in a manner identical to that described above set to 0 In loopback mode, connected to Ring Indicator (RI)"]
pub type Out1R = crate::BitReader;
#[doc = "Field `out1` writer - Auxiliary output 1 Bit3 affects the OUT1_ output in a manner identical to that described above set to 0 In loopback mode, connected to Ring Indicator (RI)"]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `out2` reader - Auxiliary output 2 Bit\\[3\\] affects the OUT2_ output in a manner identical to that described above set to 0 In loopback mode, connected to Data Carrier Detect (DCD)"]
pub type Out2R = crate::BitReader;
#[doc = "Field `out2` writer - Auxiliary output 2 Bit\\[3\\] affects the OUT2_ output in a manner identical to that described above set to 0 In loopback mode, connected to Data Carrier Detect (DCD)"]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `loop_en` reader - LoopBack mode * 0: Normal operation mode * 1: Loopback mode"]
pub type LoopEnR = crate::BitReader;
#[doc = "Field `loop_en` writer - LoopBack mode * 0: Normal operation mode * 1: Loopback mode"]
pub type LoopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `afe` reader - Auto flow control enable"]
pub type AfeR = crate::BitReader;
#[doc = "Field `afe` writer - Auto flow control enable"]
pub type AfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Terminal Ready (DTR) signal control * 0: DTR is logic 1. * 1: DTR is logic 0. This bit controls the DTR output."]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request to Send (RTS) signal control * 0: RTS is logic 1. * 1: RTS is logic 0. This bit controls the RTS_ output, Bit 1 affects the RTS_ output in a manner identical to that described above set to 0."]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auxiliary output 1 Bit3 affects the OUT1_ output in a manner identical to that described above set to 0 In loopback mode, connected to Ring Indicator (RI)"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auxiliary output 2 Bit\\[3\\] affects the OUT2_ output in a manner identical to that described above set to 0 In loopback mode, connected to Data Carrier Detect (DCD)"]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LoopBack mode * 0: Normal operation mode * 1: Loopback mode"]
    #[inline(always)]
    pub fn loop_en(&self) -> LoopEnR {
        LoopEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto flow control enable"]
    #[inline(always)]
    pub fn afe(&self) -> AfeR {
        AfeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Terminal Ready (DTR) signal control * 0: DTR is logic 1. * 1: DTR is logic 0. This bit controls the DTR output."]
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<'_, UartMcrSpec> {
        DtrW::new(self, 0)
    }
    #[doc = "Bit 1 - Request to Send (RTS) signal control * 0: RTS is logic 1. * 1: RTS is logic 0. This bit controls the RTS_ output, Bit 1 affects the RTS_ output in a manner identical to that described above set to 0."]
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<'_, UartMcrSpec> {
        RtsW::new(self, 1)
    }
    #[doc = "Bit 2 - Auxiliary output 1 Bit3 affects the OUT1_ output in a manner identical to that described above set to 0 In loopback mode, connected to Ring Indicator (RI)"]
    #[inline(always)]
    pub fn out1(&mut self) -> Out1W<'_, UartMcrSpec> {
        Out1W::new(self, 2)
    }
    #[doc = "Bit 3 - Auxiliary output 2 Bit\\[3\\] affects the OUT2_ output in a manner identical to that described above set to 0 In loopback mode, connected to Data Carrier Detect (DCD)"]
    #[inline(always)]
    pub fn out2(&mut self) -> Out2W<'_, UartMcrSpec> {
        Out2W::new(self, 3)
    }
    #[doc = "Bit 4 - LoopBack mode * 0: Normal operation mode * 1: Loopback mode"]
    #[inline(always)]
    pub fn loop_en(&mut self) -> LoopEnW<'_, UartMcrSpec> {
        LoopEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Auto flow control enable"]
    #[inline(always)]
    pub fn afe(&mut self) -> AfeW<'_, UartMcrSpec> {
        AfeW::new(self, 5)
    }
}
#[doc = "UART modem control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartMcrSpec;
impl crate::RegisterSpec for UartMcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_mcr::R`](R) reader structure"]
impl crate::Readable for UartMcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_mcr::W`](W) writer structure"]
impl crate::Writable for UartMcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_MCR to value 0"]
impl crate::Resettable for UartMcrSpec {}
