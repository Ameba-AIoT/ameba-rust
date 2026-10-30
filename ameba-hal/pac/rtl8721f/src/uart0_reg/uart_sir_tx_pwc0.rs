#[doc = "Register `UART_SIR_TX_PWC0` reader"]
pub type R = crate::R<UartSirTxPwc0Spec>;
#[doc = "Register `UART_SIR_TX_PWC0` writer"]
pub type W = crate::W<UartSirTxPwc0Spec>;
#[doc = "Field `LOWBOUND_SHIFTVAL` reader - The shift value of SIR TX pulse's left edge"]
pub type LowboundShiftvalR = crate::FieldReader<u16>;
#[doc = "Field `LOWBOUND_SHIFTVAL` writer - The shift value of SIR TX pulse's left edge"]
pub type LowboundShiftvalW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LOWBOUND_SHIFTRIGHT` reader - * 0: Shift left, minus offset value of lowbound_shiftval * 1: Shift right, plus offset value of lowbound_shiftval"]
pub type LowboundShiftrightR = crate::BitReader;
#[doc = "Field `LOWBOUND_SHIFTRIGHT` writer - * 0: Shift left, minus offset value of lowbound_shiftval * 1: Shift right, plus offset value of lowbound_shiftval"]
pub type LowboundShiftrightW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPPERBOUND_SHIFTVAL` reader - The shift value of SIR TX pulse's right edge"]
pub type UpperboundShiftvalR = crate::FieldReader<u16>;
#[doc = "Field `UPPERBOUND_SHIFTVAL` writer - The shift value of SIR TX pulse's right edge"]
pub type UpperboundShiftvalW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `UPPERBOUND_SHIFTRIGHT` reader - * 0: Shift left, minus offset value of upperbound_shiftval * 1: Shift right, plus offset value of upperbound_shiftval"]
pub type UpperboundShiftrightR = crate::BitReader;
#[doc = "Field `UPPERBOUND_SHIFTRIGHT` writer - * 0: Shift left, minus offset value of upperbound_shiftval * 1: Shift right, plus offset value of upperbound_shiftval"]
pub type UpperboundShiftrightW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - The shift value of SIR TX pulse's left edge"]
    #[inline(always)]
    pub fn lowbound_shiftval(&self) -> LowboundShiftvalR {
        LowboundShiftvalR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - * 0: Shift left, minus offset value of lowbound_shiftval * 1: Shift right, plus offset value of lowbound_shiftval"]
    #[inline(always)]
    pub fn lowbound_shiftright(&self) -> LowboundShiftrightR {
        LowboundShiftrightR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - The shift value of SIR TX pulse's right edge"]
    #[inline(always)]
    pub fn upperbound_shiftval(&self) -> UpperboundShiftvalR {
        UpperboundShiftvalR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - * 0: Shift left, minus offset value of upperbound_shiftval * 1: Shift right, plus offset value of upperbound_shiftval"]
    #[inline(always)]
    pub fn upperbound_shiftright(&self) -> UpperboundShiftrightR {
        UpperboundShiftrightR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - The shift value of SIR TX pulse's left edge"]
    #[inline(always)]
    pub fn lowbound_shiftval(&mut self) -> LowboundShiftvalW<'_, UartSirTxPwc0Spec> {
        LowboundShiftvalW::new(self, 0)
    }
    #[doc = "Bit 15 - * 0: Shift left, minus offset value of lowbound_shiftval * 1: Shift right, plus offset value of lowbound_shiftval"]
    #[inline(always)]
    pub fn lowbound_shiftright(&mut self) -> LowboundShiftrightW<'_, UartSirTxPwc0Spec> {
        LowboundShiftrightW::new(self, 15)
    }
    #[doc = "Bits 16:30 - The shift value of SIR TX pulse's right edge"]
    #[inline(always)]
    pub fn upperbound_shiftval(&mut self) -> UpperboundShiftvalW<'_, UartSirTxPwc0Spec> {
        UpperboundShiftvalW::new(self, 16)
    }
    #[doc = "Bit 31 - * 0: Shift left, minus offset value of upperbound_shiftval * 1: Shift right, plus offset value of upperbound_shiftval"]
    #[inline(always)]
    pub fn upperbound_shiftright(&mut self) -> UpperboundShiftrightW<'_, UartSirTxPwc0Spec> {
        UpperboundShiftrightW::new(self, 31)
    }
}
#[doc = "UART IRDA SIR TX pulse width control 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_sir_tx_pwc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_sir_tx_pwc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSirTxPwc0Spec;
impl crate::RegisterSpec for UartSirTxPwc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_sir_tx_pwc0::R`](R) reader structure"]
impl crate::Readable for UartSirTxPwc0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart_sir_tx_pwc0::W`](W) writer structure"]
impl crate::Writable for UartSirTxPwc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_SIR_TX_PWC0 to value 0"]
impl crate::Resettable for UartSirTxPwc0Spec {}
