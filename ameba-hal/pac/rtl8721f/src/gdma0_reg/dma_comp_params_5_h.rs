#[doc = "Register `DMA_COMP_PARAMS_5_H` reader"]
pub type R = crate::R<DmaCompParams5HSpec>;
#[doc = "Field `CH5_DTW` reader - The value of this register is derived from the DMAC_CH5_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch5DtwR = crate::FieldReader;
#[doc = "Field `CH5_STW` reader - The value of this register is derived from the DMAC_CH5_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch5StwR = crate::FieldReader;
#[doc = "Field `CH5_STAT_DST` reader - The value of this register is derived from the DMAC_CH5_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch5StatDstR = crate::BitReader;
#[doc = "Field `CH5_STAT_SRC` reader - The value of this register is derived from the DMAC_CH5_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch5StatSrcR = crate::BitReader;
#[doc = "Field `CH5_DST_SCA_EN` reader - The value of this register is derived from the DMAC_CH5_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch5DstScaEnR = crate::BitReader;
#[doc = "Field `CH5_SRC_GAT_EN` reader - The value of this register is derived from the DMAC_CH5_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch5SrcGatEnR = crate::BitReader;
#[doc = "Field `CH5_LOCK_EN` reader - The value of this register is derived from the DMAC_CH5_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch5LockEnR = crate::BitReader;
#[doc = "Field `CH5_MULTI_BLK_EN` reader - The value of this register is derived from the DMAC_CH5_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch5MultiBlkEnR = crate::BitReader;
#[doc = "Field `CH5_CTL_WB_EN` reader - The value of this register is derived from the DMAC_CH5_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch5CtlWbEnR = crate::BitReader;
#[doc = "Field `CH5_HC_LLP` reader - The value of this register is derived from the DMAC_CH5_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch5HcLlpR = crate::BitReader;
#[doc = "Field `CH5_FC` reader - The value of this register is derived from the DMAC_CH5_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
pub type Ch5FcR = crate::FieldReader;
#[doc = "Field `CH5_MAX_MULT_SIZE` reader - The value of this register is derived from the DMAC_CH5_MULT_SIZE parameter. Maxium burst transaction size. CH5_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
pub type Ch5MaxMultSizeR = crate::FieldReader;
#[doc = "Field `CH5_FIFO_DEPTH` reader - The value of this register is derived from the DMAC_CH5_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
pub type Ch5FifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The value of this register is derived from the DMAC_CH5_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch5_dtw(&self) -> Ch5DtwR {
        Ch5DtwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The value of this register is derived from the DMAC_CH5_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch5_stw(&self) -> Ch5StwR {
        Ch5StwR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The value of this register is derived from the DMAC_CH5_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch5_stat_dst(&self) -> Ch5StatDstR {
        Ch5StatDstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the DMAC_CH5_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch5_stat_src(&self) -> Ch5StatSrcR {
        Ch5StatSrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the DMAC_CH5_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch5_dst_sca_en(&self) -> Ch5DstScaEnR {
        Ch5DstScaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the DMAC_CH5_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch5_src_gat_en(&self) -> Ch5SrcGatEnR {
        Ch5SrcGatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the DMAC_CH5_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch5_lock_en(&self) -> Ch5LockEnR {
        Ch5LockEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the DMAC_CH5_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch5_multi_blk_en(&self) -> Ch5MultiBlkEnR {
        Ch5MultiBlkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the DMAC_CH5_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch5_ctl_wb_en(&self) -> Ch5CtlWbEnR {
        Ch5CtlWbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the DMAC_CH5_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch5_hc_llp(&self) -> Ch5HcLlpR {
        Ch5HcLlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - The value of this register is derived from the DMAC_CH5_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
    #[inline(always)]
    pub fn ch5_fc(&self) -> Ch5FcR {
        Ch5FcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - The value of this register is derived from the DMAC_CH5_MULT_SIZE parameter. Maxium burst transaction size. CH5_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
    #[inline(always)]
    pub fn ch5_max_mult_size(&self) -> Ch5MaxMultSizeR {
        Ch5MaxMultSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The value of this register is derived from the DMAC_CH5_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
    #[inline(always)]
    pub fn ch5_fifo_depth(&self) -> Ch5FifoDepthR {
        Ch5FifoDepthR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_5_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams5HSpec;
impl crate::RegisterSpec for DmaCompParams5HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_5_h::R`](R) reader structure"]
impl crate::Readable for DmaCompParams5HSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_5_H to value 0x2003_c81b"]
impl crate::Resettable for DmaCompParams5HSpec {
    const RESET_VALUE: u32 = 0x2003_c81b;
}
