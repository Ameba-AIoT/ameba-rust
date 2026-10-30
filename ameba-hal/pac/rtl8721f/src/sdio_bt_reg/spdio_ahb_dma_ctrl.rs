#[doc = "Register `SPDIO_AHB_DMA_CTRL` reader"]
pub type R = crate::R<SpdioAhbDmaCtrlSpec>;
#[doc = "Register `SPDIO_AHB_DMA_CTRL` writer"]
pub type W = crate::W<SpdioAhbDmaCtrlSpec>;
#[doc = "Field `SPDIO_TXFF_WLEVEL` reader - SPDIO FIFO TX water level Range 1~126, it can be modified only when AHB_DMA_CS = 000"]
pub type SpdioTxffWlevelR = crate::FieldReader;
#[doc = "Field `SPDIO_TXFF_WLEVEL` writer - SPDIO FIFO TX water level Range 1~126, it can be modified only when AHB_DMA_CS = 000"]
pub type SpdioTxffWlevelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SPDIO_RXFF_WLEVEL` reader - SPDIO FIFO RX water level Range 1~126, it can be modified only when AHB_DMA_CS = 000"]
pub type SpdioRxffWlevelR = crate::FieldReader;
#[doc = "Field `SPDIO_RXFF_WLEVEL` writer - SPDIO FIFO RX water level Range 1~126, it can be modified only when AHB_DMA_CS = 000"]
pub type SpdioRxffWlevelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `AHB_DMA_CS` reader - AHB DMA state"]
pub type AhbDmaCsR = crate::FieldReader;
#[doc = "Field `AHBM_SPDIO_READY` reader - SPDIO AHB Master Hready signal"]
pub type AhbmSpdioReadyR = crate::BitReader;
#[doc = "Field `AHBM_SPDIO_TRANS` reader - SPDIO AHB Master HTRANS signal"]
pub type AhbmSpdioTransR = crate::FieldReader;
#[doc = "Field `RX_AHB_BUSY_WAIT_CNT` reader - When SPDIO RX transfer, AHB controller will wait BUSY counter for AHB access target not response READY signal. If timeout, AHB controller will issue AHB_BUS_RES_FAIL INT to CPU."]
pub type RxAhbBusyWaitCntR = crate::FieldReader;
#[doc = "Field `RX_AHB_BUSY_WAIT_CNT` writer - When SPDIO RX transfer, AHB controller will wait BUSY counter for AHB access target not response READY signal. If timeout, AHB controller will issue AHB_BUS_RES_FAIL INT to CPU."]
pub type RxAhbBusyWaitCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AHB_BURST_TYPE` reader - This field is used to control to SPDIO AHB support Burst type. b100: Support Burst 16 DW b010: Support Burst 8 DW b001: Support Burst 4 DW Default is b111, all support."]
pub type AhbBurstTypeR = crate::FieldReader;
#[doc = "Field `AHB_BURST_TYPE` writer - This field is used to control to SPDIO AHB support Burst type. b100: Support Burst 16 DW b010: Support Burst 8 DW b001: Support Burst 4 DW Default is b111, all support."]
pub type AhbBurstTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DISPATCH_TXAGG_PKT` reader - Enable SPDIO to dispatch the Aggregated TX packet."]
pub type DispatchTxaggPktR = crate::BitReader;
#[doc = "Field `DISPATCH_TXAGG_PKT` writer - Enable SPDIO to dispatch the Aggregated TX packet."]
pub type DispatchTxaggPktW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - SPDIO FIFO TX water level Range 1~126, it can be modified only when AHB_DMA_CS = 000"]
    #[inline(always)]
    pub fn spdio_txff_wlevel(&self) -> SpdioTxffWlevelR {
        SpdioTxffWlevelR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - SPDIO FIFO RX water level Range 1~126, it can be modified only when AHB_DMA_CS = 000"]
    #[inline(always)]
    pub fn spdio_rxff_wlevel(&self) -> SpdioRxffWlevelR {
        SpdioRxffWlevelR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - AHB DMA state"]
    #[inline(always)]
    pub fn ahb_dma_cs(&self) -> AhbDmaCsR {
        AhbDmaCsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - SPDIO AHB Master Hready signal"]
    #[inline(always)]
    pub fn ahbm_spdio_ready(&self) -> AhbmSpdioReadyR {
        AhbmSpdioReadyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - SPDIO AHB Master HTRANS signal"]
    #[inline(always)]
    pub fn ahbm_spdio_trans(&self) -> AhbmSpdioTransR {
        AhbmSpdioTransR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - When SPDIO RX transfer, AHB controller will wait BUSY counter for AHB access target not response READY signal. If timeout, AHB controller will issue AHB_BUS_RES_FAIL INT to CPU."]
    #[inline(always)]
    pub fn rx_ahb_busy_wait_cnt(&self) -> RxAhbBusyWaitCntR {
        RxAhbBusyWaitCntR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - This field is used to control to SPDIO AHB support Burst type. b100: Support Burst 16 DW b010: Support Burst 8 DW b001: Support Burst 4 DW Default is b111, all support."]
    #[inline(always)]
    pub fn ahb_burst_type(&self) -> AhbBurstTypeR {
        AhbBurstTypeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Enable SPDIO to dispatch the Aggregated TX packet."]
    #[inline(always)]
    pub fn dispatch_txagg_pkt(&self) -> DispatchTxaggPktR {
        DispatchTxaggPktR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - SPDIO FIFO TX water level Range 1~126, it can be modified only when AHB_DMA_CS = 000"]
    #[inline(always)]
    pub fn spdio_txff_wlevel(&mut self) -> SpdioTxffWlevelW<'_, SpdioAhbDmaCtrlSpec> {
        SpdioTxffWlevelW::new(self, 0)
    }
    #[doc = "Bits 8:14 - SPDIO FIFO RX water level Range 1~126, it can be modified only when AHB_DMA_CS = 000"]
    #[inline(always)]
    pub fn spdio_rxff_wlevel(&mut self) -> SpdioRxffWlevelW<'_, SpdioAhbDmaCtrlSpec> {
        SpdioRxffWlevelW::new(self, 8)
    }
    #[doc = "Bits 24:27 - When SPDIO RX transfer, AHB controller will wait BUSY counter for AHB access target not response READY signal. If timeout, AHB controller will issue AHB_BUS_RES_FAIL INT to CPU."]
    #[inline(always)]
    pub fn rx_ahb_busy_wait_cnt(&mut self) -> RxAhbBusyWaitCntW<'_, SpdioAhbDmaCtrlSpec> {
        RxAhbBusyWaitCntW::new(self, 24)
    }
    #[doc = "Bits 28:30 - This field is used to control to SPDIO AHB support Burst type. b100: Support Burst 16 DW b010: Support Burst 8 DW b001: Support Burst 4 DW Default is b111, all support."]
    #[inline(always)]
    pub fn ahb_burst_type(&mut self) -> AhbBurstTypeW<'_, SpdioAhbDmaCtrlSpec> {
        AhbBurstTypeW::new(self, 28)
    }
    #[doc = "Bit 31 - Enable SPDIO to dispatch the Aggregated TX packet."]
    #[inline(always)]
    pub fn dispatch_txagg_pkt(&mut self) -> DispatchTxaggPktW<'_, SpdioAhbDmaCtrlSpec> {
        DispatchTxaggPktW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_ahb_dma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_ahb_dma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioAhbDmaCtrlSpec;
impl crate::RegisterSpec for SpdioAhbDmaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_ahb_dma_ctrl::R`](R) reader structure"]
impl crate::Readable for SpdioAhbDmaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_ahb_dma_ctrl::W`](W) writer structure"]
impl crate::Writable for SpdioAhbDmaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_AHB_DMA_CTRL to value 0x0020_0000"]
impl crate::Resettable for SpdioAhbDmaCtrlSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
