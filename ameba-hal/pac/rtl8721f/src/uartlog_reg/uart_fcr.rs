#[doc = "Register `UART_FCR` reader"]
pub type R = crate::R<UartFcrSpec>;
#[doc = "Register `UART_FCR` writer"]
pub type W = crate::W<UartFcrSpec>;
#[doc = "Field `fifo_en` reader - Error report of RCVR FIFO field enable"]
pub type FifoEnR = crate::BitReader;
#[doc = "Field `fifo_en` writer - Error report of RCVR FIFO field enable"]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `recv_clr` writer - Clear receiving"]
pub type RecvClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_mode` reader - Support DMA mode (cooperating with DW DDMA in the data path)"]
pub type DmaModeR = crate::BitReader;
#[doc = "Field `dma_mode` writer - Support DMA mode (cooperating with DW DDMA in the data path)"]
pub type DmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `recvtrg` reader - Define the 16-entry Receiver FIFO Interrupt trigger level (0~15 bytes) * 00: 1 byte * 01: FIFO_DEPTH/4 * 10: FIFO_DEPTH/2 * 11: FIFO_DEPTH-2"]
pub type RecvtrgR = crate::FieldReader;
#[doc = "Field `recvtrg` writer - Define the 16-entry Receiver FIFO Interrupt trigger level (0~15 bytes) * 00: 1 byte * 01: FIFO_DEPTH/4 * 10: FIFO_DEPTH/2 * 11: FIFO_DEPTH-2"]
pub type RecvtrgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rp_fifo_en` reader - Error report of Relay Path RCVR FIFO field enable,"]
pub type RpFifoEnR = crate::BitReader;
#[doc = "Field `rp_fifo_en` writer - Error report of Relay Path RCVR FIFO field enable,"]
pub type RpFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rp_recv_clr` writer - Clear Relay Path receiving"]
pub type RpRecvClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `log_full_en` reader - Function enable: when disable agg, and log byte is 0x0a or this fifo is empty timeout, switch tx fifo"]
pub type LogFullEnR = crate::BitReader;
#[doc = "Field `log_full_en` writer - Function enable: when disable agg, and log byte is 0x0a or this fifo is empty timeout, switch tx fifo"]
pub type LogFullEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error report of RCVR FIFO field enable"]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Support DMA mode (cooperating with DW DDMA in the data path)"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DmaModeR {
        DmaModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Define the 16-entry Receiver FIFO Interrupt trigger level (0~15 bytes) * 00: 1 byte * 01: FIFO_DEPTH/4 * 10: FIFO_DEPTH/2 * 11: FIFO_DEPTH-2"]
    #[inline(always)]
    pub fn recvtrg(&self) -> RecvtrgR {
        RecvtrgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 12 - Error report of Relay Path RCVR FIFO field enable,"]
    #[inline(always)]
    pub fn rp_fifo_en(&self) -> RpFifoEnR {
        RpFifoEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Function enable: when disable agg, and log byte is 0x0a or this fifo is empty timeout, switch tx fifo"]
    #[inline(always)]
    pub fn log_full_en(&self) -> LogFullEnR {
        LogFullEnR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error report of RCVR FIFO field enable"]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<'_, UartFcrSpec> {
        FifoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear receiving"]
    #[inline(always)]
    pub fn recv_clr(&mut self) -> RecvClrW<'_, UartFcrSpec> {
        RecvClrW::new(self, 1)
    }
    #[doc = "Bit 3 - Support DMA mode (cooperating with DW DDMA in the data path)"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DmaModeW<'_, UartFcrSpec> {
        DmaModeW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Define the 16-entry Receiver FIFO Interrupt trigger level (0~15 bytes) * 00: 1 byte * 01: FIFO_DEPTH/4 * 10: FIFO_DEPTH/2 * 11: FIFO_DEPTH-2"]
    #[inline(always)]
    pub fn recvtrg(&mut self) -> RecvtrgW<'_, UartFcrSpec> {
        RecvtrgW::new(self, 6)
    }
    #[doc = "Bit 12 - Error report of Relay Path RCVR FIFO field enable,"]
    #[inline(always)]
    pub fn rp_fifo_en(&mut self) -> RpFifoEnW<'_, UartFcrSpec> {
        RpFifoEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Relay Path receiving"]
    #[inline(always)]
    pub fn rp_recv_clr(&mut self) -> RpRecvClrW<'_, UartFcrSpec> {
        RpRecvClrW::new(self, 13)
    }
    #[doc = "Bit 14 - Function enable: when disable agg, and log byte is 0x0a or this fifo is empty timeout, switch tx fifo"]
    #[inline(always)]
    pub fn log_full_en(&mut self) -> LogFullEnW<'_, UartFcrSpec> {
        LogFullEnW::new(self, 14)
    }
}
#[doc = "UART FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFcrSpec;
impl crate::RegisterSpec for UartFcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_fcr::R`](R) reader structure"]
impl crate::Readable for UartFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_fcr::W`](W) writer structure"]
impl crate::Writable for UartFcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_FCR to value 0x50c1"]
impl crate::Resettable for UartFcrSpec {
    const RESET_VALUE: u32 = 0x50c1;
}
