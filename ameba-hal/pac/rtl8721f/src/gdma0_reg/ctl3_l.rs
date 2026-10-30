#[doc = "Register `CTL3_L` reader"]
pub type R = crate::R<Ctl3LSpec>;
#[doc = "Register `CTL3_L` writer"]
pub type W = crate::W<Ctl3LSpec>;
#[doc = "Field `CTL3_L_INT_EN` reader - Interrupt Enable Bit. If set, then all interrupt-generating sources are enabled. Functions as a global mask bit for all interrupts for the channel; raw* interrupt registers still assert if CTLx_L.INT_EN = 0."]
pub type Ctl3LIntEnR = crate::BitReader;
#[doc = "Field `CTL3_L_INT_EN` writer - Interrupt Enable Bit. If set, then all interrupt-generating sources are enabled. Functions as a global mask bit for all interrupts for the channel; raw* interrupt registers still assert if CTLx_L.INT_EN = 0."]
pub type Ctl3LIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTL3_L_DST_TR_WIDTH` reader - Destination Transfer Width.Table 6-3 lists the decoding for this field. It can't be mapped to AXI bus data width directly. For a non-memory peripheral, typically the peripheral (destination) FIFO width.Note: DST_TR_WIDTH must not be bigger than BUS width"]
pub type Ctl3LDstTrWidthR = crate::FieldReader;
#[doc = "Field `CTL3_L_DST_TR_WIDTH` writer - Destination Transfer Width.Table 6-3 lists the decoding for this field. It can't be mapped to AXI bus data width directly. For a non-memory peripheral, typically the peripheral (destination) FIFO width.Note: DST_TR_WIDTH must not be bigger than BUS width"]
pub type Ctl3LDstTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTL3_L_SRC_TR_WIDTH` reader - Source Transfer Width. Table 6-3 lists the decoding for this field.It can't be mapped to AXI bus data width directly. Since AXI bus data width is fixed (such as 32bit/64bit), AXI use byte enable signal for byte/half word transfer (still burst). So, the SRC_TR_WIDTH is only used for total block data size calculation and streaming mode byte/half word transfer (for FIFO transfer). Note: SRC_TR_WIDTH must not be bigger than BUS width."]
pub type Ctl3LSrcTrWidthR = crate::FieldReader;
#[doc = "Field `CTL3_L_SRC_TR_WIDTH` writer - Source Transfer Width. Table 6-3 lists the decoding for this field.It can't be mapped to AXI bus data width directly. Since AXI bus data width is fixed (such as 32bit/64bit), AXI use byte enable signal for byte/half word transfer (still burst). So, the SRC_TR_WIDTH is only used for total block data size calculation and streaming mode byte/half word transfer (for FIFO transfer). Note: SRC_TR_WIDTH must not be bigger than BUS width."]
pub type Ctl3LSrcTrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTL3_L_DINC` reader - Destination Address Increment. Indicates whether to increment destination address on every destination transfer. If your device is writing data to a destination peripheral FIFO with a fixed address, then set this field to \"No change.\"If destination is memory appointed by CTLx_L.TT_FC, and set DINC=No change, then DST_TR_WIDTH must be set to BUS width. And DMAC will write data to the fixed memory address until block transfer finish, but It is not recommended to write memory like this, because some memory controller not support fixed burst transaction and may transform the fixed burst transaction to incremental burst transaction. 00 = Increment 01=Not Support 1x = No change NOTE: Incrementing is done for alignment to the next CTLx_L.DST_TR_WIDTH boundary."]
pub type Ctl3LDincR = crate::FieldReader;
#[doc = "Field `CTL3_L_DINC` writer - Destination Address Increment. Indicates whether to increment destination address on every destination transfer. If your device is writing data to a destination peripheral FIFO with a fixed address, then set this field to \"No change.\"If destination is memory appointed by CTLx_L.TT_FC, and set DINC=No change, then DST_TR_WIDTH must be set to BUS width. And DMAC will write data to the fixed memory address until block transfer finish, but It is not recommended to write memory like this, because some memory controller not support fixed burst transaction and may transform the fixed burst transaction to incremental burst transaction. 00 = Increment 01=Not Support 1x = No change NOTE: Incrementing is done for alignment to the next CTLx_L.DST_TR_WIDTH boundary."]
pub type Ctl3LDincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL3_L_SINC` reader - Source Address Increment. Indicates whether to increment the source address on every source transfer. If the device is fetching data from a source peripheral FIFO with a fixed address, then set this field to \"No change.\"If source is memory appointed by CTLx_L.TT_FC, and set SINC=No change, then SRC_TR_WIDTH must be set to BUS width. And DMAC will read data from the fixed memory address until block transfer finish, but It is not recommended to read memory like this, because some memory controller not support fixed burst transaction and may transform the fixed burst transaction to incremental burst transaction. 00 = Increment 01 =Not support 1x = No change NOTE: Incrementing is done for alignment to the next CTLx_L.SRC_TR_WIDTH boundary."]
pub type Ctl3LSincR = crate::FieldReader;
#[doc = "Field `CTL3_L_SINC` writer - Source Address Increment. Indicates whether to increment the source address on every source transfer. If the device is fetching data from a source peripheral FIFO with a fixed address, then set this field to \"No change.\"If source is memory appointed by CTLx_L.TT_FC, and set SINC=No change, then SRC_TR_WIDTH must be set to BUS width. And DMAC will read data from the fixed memory address until block transfer finish, but It is not recommended to read memory like this, because some memory controller not support fixed burst transaction and may transform the fixed burst transaction to incremental burst transaction. 00 = Increment 01 =Not support 1x = No change NOTE: Incrementing is done for alignment to the next CTLx_L.SRC_TR_WIDTH boundary."]
pub type Ctl3LSincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL3_L_DEST_MSIZE` reader - Destination Burst Transaction Length. Number of data items, each of width DATA_WIDTH, to be written to the destination every time a destination burst transaction request is made from either the corresponding hardware or software handshaking interface. Write burst transaction length on bus (AWLen) will not exceed this value, but may be less than this value. If DINC is setted to \"no change\", this value should not be greater than 16 to conform to the AXI protocol. If enabled 'DMAC_CHx_MSIZE_BIGGER_THAN_FIFO_DEPTH' feature, to improve performance, can set CTLx_L.SRC_MSIZE to a high value to make DMAC send longer burst length command. But to avoid deadlock, if Source slave and Destination slave is the same slave, should not set CTLx_L.SRC_MSIZE over than FIFO_depth; Table 6-2-1 lists the decoding for this field."]
pub type Ctl3LDestMsizeR = crate::FieldReader;
#[doc = "Field `CTL3_L_DEST_MSIZE` writer - Destination Burst Transaction Length. Number of data items, each of width DATA_WIDTH, to be written to the destination every time a destination burst transaction request is made from either the corresponding hardware or software handshaking interface. Write burst transaction length on bus (AWLen) will not exceed this value, but may be less than this value. If DINC is setted to \"no change\", this value should not be greater than 16 to conform to the AXI protocol. If enabled 'DMAC_CHx_MSIZE_BIGGER_THAN_FIFO_DEPTH' feature, to improve performance, can set CTLx_L.SRC_MSIZE to a high value to make DMAC send longer burst length command. But to avoid deadlock, if Source slave and Destination slave is the same slave, should not set CTLx_L.SRC_MSIZE over than FIFO_depth; Table 6-2-1 lists the decoding for this field."]
pub type Ctl3LDestMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTL3_L_SRC_MSIZE` reader - Source Burst Transaction Length. Number of data items, each of width CTLx_L.SRC_TR_WIDTH if source is not memory, otherwise each of width DMAC_TR_WIDTH_DEFAULT, to be read from the source every time a source burst transaction request is made from either the corresponding hardware or software handshaking interface. Read burst transaction length on bus (ARLen) will not exceed this value, but may be less than this value. If SINC is setted to \"no change\", this value should not be greater than 16 to conform to the AXI protocol. If enabled 'DMAC_CHx_MSIZE_BIGGER_THAN_FIFO_DEPTH' feature, to improve performance, can set CTLx_L.SRC_MSIZE to a high value to make DMAC send longer burst length command. But to avoid deadlock, if Source slave and Destination slave is the same slave, should not set CTLx_L.SRC_MSIZE over than FIFO_depth; Table 6-2-0 lists the decoding for this field."]
pub type Ctl3LSrcMsizeR = crate::FieldReader;
#[doc = "Field `CTL3_L_SRC_MSIZE` writer - Source Burst Transaction Length. Number of data items, each of width CTLx_L.SRC_TR_WIDTH if source is not memory, otherwise each of width DMAC_TR_WIDTH_DEFAULT, to be read from the source every time a source burst transaction request is made from either the corresponding hardware or software handshaking interface. Read burst transaction length on bus (ARLen) will not exceed this value, but may be less than this value. If SINC is setted to \"no change\", this value should not be greater than 16 to conform to the AXI protocol. If enabled 'DMAC_CHx_MSIZE_BIGGER_THAN_FIFO_DEPTH' feature, to improve performance, can set CTLx_L.SRC_MSIZE to a high value to make DMAC send longer burst length command. But to avoid deadlock, if Source slave and Destination slave is the same slave, should not set CTLx_L.SRC_MSIZE over than FIFO_depth; Table 6-2-0 lists the decoding for this field."]
pub type Ctl3LSrcMsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTL3_L_SRC_GATHER_EN` reader - Source gather enable bit: 0 = Gather disabled 1 = Gather enabled Gather on the source side is applicable only when the CTLx_L.SINC bit indicates an incrementing address control and SARx should be aligned with SRC_TR_WIDTH. Dependencies: This field does not exist if DMAC_CHx_SRC_GAT_EN is not selected."]
pub type Ctl3LSrcGatherEnR = crate::BitReader;
#[doc = "Field `CTL3_L_SRC_GATHER_EN` writer - Source gather enable bit: 0 = Gather disabled 1 = Gather enabled Gather on the source side is applicable only when the CTLx_L.SINC bit indicates an incrementing address control and SARx should be aligned with SRC_TR_WIDTH. Dependencies: This field does not exist if DMAC_CHx_SRC_GAT_EN is not selected."]
pub type Ctl3LSrcGatherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTL3_L_DST_SCATTER_EN` reader - Destination scatter enable bit:0 = Scatter disabled1 = Scatter enabledScatter on the destination side is applicable only when the CTLx_L.DINC bit indicates an incrementing address control and DARx should be aligned with DST_TR_WIDTH. Dependencies: This field does not exist if DMAC_CHx_DST_SCA_EN is not selected."]
pub type Ctl3LDstScatterEnR = crate::BitReader;
#[doc = "Field `CTL3_L_DST_SCATTER_EN` writer - Destination scatter enable bit:0 = Scatter disabled1 = Scatter enabledScatter on the destination side is applicable only when the CTLx_L.DINC bit indicates an incrementing address control and DARx should be aligned with DST_TR_WIDTH. Dependencies: This field does not exist if DMAC_CHx_DST_SCA_EN is not selected."]
pub type Ctl3LDstScatterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTL3_L_TT_FC` reader - Transfer Type and Flow Control.The following transfer types are supported. • Memory to Memory • Memory to Peripheral • Peripheral to Memory • Peripheral to Peripheral Flow Control can be assigned to the RTK_DMAC, the source peripheral, or the destination peripheral. Table 6-4 lists the decoding for this field. For more information on transfer types and flow control, refer to \"Setup/Operation of RTK_DMAC Transfers\". Reset Value: 3'h0 Dependencies: If hardware handshake isn't support, then TT_FC can only be set to 3'b0. TT_FC\\[2\\] can only be read."]
pub type Ctl3LTtFcR = crate::FieldReader;
#[doc = "Field `CTL3_L_TT_FC` writer - Transfer Type and Flow Control.The following transfer types are supported. • Memory to Memory • Memory to Peripheral • Peripheral to Memory • Peripheral to Peripheral Flow Control can be assigned to the RTK_DMAC, the source peripheral, or the destination peripheral. Table 6-4 lists the decoding for this field. For more information on transfer types and flow control, refer to \"Setup/Operation of RTK_DMAC Transfers\". Reset Value: 3'h0 Dependencies: If hardware handshake isn't support, then TT_FC can only be set to 3'b0. TT_FC\\[2\\] can only be read."]
pub type Ctl3LTtFcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTL3_L_LLP_DST_EN` reader - Block chanining is enabled on the destination side only if the LLP_DST_EN field id high and LLPx_L.LOC is non-zero; for more information,see 7.4.1.1 \"Block Chaining Using Linked Lists\" Dependencies:This field does not exist if the configuration parameterDMAC_CHx_MULTI_BLK_EN is not selected or if DMAC_CHx_HC_LLP is selected; in this case, the read-back value is always 0."]
pub type Ctl3LLlpDstEnR = crate::BitReader;
#[doc = "Field `CTL3_L_LLP_DST_EN` writer - Block chanining is enabled on the destination side only if the LLP_DST_EN field id high and LLPx_L.LOC is non-zero; for more information,see 7.4.1.1 \"Block Chaining Using Linked Lists\" Dependencies:This field does not exist if the configuration parameterDMAC_CHx_MULTI_BLK_EN is not selected or if DMAC_CHx_HC_LLP is selected; in this case, the read-back value is always 0."]
pub type Ctl3LLlpDstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTL3_L_LLP_SRC_EN` reader - Block chanining is enabled on the source side only if the LLP_SRC_EN field id high and LLPx_L.LOC is non-zero; for more information,see 7.4.1.1 \"Block Chaining Using Linked Lists\" Dependencies:This field does not exist if the configuration parameterDMAC_CHx_MULTI_BLK_EN is not selected or if DMAC_CHx_HC_LLP is selected; in this case, the read-back value is always 0."]
pub type Ctl3LLlpSrcEnR = crate::BitReader;
#[doc = "Field `CTL3_L_LLP_SRC_EN` writer - Block chanining is enabled on the source side only if the LLP_SRC_EN field id high and LLPx_L.LOC is non-zero; for more information,see 7.4.1.1 \"Block Chaining Using Linked Lists\" Dependencies:This field does not exist if the configuration parameterDMAC_CHx_MULTI_BLK_EN is not selected or if DMAC_CHx_HC_LLP is selected; in this case, the read-back value is always 0."]
pub type Ctl3LLlpSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable Bit. If set, then all interrupt-generating sources are enabled. Functions as a global mask bit for all interrupts for the channel; raw* interrupt registers still assert if CTLx_L.INT_EN = 0."]
    #[inline(always)]
    pub fn ctl3_l_int_en(&self) -> Ctl3LIntEnR {
        Ctl3LIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width.Table 6-3 lists the decoding for this field. It can't be mapped to AXI bus data width directly. For a non-memory peripheral, typically the peripheral (destination) FIFO width.Note: DST_TR_WIDTH must not be bigger than BUS width"]
    #[inline(always)]
    pub fn ctl3_l_dst_tr_width(&self) -> Ctl3LDstTrWidthR {
        Ctl3LDstTrWidthR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Source Transfer Width. Table 6-3 lists the decoding for this field.It can't be mapped to AXI bus data width directly. Since AXI bus data width is fixed (such as 32bit/64bit), AXI use byte enable signal for byte/half word transfer (still burst). So, the SRC_TR_WIDTH is only used for total block data size calculation and streaming mode byte/half word transfer (for FIFO transfer). Note: SRC_TR_WIDTH must not be bigger than BUS width."]
    #[inline(always)]
    pub fn ctl3_l_src_tr_width(&self) -> Ctl3LSrcTrWidthR {
        Ctl3LSrcTrWidthR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Destination Address Increment. Indicates whether to increment destination address on every destination transfer. If your device is writing data to a destination peripheral FIFO with a fixed address, then set this field to \"No change.\"If destination is memory appointed by CTLx_L.TT_FC, and set DINC=No change, then DST_TR_WIDTH must be set to BUS width. And DMAC will write data to the fixed memory address until block transfer finish, but It is not recommended to write memory like this, because some memory controller not support fixed burst transaction and may transform the fixed burst transaction to incremental burst transaction. 00 = Increment 01=Not Support 1x = No change NOTE: Incrementing is done for alignment to the next CTLx_L.DST_TR_WIDTH boundary."]
    #[inline(always)]
    pub fn ctl3_l_dinc(&self) -> Ctl3LDincR {
        Ctl3LDincR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Source Address Increment. Indicates whether to increment the source address on every source transfer. If the device is fetching data from a source peripheral FIFO with a fixed address, then set this field to \"No change.\"If source is memory appointed by CTLx_L.TT_FC, and set SINC=No change, then SRC_TR_WIDTH must be set to BUS width. And DMAC will read data from the fixed memory address until block transfer finish, but It is not recommended to read memory like this, because some memory controller not support fixed burst transaction and may transform the fixed burst transaction to incremental burst transaction. 00 = Increment 01 =Not support 1x = No change NOTE: Incrementing is done for alignment to the next CTLx_L.SRC_TR_WIDTH boundary."]
    #[inline(always)]
    pub fn ctl3_l_sinc(&self) -> Ctl3LSincR {
        Ctl3LSincR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length. Number of data items, each of width DATA_WIDTH, to be written to the destination every time a destination burst transaction request is made from either the corresponding hardware or software handshaking interface. Write burst transaction length on bus (AWLen) will not exceed this value, but may be less than this value. If DINC is setted to \"no change\", this value should not be greater than 16 to conform to the AXI protocol. If enabled 'DMAC_CHx_MSIZE_BIGGER_THAN_FIFO_DEPTH' feature, to improve performance, can set CTLx_L.SRC_MSIZE to a high value to make DMAC send longer burst length command. But to avoid deadlock, if Source slave and Destination slave is the same slave, should not set CTLx_L.SRC_MSIZE over than FIFO_depth; Table 6-2-1 lists the decoding for this field."]
    #[inline(always)]
    pub fn ctl3_l_dest_msize(&self) -> Ctl3LDestMsizeR {
        Ctl3LDestMsizeR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length. Number of data items, each of width CTLx_L.SRC_TR_WIDTH if source is not memory, otherwise each of width DMAC_TR_WIDTH_DEFAULT, to be read from the source every time a source burst transaction request is made from either the corresponding hardware or software handshaking interface. Read burst transaction length on bus (ARLen) will not exceed this value, but may be less than this value. If SINC is setted to \"no change\", this value should not be greater than 16 to conform to the AXI protocol. If enabled 'DMAC_CHx_MSIZE_BIGGER_THAN_FIFO_DEPTH' feature, to improve performance, can set CTLx_L.SRC_MSIZE to a high value to make DMAC send longer burst length command. But to avoid deadlock, if Source slave and Destination slave is the same slave, should not set CTLx_L.SRC_MSIZE over than FIFO_depth; Table 6-2-0 lists the decoding for this field."]
    #[inline(always)]
    pub fn ctl3_l_src_msize(&self) -> Ctl3LSrcMsizeR {
        Ctl3LSrcMsizeR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - Source gather enable bit: 0 = Gather disabled 1 = Gather enabled Gather on the source side is applicable only when the CTLx_L.SINC bit indicates an incrementing address control and SARx should be aligned with SRC_TR_WIDTH. Dependencies: This field does not exist if DMAC_CHx_SRC_GAT_EN is not selected."]
    #[inline(always)]
    pub fn ctl3_l_src_gather_en(&self) -> Ctl3LSrcGatherEnR {
        Ctl3LSrcGatherEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination scatter enable bit:0 = Scatter disabled1 = Scatter enabledScatter on the destination side is applicable only when the CTLx_L.DINC bit indicates an incrementing address control and DARx should be aligned with DST_TR_WIDTH. Dependencies: This field does not exist if DMAC_CHx_DST_SCA_EN is not selected."]
    #[inline(always)]
    pub fn ctl3_l_dst_scatter_en(&self) -> Ctl3LDstScatterEnR {
        Ctl3LDstScatterEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control.The following transfer types are supported. • Memory to Memory • Memory to Peripheral • Peripheral to Memory • Peripheral to Peripheral Flow Control can be assigned to the RTK_DMAC, the source peripheral, or the destination peripheral. Table 6-4 lists the decoding for this field. For more information on transfer types and flow control, refer to \"Setup/Operation of RTK_DMAC Transfers\". Reset Value: 3'h0 Dependencies: If hardware handshake isn't support, then TT_FC can only be set to 3'b0. TT_FC\\[2\\] can only be read."]
    #[inline(always)]
    pub fn ctl3_l_tt_fc(&self) -> Ctl3LTtFcR {
        Ctl3LTtFcR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 27 - Block chanining is enabled on the destination side only if the LLP_DST_EN field id high and LLPx_L.LOC is non-zero; for more information,see 7.4.1.1 \"Block Chaining Using Linked Lists\" Dependencies:This field does not exist if the configuration parameterDMAC_CHx_MULTI_BLK_EN is not selected or if DMAC_CHx_HC_LLP is selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn ctl3_l_llp_dst_en(&self) -> Ctl3LLlpDstEnR {
        Ctl3LLlpDstEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Block chanining is enabled on the source side only if the LLP_SRC_EN field id high and LLPx_L.LOC is non-zero; for more information,see 7.4.1.1 \"Block Chaining Using Linked Lists\" Dependencies:This field does not exist if the configuration parameterDMAC_CHx_MULTI_BLK_EN is not selected or if DMAC_CHx_HC_LLP is selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn ctl3_l_llp_src_en(&self) -> Ctl3LLlpSrcEnR {
        Ctl3LLlpSrcEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable Bit. If set, then all interrupt-generating sources are enabled. Functions as a global mask bit for all interrupts for the channel; raw* interrupt registers still assert if CTLx_L.INT_EN = 0."]
    #[inline(always)]
    pub fn ctl3_l_int_en(&mut self) -> Ctl3LIntEnW<'_, Ctl3LSpec> {
        Ctl3LIntEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width.Table 6-3 lists the decoding for this field. It can't be mapped to AXI bus data width directly. For a non-memory peripheral, typically the peripheral (destination) FIFO width.Note: DST_TR_WIDTH must not be bigger than BUS width"]
    #[inline(always)]
    pub fn ctl3_l_dst_tr_width(&mut self) -> Ctl3LDstTrWidthW<'_, Ctl3LSpec> {
        Ctl3LDstTrWidthW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Source Transfer Width. Table 6-3 lists the decoding for this field.It can't be mapped to AXI bus data width directly. Since AXI bus data width is fixed (such as 32bit/64bit), AXI use byte enable signal for byte/half word transfer (still burst). So, the SRC_TR_WIDTH is only used for total block data size calculation and streaming mode byte/half word transfer (for FIFO transfer). Note: SRC_TR_WIDTH must not be bigger than BUS width."]
    #[inline(always)]
    pub fn ctl3_l_src_tr_width(&mut self) -> Ctl3LSrcTrWidthW<'_, Ctl3LSpec> {
        Ctl3LSrcTrWidthW::new(self, 4)
    }
    #[doc = "Bits 7:8 - Destination Address Increment. Indicates whether to increment destination address on every destination transfer. If your device is writing data to a destination peripheral FIFO with a fixed address, then set this field to \"No change.\"If destination is memory appointed by CTLx_L.TT_FC, and set DINC=No change, then DST_TR_WIDTH must be set to BUS width. And DMAC will write data to the fixed memory address until block transfer finish, but It is not recommended to write memory like this, because some memory controller not support fixed burst transaction and may transform the fixed burst transaction to incremental burst transaction. 00 = Increment 01=Not Support 1x = No change NOTE: Incrementing is done for alignment to the next CTLx_L.DST_TR_WIDTH boundary."]
    #[inline(always)]
    pub fn ctl3_l_dinc(&mut self) -> Ctl3LDincW<'_, Ctl3LSpec> {
        Ctl3LDincW::new(self, 7)
    }
    #[doc = "Bits 9:10 - Source Address Increment. Indicates whether to increment the source address on every source transfer. If the device is fetching data from a source peripheral FIFO with a fixed address, then set this field to \"No change.\"If source is memory appointed by CTLx_L.TT_FC, and set SINC=No change, then SRC_TR_WIDTH must be set to BUS width. And DMAC will read data from the fixed memory address until block transfer finish, but It is not recommended to read memory like this, because some memory controller not support fixed burst transaction and may transform the fixed burst transaction to incremental burst transaction. 00 = Increment 01 =Not support 1x = No change NOTE: Incrementing is done for alignment to the next CTLx_L.SRC_TR_WIDTH boundary."]
    #[inline(always)]
    pub fn ctl3_l_sinc(&mut self) -> Ctl3LSincW<'_, Ctl3LSpec> {
        Ctl3LSincW::new(self, 9)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length. Number of data items, each of width DATA_WIDTH, to be written to the destination every time a destination burst transaction request is made from either the corresponding hardware or software handshaking interface. Write burst transaction length on bus (AWLen) will not exceed this value, but may be less than this value. If DINC is setted to \"no change\", this value should not be greater than 16 to conform to the AXI protocol. If enabled 'DMAC_CHx_MSIZE_BIGGER_THAN_FIFO_DEPTH' feature, to improve performance, can set CTLx_L.SRC_MSIZE to a high value to make DMAC send longer burst length command. But to avoid deadlock, if Source slave and Destination slave is the same slave, should not set CTLx_L.SRC_MSIZE over than FIFO_depth; Table 6-2-1 lists the decoding for this field."]
    #[inline(always)]
    pub fn ctl3_l_dest_msize(&mut self) -> Ctl3LDestMsizeW<'_, Ctl3LSpec> {
        Ctl3LDestMsizeW::new(self, 11)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length. Number of data items, each of width CTLx_L.SRC_TR_WIDTH if source is not memory, otherwise each of width DMAC_TR_WIDTH_DEFAULT, to be read from the source every time a source burst transaction request is made from either the corresponding hardware or software handshaking interface. Read burst transaction length on bus (ARLen) will not exceed this value, but may be less than this value. If SINC is setted to \"no change\", this value should not be greater than 16 to conform to the AXI protocol. If enabled 'DMAC_CHx_MSIZE_BIGGER_THAN_FIFO_DEPTH' feature, to improve performance, can set CTLx_L.SRC_MSIZE to a high value to make DMAC send longer burst length command. But to avoid deadlock, if Source slave and Destination slave is the same slave, should not set CTLx_L.SRC_MSIZE over than FIFO_depth; Table 6-2-0 lists the decoding for this field."]
    #[inline(always)]
    pub fn ctl3_l_src_msize(&mut self) -> Ctl3LSrcMsizeW<'_, Ctl3LSpec> {
        Ctl3LSrcMsizeW::new(self, 14)
    }
    #[doc = "Bit 17 - Source gather enable bit: 0 = Gather disabled 1 = Gather enabled Gather on the source side is applicable only when the CTLx_L.SINC bit indicates an incrementing address control and SARx should be aligned with SRC_TR_WIDTH. Dependencies: This field does not exist if DMAC_CHx_SRC_GAT_EN is not selected."]
    #[inline(always)]
    pub fn ctl3_l_src_gather_en(&mut self) -> Ctl3LSrcGatherEnW<'_, Ctl3LSpec> {
        Ctl3LSrcGatherEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Destination scatter enable bit:0 = Scatter disabled1 = Scatter enabledScatter on the destination side is applicable only when the CTLx_L.DINC bit indicates an incrementing address control and DARx should be aligned with DST_TR_WIDTH. Dependencies: This field does not exist if DMAC_CHx_DST_SCA_EN is not selected."]
    #[inline(always)]
    pub fn ctl3_l_dst_scatter_en(&mut self) -> Ctl3LDstScatterEnW<'_, Ctl3LSpec> {
        Ctl3LDstScatterEnW::new(self, 18)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control.The following transfer types are supported. • Memory to Memory • Memory to Peripheral • Peripheral to Memory • Peripheral to Peripheral Flow Control can be assigned to the RTK_DMAC, the source peripheral, or the destination peripheral. Table 6-4 lists the decoding for this field. For more information on transfer types and flow control, refer to \"Setup/Operation of RTK_DMAC Transfers\". Reset Value: 3'h0 Dependencies: If hardware handshake isn't support, then TT_FC can only be set to 3'b0. TT_FC\\[2\\] can only be read."]
    #[inline(always)]
    pub fn ctl3_l_tt_fc(&mut self) -> Ctl3LTtFcW<'_, Ctl3LSpec> {
        Ctl3LTtFcW::new(self, 20)
    }
    #[doc = "Bit 27 - Block chanining is enabled on the destination side only if the LLP_DST_EN field id high and LLPx_L.LOC is non-zero; for more information,see 7.4.1.1 \"Block Chaining Using Linked Lists\" Dependencies:This field does not exist if the configuration parameterDMAC_CHx_MULTI_BLK_EN is not selected or if DMAC_CHx_HC_LLP is selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn ctl3_l_llp_dst_en(&mut self) -> Ctl3LLlpDstEnW<'_, Ctl3LSpec> {
        Ctl3LLlpDstEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Block chanining is enabled on the source side only if the LLP_SRC_EN field id high and LLPx_L.LOC is non-zero; for more information,see 7.4.1.1 \"Block Chaining Using Linked Lists\" Dependencies:This field does not exist if the configuration parameterDMAC_CHx_MULTI_BLK_EN is not selected or if DMAC_CHx_HC_LLP is selected; in this case, the read-back value is always 0."]
    #[inline(always)]
    pub fn ctl3_l_llp_src_en(&mut self) -> Ctl3LLlpSrcEnW<'_, Ctl3LSpec> {
        Ctl3LLlpSrcEnW::new(self, 28)
    }
}
#[doc = "Channel 3 low-32 bits Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl3_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl3_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl3LSpec;
impl crate::RegisterSpec for Ctl3LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl3_l::R`](R) reader structure"]
impl crate::Readable for Ctl3LSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl3_l::W`](W) writer structure"]
impl crate::Writable for Ctl3LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL3_L to value 0x4801"]
impl crate::Resettable for Ctl3LSpec {
    const RESET_VALUE: u32 = 0x4801;
}
