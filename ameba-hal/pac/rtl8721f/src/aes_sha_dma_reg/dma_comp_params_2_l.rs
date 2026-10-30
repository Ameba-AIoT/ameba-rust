#[doc = "Register `DMA_COMP_PARAMS_2_L` reader"]
pub type R = crate::R<DmaCompParams2LSpec>;
#[doc = "Field `CH0_DTW` reader - The value of this register is derived from the DMAC_CH0_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch0DtwR = crate::FieldReader;
#[doc = "Field `CH0_STW` reader - The value of this register is derived from the DMAC_CH0_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch0StwR = crate::FieldReader;
#[doc = "Field `CH0_STAT_DST` reader - The value of this register is derived from the DMAC_CH0_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch0StatDstR = crate::BitReader;
#[doc = "Field `CH0_STAT_SRC` reader - The value of this register is derived from the DMAC_CH0_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch0StatSrcR = crate::BitReader;
#[doc = "Field `CH0_DST_SCA_EN` reader - The value of this register is derived from the DMAC_CH0_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch0DstScaEnR = crate::BitReader;
#[doc = "Field `CH0_SRC_GAT_EN` reader - The value of this register is derived from the DMAC_CH0_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch0SrcGatEnR = crate::BitReader;
#[doc = "Field `CH0_LOCK_EN` reader - The value of this register is derived from the DMAC_CH0_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch0LockEnR = crate::BitReader;
#[doc = "Field `CH0_MULTI_BLK_EN` reader - The value of this register is derived from the DMAC_CH0_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch0MultiBlkEnR = crate::BitReader;
#[doc = "Field `CH0_CTL_WB_EN` reader - The value of this register is derived from the DMAC_CH0_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch0CtlWbEnR = crate::BitReader;
#[doc = "Field `CH0_HC_LLP` reader - The value of this register is derived from the DMAC_CH0_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch0HcLlpR = crate::BitReader;
#[doc = "Field `CH0_FC` reader - The value of this register is derived from the DMAC_CH0_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
pub type Ch0FcR = crate::FieldReader;
#[doc = "Field `CH0_MAX_MULT_SIZE` reader - The value of this register is derived from the DMAC_CH0_MULT_SIZE parameter. Maxium burst transaction size. CH0_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
pub type Ch0MaxMultSizeR = crate::FieldReader;
#[doc = "Field `CH0_FIFO_DEPTH` reader - The value of this register is derived from the DMAC_CH0_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
pub type Ch0FifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The value of this register is derived from the DMAC_CH0_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch0_dtw(&self) -> Ch0DtwR {
        Ch0DtwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The value of this register is derived from the DMAC_CH0_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch0_stw(&self) -> Ch0StwR {
        Ch0StwR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The value of this register is derived from the DMAC_CH0_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch0_stat_dst(&self) -> Ch0StatDstR {
        Ch0StatDstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the DMAC_CH0_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch0_stat_src(&self) -> Ch0StatSrcR {
        Ch0StatSrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the DMAC_CH0_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch0_dst_sca_en(&self) -> Ch0DstScaEnR {
        Ch0DstScaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the DMAC_CH0_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch0_src_gat_en(&self) -> Ch0SrcGatEnR {
        Ch0SrcGatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the DMAC_CH0_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch0_lock_en(&self) -> Ch0LockEnR {
        Ch0LockEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the DMAC_CH0_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch0_multi_blk_en(&self) -> Ch0MultiBlkEnR {
        Ch0MultiBlkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the DMAC_CH0_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch0_ctl_wb_en(&self) -> Ch0CtlWbEnR {
        Ch0CtlWbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the DMAC_CH0_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch0_hc_llp(&self) -> Ch0HcLlpR {
        Ch0HcLlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - The value of this register is derived from the DMAC_CH0_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
    #[inline(always)]
    pub fn ch0_fc(&self) -> Ch0FcR {
        Ch0FcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - The value of this register is derived from the DMAC_CH0_MULT_SIZE parameter. Maxium burst transaction size. CH0_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
    #[inline(always)]
    pub fn ch0_max_mult_size(&self) -> Ch0MaxMultSizeR {
        Ch0MaxMultSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The value of this register is derived from the DMAC_CH0_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
    #[inline(always)]
    pub fn ch0_fifo_depth(&self) -> Ch0FifoDepthR {
        Ch0FifoDepthR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_2_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams2LSpec;
impl crate::RegisterSpec for DmaCompParams2LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_2_l::R`](R) reader structure"]
impl crate::Readable for DmaCompParams2LSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_2_L to value 0x4003_cb1b"]
impl crate::Resettable for DmaCompParams2LSpec {
    const RESET_VALUE: u32 = 0x4003_cb1b;
}
