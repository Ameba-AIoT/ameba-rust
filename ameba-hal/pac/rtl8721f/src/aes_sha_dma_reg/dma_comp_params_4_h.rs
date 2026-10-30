#[doc = "Register `DMA_COMP_PARAMS_4_H` reader"]
pub type R = crate::R<DmaCompParams4HSpec>;
#[doc = "Field `CH3_DTW` reader - The value of this register is derived from the DMAC_CH3_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch3DtwR = crate::FieldReader;
#[doc = "Field `CH3_STW` reader - The value of this register is derived from the DMAC_CH3_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
pub type Ch3StwR = crate::FieldReader;
#[doc = "Field `CH3_STAT_DST` reader - The value of this register is derived from the DMAC_CH3_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch3StatDstR = crate::BitReader;
#[doc = "Field `CH3_STAT_SRC` reader - The value of this register is derived from the DMAC_CH3_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch3StatSrcR = crate::BitReader;
#[doc = "Field `CH3_DST_SCA_EN` reader - The value of this register is derived from the DMAC_CH3_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch3DstScaEnR = crate::BitReader;
#[doc = "Field `CH3_SRC_GAT_EN` reader - The value of this register is derived from the DMAC_CH3_SRC_GAT_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch3SrcGatEnR = crate::BitReader;
#[doc = "Field `CH3_LOCK_EN` reader - The value of this register is derived from the DMAC_CH3_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch3LockEnR = crate::BitReader;
#[doc = "Field `CH3_MULTI_BLK_EN` reader - The value of this register is derived from the DMAC_CH3_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch3MultiBlkEnR = crate::BitReader;
#[doc = "Field `CH3_CTL_WB_EN` reader - The value of this register is derived from the DMAC_CH3_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch3CtlWbEnR = crate::BitReader;
#[doc = "Field `CH3_HC_LLP` reader - The value of this register is derived from the DMAC_CH3_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
pub type Ch3HcLlpR = crate::BitReader;
#[doc = "Field `CH3_FC` reader - The value of this register is derived from the DMAC_CH3_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
pub type Ch3FcR = crate::FieldReader;
#[doc = "Field `CH3_MAX_MULT_SIZE` reader - The value of this register is derived from the DMAC_CH3_MULT_SIZE parameter. Maxium burst transaction size. CH3_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
pub type Ch3MaxMultSizeR = crate::FieldReader;
#[doc = "Field `CH3_FIFO_DEPTH` reader - The value of this register is derived from the DMAC_CH3_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
pub type Ch3FifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The value of this register is derived from the DMAC_CH3_DTW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch3_dtw(&self) -> Ch3DtwR {
        Ch3DtwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The value of this register is derived from the DMAC_CH3_STW parameter. 0x0 = NO_HARDCODE 0x1 = 8 0x2 = 16 0x3 = 32 0x4 = 64 0x5 = 128 0x6 = 256 0x7 = reserved"]
    #[inline(always)]
    pub fn ch3_stw(&self) -> Ch3StwR {
        Ch3StwR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The value of this register is derived from the DMAC_CH3_STAT_DST parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch3_stat_dst(&self) -> Ch3StatDstR {
        Ch3StatDstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The value of this register is derived from the DMAC_CH3_STAT_SRC parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch3_stat_src(&self) -> Ch3StatSrcR {
        Ch3StatSrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The value of this register is derived from the DMAC_CH3_DST_SCA_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch3_dst_sca_en(&self) -> Ch3DstScaEnR {
        Ch3DstScaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The value of this register is derived from the DMAC_CH3_SRC_GAT_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch3_src_gat_en(&self) -> Ch3SrcGatEnR {
        Ch3SrcGatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The value of this register is derived from the DMAC_CH3_LOCK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch3_lock_en(&self) -> Ch3LockEnR {
        Ch3LockEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The value of this register is derived from the DMAC_CH3_MULTI_BLK_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch3_multi_blk_en(&self) -> Ch3MultiBlkEnR {
        Ch3MultiBlkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The value of this register is derived from the DMAC_CH3_CTL_WB_EN parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch3_ctl_wb_en(&self) -> Ch3CtlWbEnR {
        Ch3CtlWbEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The value of this register is derived from the DMAC_CH3_HC_LLP parameter. 0x0 = FALSE 0x1 = TRUE"]
    #[inline(always)]
    pub fn ch3_hc_llp(&self) -> Ch3HcLlpR {
        Ch3HcLlpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - The value of this register is derived from the DMAC_CH3_FC parameter. 0x0 = DMA 0x1 = SRC 0x2 = DST 0x3 = ANY"]
    #[inline(always)]
    pub fn ch3_fc(&self) -> Ch3FcR {
        Ch3FcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - The value of this register is derived from the DMAC_CH3_MULT_SIZE parameter. Maxium burst transaction size. CH3_MAX_MULT_SIZE = log2(256/DMAC_DATA_WIDTH) 0x3 = 32 bits 0x2 = 64 bits 0x1 = 128 bits"]
    #[inline(always)]
    pub fn ch3_max_mult_size(&self) -> Ch3MaxMultSizeR {
        Ch3MaxMultSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:30 - The value of this register is derived from the DMAC_CH3_FIFO_DEPTH parameter. Default value is 0x0. 0x0 = 8 bytes 0x1 = 16 0x2 = 32 0x3 = 64 0x4 = 128 0x5 = Reserved 0x6 = Reserved 0x7 = Reserved"]
    #[inline(always)]
    pub fn ch3_fifo_depth(&self) -> Ch3FifoDepthR {
        Ch3FifoDepthR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "Refer to the bit table in the description for it\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_comp_params_4_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCompParams4HSpec;
impl crate::RegisterSpec for DmaCompParams4HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_comp_params_4_h::R`](R) reader structure"]
impl crate::Readable for DmaCompParams4HSpec {}
#[doc = "`reset()` method sets DMA_COMP_PARAMS_4_H to value 0x2003_cb1b"]
impl crate::Resettable for DmaCompParams4HSpec {
    const RESET_VALUE: u32 = 0x2003_cb1b;
}
