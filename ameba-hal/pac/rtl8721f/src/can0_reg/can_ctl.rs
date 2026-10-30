#[doc = "Register `CAN_CTL` reader"]
pub type R = crate::R<CanCtlSpec>;
#[doc = "Register `CAN_CTL` writer"]
pub type W = crate::W<CanCtlSpec>;
#[doc = "Field `can_en` reader - can enable"]
pub type CanEnR = crate::BitReader;
#[doc = "Field `can_en` writer - can enable"]
pub type CanEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bus_on_req` reader - request to bus_on"]
pub type BusOnReqR = crate::BitReader;
#[doc = "Field `bus_on_req` writer - request to bus_on"]
pub type BusOnReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_tri_sample` reader - triple sample mode"]
pub type CanTriSampleR = crate::BitReader;
#[doc = "Field `can_tri_sample` writer - triple sample mode"]
pub type CanTriSampleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `auto_re_tx_en` reader - auto re-transmission enabled"]
pub type AutoReTxEnR = crate::BitReader;
#[doc = "Field `auto_re_tx_en` writer - auto re-transmission enabled"]
pub type AutoReTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_mode_en` reader - test mode enable, for lbk or silent mode use"]
pub type TestModeEnR = crate::BitReader;
#[doc = "Field `test_mode_en` writer - test mode enable, for lbk or silent mode use"]
pub type TestModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_en` reader - rx fifo function enable"]
pub type RxFifoEnR = crate::BitReader;
#[doc = "Field `rx_fifo_en` writer - rx fifo function enable"]
pub type RxFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_fd_en` reader - enable CAN FD protocol"]
pub type CanFdEnR = crate::BitReader;
#[doc = "Field `can_fd_en` writer - enable CAN FD protocol"]
pub type CanFdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_rxdma_en` reader - rx dma enable"]
pub type CanRxdmaEnR = crate::BitReader;
#[doc = "Field `can_rxdma_en` writer - rx dma enable"]
pub type CanRxdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_fd_crc_mode` reader - crc mode select * 0: ISO crc * 1: non-ISO crc"]
pub type CanFdCrcModeR = crate::BitReader;
#[doc = "Field `can_fd_crc_mode` writer - crc mode select * 0: ISO crc * 1: non-ISO crc"]
pub type CanFdCrcModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_fd_ssp_auto` reader - automatic calculat ssp delay, ssp = second sample point"]
pub type CanFdSspAutoR = crate::BitReader;
#[doc = "Field `can_fd_ssp_auto` writer - automatic calculat ssp delay, ssp = second sample point"]
pub type CanFdSspAutoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - can enable"]
    #[inline(always)]
    pub fn can_en(&self) -> CanEnR {
        CanEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - request to bus_on"]
    #[inline(always)]
    pub fn bus_on_req(&self) -> BusOnReqR {
        BusOnReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - triple sample mode"]
    #[inline(always)]
    pub fn can_tri_sample(&self) -> CanTriSampleR {
        CanTriSampleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - auto re-transmission enabled"]
    #[inline(always)]
    pub fn auto_re_tx_en(&self) -> AutoReTxEnR {
        AutoReTxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - test mode enable, for lbk or silent mode use"]
    #[inline(always)]
    pub fn test_mode_en(&self) -> TestModeEnR {
        TestModeEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - rx fifo function enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RxFifoEnR {
        RxFifoEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable CAN FD protocol"]
    #[inline(always)]
    pub fn can_fd_en(&self) -> CanFdEnR {
        CanFdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - rx dma enable"]
    #[inline(always)]
    pub fn can_rxdma_en(&self) -> CanRxdmaEnR {
        CanRxdmaEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - crc mode select * 0: ISO crc * 1: non-ISO crc"]
    #[inline(always)]
    pub fn can_fd_crc_mode(&self) -> CanFdCrcModeR {
        CanFdCrcModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - automatic calculat ssp delay, ssp = second sample point"]
    #[inline(always)]
    pub fn can_fd_ssp_auto(&self) -> CanFdSspAutoR {
        CanFdSspAutoR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - can enable"]
    #[inline(always)]
    pub fn can_en(&mut self) -> CanEnW<'_, CanCtlSpec> {
        CanEnW::new(self, 0)
    }
    #[doc = "Bit 1 - request to bus_on"]
    #[inline(always)]
    pub fn bus_on_req(&mut self) -> BusOnReqW<'_, CanCtlSpec> {
        BusOnReqW::new(self, 1)
    }
    #[doc = "Bit 2 - triple sample mode"]
    #[inline(always)]
    pub fn can_tri_sample(&mut self) -> CanTriSampleW<'_, CanCtlSpec> {
        CanTriSampleW::new(self, 2)
    }
    #[doc = "Bit 3 - auto re-transmission enabled"]
    #[inline(always)]
    pub fn auto_re_tx_en(&mut self) -> AutoReTxEnW<'_, CanCtlSpec> {
        AutoReTxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - test mode enable, for lbk or silent mode use"]
    #[inline(always)]
    pub fn test_mode_en(&mut self) -> TestModeEnW<'_, CanCtlSpec> {
        TestModeEnW::new(self, 4)
    }
    #[doc = "Bit 5 - rx fifo function enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RxFifoEnW<'_, CanCtlSpec> {
        RxFifoEnW::new(self, 5)
    }
    #[doc = "Bit 6 - enable CAN FD protocol"]
    #[inline(always)]
    pub fn can_fd_en(&mut self) -> CanFdEnW<'_, CanCtlSpec> {
        CanFdEnW::new(self, 6)
    }
    #[doc = "Bit 7 - rx dma enable"]
    #[inline(always)]
    pub fn can_rxdma_en(&mut self) -> CanRxdmaEnW<'_, CanCtlSpec> {
        CanRxdmaEnW::new(self, 7)
    }
    #[doc = "Bit 8 - crc mode select * 0: ISO crc * 1: non-ISO crc"]
    #[inline(always)]
    pub fn can_fd_crc_mode(&mut self) -> CanFdCrcModeW<'_, CanCtlSpec> {
        CanFdCrcModeW::new(self, 8)
    }
    #[doc = "Bit 9 - automatic calculat ssp delay, ssp = second sample point"]
    #[inline(always)]
    pub fn can_fd_ssp_auto(&mut self) -> CanFdSspAutoW<'_, CanCtlSpec> {
        CanFdSspAutoW::new(self, 9)
    }
}
#[doc = "CAN bus control register\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanCtlSpec;
impl crate::RegisterSpec for CanCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ctl::R`](R) reader structure"]
impl crate::Readable for CanCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`can_ctl::W`](W) writer structure"]
impl crate::Writable for CanCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_CTL to value 0"]
impl crate::Resettable for CanCtlSpec {}
