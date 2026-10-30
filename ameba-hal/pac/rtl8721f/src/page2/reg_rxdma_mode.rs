#[doc = "Register `REG_RXDMA_MODE` reader"]
pub type R = crate::R<RegRxdmaModeSpec>;
#[doc = "Register `REG_RXDMA_MODE` writer"]
pub type W = crate::W<RegRxdmaModeSpec>;
#[doc = "Field `BIT_MASK_RX_PKT_PS` reader - RSVD"]
pub type BitMaskRxPktPsR = crate::BitReader;
#[doc = "Field `BIT_MASK_RX_PKT_PS` writer - RSVD"]
pub type BitMaskRxPktPsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DMA_MODE` reader - For usb use dma mode; for pcie clr this bit ; for SDIO use Rx DMA Aggregation mode"]
pub type BitDmaModeR = crate::BitReader;
#[doc = "Field `BIT_DMA_MODE` writer - For usb use dma mode; for pcie clr this bit ; for SDIO use Rx DMA Aggregation mode"]
pub type BitDmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BURST_CNT` reader - Burst number in dma mode, support at most 4burst(2'b11) P.S, DD suggest 0x11"]
pub type BitBurstCntR = crate::FieldReader;
#[doc = "Field `BIT_BURST_CNT` writer - Burst number in dma mode, support at most 4burst(2'b11) P.S, DD suggest 0x11"]
pub type BitBurstCntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_BURST_SIZE` reader - Every burst pkt len 2'b00 : 1KB 2'b01 : 512B 2'b10 : 64B Others : 64B P.S. USB 3.0 must set : 0x00 USB 2.0 High-speed must set : 0x01 USB 2.0 Full-speed must set : 0x10"]
pub type BitBurstSizeR = crate::FieldReader;
#[doc = "Field `BIT_BURST_SIZE` writer - Every burst pkt len 2'b00 : 1KB 2'b01 : 512B 2'b10 : 64B Others : 64B P.S. USB 3.0 must set : 0x00 USB 2.0 High-speed must set : 0x01 USB 2.0 Full-speed must set : 0x10"]
pub type BitBurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RXDMA_GCLK_EN` reader - rxdma_engine clock gating control,1:enable clock gating;0:disable clock gating"]
pub type BitRxdmaGclkEnR = crate::BitReader;
#[doc = "Field `BIT_RXDMA_GCLK_EN` writer - rxdma_engine clock gating control,1:enable clock gating;0:disable clock gating"]
pub type BitRxdmaGclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDMA_RESET` writer - rxdma rst"]
pub type BitRxdmaResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - RSVD"]
    #[inline(always)]
    pub fn bit_mask_rx_pkt_ps(&self) -> BitMaskRxPktPsR {
        BitMaskRxPktPsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For usb use dma mode; for pcie clr this bit ; for SDIO use Rx DMA Aggregation mode"]
    #[inline(always)]
    pub fn bit_dma_mode(&self) -> BitDmaModeR {
        BitDmaModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Burst number in dma mode, support at most 4burst(2'b11) P.S, DD suggest 0x11"]
    #[inline(always)]
    pub fn bit_burst_cnt(&self) -> BitBurstCntR {
        BitBurstCntR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Every burst pkt len 2'b00 : 1KB 2'b01 : 512B 2'b10 : 64B Others : 64B P.S. USB 3.0 must set : 0x00 USB 2.0 High-speed must set : 0x01 USB 2.0 Full-speed must set : 0x10"]
    #[inline(always)]
    pub fn bit_burst_size(&self) -> BitBurstSizeR {
        BitBurstSizeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - rxdma_engine clock gating control,1:enable clock gating;0:disable clock gating"]
    #[inline(always)]
    pub fn bit_rxdma_gclk_en(&self) -> BitRxdmaGclkEnR {
        BitRxdmaGclkEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - RSVD"]
    #[inline(always)]
    pub fn bit_mask_rx_pkt_ps(&mut self) -> BitMaskRxPktPsW<'_, RegRxdmaModeSpec> {
        BitMaskRxPktPsW::new(self, 0)
    }
    #[doc = "Bit 1 - For usb use dma mode; for pcie clr this bit ; for SDIO use Rx DMA Aggregation mode"]
    #[inline(always)]
    pub fn bit_dma_mode(&mut self) -> BitDmaModeW<'_, RegRxdmaModeSpec> {
        BitDmaModeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Burst number in dma mode, support at most 4burst(2'b11) P.S, DD suggest 0x11"]
    #[inline(always)]
    pub fn bit_burst_cnt(&mut self) -> BitBurstCntW<'_, RegRxdmaModeSpec> {
        BitBurstCntW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Every burst pkt len 2'b00 : 1KB 2'b01 : 512B 2'b10 : 64B Others : 64B P.S. USB 3.0 must set : 0x00 USB 2.0 High-speed must set : 0x01 USB 2.0 Full-speed must set : 0x10"]
    #[inline(always)]
    pub fn bit_burst_size(&mut self) -> BitBurstSizeW<'_, RegRxdmaModeSpec> {
        BitBurstSizeW::new(self, 4)
    }
    #[doc = "Bit 6 - rxdma_engine clock gating control,1:enable clock gating;0:disable clock gating"]
    #[inline(always)]
    pub fn bit_rxdma_gclk_en(&mut self) -> BitRxdmaGclkEnW<'_, RegRxdmaModeSpec> {
        BitRxdmaGclkEnW::new(self, 6)
    }
    #[doc = "Bit 7 - rxdma rst"]
    #[inline(always)]
    pub fn bit_rxdma_reset(&mut self) -> BitRxdmaResetW<'_, RegRxdmaModeSpec> {
        BitRxdmaResetW::new(self, 7)
    }
    #[doc = "Bits 8:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegRxdmaModeSpec> {
        RsvdW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxdma_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxdma_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxdmaModeSpec;
impl crate::RegisterSpec for RegRxdmaModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxdma_mode::R`](R) reader structure"]
impl crate::Readable for RegRxdmaModeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxdma_mode::W`](W) writer structure"]
impl crate::Writable for RegRxdmaModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXDMA_MODE to value 0x44"]
impl crate::Resettable for RegRxdmaModeSpec {
    const RESET_VALUE: u32 = 0x44;
}
