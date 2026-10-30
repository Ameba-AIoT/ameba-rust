#[doc = "Register `REG_TXDMA_OFFSET_CHK` reader"]
pub type R = crate::R<RegTxdmaOffsetChkSpec>;
#[doc = "Register `REG_TXDMA_OFFSET_CHK` writer"]
pub type W = crate::W<RegTxdmaOffsetChkSpec>;
#[doc = "Field `BIT_CHK_OFFSET` reader - Offset value is check by Tx DMA engine"]
pub type BitChkOffsetR = crate::FieldReader;
#[doc = "Field `BIT_CHK_OFFSET` writer - Offset value is check by Tx DMA engine"]
pub type BitChkOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_CHECK_OFFSET_EN` reader - Enable Tx DMA to check offset value"]
pub type BitCheckOffsetEnR = crate::BitReader;
#[doc = "Field `BIT_CHECK_OFFSET_EN` writer - Enable Tx DMA to check offset value"]
pub type BitCheckOffsetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DROP_DATA_EN` reader - Enable Tx DMA to drop the redundant data of packet"]
pub type BitDropDataEnR = crate::BitReader;
#[doc = "Field `BIT_DROP_DATA_EN` writer - Enable Tx DMA to drop the redundant data of packet"]
pub type BitDropDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHK_PG_TH_EN` reader - Enable Tx DMA to check total pages if it is under page threshold"]
pub type BitChkPgThEnR = crate::BitReader;
#[doc = "Field `BIT_CHK_PG_TH_EN` writer - Enable Tx DMA to check total pages if it is under page threshold"]
pub type BitChkPgThEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RST_WRPTR` reader - Reset the write pointer of sync FIFO"]
pub type BitRstWrptrR = crate::BitReader;
#[doc = "Field `BIT_RST_WRPTR` writer - Reset the write pointer of sync FIFO"]
pub type BitRstWrptrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RST_RDPTR` reader - Reset the read pointer of sync FIFO"]
pub type BitRstRdptrR = crate::BitReader;
#[doc = "Field `BIT_RST_RDPTR` writer - Reset the read pointer of sync FIFO"]
pub type BitRstRdptrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DSC_CHKSUM_EN` reader - Enable SDIO checksum"]
pub type BitDscChksumEnR = crate::BitReader;
#[doc = "Field `BIT_DSC_CHKSUM_EN` writer - Enable SDIO checksum"]
pub type BitDscChksumEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_6` reader - RSVD"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `RSVD_6` writer - RSVD"]
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_5` reader - RSVD"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `RSVD_5` writer - RSVD"]
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PG_UNDER_TH` reader - Page threshold value is check by Tx DMA engine."]
pub type BitPgUnderThR = crate::FieldReader;
#[doc = "Field `BIT_PG_UNDER_TH` writer - Page threshold value is check by Tx DMA engine."]
pub type BitPgUnderThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD_4` reader - RSVD"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_4` writer - RSVD"]
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_3` reader - RSVD"]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD_3` writer - RSVD"]
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::BitReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_PCIE_FIFO` reader - Enable pcie fifo store forward txdesc as before mode"]
pub type BitEnPcieFifoR = crate::BitReader;
#[doc = "Field `BIT_EN_PCIE_FIFO` writer - Enable pcie fifo store forward txdesc as before mode"]
pub type BitEnPcieFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_TXQUE_CLR` reader - Enable tx queue clear tx pkt in tde in usb mode"]
pub type BitEnTxqueClrR = crate::BitReader;
#[doc = "Field `BIT_EN_TXQUE_CLR` writer - Enable tx queue clear tx pkt in tde in usb mode"]
pub type BitEnTxqueClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_DMA_PERMU_P` reader - Enable every dma latch fifo ptr for pcie as usb mode as one pkt trans in permu"]
pub type BitEnDmaPermuPR = crate::BitReader;
#[doc = "Field `BIT_EN_DMA_PERMU_P` writer - Enable every dma latch fifo ptr for pcie as usb mode as one pkt trans in permu"]
pub type BitEnDmaPermuPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_CHKERR_FINISH` reader - Let checksum offload statemachine finish if checksum parser error"]
pub type BitEnChkerrFinishR = crate::BitReader;
#[doc = "Field `BIT_EN_CHKERR_FINISH` writer - Let checksum offload statemachine finish if checksum parser error"]
pub type BitEnChkerrFinishW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Offset value is check by Tx DMA engine"]
    #[inline(always)]
    pub fn bit_chk_offset(&self) -> BitChkOffsetR {
        BitChkOffsetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable Tx DMA to check offset value"]
    #[inline(always)]
    pub fn bit_check_offset_en(&self) -> BitCheckOffsetEnR {
        BitCheckOffsetEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Tx DMA to drop the redundant data of packet"]
    #[inline(always)]
    pub fn bit_drop_data_en(&self) -> BitDropDataEnR {
        BitDropDataEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Tx DMA to check total pages if it is under page threshold"]
    #[inline(always)]
    pub fn bit_chk_pg_th_en(&self) -> BitChkPgThEnR {
        BitChkPgThEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset the write pointer of sync FIFO"]
    #[inline(always)]
    pub fn bit_rst_wrptr(&self) -> BitRstWrptrR {
        BitRstWrptrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset the read pointer of sync FIFO"]
    #[inline(always)]
    pub fn bit_rst_rdptr(&self) -> BitRstRdptrR {
        BitRstRdptrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable SDIO checksum"]
    #[inline(always)]
    pub fn bit_dsc_chksum_en(&self) -> BitDscChksumEnR {
        BitDscChksumEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RSVD"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Page threshold value is check by Tx DMA engine."]
    #[inline(always)]
    pub fn bit_pg_under_th(&self) -> BitPgUnderThR {
        BitPgUnderThR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable pcie fifo store forward txdesc as before mode"]
    #[inline(always)]
    pub fn bit_en_pcie_fifo(&self) -> BitEnPcieFifoR {
        BitEnPcieFifoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable tx queue clear tx pkt in tde in usb mode"]
    #[inline(always)]
    pub fn bit_en_txque_clr(&self) -> BitEnTxqueClrR {
        BitEnTxqueClrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable every dma latch fifo ptr for pcie as usb mode as one pkt trans in permu"]
    #[inline(always)]
    pub fn bit_en_dma_permu_p(&self) -> BitEnDmaPermuPR {
        BitEnDmaPermuPR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Let checksum offload statemachine finish if checksum parser error"]
    #[inline(always)]
    pub fn bit_en_chkerr_finish(&self) -> BitEnChkerrFinishR {
        BitEnChkerrFinishR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Offset value is check by Tx DMA engine"]
    #[inline(always)]
    pub fn bit_chk_offset(&mut self) -> BitChkOffsetW<'_, RegTxdmaOffsetChkSpec> {
        BitChkOffsetW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable Tx DMA to check offset value"]
    #[inline(always)]
    pub fn bit_check_offset_en(&mut self) -> BitCheckOffsetEnW<'_, RegTxdmaOffsetChkSpec> {
        BitCheckOffsetEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Tx DMA to drop the redundant data of packet"]
    #[inline(always)]
    pub fn bit_drop_data_en(&mut self) -> BitDropDataEnW<'_, RegTxdmaOffsetChkSpec> {
        BitDropDataEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Tx DMA to check total pages if it is under page threshold"]
    #[inline(always)]
    pub fn bit_chk_pg_th_en(&mut self) -> BitChkPgThEnW<'_, RegTxdmaOffsetChkSpec> {
        BitChkPgThEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset the write pointer of sync FIFO"]
    #[inline(always)]
    pub fn bit_rst_wrptr(&mut self) -> BitRstWrptrW<'_, RegTxdmaOffsetChkSpec> {
        BitRstWrptrW::new(self, 11)
    }
    #[doc = "Bit 12 - Reset the read pointer of sync FIFO"]
    #[inline(always)]
    pub fn bit_rst_rdptr(&mut self) -> BitRstRdptrW<'_, RegTxdmaOffsetChkSpec> {
        BitRstRdptrW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable SDIO checksum"]
    #[inline(always)]
    pub fn bit_dsc_chksum_en(&mut self) -> BitDscChksumEnW<'_, RegTxdmaOffsetChkSpec> {
        BitDscChksumEnW::new(self, 13)
    }
    #[doc = "Bit 14 - RSVD"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> Rsvd6W<'_, RegTxdmaOffsetChkSpec> {
        Rsvd6W::new(self, 14)
    }
    #[doc = "Bit 15 - RSVD"]
    #[inline(always)]
    pub fn rsvd_5(&mut self) -> Rsvd5W<'_, RegTxdmaOffsetChkSpec> {
        Rsvd5W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Page threshold value is check by Tx DMA engine."]
    #[inline(always)]
    pub fn bit_pg_under_th(&mut self) -> BitPgUnderThW<'_, RegTxdmaOffsetChkSpec> {
        BitPgUnderThW::new(self, 16)
    }
    #[doc = "Bit 24 - RSVD"]
    #[inline(always)]
    pub fn rsvd_4(&mut self) -> Rsvd4W<'_, RegTxdmaOffsetChkSpec> {
        Rsvd4W::new(self, 24)
    }
    #[doc = "Bit 25 - RSVD"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> Rsvd3W<'_, RegTxdmaOffsetChkSpec> {
        Rsvd3W::new(self, 25)
    }
    #[doc = "Bit 26 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegTxdmaOffsetChkSpec> {
        Rsvd2W::new(self, 26)
    }
    #[doc = "Bit 27 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTxdmaOffsetChkSpec> {
        RsvdW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable pcie fifo store forward txdesc as before mode"]
    #[inline(always)]
    pub fn bit_en_pcie_fifo(&mut self) -> BitEnPcieFifoW<'_, RegTxdmaOffsetChkSpec> {
        BitEnPcieFifoW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable tx queue clear tx pkt in tde in usb mode"]
    #[inline(always)]
    pub fn bit_en_txque_clr(&mut self) -> BitEnTxqueClrW<'_, RegTxdmaOffsetChkSpec> {
        BitEnTxqueClrW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable every dma latch fifo ptr for pcie as usb mode as one pkt trans in permu"]
    #[inline(always)]
    pub fn bit_en_dma_permu_p(&mut self) -> BitEnDmaPermuPW<'_, RegTxdmaOffsetChkSpec> {
        BitEnDmaPermuPW::new(self, 30)
    }
    #[doc = "Bit 31 - Let checksum offload statemachine finish if checksum parser error"]
    #[inline(always)]
    pub fn bit_en_chkerr_finish(&mut self) -> BitEnChkerrFinishW<'_, RegTxdmaOffsetChkSpec> {
        BitEnChkerrFinishW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txdma_offset_chk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txdma_offset_chk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxdmaOffsetChkSpec;
impl crate::RegisterSpec for RegTxdmaOffsetChkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txdma_offset_chk::R`](R) reader structure"]
impl crate::Readable for RegTxdmaOffsetChkSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txdma_offset_chk::W`](W) writer structure"]
impl crate::Writable for RegTxdmaOffsetChkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXDMA_OFFSET_CHK to value 0x00fd_0000"]
impl crate::Resettable for RegTxdmaOffsetChkSpec {
    const RESET_VALUE: u32 = 0x00fd_0000;
}
