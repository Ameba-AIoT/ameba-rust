#[doc = "Register `DMA_COMP_PARAMS_6_H` reader"]
pub type R = crate::R<DmaCompParams6HSpec>;
#[doc = "Field `CH7_DTW` reader - The value of this register is derived from the DMAC_CH7_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch7DtwR = crate::FieldReader;
#[doc = "Field `CH7_STW` reader - The value of this register is derived from the DMAC_CH7_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch7StwR = crate::FieldReader;
#[doc = "Field `CH7_STAT_DST` reader - The value of this register is derived from the DMAC_CH7_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch7StatDstR = crate::BitReader;
#[doc = "Field `CH7_STAT_SRC` reader - The value of this register is derived from the DMAC_CH7_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch7StatSrcR = crate::BitReader;
#[doc = "Field `CH7_DST_SCA_EN` reader - The value of this register is derived from the DMAC_CH7_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch7DstScaEnR = crate::BitReader;
#[doc = "Field `CH7_SRC_GAT_EN` reader - The value of this register is derived from the DMAC_CH7_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch7SrcGatEnR = crate::BitReader;
#[doc = "Field `CH7_LOCK_EN` reader - The value of this register is derived from the DMAC_CH7_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch7LockEnR = crate::BitReader;
#[doc = "Field `CH7_MULTI_BLK_EN` reader - The value of this register is derived from the DMAC_CH7_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch7MultiBlkEnR = crate::BitReader;
#[doc = "Field `CH7_CTL_WB_EN` reader - The value of this register is derived from the DMAC_CH7_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch7CtlWbEnR = crate::BitReader;
#[doc = "Field `CH7_HC_LLP` reader - The value of this register is derived from the DMAC_CH7_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch7HcLlpR = crate::BitReader;
#[doc = "Field `CH7_FC` reader - The value of this register is derived from the DMAC_CH7_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
pub type Ch7FcR = crate::FieldReader;
#[doc = "Field `CH7_MAX_MULT_SIZE` reader - The value of this register is derived from the DMAC_CH7_MULT_SIZE parameter. Maxium burst transaction size. CH7_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
pub type Ch7MaxMultSizeR = crate::FieldReader;
#[doc = "Field `CH7_FIFO_DEPTH` reader - The value of this register is derived from the DMAC_CH7_FIFO_DEPTH parameter. Default value is 0x0. CH7_FIFO_DEPTH = log2(DMAC_CH7_FIFO_DEPTH/8) 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
pub type Ch7FifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The value of this register is derived from the DMAC_CH7_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch7_dtw(&self) -> Ch7DtwR {
        Ch7DtwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The value of this register is derived from the DMAC_CH7_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch7_stw(&self) -> Ch7StwR {
        Ch7StwR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The value of this register is derived from the DMAC_CH7_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch7_stat_dst(&self) -> Ch7StatDstR {
        Ch7StatDstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the DMAC_CH7_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch7_stat_src(&self) -> Ch7StatSrcR {
        Ch7StatSrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the DMAC_CH7_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch7_dst_sca_en(&self) -> Ch7DstScaEnR {
        Ch7DstScaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the DMAC_CH7_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch7_src_gat_en(&self) -> Ch7SrcGatEnR {
        Ch7SrcGatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the DMAC_CH7_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch7_lock_en(&self) -> Ch7LockEnR {
        Ch7LockEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the DMAC_CH7_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch7_multi_blk_en(&self) -> Ch7MultiBlkEnR {
        Ch7MultiBlkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the DMAC_CH7_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch7_ctl_wb_en(&self) -> Ch7CtlWbEnR {
        Ch7CtlWbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the DMAC_CH7_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch7_hc_llp(&self) -> Ch7HcLlpR {
        Ch7HcLlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - The value of this register is derived from the DMAC_CH7_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
    #[inline(always)]
    pub fn ch7_fc(&self) -> Ch7FcR {
        Ch7FcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - The value of this register is derived from the DMAC_CH7_MULT_SIZE parameter. Maxium burst transaction size. CH7_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
    #[inline(always)]
    pub fn ch7_max_mult_size(&self) -> Ch7MaxMultSizeR {
        Ch7MaxMultSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The value of this register is derived from the DMAC_CH7_FIFO_DEPTH parameter. Default value is 0x0. CH7_FIFO_DEPTH = log2(DMAC_CH7_FIFO_DEPTH/8) 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
    #[inline(always)]
    pub fn ch7_fifo_depth(&self) -> Ch7FifoDepthR {
        Ch7FifoDepthR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_6_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams6HSpec;
impl crate::RegisterSpec for DmaCompParams6HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_6_h::R`](R) reader structure"]
impl crate::Readable for DmaCompParams6HSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_6_H to value 0x2003_c81b"]
impl crate::Resettable for DmaCompParams6HSpec {
    const RESET_VALUE: u32 = 0x2003_c81b;
}
