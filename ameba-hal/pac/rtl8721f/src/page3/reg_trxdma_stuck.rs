#[doc = "Register `REG_TRXDMA_STUCK` reader"]
pub type R = crate::R<RegTrxdmaStuckSpec>;
#[doc = "Register `REG_TRXDMA_STUCK` writer"]
pub type W = crate::W<RegTrxdmaStuckSpec>;
#[doc = "Field `BIT_EN_DBG_STUCK` reader - enable trx dma stuck"]
pub type BitEnDbgStuckR = crate::BitReader;
#[doc = "Field `BIT_EN_DBG_STUCK` writer - enable trx dma stuck"]
pub type BitEnDbgStuckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HD_SIZE_ERR` reader - tx dma header size less than TXDESC_SIZE(40d)"]
pub type BitHdSizeErrR = crate::BitReader;
#[doc = "Field `BIT_RXBUF_ZERO_ERR` reader - rx pkt size equal to 0 when caculate buffer size"]
pub type BitRxbufZeroErrR = crate::BitReader;
#[doc = "Field `BIT_MST_BUSY` reader - axi master write/read busy"]
pub type BitMstBusyR = crate::BitReader;
#[doc = "Field `BIT_TXDMA_STUCK` reader - tx dma stuck"]
pub type BitTxdmaStuckR = crate::BitReader;
#[doc = "Field `BIT_RXDMA_STUCK` reader - rx dma stuck"]
pub type BitRxdmaStuckR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - enable trx dma stuck"]
    #[inline(always)]
    pub fn bit_en_dbg_stuck(&self) -> BitEnDbgStuckR {
        BitEnDbgStuckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx dma header size less than TXDESC_SIZE(40d)"]
    #[inline(always)]
    pub fn bit_hd_size_err(&self) -> BitHdSizeErrR {
        BitHdSizeErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx pkt size equal to 0 when caculate buffer size"]
    #[inline(always)]
    pub fn bit_rxbuf_zero_err(&self) -> BitRxbufZeroErrR {
        BitRxbufZeroErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - axi master write/read busy"]
    #[inline(always)]
    pub fn bit_mst_busy(&self) -> BitMstBusyR {
        BitMstBusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - tx dma stuck"]
    #[inline(always)]
    pub fn bit_txdma_stuck(&self) -> BitTxdmaStuckR {
        BitTxdmaStuckR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - rx dma stuck"]
    #[inline(always)]
    pub fn bit_rxdma_stuck(&self) -> BitRxdmaStuckR {
        BitRxdmaStuckR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable trx dma stuck"]
    #[inline(always)]
    pub fn bit_en_dbg_stuck(&mut self) -> BitEnDbgStuckW<'_, RegTrxdmaStuckSpec> {
        BitEnDbgStuckW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_trxdma_stuck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_trxdma_stuck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTrxdmaStuckSpec;
impl crate::RegisterSpec for RegTrxdmaStuckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_trxdma_stuck::R`](R) reader structure"]
impl crate::Readable for RegTrxdmaStuckSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_trxdma_stuck::W`](W) writer structure"]
impl crate::Writable for RegTrxdmaStuckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRXDMA_STUCK to value 0"]
impl crate::Resettable for RegTrxdmaStuckSpec {}
