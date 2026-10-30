#[doc = "Register `DMA_COMP_PARAMS_3_L` reader"]
pub type R = crate::R<DmaCompParams3LSpec>;
#[doc = "Field `CH2_DTW` reader - The value of this register is derived from the DMAC_CH2_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch2DtwR = crate::FieldReader;
#[doc = "Field `CH2_STW` reader - The value of this register is derived from the DMAC_CH2_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch2StwR = crate::FieldReader;
#[doc = "Field `CH2_STAT_DST` reader - The value of this register is derived from the DMAC_CH2_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch2StatDstR = crate::BitReader;
#[doc = "Field `CH2_STAT_SRC` reader - The value of this register is derived from the DMAC_CH2_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch2StatSrcR = crate::BitReader;
#[doc = "Field `CH2_DST_SCA_EN` reader - The value of this register is derived from the DMAC_CH2_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch2DstScaEnR = crate::BitReader;
#[doc = "Field `CH2_SRC_GAT_EN` reader - The value of this register is derived from the DMAC_CH2_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch2SrcGatEnR = crate::BitReader;
#[doc = "Field `CH2_LOCK_EN` reader - The value of this register is derived from the DMAC_CH2_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch2LockEnR = crate::BitReader;
#[doc = "Field `CH2_MULTI_BLK_EN` reader - The value of this register is derived from the DMAC_CH2_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch2MultiBlkEnR = crate::BitReader;
#[doc = "Field `CH2_CTL_WB_EN` reader - The value of this register is derived from the DMAC_CH2_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch2CtlWbEnR = crate::BitReader;
#[doc = "Field `CH2_HC_LLP` reader - The value of this register is derived from the DMAC_CH2_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch2HcLlpR = crate::BitReader;
#[doc = "Field `CH2_FC` reader - The value of this register is derived from the DMAC_CH2_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
pub type Ch2FcR = crate::FieldReader;
#[doc = "Field `CH2_MAX_MULT_SIZE` reader - The value of this register is derived from the DMAC_CH2_MULT_SIZE parameter. Maxium burst transaction size. CH2_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
pub type Ch2MaxMultSizeR = crate::FieldReader;
#[doc = "Field `CH2_FIFO_DEPTH` reader - The value of this register is derived from the DMAC_CH2_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
pub type Ch2FifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The value of this register is derived from the DMAC_CH2_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch2_dtw(&self) -> Ch2DtwR {
        Ch2DtwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The value of this register is derived from the DMAC_CH2_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch2_stw(&self) -> Ch2StwR {
        Ch2StwR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The value of this register is derived from the DMAC_CH2_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch2_stat_dst(&self) -> Ch2StatDstR {
        Ch2StatDstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the DMAC_CH2_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch2_stat_src(&self) -> Ch2StatSrcR {
        Ch2StatSrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the DMAC_CH2_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch2_dst_sca_en(&self) -> Ch2DstScaEnR {
        Ch2DstScaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the DMAC_CH2_SRC_GAT_EN parameter. For a description of this parameter, refer to page 112. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch2_src_gat_en(&self) -> Ch2SrcGatEnR {
        Ch2SrcGatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the DMAC_CH2_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch2_lock_en(&self) -> Ch2LockEnR {
        Ch2LockEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the DMAC_CH2_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch2_multi_blk_en(&self) -> Ch2MultiBlkEnR {
        Ch2MultiBlkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the DMAC_CH2_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch2_ctl_wb_en(&self) -> Ch2CtlWbEnR {
        Ch2CtlWbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the DMAC_CH2_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch2_hc_llp(&self) -> Ch2HcLlpR {
        Ch2HcLlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - The value of this register is derived from the DMAC_CH2_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
    #[inline(always)]
    pub fn ch2_fc(&self) -> Ch2FcR {
        Ch2FcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - The value of this register is derived from the DMAC_CH2_MULT_SIZE parameter. Maxium burst transaction size. CH2_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
    #[inline(always)]
    pub fn ch2_max_mult_size(&self) -> Ch2MaxMultSizeR {
        Ch2MaxMultSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The value of this register is derived from the DMAC_CH2_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
    #[inline(always)]
    pub fn ch2_fifo_depth(&self) -> Ch2FifoDepthR {
        Ch2FifoDepthR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_3_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams3LSpec;
impl crate::RegisterSpec for DmaCompParams3LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_3_l::R`](R) reader structure"]
impl crate::Readable for DmaCompParams3LSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_3_L to value 0x2003_cb1b"]
impl crate::Resettable for DmaCompParams3LSpec {
    const RESET_VALUE: u32 = 0x2003_cb1b;
}
