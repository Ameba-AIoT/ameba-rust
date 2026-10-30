#[doc = "Register `UART_IER` reader"]
pub type R = crate::R<UartIerSpec>;
#[doc = "Register `UART_IER` writer"]
pub type W = crate::W<UartIerSpec>;
#[doc = "Field `erbi` reader - Received Data Available Interrupt (ERBFI) (Rx trigger) enable"]
pub type ErbiR = crate::BitReader;
#[doc = "Field `erbi` writer - Received Data Available Interrupt (ERBFI) (Rx trigger) enable"]
pub type ErbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `elsi` reader - Receiver Line Status Interrupt (ELSI) (receiver line status) enable"]
pub type ElsiR = crate::BitReader;
#[doc = "Field `elsi` writer - Receiver Line Status Interrupt (ELSI) (receiver line status) enable"]
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `edssi` reader - Modem Status Interrupt (EDSSI) (modem status transition) enable"]
pub type EdssiR = crate::BitReader;
#[doc = "Field `edssi` writer - Modem Status Interrupt (EDSSI) (modem status transition) enable"]
pub type EdssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `emdi` reader - Rx path monitor done interrupt (EMDI) enable"]
pub type EmdiR = crate::BitReader;
#[doc = "Field `emdi` writer - Rx path monitor done interrupt (EMDI) enable"]
pub type EmdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `etoi` reader - Rx timeout interrupt (ETOI) enable"]
pub type EtoiR = crate::BitReader;
#[doc = "Field `etoi` writer - Rx timeout interrupt (ETOI) enable"]
pub type EtoiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `etpfei` reader - Tx Path FIFO Empty interrupt (ETPFEI) (Tx Path FIFO empty) enable control *000: All Tx Path FIFO empty interrupt disabled *001: Tx Path1 FIFO Empty interrupt enabled *010: Tx Path2 FIFO Empty interrupt enabled *011: Tx Path3 FIFO Empty interrupt enabled *100: Tx Path4 FIFO Empty interrupt enabled *Others: Reserved"]
pub type EtpfeiR = crate::FieldReader;
#[doc = "Field `etpfei` writer - Tx Path FIFO Empty interrupt (ETPFEI) (Tx Path FIFO empty) enable control *000: All Tx Path FIFO empty interrupt disabled *001: Tx Path1 FIFO Empty interrupt enabled *010: Tx Path2 FIFO Empty interrupt enabled *011: Tx Path3 FIFO Empty interrupt enabled *100: Tx Path4 FIFO Empty interrupt enabled *Others: Reserved"]
pub type EtpfeiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Received Data Available Interrupt (ERBFI) (Rx trigger) enable"]
    #[inline(always)]
    pub fn erbi(&self) -> ErbiR {
        ErbiR::new((self.bits & 1) != 0)
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
    #[doc = "Bit 4 - Rx path monitor done interrupt (EMDI) enable"]
    #[inline(always)]
    pub fn emdi(&self) -> EmdiR {
        EmdiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx timeout interrupt (ETOI) enable"]
    #[inline(always)]
    pub fn etoi(&self) -> EtoiR {
        EtoiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tx Path FIFO Empty interrupt (ETPFEI) (Tx Path FIFO empty) enable control *000: All Tx Path FIFO empty interrupt disabled *001: Tx Path1 FIFO Empty interrupt enabled *010: Tx Path2 FIFO Empty interrupt enabled *011: Tx Path3 FIFO Empty interrupt enabled *100: Tx Path4 FIFO Empty interrupt enabled *Others: Reserved"]
    #[inline(always)]
    pub fn etpfei(&self) -> EtpfeiR {
        EtpfeiR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Received Data Available Interrupt (ERBFI) (Rx trigger) enable"]
    #[inline(always)]
    pub fn erbi(&mut self) -> ErbiW<'_, UartIerSpec> {
        ErbiW::new(self, 0)
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
    #[doc = "Bit 4 - Rx path monitor done interrupt (EMDI) enable"]
    #[inline(always)]
    pub fn emdi(&mut self) -> EmdiW<'_, UartIerSpec> {
        EmdiW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx timeout interrupt (ETOI) enable"]
    #[inline(always)]
    pub fn etoi(&mut self) -> EtoiW<'_, UartIerSpec> {
        EtoiW::new(self, 5)
    }
    #[doc = "Bits 8:10 - Tx Path FIFO Empty interrupt (ETPFEI) (Tx Path FIFO empty) enable control *000: All Tx Path FIFO empty interrupt disabled *001: Tx Path1 FIFO Empty interrupt enabled *010: Tx Path2 FIFO Empty interrupt enabled *011: Tx Path3 FIFO Empty interrupt enabled *100: Tx Path4 FIFO Empty interrupt enabled *Others: Reserved"]
    #[inline(always)]
    pub fn etpfei(&mut self) -> EtpfeiW<'_, UartIerSpec> {
        EtpfeiW::new(self, 8)
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
