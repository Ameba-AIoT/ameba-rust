#[doc = "Register `REG_RXDMA_AGG_PG_TH` reader"]
pub type R = crate::R<RegRxdmaAggPgThSpec>;
#[doc = "Register `REG_RXDMA_AGG_PG_TH` writer"]
pub type W = crate::W<RegRxdmaAggPgThSpec>;
#[doc = "Field `BIT_RXDMA_AGG_PG_TH` reader - rxdma agg length threshold if sdio : unit 1K, others unit is 4K if use RXDMA_AGG_OLD_MOD in use, unit is 1K"]
pub type BitRxdmaAggPgThR = crate::FieldReader;
#[doc = "Field `BIT_RXDMA_AGG_PG_TH` writer - rxdma agg length threshold if sdio : unit 1K, others unit is 4K if use RXDMA_AGG_OLD_MOD in use, unit is 1K"]
pub type BitRxdmaAggPgThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_DMA_TIMEOUT_TH` reader - dma agg timeout time : unit : 32us"]
pub type BitDmaTimeoutThR = crate::FieldReader;
#[doc = "Field `BIT_DMA_TIMEOUT_TH` writer - dma agg timeout time : unit : 32us"]
pub type BitDmaTimeoutThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_PKT_NUM_WOL` reader - In wol mode, 8051 should refer this packet counter in RXFF"]
pub type BitPktNumWolR = crate::FieldReader;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::FieldReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_EN_PRE_CALC` reader - 0: rde start rxdma when aggregated length over specified threshold; 1: rde start rxdma when next packet, if aggregated, will cause the total length over specified threshold"]
pub type BitEnPreCalcR = crate::BitReader;
#[doc = "Field `BIT_EN_PRE_CALC` writer - 0: rde start rxdma when aggregated length over specified threshold; 1: rde start rxdma when next packet, if aggregated, will cause the total length over specified threshold"]
pub type BitEnPreCalcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDMA_STORE` reader - usb rx dma old mode store pkt and forward"]
pub type BitRxdmaStoreR = crate::BitReader;
#[doc = "Field `BIT_RXDMA_STORE` writer - usb rx dma old mode store pkt and forward"]
pub type BitRxdmaStoreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - rxdma agg length threshold if sdio : unit 1K, others unit is 4K if use RXDMA_AGG_OLD_MOD in use, unit is 1K"]
    #[inline(always)]
    pub fn bit_rxdma_agg_pg_th(&self) -> BitRxdmaAggPgThR {
        BitRxdmaAggPgThR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - dma agg timeout time : unit : 32us"]
    #[inline(always)]
    pub fn bit_dma_timeout_th(&self) -> BitDmaTimeoutThR {
        BitDmaTimeoutThR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In wol mode, 8051 should refer this packet counter in RXFF"]
    #[inline(always)]
    pub fn bit_pkt_num_wol(&self) -> BitPktNumWolR {
        BitPktNumWolR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 0: rde start rxdma when aggregated length over specified threshold; 1: rde start rxdma when next packet, if aggregated, will cause the total length over specified threshold"]
    #[inline(always)]
    pub fn bit_en_pre_calc(&self) -> BitEnPreCalcR {
        BitEnPreCalcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - usb rx dma old mode store pkt and forward"]
    #[inline(always)]
    pub fn bit_rxdma_store(&self) -> BitRxdmaStoreR {
        BitRxdmaStoreR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - rxdma agg length threshold if sdio : unit 1K, others unit is 4K if use RXDMA_AGG_OLD_MOD in use, unit is 1K"]
    #[inline(always)]
    pub fn bit_rxdma_agg_pg_th(&mut self) -> BitRxdmaAggPgThW<'_, RegRxdmaAggPgThSpec> {
        BitRxdmaAggPgThW::new(self, 0)
    }
    #[doc = "Bits 8:15 - dma agg timeout time : unit : 32us"]
    #[inline(always)]
    pub fn bit_dma_timeout_th(&mut self) -> BitDmaTimeoutThW<'_, RegRxdmaAggPgThSpec> {
        BitDmaTimeoutThW::new(self, 8)
    }
    #[doc = "Bits 24:28 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegRxdmaAggPgThSpec> {
        Rsvd2W::new(self, 24)
    }
    #[doc = "Bit 29 - 0: rde start rxdma when aggregated length over specified threshold; 1: rde start rxdma when next packet, if aggregated, will cause the total length over specified threshold"]
    #[inline(always)]
    pub fn bit_en_pre_calc(&mut self) -> BitEnPreCalcW<'_, RegRxdmaAggPgThSpec> {
        BitEnPreCalcW::new(self, 29)
    }
    #[doc = "Bit 30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegRxdmaAggPgThSpec> {
        RsvdW::new(self, 30)
    }
    #[doc = "Bit 31 - usb rx dma old mode store pkt and forward"]
    #[inline(always)]
    pub fn bit_rxdma_store(&mut self) -> BitRxdmaStoreW<'_, RegRxdmaAggPgThSpec> {
        BitRxdmaStoreW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxdma_agg_pg_th::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxdma_agg_pg_th::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxdmaAggPgThSpec;
impl crate::RegisterSpec for RegRxdmaAggPgThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxdma_agg_pg_th::R`](R) reader structure"]
impl crate::Readable for RegRxdmaAggPgThSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxdma_agg_pg_th::W`](W) writer structure"]
impl crate::Writable for RegRxdmaAggPgThSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXDMA_AGG_PG_TH to value 0x2003"]
impl crate::Resettable for RegRxdmaAggPgThSpec {
    const RESET_VALUE: u32 = 0x2003;
}
