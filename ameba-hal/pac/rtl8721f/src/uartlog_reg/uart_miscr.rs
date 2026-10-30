#[doc = "Register `UART_MISCR` reader"]
pub type R = crate::R<UartMiscrSpec>;
#[doc = "Register `UART_MISCR` writer"]
pub type W = crate::W<UartMiscrSpec>;
#[doc = "Field `irda_enable` reader - IrDA SIR mode enable * 0: UART mode only * 1: UART is co-work with IRDA SIR mode"]
pub type IrdaEnableR = crate::BitReader;
#[doc = "Field `irda_enable` writer - IrDA SIR mode enable * 0: UART mode only * 1: UART is co-work with IRDA SIR mode"]
pub type IrdaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txdma_en` reader - TXDMA enable, valid when dma_mode in FCR is 1'b1"]
pub type TxdmaEnR = crate::BitReader;
#[doc = "Field `txdma_en` writer - TXDMA enable, valid when dma_mode in FCR is 1'b1"]
pub type TxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxdma_en` reader - RXDMA enable, valid when dma_mode in FCR is 1'b1"]
pub type RxdmaEnR = crate::BitReader;
#[doc = "Field `rxdma_en` writer - RXDMA enable, valid when dma_mode in FCR is 1'b1"]
pub type RxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txdma_burstsize` reader - TXDMA burstsize, maximum value is 16"]
pub type TxdmaBurstsizeR = crate::FieldReader;
#[doc = "Field `txdma_burstsize` writer - TXDMA burstsize, maximum value is 16"]
pub type TxdmaBurstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `rxdma_burstsize` reader - RXDMA burstsize, maximum value is 16"]
pub type RxdmaBurstsizeR = crate::FieldReader;
#[doc = "Field `rxdma_burstsize` writer - RXDMA burstsize, maximum value is 16"]
pub type RxdmaBurstsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `irda_tx_inv` reader - irda_tx_o reverse output enable"]
pub type IrdaTxInvR = crate::BitReader;
#[doc = "Field `irda_tx_inv` writer - irda_tx_o reverse output enable"]
pub type IrdaTxInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irda_rx_inv` reader - irda_rx_i reverse input enable"]
pub type IrdaRxInvR = crate::BitReader;
#[doc = "Field `irda_rx_inv` writer - irda_rx_i reverse input enable"]
pub type IrdaRxInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxdma_owner` reader - Indicate the DMA flow controller for UART Rx DMA1 * 0: GDMA * 1: UART, interrupt enable signal other than erpd should be masked"]
pub type RxdmaOwnerR = crate::BitReader;
#[doc = "Field `rxdma_owner` writer - Indicate the DMA flow controller for UART Rx DMA1 * 0: GDMA * 1: UART, interrupt enable signal other than erpd should be masked"]
pub type RxdmaOwnerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy_data` reader - Store dummy data, when UART is DMA flow controller, dummy data is padded for last RXDMA burst request"]
pub type DummyDataR = crate::FieldReader;
#[doc = "Field `dummy_data` writer - Store dummy data, when UART is DMA flow controller, dummy data is padded for last RXDMA burst request"]
pub type DummyDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `clr_dummy_flag` writer - Indicate the clearing of dummy data"]
pub type ClrDummyFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IrDA SIR mode enable * 0: UART mode only * 1: UART is co-work with IRDA SIR mode"]
    #[inline(always)]
    pub fn irda_enable(&self) -> IrdaEnableR {
        IrdaEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDMA enable, valid when dma_mode in FCR is 1'b1"]
    #[inline(always)]
    pub fn txdma_en(&self) -> TxdmaEnR {
        TxdmaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXDMA enable, valid when dma_mode in FCR is 1'b1"]
    #[inline(always)]
    pub fn rxdma_en(&self) -> RxdmaEnR {
        RxdmaEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - TXDMA burstsize, maximum value is 16"]
    #[inline(always)]
    pub fn txdma_burstsize(&self) -> TxdmaBurstsizeR {
        TxdmaBurstsizeR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - RXDMA burstsize, maximum value is 16"]
    #[inline(always)]
    pub fn rxdma_burstsize(&self) -> RxdmaBurstsizeR {
        RxdmaBurstsizeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - irda_tx_o reverse output enable"]
    #[inline(always)]
    pub fn irda_tx_inv(&self) -> IrdaTxInvR {
        IrdaTxInvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - irda_rx_i reverse input enable"]
    #[inline(always)]
    pub fn irda_rx_inv(&self) -> IrdaRxInvR {
        IrdaRxInvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicate the DMA flow controller for UART Rx DMA1 * 0: GDMA * 1: UART, interrupt enable signal other than erpd should be masked"]
    #[inline(always)]
    pub fn rxdma_owner(&self) -> RxdmaOwnerR {
        RxdmaOwnerR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Store dummy data, when UART is DMA flow controller, dummy data is padded for last RXDMA burst request"]
    #[inline(always)]
    pub fn dummy_data(&self) -> DummyDataR {
        DummyDataR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA SIR mode enable * 0: UART mode only * 1: UART is co-work with IRDA SIR mode"]
    #[inline(always)]
    pub fn irda_enable(&mut self) -> IrdaEnableW<'_, UartMiscrSpec> {
        IrdaEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - TXDMA enable, valid when dma_mode in FCR is 1'b1"]
    #[inline(always)]
    pub fn txdma_en(&mut self) -> TxdmaEnW<'_, UartMiscrSpec> {
        TxdmaEnW::new(self, 1)
    }
    #[doc = "Bit 2 - RXDMA enable, valid when dma_mode in FCR is 1'b1"]
    #[inline(always)]
    pub fn rxdma_en(&mut self) -> RxdmaEnW<'_, UartMiscrSpec> {
        RxdmaEnW::new(self, 2)
    }
    #[doc = "Bits 3:7 - TXDMA burstsize, maximum value is 16"]
    #[inline(always)]
    pub fn txdma_burstsize(&mut self) -> TxdmaBurstsizeW<'_, UartMiscrSpec> {
        TxdmaBurstsizeW::new(self, 3)
    }
    #[doc = "Bits 8:12 - RXDMA burstsize, maximum value is 16"]
    #[inline(always)]
    pub fn rxdma_burstsize(&mut self) -> RxdmaBurstsizeW<'_, UartMiscrSpec> {
        RxdmaBurstsizeW::new(self, 8)
    }
    #[doc = "Bit 13 - irda_tx_o reverse output enable"]
    #[inline(always)]
    pub fn irda_tx_inv(&mut self) -> IrdaTxInvW<'_, UartMiscrSpec> {
        IrdaTxInvW::new(self, 13)
    }
    #[doc = "Bit 14 - irda_rx_i reverse input enable"]
    #[inline(always)]
    pub fn irda_rx_inv(&mut self) -> IrdaRxInvW<'_, UartMiscrSpec> {
        IrdaRxInvW::new(self, 14)
    }
    #[doc = "Bit 15 - Indicate the DMA flow controller for UART Rx DMA1 * 0: GDMA * 1: UART, interrupt enable signal other than erpd should be masked"]
    #[inline(always)]
    pub fn rxdma_owner(&mut self) -> RxdmaOwnerW<'_, UartMiscrSpec> {
        RxdmaOwnerW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Store dummy data, when UART is DMA flow controller, dummy data is padded for last RXDMA burst request"]
    #[inline(always)]
    pub fn dummy_data(&mut self) -> DummyDataW<'_, UartMiscrSpec> {
        DummyDataW::new(self, 16)
    }
    #[doc = "Bit 24 - Indicate the clearing of dummy data"]
    #[inline(always)]
    pub fn clr_dummy_flag(&mut self) -> ClrDummyFlagW<'_, UartMiscrSpec> {
        ClrDummyFlagW::new(self, 24)
    }
}
#[doc = "UART MISC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_miscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_miscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartMiscrSpec;
impl crate::RegisterSpec for UartMiscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_miscr::R`](R) reader structure"]
impl crate::Readable for UartMiscrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_miscr::W`](W) writer structure"]
impl crate::Writable for UartMiscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_MISCR to value 0x0420"]
impl crate::Resettable for UartMiscrSpec {
    const RESET_VALUE: u32 = 0x0420;
}
