#[doc = "Register `REG_RXPKT_NUM` reader"]
pub type R = crate::R<RegRxpktNumSpec>;
#[doc = "Register `REG_RXPKT_NUM` writer"]
pub type W = crate::W<RegRxpktNumSpec>;
#[doc = "Field `BIT_FW_UPD_RXPKT_RDPTR` reader - FW shall update this register before FW write RXPKT_RELEASE_POLL to 1."]
pub type BitFwUpdRxpktRdptrR = crate::FieldReader<u16>;
#[doc = "Field `BIT_FW_UPD_RXPKT_RDPTR` writer - FW shall update this register before FW write RXPKT_RELEASE_POLL to 1."]
pub type BitFwUpdRxpktRdptrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_RXPKT_RELEASE_POLL` reader - When this bit is set, RXDMA will decrease RX PKT counter by one. Before this bit is polled, FW shall update RXFF_RD_PTR first."]
pub type BitRxpktReleasePollR = crate::BitReader;
#[doc = "Field `BIT_RXDMA_IDLE` reader - When RXMA finishes on-going DMA operation, RXMDA will report idle state in this bit. FW can start releasing packets after RXDMA entering idle mode."]
pub type BitRxdmaIdleR = crate::BitReader;
#[doc = "Field `BIT_FW_RELEASE_EN` reader - When this bit is set, RXDMA will enter this mode after on-going RXDMA packet to host completed, and stop DMA packet to host. RXDMA will then report"]
pub type BitFwReleaseEnR = crate::BitReader;
#[doc = "Field `BIT_FW_RELEASE_EN` writer - When this bit is set, RXDMA will enter this mode after on-going RXDMA packet to host completed, and stop DMA packet to host. RXDMA will then report"]
pub type BitFwReleaseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXDMA_REQ` reader - rxdma has request"]
pub type BitRxdmaReqR = crate::BitReader;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_RXPKT_QUEUED` reader - The number of packets in RXPKTBUF."]
pub type BitRxpktQueuedR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - FW shall update this register before FW write RXPKT_RELEASE_POLL to 1."]
    #[inline(always)]
    pub fn bit_fw_upd_rxpkt_rdptr(&self) -> BitFwUpdRxpktRdptrR {
        BitFwUpdRxpktRdptrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - When this bit is set, RXDMA will decrease RX PKT counter by one. Before this bit is polled, FW shall update RXFF_RD_PTR first."]
    #[inline(always)]
    pub fn bit_rxpkt_release_poll(&self) -> BitRxpktReleasePollR {
        BitRxpktReleasePollR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When RXMA finishes on-going DMA operation, RXMDA will report idle state in this bit. FW can start releasing packets after RXDMA entering idle mode."]
    #[inline(always)]
    pub fn bit_rxdma_idle(&self) -> BitRxdmaIdleR {
        BitRxdmaIdleR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When this bit is set, RXDMA will enter this mode after on-going RXDMA packet to host completed, and stop DMA packet to host. RXDMA will then report"]
    #[inline(always)]
    pub fn bit_fw_release_en(&self) -> BitFwReleaseEnR {
        BitFwReleaseEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - rxdma has request"]
    #[inline(always)]
    pub fn bit_rxdma_req(&self) -> BitRxdmaReqR {
        BitRxdmaReqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:23 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31 - The number of packets in RXPKTBUF."]
    #[inline(always)]
    pub fn bit_rxpkt_queued(&self) -> BitRxpktQueuedR {
        BitRxpktQueuedR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - FW shall update this register before FW write RXPKT_RELEASE_POLL to 1."]
    #[inline(always)]
    pub fn bit_fw_upd_rxpkt_rdptr(&mut self) -> BitFwUpdRxpktRdptrW<'_, RegRxpktNumSpec> {
        BitFwUpdRxpktRdptrW::new(self, 0)
    }
    #[doc = "Bit 18 - When this bit is set, RXDMA will enter this mode after on-going RXDMA packet to host completed, and stop DMA packet to host. RXDMA will then report"]
    #[inline(always)]
    pub fn bit_fw_release_en(&mut self) -> BitFwReleaseEnW<'_, RegRxpktNumSpec> {
        BitFwReleaseEnW::new(self, 18)
    }
    #[doc = "Bits 21:23 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegRxpktNumSpec> {
        RsvdW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxpkt_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxpkt_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxpktNumSpec;
impl crate::RegisterSpec for RegRxpktNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxpkt_num::R`](R) reader structure"]
impl crate::Readable for RegRxpktNumSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxpkt_num::W`](W) writer structure"]
impl crate::Writable for RegRxpktNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXPKT_NUM to value 0"]
impl crate::Resettable for RegRxpktNumSpec {}
