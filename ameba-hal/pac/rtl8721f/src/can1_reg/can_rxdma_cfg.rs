#[doc = "Register `CAN_RXDMA_CFG` reader"]
pub type R = crate::R<CanRxdmaCfgSpec>;
#[doc = "Register `CAN_RXDMA_CFG` writer"]
pub type W = crate::W<CanRxdmaCfgSpec>;
#[doc = "Field `can_rxdma_frame_size` reader - can rx dma data frame size = ceil(<rx_dma_data bytes number>/4 ), for example: rx_dma_data is 9~12 bytes, frame size is 3"]
pub type CanRxdmaFrameSizeR = crate::FieldReader<u16>;
#[doc = "Field `can_rxdma_src_msize` reader - rx dma burst size: 'b00: dma burst size=1 'b01: dma burst size=4 'b10: dma burst size=8 'b11: dma burst size=16"]
pub type CanRxdmaSrcMsizeR = crate::FieldReader;
#[doc = "Field `can_rxdma_src_msize` writer - rx dma burst size: 'b00: dma burst size=1 'b01: dma burst size=4 'b10: dma burst size=8 'b11: dma burst size=16"]
pub type CanRxdmaSrcMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `can_rxdma_owner` reader - can rxdma owner is : 0:GDMA 1:CAN"]
pub type CanRxdmaOwnerR = crate::BitReader;
#[doc = "Field `can_rxdma_owner` writer - can rxdma owner is : 0:GDMA 1:CAN"]
pub type CanRxdmaOwnerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - can rx dma data frame size = ceil(<rx_dma_data bytes number>/4 ), for example: rx_dma_data is 9~12 bytes, frame size is 3"]
    #[inline(always)]
    pub fn can_rxdma_frame_size(&self) -> CanRxdmaFrameSizeR {
        CanRxdmaFrameSizeR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:17 - rx dma burst size: 'b00: dma burst size=1 'b01: dma burst size=4 'b10: dma burst size=8 'b11: dma burst size=16"]
    #[inline(always)]
    pub fn can_rxdma_src_msize(&self) -> CanRxdmaSrcMsizeR {
        CanRxdmaSrcMsizeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - can rxdma owner is : 0:GDMA 1:CAN"]
    #[inline(always)]
    pub fn can_rxdma_owner(&self) -> CanRxdmaOwnerR {
        CanRxdmaOwnerR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - rx dma burst size: 'b00: dma burst size=1 'b01: dma burst size=4 'b10: dma burst size=8 'b11: dma burst size=16"]
    #[inline(always)]
    pub fn can_rxdma_src_msize(&mut self) -> CanRxdmaSrcMsizeW<'_, CanRxdmaCfgSpec> {
        CanRxdmaSrcMsizeW::new(self, 16)
    }
    #[doc = "Bit 24 - can rxdma owner is : 0:GDMA 1:CAN"]
    #[inline(always)]
    pub fn can_rxdma_owner(&mut self) -> CanRxdmaOwnerW<'_, CanRxdmaCfgSpec> {
        CanRxdmaOwnerW::new(self, 24)
    }
}
#[doc = "RX dma configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`can_rxdma_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_rxdma_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRxdmaCfgSpec;
impl crate::RegisterSpec for CanRxdmaCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rxdma_cfg::R`](R) reader structure"]
impl crate::Readable for CanRxdmaCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`can_rxdma_cfg::W`](W) writer structure"]
impl crate::Writable for CanRxdmaCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RXDMA_CFG to value 0x0001_0002"]
impl crate::Resettable for CanRxdmaCfgSpec {
    const RESET_VALUE: u32 = 0x0001_0002;
}
