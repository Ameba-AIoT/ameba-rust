#[doc = "Register `CTRLR2` reader"]
pub type R = crate::R<Ctrlr2Spec>;
#[doc = "Register `CTRLR2` writer"]
pub type W = crate::W<Ctrlr2Spec>;
#[doc = "Field `SO_DNUM` reader - Indicates SO input pin of SPI Flash is connected to spi_sout\\[0\\]or \\[1\\]. It refers to SPI Flash and also the system connection with SPI_Flash (see 6.1.3) If setting 0, it connects to spi_sout\\[0\\] (single-channel connection). Otherwise, it connects to spi_sout\\[1\\] (multi-channels connection). (Default is configured to 1to support multi-channel connection)"]
pub type SoDnumR = crate::BitReader;
#[doc = "Field `SO_DNUM` writer - Indicates SO input pin of SPI Flash is connected to spi_sout\\[0\\]or \\[1\\]. It refers to SPI Flash and also the system connection with SPI_Flash (see 6.1.3) If setting 0, it connects to spi_sout\\[0\\] (single-channel connection). Otherwise, it connects to spi_sout\\[1\\] (multi-channels connection). (Default is configured to 1to support multi-channel connection)"]
pub type SoDnumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPN_SET` reader - To implement write protect function. spi_wen_out and the bit of spi_sout which connects to WPN would be initial to 0. WPN_SET: If setting 1 is protected. Otherwise, is not protected.(Configuration Form is using \"WPN_SET_n\": If setting 1 is NOT protected. Otherwise, is protected.)"]
pub type WpnSetR = crate::BitReader;
#[doc = "Field `WPN_SET` writer - To implement write protect function. spi_wen_out and the bit of spi_sout which connects to WPN would be initial to 0. WPN_SET: If setting 1 is protected. Otherwise, is not protected.(Configuration Form is using \"WPN_SET_n\": If setting 1 is NOT protected. Otherwise, is protected.)"]
pub type WpnSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPN_DNUM` reader - Indicates the WPn input pin of SPI Flash is connected to spi_sout\\[2\\]or \\[3\\]. If setting 0, it connects to spi_sdata\\[2\\]. Otherwise, it connects to spi_sdata\\[3\\]."]
pub type WpnDnumR = crate::BitReader;
#[doc = "Field `WPN_DNUM` writer - Indicates the WPn input pin of SPI Flash is connected to spi_sout\\[2\\]or \\[3\\]. If setting 0, it connects to spi_sdata\\[2\\]. Otherwise, it connects to spi_sdata\\[3\\]."]
pub type WpnDnumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR_FIXED` reader - Indicate the AXI FIXED read mode on DR. (after 2021/01). It can only work on address that aligns to request size. Ex: (1) arsize=0, addr\\[1:0\\] can be 00, 01, 10, 11 (2) arsize=1, addr\\[1:0\\] can be 00, 10 (3) arsize=2, addr\\[1:0\\] can be 00. If setting 1, user mode read DR will be FIXED. Otherwise, user mode read DR will be INCR."]
pub type DrFixedR = crate::BitReader;
#[doc = "Field `DR_FIXED` writer - Indicate the AXI FIXED read mode on DR. (after 2021/01). It can only work on address that aligns to request size. Ex: (1) arsize=0, addr\\[1:0\\] can be 00, 01, 10, 11 (2) arsize=1, addr\\[1:0\\] can be 00, 10 (3) arsize=2, addr\\[1:0\\] can be 00. If setting 1, user mode read DR will be FIXED. Otherwise, user mode read DR will be INCR."]
pub type DrFixedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_ENTRY` reader - If using SPIC_HAS_DMA (NO_MERGE_FIFO) SPIC, this field indicates the valid entry of TX_FIFO. If using MERGE_FIFO SPIC, there is only one FIFO is in the SPIC design, and this field indicates the valid entry of the FIFO. It is an index of 2 and it should be equal or smaller than FIFO_ABW. The user can modify it to profile with a different entry. Ex: if FIFO entry is 32, FIFO_ENTYR <=5."]
pub type TxFifoEntryR = crate::FieldReader;
#[doc = "Field `TX_FIFO_ENTRY` writer - If using SPIC_HAS_DMA (NO_MERGE_FIFO) SPIC, this field indicates the valid entry of TX_FIFO. If using MERGE_FIFO SPIC, there is only one FIFO is in the SPIC design, and this field indicates the valid entry of the FIFO. It is an index of 2 and it should be equal or smaller than FIFO_ABW. The user can modify it to profile with a different entry. Ex: if FIFO entry is 32, FIFO_ENTYR <=5."]
pub type TxFifoEntryW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_FIFO_ENTRY` reader - If using SPIC_HAS_DMA (NO_MERGE_FIFO) SPIC, this field indicates the valid entry of RX FIFO. It is an index of 2 and it should be equal or smaller than 5. The user can modify it to profile with a different entry. Ex: if RX_FIFO entry is 32, RX_FIFO_ENTYR <=5."]
pub type RxFifoEntryR = crate::FieldReader;
#[doc = "Field `RX_FIFO_ENTRY` writer - If using SPIC_HAS_DMA (NO_MERGE_FIFO) SPIC, this field indicates the valid entry of RX FIFO. It is an index of 2 and it should be equal or smaller than 5. The user can modify it to profile with a different entry. Ex: if RX_FIFO entry is 32, RX_FIFO_ENTYR <=5."]
pub type RxFifoEntryW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DM_ACT` reader - DM (Data Mask) is active high or low, which is defined by specific PSRAM SPEC. If DM is active high (DM=1 means \"don't write\"), DM_ACT should be set to 1 and the corresponding byte of write data will not be written into the PSRAM. Available only if configuration of DM_EN is defined."]
pub type DmActR = crate::BitReader;
#[doc = "Field `DM_ACT` writer - DM (Data Mask) is active high or low, which is defined by specific PSRAM SPEC. If DM is active high (DM=1 means \"don't write\"), DM_ACT should be set to 1 and the corresponding byte of write data will not be written into the PSRAM. Available only if configuration of DM_EN is defined."]
pub type DmActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL_WR` reader - Set to enable full write in both user mode and auto mode. If FULL_WR=1, SPIC will ignore pushing DM_DR in user mode. If FULL_WR = 0, SPIC will use spi_dm to do partial write in auto mode. Available only if configuration of DM_EN is defined."]
pub type FullWrR = crate::BitReader;
#[doc = "Field `FULL_WR` writer - Set to enable full write in both user mode and auto mode. If FULL_WR=1, SPIC will ignore pushing DM_DR in user mode. If FULL_WR = 0, SPIC will use spi_dm to do partial write in auto mode. Available only if configuration of DM_EN is defined."]
pub type FullWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_CA_DIS` reader - Set to disable spi_dm_oe_n (let the device to drive DM) when SPIC pushing CMD and ADDR for both user mode and auto mode. Used to implement write variable latency in Winbond psram."]
pub type DmCaDisR = crate::BitReader;
#[doc = "Field `DM_CA_DIS` writer - Set to disable spi_dm_oe_n (let the device to drive DM) when SPIC pushing CMD and ADDR for both user mode and auto mode. Used to implement write variable latency in Winbond psram."]
pub type DmCaDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRAP_IMPR_DIS` reader - Set to disable wrap_align function (convert wrap to incremental through aligning start address to burst boundary) in auto mode. If DIS_WRAP_ALIGN = 1, SPIC will chop wrap transation into two incremental transactions."]
pub type WrapImprDisR = crate::BitReader;
#[doc = "Field `WRAP_IMPR_DIS` writer - Set to disable wrap_align function (convert wrap to incremental through aligning start address to burst boundary) in auto mode. If DIS_WRAP_ALIGN = 1, SPIC will chop wrap transation into two incremental transactions."]
pub type WrapImprDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_VALID_EN` reader - Set to enable variable read latency. SPIC will not sample read data until rx_data_valid = 1. If RD_VALID_EN = 0, SPIC will sample read data according to internal counter."]
pub type RdValidEnR = crate::BitReader;
#[doc = "Field `RD_VALID_EN` writer - Set to enable variable read latency. SPIC will not sample read data until rx_data_valid = 1. If RD_VALID_EN = 0, SPIC will sample read data according to internal counter."]
pub type RdValidEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_VL_EN` reader - Set to enable variable write latency. SPIC will set variable write latency according to RWDS. If WR_VL_EN = 0, SPIC will set fix write latency."]
pub type WrVlEnR = crate::BitReader;
#[doc = "Field `WR_VL_EN` writer - Set to enable variable write latency. SPIC will set variable write latency according to RWDS. If WR_VL_EN = 0, SPIC will set fix write latency."]
pub type WrVlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_WEIGHT` reader - Set to determine the read and write priority. RD_WEIGHT should >= 2 (if set to 0 or 1, RD_WEIGHT will use the previous value). Ex: if RD_WEIGHT = 5, SPIC will accept 1 write cmd after accepting 4 read cmd."]
pub type RdWeightR = crate::FieldReader;
#[doc = "Field `RD_WEIGHT` writer - Set to determine the read and write priority. RD_WEIGHT should >= 2 (if set to 0 or 1, RD_WEIGHT will use the previous value). Ex: if RD_WEIGHT = 5, SPIC will accept 1 write cmd after accepting 4 read cmd."]
pub type RdWeightW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Indicates SO input pin of SPI Flash is connected to spi_sout\\[0\\]or \\[1\\]. It refers to SPI Flash and also the system connection with SPI_Flash (see 6.1.3) If setting 0, it connects to spi_sout\\[0\\] (single-channel connection). Otherwise, it connects to spi_sout\\[1\\] (multi-channels connection). (Default is configured to 1to support multi-channel connection)"]
    #[inline(always)]
    pub fn so_dnum(&self) -> SoDnumR {
        SoDnumR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - To implement write protect function. spi_wen_out and the bit of spi_sout which connects to WPN would be initial to 0. WPN_SET: If setting 1 is protected. Otherwise, is not protected.(Configuration Form is using \"WPN_SET_n\": If setting 1 is NOT protected. Otherwise, is protected.)"]
    #[inline(always)]
    pub fn wpn_set(&self) -> WpnSetR {
        WpnSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the WPn input pin of SPI Flash is connected to spi_sout\\[2\\]or \\[3\\]. If setting 0, it connects to spi_sdata\\[2\\]. Otherwise, it connects to spi_sdata\\[3\\]."]
    #[inline(always)]
    pub fn wpn_dnum(&self) -> WpnDnumR {
        WpnDnumR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicate the AXI FIXED read mode on DR. (after 2021/01). It can only work on address that aligns to request size. Ex: (1) arsize=0, addr\\[1:0\\] can be 00, 01, 10, 11 (2) arsize=1, addr\\[1:0\\] can be 00, 10 (3) arsize=2, addr\\[1:0\\] can be 00. If setting 1, user mode read DR will be FIXED. Otherwise, user mode read DR will be INCR."]
    #[inline(always)]
    pub fn dr_fixed(&self) -> DrFixedR {
        DrFixedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - If using SPIC_HAS_DMA (NO_MERGE_FIFO) SPIC, this field indicates the valid entry of TX_FIFO. If using MERGE_FIFO SPIC, there is only one FIFO is in the SPIC design, and this field indicates the valid entry of the FIFO. It is an index of 2 and it should be equal or smaller than FIFO_ABW. The user can modify it to profile with a different entry. Ex: if FIFO entry is 32, FIFO_ENTYR <=5."]
    #[inline(always)]
    pub fn tx_fifo_entry(&self) -> TxFifoEntryR {
        TxFifoEntryR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If using SPIC_HAS_DMA (NO_MERGE_FIFO) SPIC, this field indicates the valid entry of RX FIFO. It is an index of 2 and it should be equal or smaller than 5. The user can modify it to profile with a different entry. Ex: if RX_FIFO entry is 32, RX_FIFO_ENTYR <=5."]
    #[inline(always)]
    pub fn rx_fifo_entry(&self) -> RxFifoEntryR {
        RxFifoEntryR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DM (Data Mask) is active high or low, which is defined by specific PSRAM SPEC. If DM is active high (DM=1 means \"don't write\"), DM_ACT should be set to 1 and the corresponding byte of write data will not be written into the PSRAM. Available only if configuration of DM_EN is defined."]
    #[inline(always)]
    pub fn dm_act(&self) -> DmActR {
        DmActR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set to enable full write in both user mode and auto mode. If FULL_WR=1, SPIC will ignore pushing DM_DR in user mode. If FULL_WR = 0, SPIC will use spi_dm to do partial write in auto mode. Available only if configuration of DM_EN is defined."]
    #[inline(always)]
    pub fn full_wr(&self) -> FullWrR {
        FullWrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set to disable spi_dm_oe_n (let the device to drive DM) when SPIC pushing CMD and ADDR for both user mode and auto mode. Used to implement write variable latency in Winbond psram."]
    #[inline(always)]
    pub fn dm_ca_dis(&self) -> DmCaDisR {
        DmCaDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set to disable wrap_align function (convert wrap to incremental through aligning start address to burst boundary) in auto mode. If DIS_WRAP_ALIGN = 1, SPIC will chop wrap transation into two incremental transactions."]
    #[inline(always)]
    pub fn wrap_impr_dis(&self) -> WrapImprDisR {
        WrapImprDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set to enable variable read latency. SPIC will not sample read data until rx_data_valid = 1. If RD_VALID_EN = 0, SPIC will sample read data according to internal counter."]
    #[inline(always)]
    pub fn rd_valid_en(&self) -> RdValidEnR {
        RdValidEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set to enable variable write latency. SPIC will set variable write latency according to RWDS. If WR_VL_EN = 0, SPIC will set fix write latency."]
    #[inline(always)]
    pub fn wr_vl_en(&self) -> WrVlEnR {
        WrVlEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Set to determine the read and write priority. RD_WEIGHT should >= 2 (if set to 0 or 1, RD_WEIGHT will use the previous value). Ex: if RD_WEIGHT = 5, SPIC will accept 1 write cmd after accepting 4 read cmd."]
    #[inline(always)]
    pub fn rd_weight(&self) -> RdWeightR {
        RdWeightR::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates SO input pin of SPI Flash is connected to spi_sout\\[0\\]or \\[1\\]. It refers to SPI Flash and also the system connection with SPI_Flash (see 6.1.3) If setting 0, it connects to spi_sout\\[0\\] (single-channel connection). Otherwise, it connects to spi_sout\\[1\\] (multi-channels connection). (Default is configured to 1to support multi-channel connection)"]
    #[inline(always)]
    pub fn so_dnum(&mut self) -> SoDnumW<'_, Ctrlr2Spec> {
        SoDnumW::new(self, 0)
    }
    #[doc = "Bit 1 - To implement write protect function. spi_wen_out and the bit of spi_sout which connects to WPN would be initial to 0. WPN_SET: If setting 1 is protected. Otherwise, is not protected.(Configuration Form is using \"WPN_SET_n\": If setting 1 is NOT protected. Otherwise, is protected.)"]
    #[inline(always)]
    pub fn wpn_set(&mut self) -> WpnSetW<'_, Ctrlr2Spec> {
        WpnSetW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates the WPn input pin of SPI Flash is connected to spi_sout\\[2\\]or \\[3\\]. If setting 0, it connects to spi_sdata\\[2\\]. Otherwise, it connects to spi_sdata\\[3\\]."]
    #[inline(always)]
    pub fn wpn_dnum(&mut self) -> WpnDnumW<'_, Ctrlr2Spec> {
        WpnDnumW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicate the AXI FIXED read mode on DR. (after 2021/01). It can only work on address that aligns to request size. Ex: (1) arsize=0, addr\\[1:0\\] can be 00, 01, 10, 11 (2) arsize=1, addr\\[1:0\\] can be 00, 10 (3) arsize=2, addr\\[1:0\\] can be 00. If setting 1, user mode read DR will be FIXED. Otherwise, user mode read DR will be INCR."]
    #[inline(always)]
    pub fn dr_fixed(&mut self) -> DrFixedW<'_, Ctrlr2Spec> {
        DrFixedW::new(self, 3)
    }
    #[doc = "Bits 4:7 - If using SPIC_HAS_DMA (NO_MERGE_FIFO) SPIC, this field indicates the valid entry of TX_FIFO. If using MERGE_FIFO SPIC, there is only one FIFO is in the SPIC design, and this field indicates the valid entry of the FIFO. It is an index of 2 and it should be equal or smaller than FIFO_ABW. The user can modify it to profile with a different entry. Ex: if FIFO entry is 32, FIFO_ENTYR <=5."]
    #[inline(always)]
    pub fn tx_fifo_entry(&mut self) -> TxFifoEntryW<'_, Ctrlr2Spec> {
        TxFifoEntryW::new(self, 4)
    }
    #[doc = "Bits 8:11 - If using SPIC_HAS_DMA (NO_MERGE_FIFO) SPIC, this field indicates the valid entry of RX FIFO. It is an index of 2 and it should be equal or smaller than 5. The user can modify it to profile with a different entry. Ex: if RX_FIFO entry is 32, RX_FIFO_ENTYR <=5."]
    #[inline(always)]
    pub fn rx_fifo_entry(&mut self) -> RxFifoEntryW<'_, Ctrlr2Spec> {
        RxFifoEntryW::new(self, 8)
    }
    #[doc = "Bit 12 - DM (Data Mask) is active high or low, which is defined by specific PSRAM SPEC. If DM is active high (DM=1 means \"don't write\"), DM_ACT should be set to 1 and the corresponding byte of write data will not be written into the PSRAM. Available only if configuration of DM_EN is defined."]
    #[inline(always)]
    pub fn dm_act(&mut self) -> DmActW<'_, Ctrlr2Spec> {
        DmActW::new(self, 12)
    }
    #[doc = "Bit 13 - Set to enable full write in both user mode and auto mode. If FULL_WR=1, SPIC will ignore pushing DM_DR in user mode. If FULL_WR = 0, SPIC will use spi_dm to do partial write in auto mode. Available only if configuration of DM_EN is defined."]
    #[inline(always)]
    pub fn full_wr(&mut self) -> FullWrW<'_, Ctrlr2Spec> {
        FullWrW::new(self, 13)
    }
    #[doc = "Bit 14 - Set to disable spi_dm_oe_n (let the device to drive DM) when SPIC pushing CMD and ADDR for both user mode and auto mode. Used to implement write variable latency in Winbond psram."]
    #[inline(always)]
    pub fn dm_ca_dis(&mut self) -> DmCaDisW<'_, Ctrlr2Spec> {
        DmCaDisW::new(self, 14)
    }
    #[doc = "Bit 15 - Set to disable wrap_align function (convert wrap to incremental through aligning start address to burst boundary) in auto mode. If DIS_WRAP_ALIGN = 1, SPIC will chop wrap transation into two incremental transactions."]
    #[inline(always)]
    pub fn wrap_impr_dis(&mut self) -> WrapImprDisW<'_, Ctrlr2Spec> {
        WrapImprDisW::new(self, 15)
    }
    #[doc = "Bit 16 - Set to enable variable read latency. SPIC will not sample read data until rx_data_valid = 1. If RD_VALID_EN = 0, SPIC will sample read data according to internal counter."]
    #[inline(always)]
    pub fn rd_valid_en(&mut self) -> RdValidEnW<'_, Ctrlr2Spec> {
        RdValidEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Set to enable variable write latency. SPIC will set variable write latency according to RWDS. If WR_VL_EN = 0, SPIC will set fix write latency."]
    #[inline(always)]
    pub fn wr_vl_en(&mut self) -> WrVlEnW<'_, Ctrlr2Spec> {
        WrVlEnW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Set to determine the read and write priority. RD_WEIGHT should >= 2 (if set to 0 or 1, RD_WEIGHT will use the previous value). Ex: if RD_WEIGHT = 5, SPIC will accept 1 write cmd after accepting 4 read cmd."]
    #[inline(always)]
    pub fn rd_weight(&mut self) -> RdWeightW<'_, Ctrlr2Spec> {
        RdWeightW::new(self, 19)
    }
}
#[doc = "Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlr2Spec;
impl crate::RegisterSpec for Ctrlr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlr2::R`](R) reader structure"]
impl crate::Readable for Ctrlr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlr2::W`](W) writer structure"]
impl crate::Writable for Ctrlr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLR2 to value 0x0010_3551"]
impl crate::Resettable for Ctrlr2Spec {
    const RESET_VALUE: u32 = 0x0010_3551;
}
