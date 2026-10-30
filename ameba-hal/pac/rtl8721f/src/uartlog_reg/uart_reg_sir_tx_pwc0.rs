#[doc = "Register `UART_REG_SIR_TX_PWC0` reader"]
pub type R = crate::R<UartRegSirTxPwc0Spec>;
#[doc = "Register `UART_REG_SIR_TX_PWC0` writer"]
pub type W = crate::W<UartRegSirTxPwc0Spec>;
#[doc = "Field `lowbound_shiftval` reader - The shift value of SIR Tx pulse's left edge"]
pub type LowboundShiftvalR = crate::FieldReader<u16>;
#[doc = "Field `lowbound_shiftval` writer - The shift value of SIR Tx pulse's left edge"]
pub type LowboundShiftvalW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `lowbound_shiftright` reader - Indicate the shift type of SIR Tx pulse's left edge * 0: Shift left, minus offset value of lowbound_shiftval * 1: Shift right, plus offset value of lowbound_shiftval"]
pub type LowboundShiftrightR = crate::BitReader;
#[doc = "Field `lowbound_shiftright` writer - Indicate the shift type of SIR Tx pulse's left edge * 0: Shift left, minus offset value of lowbound_shiftval * 1: Shift right, plus offset value of lowbound_shiftval"]
pub type LowboundShiftrightW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `upperbound_shiftval` reader - The shift value of SIR Tx pulse's right edge"]
pub type UpperboundShiftvalR = crate::FieldReader<u16>;
#[doc = "Field `upperbound_shiftval` writer - The shift value of SIR Tx pulse's right edge"]
pub type UpperboundShiftvalW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `upperbound_shiftright` reader - Indicate the shift type of SIR Tx pulse's right edge * 0: Shift left, minus offset value of upperbound_shiftval * 1: Shift right, plus offset value of upperbound_shiftval"]
pub type UpperboundShiftrightR = crate::BitReader;
#[doc = "Field `upperbound_shiftright` writer - Indicate the shift type of SIR Tx pulse's right edge * 0: Shift left, minus offset value of upperbound_shiftval * 1: Shift right, plus offset value of upperbound_shiftval"]
pub type UpperboundShiftrightW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - The shift value of SIR Tx pulse's left edge"]
    #[inline(always)]
    pub fn lowbound_shiftval(&self) -> LowboundShiftvalR {
        LowboundShiftvalR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Indicate the shift type of SIR Tx pulse's left edge * 0: Shift left, minus offset value of lowbound_shiftval * 1: Shift right, plus offset value of lowbound_shiftval"]
    #[inline(always)]
    pub fn lowbound_shiftright(&self) -> LowboundShiftrightR {
        LowboundShiftrightR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - The shift value of SIR Tx pulse's right edge"]
    #[inline(always)]
    pub fn upperbound_shiftval(&self) -> UpperboundShiftvalR {
        UpperboundShiftvalR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Indicate the shift type of SIR Tx pulse's right edge * 0: Shift left, minus offset value of upperbound_shiftval * 1: Shift right, plus offset value of upperbound_shiftval"]
    #[inline(always)]
    pub fn upperbound_shiftright(&self) -> UpperboundShiftrightR {
        UpperboundShiftrightR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - The shift value of SIR Tx pulse's left edge"]
    #[inline(always)]
    pub fn lowbound_shiftval(&mut self) -> LowboundShiftvalW<'_, UartRegSirTxPwc0Spec> {
        LowboundShiftvalW::new(self, 0)
    }
    #[doc = "Bit 15 - Indicate the shift type of SIR Tx pulse's left edge * 0: Shift left, minus offset value of lowbound_shiftval * 1: Shift right, plus offset value of lowbound_shiftval"]
    #[inline(always)]
    pub fn lowbound_shiftright(&mut self) -> LowboundShiftrightW<'_, UartRegSirTxPwc0Spec> {
        LowboundShiftrightW::new(self, 15)
    }
    #[doc = "Bits 16:30 - The shift value of SIR Tx pulse's right edge"]
    #[inline(always)]
    pub fn upperbound_shiftval(&mut self) -> UpperboundShiftvalW<'_, UartRegSirTxPwc0Spec> {
        UpperboundShiftvalW::new(self, 16)
    }
    #[doc = "Bit 31 - Indicate the shift type of SIR Tx pulse's right edge * 0: Shift left, minus offset value of upperbound_shiftval * 1: Shift right, plus offset value of upperbound_shiftval"]
    #[inline(always)]
    pub fn upperbound_shiftright(&mut self) -> UpperboundShiftrightW<'_, UartRegSirTxPwc0Spec> {
        UpperboundShiftrightW::new(self, 31)
    }
}
#[doc = "UART IRDA SIR Tx pulse width control 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_reg_sir_tx_pwc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_reg_sir_tx_pwc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRegSirTxPwc0Spec;
impl crate::RegisterSpec for UartRegSirTxPwc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_reg_sir_tx_pwc0::R`](R) reader structure"]
impl crate::Readable for UartRegSirTxPwc0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart_reg_sir_tx_pwc0::W`](W) writer structure"]
impl crate::Writable for UartRegSirTxPwc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_REG_SIR_TX_PWC0 to value 0"]
impl crate::Resettable for UartRegSirTxPwc0Spec {}
