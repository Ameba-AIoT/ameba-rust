#[doc = "Register `UART_IER` reader"]
pub type R = crate::R<UartIerSpec>;
#[doc = "Register `UART_IER` writer"]
pub type W = crate::W<UartIerSpec>;
#[doc = "Field `ERBI` reader - Received Data Available Interrupt (ERBFI) (Rx trigger) enable"]
pub type ErbiR = crate::BitReader;
#[doc = "Field `ERBI` writer - Received Data Available Interrupt (ERBFI) (Rx trigger) enable"]
pub type ErbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETBEI` reader - Transmitter FIFO Empty interrupt (ETBEI) (Tx FIFO empty) enable"]
pub type EtbeiR = crate::BitReader;
#[doc = "Field `ETBEI` writer - Transmitter FIFO Empty interrupt (ETBEI) (Tx FIFO empty) enable"]
pub type EtbeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELSI` reader - Receiver Line Status Interrupt (ELSI) (receiver line status) enable"]
pub type ElsiR = crate::BitReader;
#[doc = "Field `ELSI` writer - Receiver Line Status Interrupt (ELSI) (receiver line status) enable"]
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDSSI` reader - Modem Status Interrupt (EDSSI) (modem status transition) enable"]
pub type EdssiR = crate::BitReader;
#[doc = "Field `EDSSI` writer - Modem Status Interrupt (EDSSI) (modem status transition) enable"]
pub type EdssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMDI` reader - Rx path monitor done interrupt enable"]
pub type EmdiR = crate::BitReader;
#[doc = "Field `EMDI` writer - Rx path monitor done interrupt enable"]
pub type EmdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETOI` reader - Rx timeout interrupt enable"]
pub type EtoiR = crate::BitReader;
#[doc = "Field `ETOI` writer - Rx timeout interrupt enable"]
pub type EtoiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERXNDI` reader - Rx noise detected interrupt enable"]
pub type ErxndiR = crate::BitReader;
#[doc = "Field `ERXNDI` writer - Rx noise detected interrupt enable"]
pub type ErxndiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERETI` reader - RXFIFO_EMPTY_TIMEOUT interrupt enable"]
pub type EretiR = crate::BitReader;
#[doc = "Field `ERETI` writer - RXFIFO_EMPTY_TIMEOUT interrupt enable"]
pub type EretiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Received Data Available Interrupt (ERBFI) (Rx trigger) enable"]
    #[inline(always)]
    pub fn erbi(&self) -> ErbiR {
        ErbiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter FIFO Empty interrupt (ETBEI) (Tx FIFO empty) enable"]
    #[inline(always)]
    pub fn etbei(&self) -> EtbeiR {
        EtbeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Line Status Interrupt (ELSI) (receiver line status) enable"]
    #[inline(always)]
    pub fn elsi(&self) -> ElsiR {
        ElsiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Modem Status Interrupt (EDSSI) (modem status transition) enable"]
    #[inline(always)]
    pub fn edssi(&self) -> EdssiR {
        EdssiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx path monitor done interrupt enable"]
    #[inline(always)]
    pub fn emdi(&self) -> EmdiR {
        EmdiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx timeout interrupt enable"]
    #[inline(always)]
    pub fn etoi(&self) -> EtoiR {
        EtoiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx noise detected interrupt enable"]
    #[inline(always)]
    pub fn erxndi(&self) -> ErxndiR {
        ErxndiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RXFIFO_EMPTY_TIMEOUT interrupt enable"]
    #[inline(always)]
    pub fn ereti(&self) -> EretiR {
        EretiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Received Data Available Interrupt (ERBFI) (Rx trigger) enable"]
    #[inline(always)]
    pub fn erbi(&mut self) -> ErbiW<'_, UartIerSpec> {
        ErbiW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitter FIFO Empty interrupt (ETBEI) (Tx FIFO empty) enable"]
    #[inline(always)]
    pub fn etbei(&mut self) -> EtbeiW<'_, UartIerSpec> {
        EtbeiW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Line Status Interrupt (ELSI) (receiver line status) enable"]
    #[inline(always)]
    pub fn elsi(&mut self) -> ElsiW<'_, UartIerSpec> {
        ElsiW::new(self, 2)
    }
    #[doc = "Bit 3 - Modem Status Interrupt (EDSSI) (modem status transition) enable"]
    #[inline(always)]
    pub fn edssi(&mut self) -> EdssiW<'_, UartIerSpec> {
        EdssiW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx path monitor done interrupt enable"]
    #[inline(always)]
    pub fn emdi(&mut self) -> EmdiW<'_, UartIerSpec> {
        EmdiW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx timeout interrupt enable"]
    #[inline(always)]
    pub fn etoi(&mut self) -> EtoiW<'_, UartIerSpec> {
        EtoiW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx noise detected interrupt enable"]
    #[inline(always)]
    pub fn erxndi(&mut self) -> ErxndiW<'_, UartIerSpec> {
        ErxndiW::new(self, 6)
    }
    #[doc = "Bit 7 - RXFIFO_EMPTY_TIMEOUT interrupt enable"]
    #[inline(always)]
    pub fn ereti(&mut self) -> EretiW<'_, UartIerSpec> {
        EretiW::new(self, 7)
    }
}
#[doc = "UART interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIerSpec;
impl crate::RegisterSpec for UartIerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_ier::R`](R) reader structure"]
impl crate::Readable for UartIerSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_ier::W`](W) writer structure"]
impl crate::Writable for UartIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_IER to value 0"]
impl crate::Resettable for UartIerSpec {}
