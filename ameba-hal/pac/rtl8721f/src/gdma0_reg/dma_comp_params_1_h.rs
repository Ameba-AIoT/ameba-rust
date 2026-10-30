#[doc = "Register `DMA_COMP_PARAMS_1_H` reader"]
pub type R = crate::R<DmaCompParams1HSpec>;
#[doc = "Field `INTR_IO` reader - Interrupt output format.The value of this register is derived from the DMAC_INTR_IO parameter. 0x0 = ALL 0x1 = TYPE 0x2 = COMBINED 0x3 = reserved"]
pub type IntrIoR = crate::FieldReader;
#[doc = "Field `NUM_CHANNELS` reader - Number of channels. The value of this register is derived from the DMAC_NUM_CHASNNELS parameter. NUM_CHANNELS = DMAC_NUM_CHASNNELS -1 0x0 = 1 ... 0xf = 16"]
pub type NumChannelsR = crate::FieldReader;
#[doc = "Field `NUM_MASTER_INT` reader - Number of master interface. The value of this register is derived from the DMAC_NUM_MASTER_INT parameter. This value is fixed to 1 (only 2 AXI masters supported now.) 0x0 = Reserved 0x1 = 2 0x2 = Reserved 0x3 = Reserved"]
pub type NumMasterIntR = crate::FieldReader;
#[doc = "Field `S_DATA_WIDTH` reader - APB slave interface data width.The value of this register is derived from the DMAC_S_DATA_WIDTH parameter. The width is fixed to 32 bits for now. S_DATA_WIDTH = log2(DMAC_DATA_WIDTH/32) 0x0 = 32 bits 0x1 = Reserved 0x2 = Reserved 0x3 = Reserved"]
pub type SDataWidthR = crate::FieldReader;
#[doc = "Field `M1_DATA_WIDTH` reader - AXI Master1 data width. The value of this register is derived from the DMAC_M1_DATA_WIDTH parameter. Master1 data width should be equal to that of Master 2. M1_DATA_WIDTH = log2(DMAC_DATA_WIDTH/32) 0x0 = 32 bits 0x1 = 64 bits 0x2 = 128 bits 0x3 = Reserved"]
pub type M1DataWidthR = crate::FieldReader;
#[doc = "Field `M2_DATA_WIDTH` reader - AXI Master2 data width. The value of this register is derived from the DMAC_M2_DATA_WIDTH parameter. Master2 data width should be equal to that of Master 1. M2_DATA_WIDTH = log2(DMAC_DATA_WIDTH/32) 0x0 = 32 bits 0x1 = 64 bits 0x2 = 128 bits 0x3 = Reserved"]
pub type M2DataWidthR = crate::FieldReader;
#[doc = "Field `NUM_HS_INT` reader - Number of hardware handshake interfaces. The value of this register is derived from the DMAC_NUM_HS_INT parameter. NUM_HAS_INT = DMAC_NUM_HS_INT\\[4:0\\] 0x00 = 0 ... 0x10 = 16"]
pub type NumHsIntR = crate::FieldReader;
#[doc = "Field `ADD_ENCODED_PARAMS` reader - The value of this register is derived from the DMAC_ADD_ENCODED_PARAMS parameter. 0 = FALSE 1 = TRUE"]
pub type AddEncodedParamsR = crate::BitReader;
#[doc = "Field `STATIC_ENDIAN_SELECT` reader - The value of this register is derived from the DMAC_STATIC_ENDIAN_SELECT parameter. 0 = FALSE (dynamic endian fixed) 1 = TRUE (not supported)"]
pub type StaticEndianSelectR = crate::BitReader;
#[doc = "Field `HIGH_BIT_NUM_HS_INT1` reader - Number of hardware handshake interfaces. If it is greater than 31 and less than 64, this bit read value is 1."]
pub type HighBitNumHsInt1R = crate::BitReader;
#[doc = "Field `HIGH_BIT_NUM_HS_INT2` reader - Number of hardware handshake interfaces. If it is greater than 63 and less than 128, this bit read value is 1."]
pub type HighBitNumHsInt2R = crate::BitReader;
#[doc = "Field `HIGH_BIT_NUM_HS_INT3` reader - Number of hardware handshake interfaces. If it is 128, this bit read value is 1."]
pub type HighBitNumHsInt3R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Interrupt output format.The value of this register is derived from the DMAC_INTR_IO parameter. 0x0 = ALL 0x1 = TYPE 0x2 = COMBINED 0x3 = reserved"]
    #[inline(always)]
    pub fn intr_io(&self) -> IntrIoR {
        IntrIoR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 6:9 - Number of channels. The value of this register is derived from the DMAC_NUM_CHASNNELS parameter. NUM_CHANNELS = DMAC_NUM_CHASNNELS -1 0x0 = 1 ... 0xf = 16"]
    #[inline(always)]
    pub fn num_channels(&self) -> NumChannelsR {
        NumChannelsR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Number of master interface. The value of this register is derived from the DMAC_NUM_MASTER_INT parameter. This value is fixed to 1 (only 2 AXI masters supported now.) 0x0 = Reserved 0x1 = 2 0x2 = Reserved 0x3 = Reserved"]
    #[inline(always)]
    pub fn num_master_int(&self) -> NumMasterIntR {
        NumMasterIntR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - APB slave interface data width.The value of this register is derived from the DMAC_S_DATA_WIDTH parameter. The width is fixed to 32 bits for now. S_DATA_WIDTH = log2(DMAC_DATA_WIDTH/32) 0x0 = 32 bits 0x1 = Reserved 0x2 = Reserved 0x3 = Reserved"]
    #[inline(always)]
    pub fn s_data_width(&self) -> SDataWidthR {
        SDataWidthR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - AXI Master1 data width. The value of this register is derived from the DMAC_M1_DATA_WIDTH parameter. Master1 data width should be equal to that of Master 2. M1_DATA_WIDTH = log2(DMAC_DATA_WIDTH/32) 0x0 = 32 bits 0x1 = 64 bits 0x2 = 128 bits 0x3 = Reserved"]
    #[inline(always)]
    pub fn m1_data_width(&self) -> M1DataWidthR {
        M1DataWidthR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - AXI Master2 data width. The value of this register is derived from the DMAC_M2_DATA_WIDTH parameter. Master2 data width should be equal to that of Master 1. M2_DATA_WIDTH = log2(DMAC_DATA_WIDTH/32) 0x0 = 32 bits 0x1 = 64 bits 0x2 = 128 bits 0x3 = Reserved"]
    #[inline(always)]
    pub fn m2_data_width(&self) -> M2DataWidthR {
        M2DataWidthR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 22:26 - Number of hardware handshake interfaces. The value of this register is derived from the DMAC_NUM_HS_INT parameter. NUM_HAS_INT = DMAC_NUM_HS_INT\\[4:0\\] 0x00 = 0 ... 0x10 = 16"]
    #[inline(always)]
    pub fn num_hs_int(&self) -> NumHsIntR {
        NumHsIntR::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - The value of this register is derived from the DMAC_ADD_ENCODED_PARAMS parameter. 0 = FALSE 1 = TRUE"]
    #[inline(always)]
    pub fn add_encoded_params(&self) -> AddEncodedParamsR {
        AddEncodedParamsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The value of this register is derived from the DMAC_STATIC_ENDIAN_SELECT parameter. 0 = FALSE (dynamic endian fixed) 1 = TRUE (not supported)"]
    #[inline(always)]
    pub fn static_endian_select(&self) -> StaticEndianSelectR {
        StaticEndianSelectR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Number of hardware handshake interfaces. If it is greater than 31 and less than 64, this bit read value is 1."]
    #[inline(always)]
    pub fn high_bit_num_hs_int1(&self) -> HighBitNumHsInt1R {
        HighBitNumHsInt1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Number of hardware handshake interfaces. If it is greater than 63 and less than 128, this bit read value is 1."]
    #[inline(always)]
    pub fn high_bit_num_hs_int2(&self) -> HighBitNumHsInt2R {
        HighBitNumHsInt2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Number of hardware handshake interfaces. If it is 128, this bit read value is 1."]
    #[inline(always)]
    pub fn high_bit_num_hs_int3(&self) -> HighBitNumHsInt3R {
        HighBitNumHsInt3R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_1_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams1HSpec;
impl crate::RegisterSpec for DmaCompParams1HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_1_h::R`](R) reader structure"]
impl crate::Readable for DmaCompParams1HSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_1_H to value 0x0f80_05c0"]
impl crate::Resettable for DmaCompParams1HSpec {
    const RESET_VALUE: u32 = 0x0f80_05c0;
}
