#[doc = "Register `REG_TXDMA_PQ_MAP` reader"]
pub type R = crate::R<RegTxdmaPqMapSpec>;
#[doc = "Register `REG_TXDMA_PQ_MAP` writer"]
pub type W = crate::W<RegTxdmaPqMapSpec>;
#[doc = "Field `BIT_RXDMA_BW_EN` reader - Enable RXDMA Arbitrator priority for Host interface."]
pub type BitRxdmaBwEnR = crate::BitReader;
#[doc = "Field `BIT_RXDMA_BW_EN` writer - Enable RXDMA Arbitrator priority for Host interface."]
pub type BitRxdmaBwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENSHFT` reader - When this bit is set, RX shift to fit alignment is enabled."]
pub type BitEnshftR = crate::BitReader;
#[doc = "Field `BIT_ENSHFT` writer - When this bit is set, RX shift to fit alignment is enabled."]
pub type BitEnshftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDMA_AGG_EN` reader - When this bit is set, then RXDMA will inform host to receive RX packets, when packet page number in RXFF exceeds the threshold set in RXDMA_AGG_PG_TH register. When this bit is set, then page size of receive packet buffer shall be set larger than 64 bytes/page"]
pub type BitRxdmaAggEnR = crate::BitReader;
#[doc = "Field `BIT_RXDMA_AGG_EN` writer - When this bit is set, then RXDMA will inform host to receive RX packets, when packet page number in RXFF exceeds the threshold set in RXDMA_AGG_PG_TH register. When this bit is set, then page size of receive packet buffer shall be set larger than 64 bytes/page"]
pub type BitRxdmaAggEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TXDMA_VOQ_MAP` reader - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaVoqMapR = crate::FieldReader;
#[doc = "Field `BIT_TXDMA_VOQ_MAP` writer - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaVoqMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TXDMA_VIQ_MAP` reader - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaViqMapR = crate::FieldReader;
#[doc = "Field `BIT_TXDMA_VIQ_MAP` writer - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaViqMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TXDMA_BEQ_MAP` reader - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaBeqMapR = crate::FieldReader;
#[doc = "Field `BIT_TXDMA_BEQ_MAP` writer - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaBeqMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TXDMA_BKQ_MAP` reader - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaBkqMapR = crate::FieldReader;
#[doc = "Field `BIT_TXDMA_BKQ_MAP` writer - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaBkqMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TXDMA_MGQ_MAP` reader - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaMgqMapR = crate::FieldReader;
#[doc = "Field `BIT_TXDMA_MGQ_MAP` writer - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaMgqMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TXDMA_HIQ_MAP` reader - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaHiqMapR = crate::FieldReader;
#[doc = "Field `BIT_TXDMA_HIQ_MAP` writer - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaHiqMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_TXDMA_CMQ_MAP` reader - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaCmqMapR = crate::FieldReader;
#[doc = "Field `BIT_TXDMA_CMQ_MAP` writer - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
pub type BitTxdmaCmqMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - Enable RXDMA Arbitrator priority for Host interface."]
    #[inline(always)]
    pub fn bit_rxdma_bw_en(&self) -> BitRxdmaBwEnR {
        BitRxdmaBwEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is set, RX shift to fit alignment is enabled."]
    #[inline(always)]
    pub fn bit_enshft(&self) -> BitEnshftR {
        BitEnshftR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set, then RXDMA will inform host to receive RX packets, when packet page number in RXFF exceeds the threshold set in RXDMA_AGG_PG_TH register. When this bit is set, then page size of receive packet buffer shall be set larger than 64 bytes/page"]
    #[inline(always)]
    pub fn bit_rxdma_agg_en(&self) -> BitRxdmaAggEnR {
        BitRxdmaAggEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_voq_map(&self) -> BitTxdmaVoqMapR {
        BitTxdmaVoqMapR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_viq_map(&self) -> BitTxdmaViqMapR {
        BitTxdmaViqMapR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_beq_map(&self) -> BitTxdmaBeqMapR {
        BitTxdmaBeqMapR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_bkq_map(&self) -> BitTxdmaBkqMapR {
        BitTxdmaBkqMapR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_mgq_map(&self) -> BitTxdmaMgqMapR {
        BitTxdmaMgqMapR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_hiq_map(&self) -> BitTxdmaHiqMapR {
        BitTxdmaHiqMapR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_cmq_map(&self) -> BitTxdmaCmqMapR {
        BitTxdmaCmqMapR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RXDMA Arbitrator priority for Host interface."]
    #[inline(always)]
    pub fn bit_rxdma_bw_en(&mut self) -> BitRxdmaBwEnW<'_, RegTxdmaPqMapSpec> {
        BitRxdmaBwEnW::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit is set, RX shift to fit alignment is enabled."]
    #[inline(always)]
    pub fn bit_enshft(&mut self) -> BitEnshftW<'_, RegTxdmaPqMapSpec> {
        BitEnshftW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit is set, then RXDMA will inform host to receive RX packets, when packet page number in RXFF exceeds the threshold set in RXDMA_AGG_PG_TH register. When this bit is set, then page size of receive packet buffer shall be set larger than 64 bytes/page"]
    #[inline(always)]
    pub fn bit_rxdma_agg_en(&mut self) -> BitRxdmaAggEnW<'_, RegTxdmaPqMapSpec> {
        BitRxdmaAggEnW::new(self, 2)
    }
    #[doc = "Bit 3 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegTxdmaPqMapSpec> {
        Rsvd2W::new(self, 3)
    }
    #[doc = "Bits 4:5 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_voq_map(&mut self) -> BitTxdmaVoqMapW<'_, RegTxdmaPqMapSpec> {
        BitTxdmaVoqMapW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_viq_map(&mut self) -> BitTxdmaViqMapW<'_, RegTxdmaPqMapSpec> {
        BitTxdmaViqMapW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_beq_map(&mut self) -> BitTxdmaBeqMapW<'_, RegTxdmaPqMapSpec> {
        BitTxdmaBeqMapW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_bkq_map(&mut self) -> BitTxdmaBkqMapW<'_, RegTxdmaPqMapSpec> {
        BitTxdmaBkqMapW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_mgq_map(&mut self) -> BitTxdmaMgqMapW<'_, RegTxdmaPqMapSpec> {
        BitTxdmaMgqMapW::new(self, 12)
    }
    #[doc = "Bits 14:15 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_hiq_map(&mut self) -> BitTxdmaHiqMapW<'_, RegTxdmaPqMapSpec> {
        BitTxdmaHiqMapW::new(self, 14)
    }
    #[doc = "Bits 16:17 - 00: Extra priority queue 01: Low priority queue 10: Normal priority queue 11: High priority queue"]
    #[inline(always)]
    pub fn bit_txdma_cmq_map(&mut self) -> BitTxdmaCmqMapW<'_, RegTxdmaPqMapSpec> {
        BitTxdmaCmqMapW::new(self, 16)
    }
    #[doc = "Bits 18:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTxdmaPqMapSpec> {
        RsvdW::new(self, 18)
    }
}
#[doc = "RX DMA Fuction enable\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txdma_pq_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txdma_pq_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxdmaPqMapSpec;
impl crate::RegisterSpec for RegTxdmaPqMapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txdma_pq_map::R`](R) reader structure"]
impl crate::Readable for RegTxdmaPqMapSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txdma_pq_map::W`](W) writer structure"]
impl crate::Writable for RegTxdmaPqMapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXDMA_PQ_MAP to value 0"]
impl crate::Resettable for RegTxdmaPqMapSpec {}
