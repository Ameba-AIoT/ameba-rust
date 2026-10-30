#[doc = "Register `UART_FCR` reader"]
pub type R = crate::R<UartFcrSpec>;
#[doc = "Register `UART_FCR` writer"]
pub type W = crate::W<UartFcrSpec>;
#[doc = "Field `FIFO_EN` reader - Error report of RCVR FIFO field enable, only valid when the LSR bit\\[7\\] is set to 1."]
pub type FifoEnR = crate::BitReader;
#[doc = "Field `FIFO_EN` writer - Error report of RCVR FIFO field enable, only valid when the LSR bit\\[7\\] is set to 1."]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECV_CLR` writer - Clear receiving"]
pub type RecvClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMIT_CLR` writer - Clear xmitckt"]
pub type XmitClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MODE` reader - Support dma mode (cooperate with DW DDMA in the data path)"]
pub type DmaModeR = crate::BitReader;
#[doc = "Field `DMA_MODE` writer - Support dma mode (cooperate with DW DDMA in the data path)"]
pub type DmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECVTRG` reader - Define the 16-entries Receiver FIFO Interrupt trigger level (0~15 bytes) * 00: 1 byte * 01: 16 bytes (FIFO_DEPTH/4) * 10: 32 bytes (FIFO_DEPTH/2) * 11: 62 bytes (FIFO_DEPTH-2)"]
pub type RecvtrgR = crate::FieldReader;
#[doc = "Field `RECVTRG` writer - Define the 16-entries Receiver FIFO Interrupt trigger level (0~15 bytes) * 00: 1 byte * 01: 16 bytes (FIFO_DEPTH/4) * 10: 32 bytes (FIFO_DEPTH/2) * 11: 62 bytes (FIFO_DEPTH-2)"]
pub type RecvtrgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Error report of RCVR FIFO field enable, only valid when the LSR bit\\[7\\] is set to 1."]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Support dma mode (cooperate with DW DDMA in the data path)"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DmaModeR {
        DmaModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Define the 16-entries Receiver FIFO Interrupt trigger level (0~15 bytes) * 00: 1 byte * 01: 16 bytes (FIFO_DEPTH/4) * 10: 32 bytes (FIFO_DEPTH/2) * 11: 62 bytes (FIFO_DEPTH-2)"]
    #[inline(always)]
    pub fn recvtrg(&self) -> RecvtrgR {
        RecvtrgR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Error report of RCVR FIFO field enable, only valid when the LSR bit\\[7\\] is set to 1."]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<'_, UartFcrSpec> {
        FifoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear receiving"]
    #[inline(always)]
    pub fn recv_clr(&mut self) -> RecvClrW<'_, UartFcrSpec> {
        RecvClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear xmitckt"]
    #[inline(always)]
    pub fn xmit_clr(&mut self) -> XmitClrW<'_, UartFcrSpec> {
        XmitClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Support dma mode (cooperate with DW DDMA in the data path)"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DmaModeW<'_, UartFcrSpec> {
        DmaModeW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Define the 16-entries Receiver FIFO Interrupt trigger level (0~15 bytes) * 00: 1 byte * 01: 16 bytes (FIFO_DEPTH/4) * 10: 32 bytes (FIFO_DEPTH/2) * 11: 62 bytes (FIFO_DEPTH-2)"]
    #[inline(always)]
    pub fn recvtrg(&mut self) -> RecvtrgW<'_, UartFcrSpec> {
        RecvtrgW::new(self, 6)
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
#[doc = "`reset()` method sets UART_FCR to value 0xc1"]
impl crate::Resettable for UartFcrSpec {
    const RESET_VALUE: u32 = 0xc1;
}
